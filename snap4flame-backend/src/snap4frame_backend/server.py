import asyncio
import json
import os
import typing

import aiohttp
from aiohttp import web
from aiohttp.http_websocket import WSMessage
from dotenv import load_dotenv
from google.api_core.exceptions import NotFound
from google.cloud import ndb, pubsub_v1
from google.cloud.pubsub_v1.types import Subscription

from snap4frame_backend import log as logging
from snap4frame_backend.metaclass import Singleton

load_dotenv()

ClientId = str
ProjectToken = str


class report(ndb.Model):  # noqa: N801
    # disabled because it's not used & have convention issues
    # timestamp = ndb.DateTimeProperty(auto_now=True)
    report = ndb.TextProperty()
    resolved = ndb.BooleanProperty(default=False)
    token = ndb.StringProperty()

    def to_dict(self):
        result = super().to_dict()
        result['id'] = self.key.id()
        return result

class PubSubWorker(metaclass=Singleton):
    # map of active subscriptions. contains as key the project_token and as value the listener_id
    map_subscriptions: typing.Dict[
        ProjectToken, typing.Dict[ClientId, Subscription]
    ] = {}
    # map of listener_id to project_token
    interest_map: typing.Dict[ClientId, ProjectToken] = {}

    def __init__(self, project: str, topic: str):
        self.project = project
        self.topic = topic
        self.subscriber = pubsub_v1.SubscriberClient()

    def bind_project4client(
        self, project_token: str, callback: typing.Callable, client_id: str
    ):
        if client_id in self.map_subscriptions.get(project_token, {}):
            return self.map_subscriptions[project_token][client_id]  # already bound
        self.unbind_project4client(client_id)

        subscription_path = self.get_subscription(project_token)
        subscription = self.subscriber.subscribe(subscription_path, callback=callback)
        self.register_client(project_token, client_id, subscription)

    def unbind_project4client(self, client_id: str):
        old_project_token, old_subscription = self.unregister_client(client_id)

        if not old_subscription:
            return

        if len(self.map_subscriptions.get(old_project_token, {})):
            old_subscription.cancel()
            return

        # Delete the subscription using the SubscriberClient
        subscription_path = self.get_subscription_path(old_project_token)
        self.subscriber.delete_subscription({"subscription": subscription_path})

    def register_client(
        self, project_token: str, client_id: str, subscription: Subscription
    ):
        self.interest_map[client_id] = project_token
        if project_token not in self.map_subscriptions:
            self.map_subscriptions[project_token] = {}
        self.map_subscriptions[project_token][client_id] = subscription

    def unregister_client(
        self, client_id: str
    ) -> typing.Tuple[ProjectToken, Subscription]:
        project_token = self.interest_map.pop(client_id, None)
        active_subscription = self.map_subscriptions.get(project_token, {}).pop(
            client_id, None
        )
        return project_token, active_subscription

    def get_subscription_path(self, project_token: str):
        # maybe use the project_token as subscription name
        return self.subscriber.subscription_path(
            self.project, f"project-{project_token}"
        )

    def get_subscription(self, project_token: str):
        assert self.subscriber

        subscription_path = self.get_subscription_path(project_token)

        try:
            self.subscriber.get_subscription({"subscription": subscription_path})
            return subscription_path
        except NotFound:
            topic_path = self.subscriber.topic_path(self.project, self.topic)
            self.subscriber.create_subscription(
                request={
                    "name": subscription_path,
                    "topic": topic_path,
                    "filter": f'attributes.token="{project_token}"',
                },
            )
            return subscription_path
        except Exception as e:
            logging.exception("Error getting subscription: %s", e)
            # TODO see if the error is related to the subscription creation
            # topic_path = self.subscriber.topic_path(self.project, self.topic)
            # ret = self.subscriber.create_subscription(
            #     request={
            #         "name": subscription_path,
            #         "topic": topic_path,
            #         "filter": f'attributes.token="{project_token}"',
            #     },
            # )
            # return subscription_path
            return None


class ClientListener:
    def __init__(  # noqa: PLR0913
        self,
        websocket: web.WebSocketResponse,
        project_token: str = "",
        pubsub_worker: PubSubWorker = None,
        db_client: ndb.Client = None,
        event_loop: asyncio.AbstractEventLoop = None,
    ):
        self.websocket = websocket
        self.project_token = project_token
        self.pubsub_worker = pubsub_worker
        self.db_client = db_client
        self.client_id = id(self)
        self.event_loop = event_loop or asyncio.get_event_loop()

    def get_report(self, report_id: str):
        with self.db_client.context():
            entity = report.get_by_id(int(report_id))
            # key = ndb.Key("report", int(report_id))
            # entity = key.get()
            if not entity:
                print(f"Report with ID {report_id} not found")
                return
            return entity

    def sync_on_pubsub_message(self, message: pubsub_v1.subscriber.message.Message):
        self.event_loop.run_until_complete(self.on_pubsub_message(message))

    async def on_pubsub_message(self, message: pubsub_v1.subscriber.message.Message):
        logging.info(f"Received message: {message}")
        data = json.loads(message.data)
        report_id = data.get("reportID")
        message.ack()

        if not report_id:
            logging.warning(f"Invalid message: {data}")
            return

        entity = self.get_report(report_id)
        if not entity:
            return

        client_message = {
            "type": "entity",
            "data": {"report": entity.to_dict()},
        }

        await self.websocket.send_json(client_message)

    async def process_bind_project(self, params: dict):
        self.pubsub_worker.bind_project4client(
            params["project_token"],
            self.sync_on_pubsub_message,
            self.client_id,
        )

        # send the client the current state of the project (all reports)
        with self.db_client.context():
            reports = report.query(report.token == params["project_token"]).fetch()
        client_message = {
            "type": "all_reports",
            "data": {"reports": [r.to_dict() for r in reports]},
        }

        await self.websocket.send_json(client_message)

    async def process_report_solved(self, params: dict):
        report_id = params.get("report_id")
        if not report_id:
            print(f"Invalid message: {params}")
            return

        entity = self.get_report(report_id)
        if not entity:
            return

        entity.resolved = True
        with self.db_client.context():
            entity.put()

    async def on_client_message(self, message: WSMessage):
        print(f"Received message: {message}")
        data = json.loads(message.data)

        if not all(k in data for k in ("action", "params")):
            print(f"Invalid message: {data}")
            return

        action = data["action"]
        params = data["params"]

        if action == "bind_project":
            # Bind the project to the websocket
            return await self.process_bind_project(params)
        elif action == "report_solved":
            # Bind the project to the websocket
            return await self.process_report_solved(params)
        else:
            print(f"Unknown action: {action}")

    def __enter__(self):
        pass

    def __exit__(self, exc_type, exc_val, exc_tb):
        self.pubsub_worker.unbind_project4client(self.client_id)
        pass


class WebSocketServer:
    def __init__(self, project: str, topic: str = "", ws_timeout: int = 60):
        self.app = web.Application()
        self.app.router.add_get("/ws", self.websocket_handler)

        self.ws_timeout = ws_timeout
        self.event_loop = asyncio.get_event_loop()

        self.pubsub_worker = PubSubWorker(project=project, topic=topic)
        self.db_client = ndb.Client()

    async def websocket_handler(self, request):
        ws = web.WebSocketResponse()
        await ws.prepare(request)

        client = ClientListener(
            websocket=ws,
            pubsub_worker=self.pubsub_worker,
            db_client=self.db_client,
            event_loop=self.event_loop,
        )

        with client:
            while True:
                msg = await ws.receive(self.ws_timeout)
                if msg.type == aiohttp.WSMsgType.TEXT:
                    asyncio.create_task(client.on_client_message(msg))
                elif msg.type == aiohttp.WSMsgType.ERROR:
                    print(
                        f"WebSocket connection closed with exception {ws.exception()}"
                    )
                elif msg.type in (
                    aiohttp.WSMsgType.CLOSE,
                    aiohttp.WSMsgType.CLOSING,
                    aiohttp.WSMsgType.CLOSED,
                ):
                    break
                elif msg.type == aiohttp.WSMsgType.PING:
                    await ws.pong()
                elif msg.type == aiohttp.WSMsgType.PONG:
                    print("Pong message received")
        return ws

    def run(self, host="localhost", port=80):
        web.run_app(self.app, host=host, port=port)


if __name__ == "__main__":
    server = WebSocketServer(
        project=os.getenv("GOOGLE_CLOUD_PROJECT"),
        topic=os.getenv("PUBSUB_TOPIC_ID"),
    )
    server.run()

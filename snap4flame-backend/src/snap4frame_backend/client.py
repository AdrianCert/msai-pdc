import asyncio
import json

import websockets


async def hello():
    # uri = "ws://34.154.51.251/ws"
    uri = "ws://localhost:80/ws"
    async with websockets.connect(uri) as websocket:
        await websocket.send(
            json.dumps(
                # {
                #     "action": "report_solved",
                #     "params": {"report_id": "5071651448291328"},
                # }
                {
                    "action": "bind_project",
                    "params": {"project_token": "31f6de6f-dc87-48ea-8a67-c54767cb1bfe"},
                }
            )
        )
        while True:
            try:
                response = await websocket.recv()
                print(f"Received: {response}")
            except Exception as e:
                print(f"Error receiving message: {e}")
                break


asyncio.get_event_loop().run_until_complete(hello())

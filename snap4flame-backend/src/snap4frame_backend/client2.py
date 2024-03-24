import asyncio
import json

import websockets


async def receive_json(uri, output_file):
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
                message = await websocket.recv()
                data = json.loads(message)
                with open(output_file, 'a') as f:
                    json.dump(data, f)
                    f.write('\n')
            except websockets.exceptions.ConnectionClosed:
                print("Connection with server closed")
                break
            except Exception as e:
                print(f"Error: {e}")
                break

asyncio.run(receive_json('ws://localhost:80/ws', 'output.json'))
from fastapi import WebSocket
from typing import List, Dict

from loguru import logger


class WebsocketConnectionManager:
    _instance = None

    def __init__(self):
        self.active_connections: List[WebSocket] = []


    async def connect(self, websocket: WebSocket):
        await websocket.accept()
        self.active_connections.append(websocket)

    def disconnect(self, websocket: WebSocket):
        self.active_connections.remove(websocket)

    async def broadcast(self, message: Dict | str):
        for connection in self.active_connections:
            await connection.send_json(message)

    async def terminate_connections(self):
        termination_message = {"type": "termination", "message": "Server is shutting down."}
        for connection in self.active_connections:
            try:
                await connection.send_json(termination_message)
                await connection.close()
            except Exception as e:
                logger.info(f"Error terminating connection: {e}")
        self.active_connections.clear()

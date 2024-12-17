from fastapi import APIRouter, WebSocket, WebSocketDisconnect
from loguru import logger

from server.app.globals import db, ws_conn
from server.app.services.agents import graph_data

websocket_router = APIRouter()


@websocket_router.websocket("/ws")
async def websocket_endpoint(websocket: WebSocket):
    await ws_conn.connect(websocket)

    try:
        while True:
            data = await websocket.receive_json()
            logger.info(f"Received from frontend: {data}")
            match data.get("action"):
                case "get_graph":
                    logger.info(f"Sending graph data: {graph_data}")
                    await websocket.send_json(graph_data)
                case "get_logs":
                    logs = db.get_all_logs()
                    logger.info(f"Sending logs: {logs}")
                    await websocket.send_json({"logs": logs})

    except WebSocketDisconnect:
        ws_conn.disconnect(websocket)
        logger.info("Frontend disconnected")

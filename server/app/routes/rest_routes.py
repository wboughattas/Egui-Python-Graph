from server.app.globals import ws_conn, db
from server.app.services.agents import graph_data
from server.app.services.models import Node
from fastapi import APIRouter

rest_router = APIRouter()


@rest_router.post("/update_traversal/")
async def update_traversal(node: Node):
    graph_data["current_node"] = node.id
    await ws_conn.broadcast(graph_data)

    log_message = f"Current Node: {graph_data['current_node']}"
    db.add_log(log_message)
    await ws_conn.broadcast({"log": log_message})

    return {"message": "Traversal updated", "current_node": graph_data["current_node"]}

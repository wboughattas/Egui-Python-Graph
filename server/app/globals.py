from server.app.manager.connection_manager import WebsocketConnectionManager
from server.app.services.database import Database

db = Database(db_path="logs.db")
ws_conn = WebsocketConnectionManager()

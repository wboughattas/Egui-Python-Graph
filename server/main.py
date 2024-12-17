import uvicorn
from fastapi import FastAPI
from contextlib import asynccontextmanager
from loguru import logger
from starlette.responses import RedirectResponse

from server.app.globals import db, ws_conn
from server.app.routes.websocket_routes import websocket_router
from server.app.routes.rest_routes import rest_router


@asynccontextmanager
async def lifespan(app: FastAPI):  # noqa
    db.connect()
    logger.info("Database connected.")
    try:
        yield
    finally:
        logger.info("Shutting down WebSocket connections...")
        await ws_conn.terminate_connections()
        logger.info("WebSocket connections terminated.")
        db.disconnect()
        logger.info("Database disconnected.")


app = FastAPI(lifespan=lifespan)
app.include_router(websocket_router)
app.include_router(rest_router)


@app.get("/")
async def root():
    return RedirectResponse(url="/docs")


if __name__ == "__main__":
    uvicorn.run(app, host="0.0.0.0", port=8000)

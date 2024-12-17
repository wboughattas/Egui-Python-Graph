import sqlite3


class Database:
    _instance = None

    def __init__(self, db_path: str):
        self.db_path = db_path
        self.connection = None

    def connect(self):
        self.connection = sqlite3.connect(self.db_path, check_same_thread=False)
        self.connection.execute(
            "CREATE TABLE IF NOT EXISTS logs (id INTEGER PRIMARY KEY, message TEXT)"  # noqa
        )
        self.connection.commit()

    def disconnect(self):
        if self.connection:
            self.connection.close()
            self.connection = None

    def add_log(self, message: str):
        if not self.connection:
            raise RuntimeError("Database is not connected.")
        cursor = self.connection.cursor()
        cursor.execute("INSERT INTO logs (message) VALUES (?)", (message,))  # noqa
        self.connection.commit()

    def get_all_logs(self):
        if not self.connection:
            raise RuntimeError("Database is not connected.")
        cursor = self.connection.cursor()
        cursor.execute("SELECT message FROM logs ORDER BY id")  # noqa
        return [row[0] for row in cursor.fetchall()]

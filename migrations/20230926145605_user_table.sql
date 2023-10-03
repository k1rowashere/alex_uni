CREATE TABLE IF NOT EXISTS
  users (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    username TEXT UNIQUE NOT NULL,
    password TEXT NOT NULL,
    email TEXT NOT NULL,
    created_at DATETIME NOT NULL DEFAULT (datetime('now')),
    name TEXT NOT NULL
  );

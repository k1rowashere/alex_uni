CREATE TABLE IF NOT EXISTS
  programs (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    code TEXT NOT NULL,
    by_law INTEGER NOT NULL
)

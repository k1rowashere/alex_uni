CREATE TABLE IF NOT EXISTS
  subjects (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    code TEXT NOT NULL,
    level INTEGER NOT NULL,
    credit INTEGER NOT NULL,
    pre_req TEXT NOT NULL
  );

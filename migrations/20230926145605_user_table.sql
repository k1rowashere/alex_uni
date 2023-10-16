CREATE TABLE IF NOT EXISTS
  users (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    username TEXT UNIQUE NOT NULL,
    password TEXT NOT NULL,
    email TEXT NOT NULL,
    name TEXT NOT NULL,
    created_at DATETIME NOT NULL DEFAULT (datetime('now')),
    user_type TEXT NOT NULL DEFAULT 'student' CHECK (user_type IN ('student', 'prof', 'admin')),
    profile_id INTEGER REFERENCES student_profile (id)
  );

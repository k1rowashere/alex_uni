CREATE TABLE
  IF NOT EXISTS completed (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    student_id INTEGER NOT NULL REFERENCES students (id),
    subject_id INTEGER NOT NULL REFERENCES subjects (id),
    completed_on TEXT NOT NULL,
    term_no INTEGER NOT NULL,
    term_abs TEXT NOT NULL,
    gpa REAL NOT NULL
  ) STRICT;

CREATE TABLE IF NOT EXISTS
  term_subscribers (
    student_id INTEGER NOT NULL REFERENCES users(id),
    term_subject_id INTEGER NOT NULL REFERENCES term_subjects(id)
  ) STRICT;

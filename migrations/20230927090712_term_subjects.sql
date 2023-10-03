CREATE TABLE IF NOT EXISTS
  term_subjects (
    id INTEGER PRIMARY KEY,
    max_count INTEGER NOT NULL,
    group_no INTEGER NOT NULL,
    sec_no INTEGER NOT NULL CHECK (sec_no BETWEEN 1 AND 2),
    subject_id INTEGER NOT NULL REFERENCES subjects (id),
    prof_id INTEGER NOT NULL REFERENCES professors (id),
    lec_id INTEGER NOT NULL REFERENCES classes (id),
    tut_id INTEGER REFERENCES classes (id),
    lab_id INTEGER REFERENCES classes (id)
  ) STRICT;

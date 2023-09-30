CREATE TABLE IF NOT EXISTS
  classes (
    id INTEGER PRIMARY KEY,
    type TEXT NOT NULL,
    day_of_week INTEGER NOT NULL,
    week_parity INTEGER,
    period_start INTEGER NOT NULL,
    period_end INTEGER NOT NULL,
    subject_id INTEGER NOT NULL REFERENCES subjects (id),
    location_id INTEGER NOT NULL REFERENCES locations (id)
  ) STRICT;

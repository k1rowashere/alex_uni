CREATE TABLE IF NOT EXISTS
  classes (
    id INTEGER PRIMARY KEY,
    type TEXT NOT NULL CHECK (type IN ('lec', 'tut', 'lab')),
    day_of_week INTEGER NOT NULL CHECK (day_of_week BETWEEN 0 AND 6),
    week_parity INTEGER CHECK (week_parity IN (0, 1))
    period_start INTEGER NOT NULL CHECK (period_start BETWEEN 0 AND 11),
    period_end INTEGER NOT NULL CHECK (period_start BETWEEN 0 AND 11),
    subject_id INTEGER NOT NULL REFERENCES subjects (id),
    location_id INTEGER NOT NULL REFERENCES locations (id)
  ) STRICT;

CREATE TABLE
  IF NOT EXISTS classes (
    id INTEGER PRIMARY KEY,
    type TEXT NOT NULL CHECK (type IN ('lec', 'tut', 'lab')),
    day_of_week TEXT NOT NULL CHECK (
      day_of_week IN (
        'monday',
        'tuesday',
        'wednesday',
        'thursday',
        'friday',
        'saturday',
        'sunday'
      )
    ),
    week_parity TEXT NOT NULL DEFAULT 'both' CHECK (week_parity IN ('odd', 'even', 'both')),
    period_start INTEGER NOT NULL CHECK (period_start BETWEEN 0 AND 11),
    period_end INTEGER NOT NULL CHECK (period_start BETWEEN 0 AND 11),
    subject_id INTEGER NOT NULL REFERENCES subjects (id),
    location_id INTEGER NOT NULL REFERENCES locations (id)
  ) STRICT;

CREATE TABLE IF NOT EXISTS
  locations (
    id INTEGER PRIMARY KEY,
    building TEXT NOT NULL CHECK (building IN ("electricity", "mechanics", "preparatory_south", "preparatory_north", "ssp")),
    floor INTEGER NOT NULL,
    room TEXT NOT NULL
  ) STRICT;

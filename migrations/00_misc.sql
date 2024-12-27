CREATE TYPE building AS ENUM (
    'electricity',
    'mechanics',
    'preparatory_south',
    'preparatory_north',
    'ssp'
    );
CREATE TYPE term_season AS ENUM (
    'fall',
    'spring',
    'summer'
    );

CREATE TYPE term AS (
    year INTEGER ,
    season term_season
);


CREATE TABLE programs (
  id INTEGER PRIMARY KEY,
  name TEXT NOT NULL,
  code TEXT NOT NULL,
  by_law INTEGER NOT NULL
);


CREATE TABLE locations (
  id INTEGER PRIMARY KEY,
  building BUILDING NOT NULL,
  floor INTEGER NOT NULL,
  room TEXT NOT NULL
);

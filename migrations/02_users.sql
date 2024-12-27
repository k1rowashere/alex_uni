CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  username TEXT UNIQUE NOT NULL,
  password TEXT NOT NULL,
  email TEXT NOT NULL,
  name TEXT NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT now()
);

CREATE TABLE user_roles (
  id INTEGER NOT NULL REFERENCES users,
  role TEXT NOT NULL,
  PRIMARY KEY (id, role)
);

CREATE TABLE students (
  id SERIAL PRIMARY KEY REFERENCES users,
  student_id INTEGER NOT NULL UNIQUE,
  program_id INTEGER NOT NULL REFERENCES programs,
  name_en TEXT NOT NULL,
  name_ar TEXT NOT NULL,
  nationality TEXT NOT NULL,
  gender TEXT,
  birth_date DATE,
  birth_place TEXT,
  national_id TEXT,
  passport_no TEXT,
  city TEXT,
  address TEXT,
  phone_no TEXT,
  mobile_no TEXT,
  email TEXT,
  --
  prev_school TEXT,
  prev_qualification TEXT,
  prev_graduation_year INTEGER,
  prev_score INTEGER,
  prev_percent REAL,
  --
  guardian_name TEXT,
  guardian_occupation TEXT,
  guardian_phone_no TEXT,
  guardian_mobile_no TEXT,
  guardian_email TEXT,
  guardian_address TEXT
);

CREATE TABLE professors (
  id SERIAL PRIMARY KEY REFERENCES users,
  name TEXT NOT NULL
);

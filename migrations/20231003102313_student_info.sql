CREATE TABLE IF NOT EXISTS
  student_profile (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    name_en TEXT NOT NULL,
    name_ar TEXT NOT NULL,
    program_id INTEGER NOT NULL REFERENCES programs (id),
    nationality TEXT NOT NULL,
    gender TEXT,
    birth_date DATE,
    birth_place TEXT,
    national_id TEXT,
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

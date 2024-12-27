 CREATE TABLE course_grades (
  id INTEGER PRIMARY KEY,
  term_course_id INTEGER REFERENCES term_courses,
  requirement TEXT,
  requirement_type TEXT NOT NULL,
  max_score INTEGER NOT NULL
);

 CREATE TABLE student_grades (
  student_id INTEGER REFERENCES students,
  grade_id INTEGER REFERENCES course_grades,
  score DECIMAL(2, 2),
  PRIMARY KEY (student_id, grade_id)
);

 CREATE TABLE completed (
  student_id INTEGER NOT NULL REFERENCES students,
  course_id INTEGER NOT NULL REFERENCES courses,
  term_year INTEGER NOT NULL,
  term_season term_season NOT NULL,
  completed_on TIMESTAMP NOT NULL DEFAULT now(),
  -- grades TEXT
  score INTEGER NOT NULL,
  PRIMARY KEY (student_id, course_id, term_year, term_season)
);

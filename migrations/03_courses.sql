 CREATE TABLE courses (
  id INTEGER PRIMARY KEY,
  name TEXT NOT NULL,
  code TEXT NOT NULL,
  level INTEGER NOT NULL,
  credit INTEGER NOT NULL
);

 CREATE TABLE course_prerequisites (
  course_id INTEGER NOT NULL REFERENCES courses,
  prerequisite_id INTEGER NOT NULL REFERENCES courses,
  PRIMARY KEY (course_id, prerequisite_id)
);

 CREATE TABLE classes (
  id INTEGER PRIMARY KEY,
  day_of_week TEXT CHECK (
    day_of_week IN ('mon', 'tue', 'wed', 'thu', 'fri', 'sat', 'sun')
  ),
  week_parity TEXT DEFAULT 'both' CHECK (week_parity IN ('odd', 'even', 'both')),
  period_start INTEGER CHECK (period_start BETWEEN 0 AND 11),
  period_end INTEGER NOT NULL CHECK (period_start BETWEEN 0 AND 11),
  type TEXT CHECK (type IN ('lec', 'tut', 'lab')),
  course_id INTEGER NOT NULL REFERENCES courses,
  location_id INTEGER NOT NULL REFERENCES locations,
  UNIQUE (
    day_of_week,
    week_parity,
    period_start,
    location_id
  )
);

 CREATE TABLE term_courses (
  id INTEGER PRIMARY KEY,
  group_no INTEGER,
  sec_no INTEGER,
  max_seats INTEGER,
  course_id INTEGER NOT NULL REFERENCES courses,
  prof_id INTEGER NOT NULL REFERENCES professors,
  lec_id INTEGER NOT NULL REFERENCES classes,
  tut_id INTEGER REFERENCES classes,
  lab_id INTEGER REFERENCES classes,
  UNIQUE (group_no, sec_no, course_id)
);

 CREATE TABLE course_enrollment (
  student_id INTEGER REFERENCES students,
  term_course_id INTEGER REFERENCES term_courses,
  enrolled_at TIMESTAMP,
  withdrawn_at TIMESTAMP,
  PRIMARY KEY (student_id, term_course_id)
);

-- View to display classes with all relevant information
-- For backend use
CREATE VIEW classes_view AS
SELECT DISTINCT
  classes.id,
  classes.type as ctype,
  professors.name as prof,
  courses.name,
  courses.code,
  locations.building as "building: Building",
  locations.floor,
  locations.room,
  classes.day_of_week as "day_of_week: DayOfWeek",
  classes.period_start,
  classes.period_end,
  classes.week_parity as "week_parity: WeekParity",
  term_courses.sec_no as "section: Section"
FROM
  term_courses
  JOIN classes ON term_courses.lec_id = classes.id
  OR term_courses.lab_id = classes.id
  OR term_courses.tut_id = classes.id
  JOIN courses ON classes.course_id = courses.id
  JOIN locations ON classes.location_id = locations.id
  JOIN professors ON term_courses.prof_id = professors.id;

--
--
-- CREATE TRIGGER validate_class_type BEFORE INSERT ON term_courses BEGIN
-- -- Ensure that the class type is valid
--    SELECT RAISE (FAIL, 'Invalid class type')
--     WHERE (
--           EXISTS (
--              SELECT COUNT(*)
--                FROM classes
--               WHERE id = NEW.lec_id
--                 AND type != 'lec'
--                  OR id = NEW.tut_id
--                 AND type != 'tut'
--                  OR id = NEW.lab_id
--                 AND type != 'lab'
--           )
--           );
--
-- END;
--
-- CREATE TRIGGER validate_prerequisites BEFORE INSERT ON course_enrollment BEGIN
-- -- Ensure the student has met all prerequisites
--    SELECT RAISE (FAIL, 'Prerequisite not met for enrollment')
--     WHERE EXISTS (
--              SELECT 1
--                FROM term_courses tc
--             NATURAL JOIN course_prerequisites cp
--               WHERE tc.course_id = NEW.term_course_id
--                 AND cp.prerequisite_id NOT IN (
--                        SELECT ce.term_course_id
--                          FROM course_enrollment ce
--                         WHERE ce.student_id = NEW.student_id
--                     )
--           );
--
-- END;
--
-- CREATE TRIGGER check_seat_availability BEFORE INSERT ON course_enrollment BEGIN
-- -- Check if the course group has available seats
--    SELECT RAISE (FAIL, 'No available seats for course')
--     WHERE (
--              SELECT COUNT(*)
--                FROM course_enrollment ce
--               WHERE ce.term_course_id IN (
--                        SELECT tc.course_id
--                          FROM term_courses tc
--                         WHERE tc.group_no = (
--                                  SELECT group_no
--                                    FROM term_courses
--                                   WHERE course_id = NEW.term_course_id
--                               )
--                     )
--           ) >= (
--              SELECT max_seats
--                FROM term_courses
--               WHERE course_id = NEW.term_course_id
--           );
--
-- END;
--
--
-- -- TODO: check for course enrollment conflicts: max credit hours, time conflicts, etc.
-- -- (possibly in backend code)

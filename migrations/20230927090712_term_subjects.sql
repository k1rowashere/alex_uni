CREATE TABLE IF NOT EXISTS
  term_subjects (
    id INTEGER PRIMARY KEY,
    max_seats INTEGER NOT NULL,
    group_no INTEGER NOT NULL,
    sec_no INTEGER NOT NULL CHECK (sec_no BETWEEN 1 AND 2),
    subject_id INTEGER NOT NULL REFERENCES subjects (id),
    prof_id INTEGER NOT NULL REFERENCES professors (id),
    lec_id INTEGER NOT NULL REFERENCES classes (id),
    tut_id INTEGER REFERENCES classes (id),
    lab_id INTEGER REFERENCES classes (id)
  ) STRICT;

-- check that lec_id, tut_id, lab_id all reference a class with the same type
CREATE TRIGGER IF NOT EXISTS 
  correct_reference_check 
BEFORE INSERT ON term_subjects
FOR EACH ROW
BEGIN
  SELECT CASE
    WHEN ((SELECT type FROM classes WHERE id = NEW.lec_id) != 'lec') THEN
      RAISE (FAIL, "lec_id does not reference a lecture class")
    WHEN ((SELECT type FROM classes WHERE id = NEW.tut_id) != 'tut') THEN
      RAISE (FAIL, "tut_id does not reference a tutorial class")
    WHEN ((SELECT type FROM classes WHERE id = NEW.lab_id) != 'lab') THEN
      RAISE (FAIL, "lab_id does not reference a lab class")
  END;
END;

CREATE TRIGGER IF NOT EXISTS
  correct_reference_check_u 
BEFORE UPDATE OF lec_id, tut_id, lab_id ON term_subjects
FOR EACH ROW
BEGIN
  SELECT CASE
    WHEN ((SELECT type FROM classes WHERE id = NEW.lec_id) != 'lec') THEN
      RAISE (FAIL, "lec_id does not reference a lecture class")
    WHEN ((SELECT type FROM classes WHERE id = NEW.tut_id) != 'tut') THEN
      RAISE (FAIL, "tut_id does not reference a tutorial class")
    WHEN ((SELECT type FROM classes WHERE id = NEW.lab_id) != 'lab') THEN
      RAISE (FAIL, "lab_id does not reference a lab class")
  END;
END;

CREATE TABLE IF NOT EXISTS
  subjects (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    code TEXT NOT NULL,
    level INTEGER NOT NULL,
    credit INTEGER NOT NULL,
    pre_req JSON NOT NULL DEFAULT '[]' CHECK (json_type(pre_req) = 'array')
  );

CREATE TRIGGER IF NOT EXISTS
  check_subject_pre_req
  BEFORE INSERT ON subjects
  FOR EACH ROW
    BEGIN
      SELECT RAISE(FAIL, 'Invalid pre-requisites')
      WHERE (
        SELECT COUNT(*)
        FROM subjects
        WHERE id IN (
          SELECT json_each.value
          FROM json_each(NEW.pre_req)
        )
      ) != json_array_length(NEW.pre_req);
    END;

CREATE TRIGGER IF NOT EXISTS
  check_subject_pre_req_u
  BEFORE UPDATE OF pre_req ON subjects
  FOR EACH ROW
    BEGIN
      SELECT RAISE(FAIL, 'Invalid pre-requisites')
      WHERE (
        SELECT COUNT(*)
        FROM subjects
        WHERE id IN (
          SELECT json_each.value
          FROM json_each(NEW.pre_req)
        )
      ) != json_array_length(NEW.pre_req);
    END;

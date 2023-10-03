CREATE VIEW
  IF NOT EXISTS classes_view AS 
SELECT DISTINCT
  classes.id,
  classes.type as ctype,
  professors.name as prof,
  subjects.name,
  subjects.code,
  locations.building,
  locations.floor,
  locations.room,
  classes.day_of_week,
  classes.period_start,
  classes.period_end,
  classes.week_parity,
  term_subjects.sec_no
FROM
  term_subjects
  INNER JOIN classes ON term_subjects.lec_id = classes.id
    OR term_subjects.lab_id = classes.id
    OR term_subjects.tut_id = classes.id
  INNER JOIN subjects ON classes.subject_id = subjects.id
  INNER JOIN locations ON classes.location_id = locations.id
  INNER JOIN professors ON term_subjects.prof_id = professors.id;

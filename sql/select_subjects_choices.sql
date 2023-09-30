SELECT
  s.level AS "level: u8",
  s.name,
  s.code,
  json_group_array(ts.id) AS "choices!: sqlx::types::Json<Vec<SubjectId>>"
FROM term_subjects AS ts
INNER JOIN subjects As s ON ts.subject_id = s.id
INNER JOIN (
  SELECT count(*) AS count 
  FROM term_subscribers AS tsub
  INNER JOIN term_subjects AS ts 
    ON tsub.term_subject_id = ts.id
)
WHERE
  s.id NOT IN (
    SELECT completed.subject_id 
    FROM completed 
    WHERE completed.student_id = ?
  )
  AND s.id IN (
    SELECT value 
    FROM json_each(s.pre_req)
  )
GROUP BY
  s.id,
  ts.subject_id;

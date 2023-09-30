SELECT
    classes_view.*
FROM
    classes_view AS cv 
INNER JOIN term_subjects AS ts 
    ON cv.id IN (ts.lec_id, ts.lab_id, ts.tut_id)
WHERE
    ts.id = ?;

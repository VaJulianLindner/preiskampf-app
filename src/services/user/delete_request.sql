DELETE FROM
    contacts 
WHERE 
    id = $2
    AND
    (to_user_id = $1 OR by_user_id = $1)
RETURNING *
INSERT INTO 
    contacts (by_user_id, to_user_id, state)
VALUES
    ($1, (
        SELECT 
            users.id
        FROM
            users
        WHERE 
            email = $2
    ), $3)
RETURNING
    state
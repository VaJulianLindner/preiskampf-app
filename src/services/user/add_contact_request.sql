WITH added_user AS (
    SELECT 
        users.id, users.email
    FROM
        users
    WHERE 
        email = $2
)

INSERT INTO 
    contacts (by_user_id, to_user_id, state)
    VALUES
        ($1, (SELECT added_user.id FROM added_user), $3)
    RETURNING 
        to_user_id, by_user_id, state, contacts.id AS contact_id, (SELECT added_user.email FROM added_user) AS email
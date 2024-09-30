UPDATE 
    contacts 
SET 
    state = 'confirmed'
WHERE 
    by_user_id = $1
RETURNING
    to_user_id, by_user_id, state, id as contact_id, (
        SELECT users.email FROM users WHERE users.id = contacts.by_user_id
    ) AS email
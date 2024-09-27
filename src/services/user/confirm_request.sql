UPDATE 
    contacts 
SET 
    state = 'confirmed'
WHERE 
    id = $1
RETURNING
    to_user_id, by_user_id, state, id as contact_id, (
        SELECT users.email FROM users WHERE users.id = contacts.by_user_id
    ) AS email


-- WITH confirmed_contact AS (
--     SELECT 
--         contacts.id, contacts.to_user_id, contacts.by_user_id
--     FROM
--         contacts
--     WHERE 
--         id = $1
-- )

-- INSERT INTO
--     contacts (by_user_id, to_user_id, state)
--     VALUES
--         (
--             (SELECT confirmed_contact.to_user_id FROM confirmed_contact),
--             (SELECT confirmed_contact.by_user_id FROM confirmed_contact),
--             'confirmed'
--         )

-- UPDATE 
--     confirmed_contact 
-- SET 
--     state = 'confirmed'
-- RETURNING
--     to_user_id, by_user_id, state, id as contact_id, (
--         SELECT users.email FROM users WHERE users.id = contacts.by_user_id
--     ) AS email
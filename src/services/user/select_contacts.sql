SELECT
    *
FROM
    contacts
    LEFT JOIN (
        SELECT id, email FROM users
    ) AS users_table ON contacts.to_user_id = users_table.id
WHERE
    contacts.by_user_id = $1 AND contacts.state = 'confirmed'
ORDER BY
    contacts.created_at ASC
LIMIT 10
INSERT INTO 
    users (email, password, confirmation_token)
VALUES
    ($1, $2, $3)
RETURNING
    email, id, confirmation_token
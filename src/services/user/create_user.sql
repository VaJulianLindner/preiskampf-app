INSERT INTO 
    users (email, password)
VALUES
    ($1, $2)
RETURNING
    email, password, id, username
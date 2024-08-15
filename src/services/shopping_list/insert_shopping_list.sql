INSERT INTO 
    shopping_lists (user_id, name, emoji_presentation) 
VALUES 
    ($1, $2, $3)
RETURNING 
    id, name, user_id, emoji_presentation, created_at
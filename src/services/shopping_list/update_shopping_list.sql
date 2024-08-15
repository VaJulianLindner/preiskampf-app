UPDATE 
    shopping_lists 
SET 
    user_id = $1, name = $2, emoji_presentation = $3 
WHERE 
    id = $4 
RETURNING 
    id, name, user_id, emoji_presentation, created_at
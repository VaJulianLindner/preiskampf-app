DELETE 
FROM 
    shopping_lists
WHERE 
    user_id = $1 AND id = $2
RETURNING
    id, name, user_id, created_at, emoji_presentation
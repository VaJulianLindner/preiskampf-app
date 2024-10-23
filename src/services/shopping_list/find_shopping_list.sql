SELECT 
    id, name, user_id, emoji_presentation, created_at, COUNT(*) OVER() AS total
FROM 
    shopping_lists
WHERE
    id = $1 AND user_id = $2
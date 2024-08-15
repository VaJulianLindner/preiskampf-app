SELECT 
    id, name, user_id, emoji_presentation, created_at, COUNT(*) OVER() AS total
FROM 
    shopping_lists
WHERE
    shopping_lists.user_id = $1
LIMIT 
    $2
OFFSET
    $3
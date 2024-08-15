INSERT INTO 
    user_selected_shopping_list (user_id, id) 
VALUES 
    ($1, $2)
ON CONFLICT (user_id) DO UPDATE SET 
    user_id = EXCLUDED.user_id,
    id = EXCLUDED.id
RETURNING
    id
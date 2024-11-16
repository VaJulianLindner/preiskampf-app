UPDATE users
SET confirmation_token = NULL
WHERE
    users.confirmation_token = $1
RETURNING
    users.email, 
    users.password, 
    users.id, 
    users.username, 
    users.address, 
    users.confirmation_token
    -- user_selected_shopping_list.id as selected_shopping_list_id,
    -- st_x(location::geometry) as address_lng,
    -- st_y(location::geometry) as address_lat
UPDATE 
    users 
SET 
    username = $2, address = $3, location = st_point($4, $5)
WHERE 
    id = $1
RETURNING
    email, password, id, username, address, (
        SELECT user_selected_shopping_list.id FROM user_selected_shopping_list WHERE user_selected_shopping_list.user_id = $1
    ) as selected_shopping_list_id,
    st_x(location::geometry) as address_lng,
    st_y(location::geometry) as address_lat
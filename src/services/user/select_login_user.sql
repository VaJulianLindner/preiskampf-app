SELECT 
    users.email, 
    users.password, 
    users.id, 
    users.username, 
    users.address, 
    users.confirmation_token,
    user_selected_shopping_list.id as selected_shopping_list_id,
    st_x(location::geometry) as address_lng,
    st_y(location::geometry) as address_lat
FROM (
    users 
    LEFT JOIN 
        user_selected_shopping_list ON users.id = user_selected_shopping_list.user_id
)
WHERE 
    email = $1 AND password = $2
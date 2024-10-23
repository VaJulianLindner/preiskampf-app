SELECT 
    shopping_lists_items.product_id
FROM 
    shopping_lists
LEFT JOIN 
    shopping_lists_items ON shopping_lists.id = shopping_lists_items.shopping_list_id
WHERE
    shopping_lists.id = $1 AND shopping_lists.user_id = $2
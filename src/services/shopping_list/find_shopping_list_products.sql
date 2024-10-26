SELECT 
    shopping_lists_items.product_id,
    shopping_lists_items.created_at AS added_at,
    products.id,
    products.created_at,
    products.name,
    products.images,
    products.url,
    products.market_id,
    COUNT(*) OVER() AS total
FROM 
    shopping_lists_items
LEFT JOIN
    products ON products.id = shopping_lists_items.product_id
WHERE
    shopping_lists_items.shopping_list_id = $1
ORDER BY
    added_at DESC
LIMIT $2 OFFSET $3
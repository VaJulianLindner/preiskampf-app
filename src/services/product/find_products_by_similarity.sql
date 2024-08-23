SELECT 
    products_table.id,
    products_table.created_at,
    products_table.name,
    products_table.images,
    products_table.url,
    products_table.market_id,
    prices_table.price,
    prices_table.currency,
    COUNT(*) OVER() AS total
FROM
    products products_table
LEFT JOIN
    (
        SELECT prices.product_id, prices.price, prices.currency
        FROM prices
        ORDER BY prices.created_at DESC
        LIMIT 1
    ) AS prices_table
ON
    products_table.id = prices_table.product_id
WHERE
    similarity(products_table.name, '{}') > 0.2
ORDER BY
    products_table.name <-> '{}'
LIMIT 
    {} 
OFFSET
    {}
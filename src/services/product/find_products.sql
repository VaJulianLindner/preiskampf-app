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
    -- TODO use max aggregation on created_at to only get the first?
    (
        SELECT prices.product_id, MAX(prices.price) AS price, MAX(prices.currency) AS currency, MAX(prices.created_at) AS created_at
        FROM prices
        GROUP BY prices.product_id
        -- ORDER BY created_at DESC
    ) AS prices_table
ON
    products_table.id = prices_table.product_id
-- AND prices_table.id = (
--     SELECT MAX(prices.created_at) AS created_at
-- )
ORDER BY 
    products_table.{} {}
LIMIT 
    {} 
OFFSET
    {}
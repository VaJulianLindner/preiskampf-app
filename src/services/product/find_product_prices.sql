SELECT
  prices_table.created_at,
  prices_table.currency,
  prices_table.price
FROM
  prices prices_table
WHERE
  prices_table.product_id = $1
ORDER BY
  prices_table.created_at DESC
  -- TODO GROUP BY price? => would need an additional index
LIMIT 10
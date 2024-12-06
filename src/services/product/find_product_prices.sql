SELECT
  prices_table.created_at,
  prices_table.currency,
  prices_table.price
FROM
  prices prices_table
WHERE
  prices_table.product_id = $1
ORDER BY
  prices_table.created_at ASC
LIMIT 10
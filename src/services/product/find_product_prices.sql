SELECT
  prices_table.created_at,
  prices_table.currency,
  prices_table.price
FROM
  products products_table
  JOIN prices prices_table ON products_table.id = prices_table.product_id
WHERE
  products_table.id = $1
ORDER BY
  products_table.created_at DESC
LIMIT 10
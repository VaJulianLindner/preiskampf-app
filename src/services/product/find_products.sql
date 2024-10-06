SELECT
  products_table.id,
  products_table.created_at,
  products_table.name,
  products_table.images,
  products_table.url,
  products_table.market_id,
  products_table.price,
  products_table.currency,
  COUNT(*) OVER () AS total
FROM
  products products_table
ORDER BY
  products_table.{} {}
LIMIT
{}
OFFSET
{}
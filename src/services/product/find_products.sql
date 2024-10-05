SELECT
  products_table.id,
  products_table.created_at,
  products_table.name,
  products_table.images,
  products_table.url,
  products_table.market_id,
  prices_table.price,
  prices_table.currency,
  prices_table.ranked_created_at AS prices_creation_rank,
  COUNT(*) OVER () AS total
FROM
  products products_table
  LEFT JOIN (
    SELECT
      product_id,
      RANK() OVER (
        PARTITION BY product_id
        ORDER BY
          created_at DESC
      ) as ranked_created_at,
      price,
      currency
    FROM
      prices
  ) AS prices_table ON products_table.id = prices_table.product_id
  AND prices_table.ranked_created_at = 1
-- FROM
--   products products_table
--   LEFT JOIN prices prices_table ON products_table.id = prices_table.product_id
ORDER BY
  products_table.{} {}
LIMIT
{}
OFFSET
{}
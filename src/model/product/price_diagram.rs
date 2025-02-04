use crate::{
    core::context::Context,
    model::product::Price,
    view::product::price_diagram::PriceDiagramTemplate,
};
use askama::Template;

pub struct PriceDiagram {
    pub prices: Vec<Price>,
    pub min_price: usize,
    pub max_price: usize,
}

impl PriceDiagram {
    pub fn from_prices(prices: &Vec<Price>) -> Self {
        let (min, max) = calculate_price_stats(&prices);
        Self {
            prices: prices.to_vec(),
            min_price: min,
            max_price: max,
        }
    }

    pub fn render_with_context(self, context: &Context) -> Result<String, askama::Error> {
        // TODO this should be rendered from context/partials?
        PriceDiagramTemplate { model: self, context, authenticated_user: &None }.render()
    }

    pub fn get_price_position_y(&self, price: &Price, height: usize) -> f64 {
        if price.price.is_none() {
            return 0f64;
        }

        let parsed_price = price.price.unwrap().try_into().unwrap_or(0) as usize;
        let pixel_per_count = height as f64 / self.max_price as f64;
        let price_diff = self.max_price - parsed_price;
        let pixel_offset = price_diff as f64 * pixel_per_count;
        pixel_offset
    }
}

fn calculate_price_stats(prices: &Vec<Price>) -> (usize, usize) {
    if prices.len() == 0 {
        return (0, 0);
    }

    let mut max = 0usize;
    let mut min = usize::MAX;
    let mut min_timestamp = i64::MAX;
    let mut max_timestamp = 0;

    // TODO sth like get max 10 latest prices or param with max_depth?
    prices.iter().for_each(|v| {
        match v.price {
            Some(price) => {
                let price: usize = price.try_into().unwrap_or(0);
                if price > max {
                    max = price;
                }
                if price < min {
                    min = price;
                }
            },
            None => ()
        };
        match v.created_at {
            Some(date) => {
                let timestamp = date.timestamp();
                if timestamp > max_timestamp {
                    max_timestamp = timestamp;
                }
                if timestamp < min_timestamp {
                    min_timestamp = timestamp;
                }
            },
            None => ()
        }
    });
    
    let _timestamp_range = max_timestamp - min_timestamp;
    (min as usize, max as usize)
}
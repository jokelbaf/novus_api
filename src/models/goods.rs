use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ProductLabel {
    pub id: i64,
    pub title: String,
    pub title_color: String,
    pub background_color: String,
}

#[derive(Deserialize, Debug)]
pub struct BasicProduct {
    pub id: i64,
    pub title: String,
    pub image: String,
    pub is_available: bool,
    pub price_type: String,
    pub price: String,
    pub discount_type: String,
    pub price_with_discount: Option<String>,
    pub labels: Vec<ProductLabel>,
}

#[derive(Deserialize, Debug)]
pub struct GoodsRsp {
    pub total_count: i64,
    pub limit: u16,
    pub page: u16,
    pub data: Vec<BasicProduct>,
}

#[derive(Deserialize, Debug)]
pub struct ProductParameter {
    pub id: i64,
    pub title: String,
    pub value: String,
}

#[derive(Deserialize, Debug)]
pub struct NutritionalValue {
    pub id: i64,
    pub title: String,
    pub value: String,
}

#[derive(Deserialize, Debug)]
pub struct Product {
    pub id: i64,
    pub shop_id: String,
    pub shop_address: String,
    pub title: String,
    pub description: String,
    pub image: String,
    pub is_available: bool,
    pub price_type: String,
    pub price: String,
    pub discount_type: String,
    pub price_with_discount: Option<String>,
    pub start_date: Option<i64>,
    pub finish_date: Option<i64>,
    pub promotion_id: Option<Vec<String>>,
    // pub ingredients: null, - always null, not present in a single product
    pub goods_parameters: Vec<ProductParameter>,
    pub nutritional_value: Vec<NutritionalValue>,
    pub labels: Vec<ProductLabel>
}

#[derive(Deserialize, Debug)]
pub struct GoodsFilterValue {
    pub id: i64,
    pub title: String,
    pub goods_quantity: i64,
}

#[derive(Deserialize, Debug)]
pub struct GoodsFilter {
    pub id: i16,
    pub title: String,
    pub alias: String,
    #[serde(rename = "type")]
    pub t: String,
    pub values: Vec<GoodsFilterValue>,
}

#[derive(Deserialize, Debug)]
pub struct GoodsFiltersRsp {
    pub total_count: i64,
    pub data: Vec<GoodsFilter>,
}

#[derive(Deserialize, Debug)]
pub struct GoodsSorting {
    pub id: i16,
    pub title: String,
    pub is_default: bool,
}

#[derive(Deserialize, Debug)]
pub struct GoodsSortingsRsp {
    pub data: Vec<GoodsSorting>,
}

pub enum SortingOrder {
    Relevant,
    PriceIncreasing,
    PriceDecreasing,
    Custom(i16),
}

impl SortingOrder {
    pub fn value(&self) -> i16 {
        match self {
            SortingOrder::Relevant => 1,
            SortingOrder::PriceIncreasing => 2,
            SortingOrder::PriceDecreasing => 3,
            SortingOrder::Custom(value) => *value,
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct Promotion {
    pub id: i64,
    pub image_promotion_page: String,
    pub image: String,
    pub title: String,
    pub short_description: String,
    pub description: String,
    pub start_date: i64,
    pub finish_date: i64,
    pub show_counter: bool,
    pub count_down: i64,
    pub is_testing: bool,
}

#[derive(Deserialize, Debug)]
pub struct PromotionsRsp {
    pub total_count: i64,
    pub limit: i64,
    pub data: Vec<Promotion>,
}

#[derive(Deserialize, Debug)]
pub struct SearchProduct {
    pub id: i64,
    pub title: String,
    pub image: String,
    pub is_available: bool,
    pub price_type: String,
    pub price: String,
    pub discount_type: String,
    pub price_with_discount: String,
}

#[derive(Deserialize, Debug)]
pub struct GoodsSearchRsp {
    pub total_count: i64,
    pub data: Vec<SearchProduct>,
}

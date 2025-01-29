use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct BasicPurchase {
    pub shop_id: String,
    pub shop_address: String,
    pub date: i64,
    pub bonus: String,
    pub amount: String,
    pub check_number: String,
    pub total_discount: String,
    pub work_station_id: String,
}

#[derive(Deserialize, Debug)]
pub struct PurchaseOperations {
    pub month: i64,
    pub amount: String,
    #[serde(rename = "bonuses_accured")]
    pub bonuses_accrued: String,
    pub discounts: String,
    pub data: Vec<BasicPurchase>,
}

#[derive(Deserialize, Debug)]
pub struct PurchasesHistoryRsp {
    pub total_count: i64,
    pub limit: u16,
    pub page: u16,
    pub data: Vec<PurchaseOperations>,
}

#[derive(Deserialize, Debug)]
pub struct PurchaseCoupon {
    pub id: i64,
    pub title: String,
    pub reward_description: String,
}

#[derive(Deserialize, Debug)]
pub struct PurchaseBonusesDetails {
    pub goods_title: String,
    pub amount: String,
}

#[derive(Deserialize, Debug)]
pub struct PurchaseItem {
    pub id: i64,
    pub price_type: String,
    pub title: String,
    pub quantity: String,
    pub amount: String,
    pub old_price: Option<String>,
    pub image: String,
    pub item_price: String,
}

#[derive(Deserialize, Debug)]
pub struct Purchase {
    pub check_number: String,
    pub shop_id: String,
    pub date: i64,
    pub shop_address: String,
    pub amount: String,
    pub bonuses_accrued: String,
    pub bonuses_details: Vec<PurchaseBonusesDetails>,
    pub total_promotion_saving: String,
    pub total_discount_saving: String,
    pub bonuses_written_off: String,
    pub payment_method: String,
    pub goods: Vec<PurchaseItem>,
    pub coupons: Vec<PurchaseCoupon>,
}

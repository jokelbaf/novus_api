use serde::{ Serialize, Deserialize };

#[derive(Deserialize, Debug)]
pub struct LoyaltyProgram {
    pub available_coupons: i32,
    pub active_coupons: i32,
    pub bonuses: String,
}

#[derive(Deserialize, Debug)]
pub struct User {
    pub id: i64,
    pub image: Option<String>,
    pub external_customer_id: i64,
    pub first_name: String,
    pub patronymic: String,
    pub last_name: String,
    pub gender: Option<String>,
    pub birth_date: i64,
    pub is_birth_date_changeable: bool,
    pub phone: String,
    pub loyalty_program: LoyaltyProgram,
    pub account_number: String,
    pub qr_code: String,
    pub email: Option<String>,
    pub mail_verification_status: Option<String>,
}

#[derive(Serialize, Debug)]
pub struct UserUpdateReq {
    pub birth_date: i64,
    pub email: Option<String>,
    pub first_name: String,
    pub last_name: String,
    pub patronymic: String,
    pub gender: Option<String>,
}

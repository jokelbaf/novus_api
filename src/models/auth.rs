use serde::{ Serialize, Deserialize };


#[derive(Serialize, Debug)]
pub struct SendCodeReq {
    pub ios_id: String,
    pub phone: String,
}

#[derive(Deserialize, Debug)]
pub struct SendCodeRsp {
    pub auth_token: String,
}

#[derive(Serialize, Debug)]
pub struct CheckUserByPhoneReq {
    pub ios_id: String,
    pub auth_token: String,
    pub phone: String,
}

#[derive(Deserialize, Debug)]
pub struct CheckUserByPhoneRsp {
    pub message: String,
    pub ttl: i32,
    pub attempts_left: u8,
}

#[derive(Serialize, Debug)]
pub struct ConfirmWithOTPReq {
    pub otp: String,
    pub phone: String,
    pub referral_user_id: Option<u64>,
    pub auth_token: String,
    pub ios_id: String,
}

#[derive(Deserialize, Debug)]
pub struct ConfirmWithOTPRsp {
    pub token: String,
    pub refresh_token: String,
    pub first_authorization: bool,
    pub bonus_type: Option<String>,
    pub bonuses: String,
    pub user_first_name: String,
    pub blocked_card: bool,
}

#[derive(Serialize, Debug)]
pub struct RefreshTokenReq {
    pub refresh_token: String,
}

#[derive(Deserialize, Debug)]
pub struct RefreshTokenRsp {
    pub token: String,
    pub refresh_token: String,
}

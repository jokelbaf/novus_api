use super::http::{ HttpClient, ReqMethod };
use crate::models::auth::{
    SendCodeReq,
    SendCodeRsp,
    CheckUserByPhoneReq,
    CheckUserByPhoneRsp,
    ConfirmWithOTPReq,
    ConfirmWithOTPRsp,
    RefreshTokenReq,
    RefreshTokenRsp,
};
use crate::errors::NovusError;

pub struct AuthClient {
    pub token: Option<String>,
    pub refresh_token: Option<String>,
    pub ios_id: String,
    http_client: HttpClient,
}

impl AuthClient {
    pub fn new() -> Self {
        let ios_id = String::from("CB7512AC-0292-45E4-8B76-275A8FDEEA14");
        Self {
            ios_id,
            token: None,
            refresh_token: None,
            http_client: HttpClient::new(),
        }
    }

    pub async fn send_code(&self, number: u64) -> Result<SendCodeRsp, NovusError> {
        let body = SendCodeReq {
            ios_id: self.ios_id.clone(),
            phone: number.to_string(),
        };
        self.http_client.request::<SendCodeReq, SendCodeRsp>(
            ReqMethod::POST,
            "auth/auth_token".to_string(),
            None,
            Some(body),
            None,
        ).await
    }

    pub async fn check_by_phone(
        &self,
        number: u64,
        auth_token: String
    ) -> Result<CheckUserByPhoneRsp, NovusError> {
        let body = CheckUserByPhoneReq {
            auth_token,
            ios_id: self.ios_id.clone(),
            phone: number.to_string(),
        };
        self.http_client.request::<CheckUserByPhoneReq, CheckUserByPhoneRsp>(
            ReqMethod::POST,
            "auth/check_user_by_phone".to_string(),
            None,
            Some(body),
            None,
        ).await
    }

    pub async fn confirm_with_otp(
        &mut self,
        number: u64,
        otp: u16,
        auth_token: String,
        referral_user_id: Option<u64>
    ) -> Result<ConfirmWithOTPRsp, NovusError> {
        let body = ConfirmWithOTPReq {
            auth_token,
            referral_user_id,
            otp: otp.to_string(),
            phone: number.to_string(),
            ios_id: self.ios_id.clone(),
        };
        let rsp = self.http_client.request::<ConfirmWithOTPReq, ConfirmWithOTPRsp>(
            ReqMethod::POST,
            "auth/confirm_with_otp".to_string(),
            None,
            Some(body),
            None,
        ).await?;

        self.token = Some(rsp.token.clone());
        self.refresh_token = Some(rsp.refresh_token.clone());

        Ok(rsp)
    }

    pub async fn refresh_token(&mut self) -> Result<RefreshTokenRsp, NovusError> {
        let refresh_token = self
            .refresh_token
            .as_ref()
            .ok_or(NovusError::RefreshTokenMissingError)?;

        let body = RefreshTokenReq {
            refresh_token: refresh_token.clone(),
        };

        let rsp = self
            .http_client
            .request::<RefreshTokenReq, RefreshTokenRsp>(
                ReqMethod::POST,
                "auth/refresh_token".to_string(),
                None,
                Some(body),
                None,
            )
            .await?;

        self.token = Some(rsp.token.clone());
        self.refresh_token = Some(rsp.refresh_token.clone());

        Ok(rsp)
    }
}

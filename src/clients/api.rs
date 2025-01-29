use reqwest::header::{
    HeaderMap,
    HeaderValue
};
use std::collections::HashMap;

use crate::models::auth::{
    ConfirmWithOTPRsp,
    RefreshTokenRsp
};
use crate::models::user::{
    User,
    UserUpdateReq
};
use crate::models::bonuses::{
    BonusesTypesRsp,
    BonusesType,
    BonusesHistoryRsp,
};
use crate::models::goods::{
    GoodsFiltersRsp,
    GoodsRsp,
    GoodsSortingsRsp,
    Product,
    SortingOrder,
    PromotionsRsp,
    GoodsSearchRsp,
};
use crate::models::purchases::{
    Purchase,
    PurchasesHistoryRsp,
};
use crate::errors::NovusError;

use super::http::{ HttpClient, ReqMethod };
use super::auth::AuthClient;


pub struct APIClient {
    auth_client: AuthClient,
    http_client: HttpClient,
}

impl APIClient {
    pub fn new() -> Self {
        Self {
            auth_client: AuthClient::new(),
            http_client: HttpClient::new(),
        }
    }

    pub fn refresh_token(mut self, refresh_token: String) -> Self {
        self.auth_client.refresh_token = Some(refresh_token);
        self
    }

    pub fn token(mut self, token: String) -> Self {
        self.auth_client.token = Some(token);
        self
    }

    pub fn base_url(mut self, base_url: String) -> Self {
        self.http_client.base_url = base_url;
        self
    }

    pub fn private_key(mut self, private_key: String) -> Self {
        self.http_client.private_key = private_key;
        self
    }

    pub fn ios_id(mut self, ios_id: String) -> Self {
        self.auth_client.ios_id = ios_id;
        self
    }

    pub async fn send_code(&self, number: u64) -> Result<String, NovusError> {
        let rsp = self.auth_client.send_code(number).await?;
        self.auth_client.check_by_phone(number, rsp.auth_token.clone()).await?;
        Ok(rsp.auth_token)
    }

    pub async fn login_with_otp(
        &mut self,
        number: u64,
        otp: u16,
        auth_token: String,
        referral_user_id: Option<u64>
    ) -> Result<ConfirmWithOTPRsp, NovusError> {
        self.auth_client.confirm_with_otp(number, otp, auth_token, referral_user_id).await
    }

    pub async fn refresh_tokens(&mut self) -> Result<RefreshTokenRsp, NovusError> {
        self.auth_client.refresh_token().await
    }

    pub async fn get_profile(&self) -> Result<User, NovusError> {
        let token = self
            .auth_client
            .token
            .as_deref()
            .ok_or(NovusError::TokenMissingError)?;

        let mut headers = HeaderMap::new();
        headers.insert(
            "user_token",
            HeaderValue::from_str(token).map_err(NovusError::HeaderValueError)?,
        );

        self.http_client
            .request::<(), User>(
                ReqMethod::GET,
                "user/profile".to_string(),
                Some(headers),
                None::<()>,
                None,
            )
            .await
    }

    pub async fn update_profile(
        &self,
        birth_date: i64,
        email: Option<String>,
        first_name: String,
        last_name: String,
        patronymic: String,
        gender: Option<String>,
    ) -> Result<User, NovusError> {
        let token = self
            .auth_client
            .token
            .as_deref()
            .ok_or(NovusError::TokenMissingError)?;

        let mut headers = HeaderMap::new();
        headers.insert(
            "user_token",
            HeaderValue::from_str(token).map_err(NovusError::HeaderValueError)?,
        );

        let body = UserUpdateReq {
            birth_date,
            email,
            first_name,
            last_name,
            patronymic,
            gender,
        };

        self.http_client
            .request::<UserUpdateReq, User>(
                ReqMethod::PUT,
                "user/profile".to_string(),
                Some(headers),
                Some(body),
                None,
            )
            .await
    }

    pub async fn get_goods(
        &self,
        page: u16,
        limit: u16,
        search: Option<String>,
        is_with_discount: bool,
        sorting_order: SortingOrder,
    ) -> Result<GoodsRsp, NovusError> {
        let mut query_params = HashMap::from([
            ("page".to_string(), page.to_string()),
            ("limit".to_string(), limit.to_string()),
            ("is_with_discount".to_string(), is_with_discount.to_string()),
            ("sorting_id".to_string(), sorting_order.value().to_string()),
        ]);
    
        if let Some(s) = search {
            query_params.insert("search".to_string(), s);
        }

        self.http_client
            .request::<(), GoodsRsp>(
                ReqMethod::GET,
                "goods".to_string(),
                None,
                None::<()>,
                Some(query_params),
            )
            .await
    }

    pub async fn get_novelty_goods(
        &self,
        page: u16,
        limit: u16,
        sorting_order: SortingOrder,
    ) -> Result<GoodsRsp, NovusError> {
        let query_params = HashMap::from([
            ("page".to_string(), page.to_string()),
            ("limit".to_string(), limit.to_string()),
            ("sorting_id".to_string(), sorting_order.value().to_string()),
        ]);

        self.http_client
            .request::<(), GoodsRsp>(
                ReqMethod::GET,
                "novelty_goods".to_string(),
                None,
                None::<()>,
                Some(query_params),
            )
            .await
    }

    pub async fn get_product(&self, id: i64) -> Result<Product, NovusError> {
        self.http_client
            .request::<(), Product>(
                ReqMethod::GET,
                format!("goods/{}", id.to_string()),
                None,
                None::<()>,
                None,
            )
            .await
    }

    pub async fn get_goods_recommendations(
        &self,
        page: u16,
        limit: u16,
    ) -> Result<GoodsRsp, NovusError> {
        let query_params = HashMap::from([
            ("page".to_string(), page.to_string()),
            ("limit".to_string(), limit.to_string()),
        ]);

        let mut headers = HeaderMap::new();
        if let Some(token) = self.auth_client.token.clone() {
            headers.insert(
                "user_token",
                HeaderValue::from_str(&token).map_err(NovusError::HeaderValueError)?,
            );
        }

        self.http_client
            .request::<(), GoodsRsp>(
                ReqMethod::GET,
                "goods_recommendations".to_string(),
                Some(headers),
                None::<()>,
                Some(query_params),
            )
            .await
    }

    pub async fn get_goods_filters(&self) -> Result<GoodsFiltersRsp, NovusError> {
        self.http_client
            .request::<(), GoodsFiltersRsp>(
                ReqMethod::GET,
                "goods/filters".to_string(),
                None,
                None::<()>,
                None,
            )
            .await
    }

    pub async fn get_goods_sortings(&self) -> Result<GoodsSortingsRsp, NovusError> {
        self.http_client
            .request::<(), GoodsSortingsRsp>(
                ReqMethod::GET,
                "goods/sortings".to_string(),
                None,
                None::<()>,
                None,
            )
            .await
    }

    pub async fn get_bonuses_types(&self) -> Result<BonusesTypesRsp, NovusError> {
        self.http_client
            .request::<(), BonusesTypesRsp>(
                ReqMethod::GET,
                "v2/user/bonuses_types".to_string(),
                None,
                None::<()>,
                None,
            )
            .await
    }

    pub async fn get_bonuses_history(
        &self,
        page: u16,
        limit: u16,
        bonuses_type: Option<BonusesType>,
    ) -> Result<BonusesHistoryRsp, NovusError> {
        let token = self
            .auth_client
            .token
            .as_deref()
            .ok_or(NovusError::TokenMissingError)?;

        let mut headers = HeaderMap::new();
        headers.insert(
            "user_token",
            HeaderValue::from_str(token).map_err(NovusError::HeaderValueError)?,
        );

        let mut query_params = HashMap::from([
            ("page".to_string(), page.to_string()),
            ("limit".to_string(), limit.to_string()),
        ]);
        if let Some(bonuses_type) = bonuses_type {
            query_params.insert("type_id".to_string(), bonuses_type.value().to_string());
        }

        self.http_client
            .request::<(), BonusesHistoryRsp>(
                ReqMethod::GET,
                "v2/user/bonuses".to_string(),
                Some(headers),
                None::<()>,
                Some(query_params),
            )
            .await
    }

    pub async fn get_promotions(
        &self,
        page: u16,
        limit: u16,
    ) -> Result<PromotionsRsp, NovusError> {
        let query_params = HashMap::from([
            ("page".to_string(), page.to_string()),
            ("limit".to_string(), limit.to_string()),
        ]);
        self.http_client
            .request::<(), PromotionsRsp>(
                ReqMethod::GET,
                "promotions".to_string(),
                None,
                None::<()>,
                Some(query_params),
            )
            .await
    }

    pub async fn get_purchases_history(
        &self,
        page: u16,
        limit: u16,
    ) -> Result<PurchasesHistoryRsp, NovusError> {
        let token = self
            .auth_client
            .token
            .as_deref()
            .ok_or(NovusError::TokenMissingError)?;

        let mut headers = HeaderMap::new();
        headers.insert(
            "user_token",
            HeaderValue::from_str(token).map_err(NovusError::HeaderValueError)?,
        );

        let query_params = HashMap::from([
            ("page".to_string(), page.to_string()),
            ("limit".to_string(), limit.to_string()),
        ]);

        self.http_client
            .request::<(), PurchasesHistoryRsp>(
                ReqMethod::GET,
                "v2/user/purchases".to_string(),
                Some(headers),
                None::<()>,
                Some(query_params),
            )
            .await
    }

    pub async fn get_purchase(
        &self,
        store_id: i64,
        date: i64,
        check_number: String,
    ) -> Result<Purchase, NovusError> {
        let query_params = HashMap::from([
            ("store".to_string(), store_id.to_string()),
            ("date".to_string(), date.to_string()),
            ("check_number".to_string(), check_number),
        ]);
        self.http_client
            .request::<(), Purchase>(
                ReqMethod::GET,
                "v2/user/purchase".to_string(),
                None,
                None::<()>,
                Some(query_params),
            )
            .await
    }

    pub async fn search_goods(
        &self,
        query: String,
        is_with_discount: bool,
    ) -> Result<GoodsSearchRsp, NovusError> {
        let query_params = HashMap::from([
            ("input".to_string(), query),
            ("is_with_discount".to_string(), is_with_discount.to_string()),
        ]);
        self.http_client
            .request::<(), GoodsSearchRsp>(
                ReqMethod::GET,
                "goods/search".to_string(),
                None,
                None::<()>,
                Some(query_params),
            )
            .await
    }
}

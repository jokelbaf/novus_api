use reqwest::{ Client, RequestBuilder, StatusCode, Url };
use reqwest::header::{ HeaderMap, HeaderValue };
use serde::{ Serialize, de::DeserializeOwned };
use std::collections::HashMap;

use crate::errors::NovusError;

pub enum ReqMethod {
    GET,
    PUT,
    POST,
}

pub struct HttpClient {
    pub base_url: String,
    pub private_key: String,
    client: Client,
}

impl HttpClient {
    pub fn new() -> Self {
        let base_url = String::from("https://api.novus.online");
        let private_key = String::from("070696aa5d8844e3c71e90604a7b0a11dc0c99638d3f2e0bf53c2f091ac52d4c");
        Self {
            base_url,
            private_key,
            client: Client::new(),
        }
    }

    pub async fn request<T, R>(
        &self,
        method: ReqMethod,
        path: String,
        headers: Option<HeaderMap>,
        body: Option<T>,
        query_params: Option<HashMap<String, String>>,
    ) -> Result<R, NovusError>
    where
        T: Serialize,
        R: DeserializeOwned,
    {
        let mut url = Url::parse(&self.base_url).map_err(|e| NovusError::UrlError(e.to_string()))?;
        url.set_path(&path);

        if let Some(params) = query_params {
            for (key, value) in params {
                url.query_pairs_mut().append_pair(&key, &value);
            }
        }

        let mut request_builder: RequestBuilder = match method {
            ReqMethod::GET => self.client.get(url),
            ReqMethod::PUT => self.client.put(url),
            ReqMethod::POST => self.client.post(url),
        };
    
        let mut headers = if let Some(h) = headers {
            h
        } else {
            HeaderMap::new()
        };

        match HeaderValue::from_str(&self.private_key) {
            Ok(header_value) => {
                headers.insert("private_key", header_value);
            },
            Err(e) => return Err(NovusError::HeaderValueError(e)),
        };

        request_builder = request_builder.headers(headers);


        if let Some(body) = body {
            request_builder = request_builder.json(&body);
        }

        let response = request_builder.send().await.map_err(NovusError::NetworkError)?;

        match response.status() {
            StatusCode::OK => {
                let response_body = response.text().await.map_err(NovusError::NetworkError)?;
                serde_json::from_str(&response_body).map_err(|e| {
                    let error_message = format!("Failed to deserialize response: {}. Response body: {}", e, response_body);
                    NovusError::DeserializationError(error_message)
                })
            },
            status => Err(NovusError::StatusError(status))
        }
    }
}

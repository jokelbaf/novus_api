pub mod clients {
    pub mod api;
    mod auth;
    mod http;
}
pub mod models {
    pub mod auth;
    pub mod user;
    pub mod goods;
    pub mod bonuses;
    pub mod purchases;
}
pub mod errors;

pub use clients::api::APIClient;
pub use errors::NovusError;

pub use models::auth::*;
pub use models::user::*;
pub use models::goods::*;
pub use models::bonuses::*;
pub use models::purchases::*;

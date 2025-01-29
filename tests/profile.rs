use std::env;
use dotenvy::dotenv;

use novus_api::APIClient;


#[tokio::test]
async fn get_profile_test() {
    dotenv().ok();

    let token = env::var("TOKEN").expect("TOKEN not set");
    let user_id = env::var("USER_ID").expect("USER_ID not set");
    let user_phone = env::var("USER_PHONE").expect("USER_PHONE not set");

    let client = APIClient::new().token(token);

    let profile = client.get_profile().await.unwrap();

    assert_eq!(profile.id, user_id.parse::<i64>().unwrap());
    assert_eq!(profile.phone, user_phone);
}

#[tokio::test]
async fn update_profile_test() {
    dotenv().ok();

    let token = env::var("TOKEN").expect("TOKEN not set");
    let client = APIClient::new().token(token);

    const BIRTH_DATE: i64 = -7200;
    let first_name = String::from("Test");
    let last_name = String::from("Test");

    let profile = client.update_profile(
        BIRTH_DATE,
        None,
        first_name.clone(),
        last_name.clone(),
        String::from(""),
        None,
    ).await.unwrap();

    assert_eq!(profile.first_name, first_name);
    assert_eq!(profile.last_name, last_name);
    assert_eq!(profile.birth_date, BIRTH_DATE);
}

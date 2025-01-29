use novus_api::{ APIClient, SortingOrder };


#[tokio::test]
async fn get_goods_test() {
    let client = APIClient::new();
    let goods = client.get_goods(
        1,
        250,
        None,
        true,
        SortingOrder::PriceDecreasing,
    ).await.unwrap();

    assert_eq!(goods.page, 1);
    assert_eq!(goods.limit, 250);
    assert_eq!(goods.data.len(), 250);
}

#[tokio::test]
async fn get_novelty_goods_test() {
    let client = APIClient::new();
    let goods = client.get_novelty_goods(1, 5, SortingOrder::PriceDecreasing).await.unwrap();

    assert_eq!(goods.page, 1);
    assert_eq!(goods.limit, 5);
    assert_eq!(goods.data.len(), 5);
}

#[tokio::test]
async fn get_goods_recommendations_test() {
    let client = APIClient::new();
    let goods = client.get_goods_recommendations(1, 10).await.unwrap();

    assert_eq!(goods.page, 1);
    assert_eq!(goods.limit, 10);
    assert_eq!(goods.data.len(), 10);
}

#[tokio::test]
async fn search_goods_test() {
    let client = APIClient::new();
    let goods = client.search_goods(String::from("риба"), true).await.unwrap();

    assert!(goods.data.len() > 0);
    assert!(goods.data[0].title.to_lowercase().contains("риба"));
}

#[tokio::test]
async fn get_goods_filters_test() {
    let client = APIClient::new();
    let filters = client.get_goods_filters().await.unwrap();

    assert!(filters.data.len() > 0);
}

#[tokio::test]
async fn get_goods_sortings_test() {
    let client = APIClient::new();
    let sortings = client.get_goods_sortings().await.unwrap();

    assert!(sortings.data.len() > 0);
}

#[tokio::test]
async fn get_product_test() {
    let client = APIClient::new();
    let product = client.get_product(10).await.unwrap();

    assert_eq!(product.id, 10);
}

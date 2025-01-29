# Novus API

The package is a wrapper around reverse engineered Novus API. It provides a simple interface to search for goods, fetch/update profile info and other stuff.

## Features

The package provides the following features:
- Full login implementation with mobile number and OTP + tokens refresh;
- Fetch/update profile info such as name, surname, email, birth date, etc;
- Search for goods/novelty goods, get recommendations and filters;
- Obtain full product info;
- Get promotions and discounts;
- Fetch purchases history, get purchase details;
- Get bonuses history, current bonuses amount.

## Installation

```bash
cargo add --git https://github.com/jokelbaf/novus_api.git novus_api
```

## Usage

```rust
use novus_api::{ APIClient, SortingOrder };


#[tokio::main]
async fn main() {
    let client = APIClient::new();
    let product = client.get_product(10).await.unwrap();

    println!("{:?}", product);
}
```

You can see more examples in [`tests`](tests) directory.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

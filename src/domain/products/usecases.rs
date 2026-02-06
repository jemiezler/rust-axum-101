use super::entities;
use crate::shared::types::result::DomainResult;

pub fn find_one_product(product_id: i32) -> DomainResult<entities::Product, String> {
    let mut products: Vec<entities::Product> = Vec::new();

    products.push(entities::Product {
        id: 1,
        title: String::from("Tom Yum Kung"),
        description: String::from("Thai's food"),
        created_at: String::from("2022-01-01"),
        updated_at: String::from("2022-01-01"),
        price: 100.0,
    });
    products.push(entities::Product {
        id: 2,
        title: String::from("Shushi"),
        description: String::from("Japanese's food"),
        created_at: String::from("2022-01-01"),
        updated_at: String::from("2022-01-01"),
        price: 100.0,
    });
    products.push(entities::Product {
        id: 3,
        title: String::from("Roti"),
        description: String::from("Indian's food"),
        created_at: String::from("2022-01-01"),
        updated_at: String::from("2022-01-01"),
        price: 100.0,
    });

    for product in products {
        if product.id == product_id {
            return DomainResult::Ok(product);
        };
    }
    DomainResult::NotFound
}

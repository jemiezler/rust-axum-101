use chrono::Utc;
use uuid::Uuid;

use super::entities;
use crate::shared::types::result::DomainResult;

pub fn find_one_product(product_id: Uuid) -> DomainResult<entities::Product, String> {
    let mut products: Vec<entities::Product> = Vec::new();

    products.push(entities::Product {
        id: Uuid::new_v4(),
        title: String::from("Tom Yum Kung"),
        description: String::from("Thai's food"),
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc(),
        price: 100.0,
    });
    products.push(entities::Product {
        id: Uuid::new_v4(),
        title: String::from("Shushi"),
        description: String::from("Japanese's food"),
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc(),
        price: 100.0,
    });
    products.push(entities::Product {
        id: Uuid::new_v4(),
        title: String::from("Roti"),
        description: String::from("Indian's food"),
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc(),
        price: 100.0,
    });

    for product in products {
        if product.id == product_id {
            return DomainResult::Ok(product);
        };
    }
    DomainResult::NotFound
}

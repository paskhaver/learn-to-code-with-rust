use fake::{Fake, Faker};

use warehouse::ProductCategory;

/// Create a fictional product category
fn main() {
    let random_category: ProductCategory = Faker.fake();
    println!("{:?}", random_category); // Hammer
}

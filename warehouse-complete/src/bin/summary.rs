use warehouse::{FLOOR_SPACE, INVENTORY_MANAGER, ORDERS_MANAGER};

/// Get a summary of the current managers
fn main() {
    println!(
        "Our managers are {} and {}. We have {} square feet",
        INVENTORY_MANAGER, ORDERS_MANAGER, FLOOR_SPACE
    );
}

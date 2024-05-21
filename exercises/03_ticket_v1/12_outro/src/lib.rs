// TODO: Define a new `Order` type.
//   It should keep track of three pieces of information: `product_name`, `quantity`, and `unit_price`.
//   The product name can't be empty and it can't be longer than 300 characters.
//   The quantity must be strictly greater than zero.
//   The unit price is in cents and must be strictly greater than zero.
//   Order must include a method named `total` that returns the total price of the order.
//   Order must provide setters and getters for each field.
//
// Tests are located in a different place this time—in the `tests` folder.
// The `tests` folder is a special location for `cargo`. It's where it looks for **integration tests**.
// Integration here has a very specific meaning: they test **the public API** of your project.
// You'll need to pay attention to the visibility of your types and methods; integration
// tests can't access private or `pub(crate)` items.

pub struct Order {
    product_name: String,
    quantity: usize,
    unit_price: u32,
}

impl Order {
    pub fn new(product_name: String, quantity: usize, unit_price: u32) -> Self {
        Self::ensure_product_name(&product_name);
        Self::ensure_quantity(quantity);
        Self::ensure_unit_price(unit_price);
        Self {
            product_name,
            quantity,
            unit_price
        }
    }

    pub fn product_name(&self) -> &str {
        &self.product_name
    }

    pub fn quantity(&self) -> &usize {
        &self.quantity
    }

    pub fn unit_price(&self) -> &u32 {
        &self.unit_price
    }

    pub fn total(&self) -> u32 {
        self.quantity as u32 * self.unit_price
    }

    pub fn set_product_name(&mut self, product_name: String) {
        Self::ensure_product_name(&product_name);
        self.product_name = product_name;
    }

    pub fn set_quantity(&mut self, quantity: usize) {
        Self::ensure_quantity(quantity);
        self.quantity = quantity;
    }

    pub fn set_unit_price(&mut self, unit_price: u32) {
        Self::ensure_unit_price(unit_price);
        self.unit_price = unit_price;
    }

    fn ensure_product_name(product_name: &str) {
        if product_name.is_empty() || product_name.len() > 300 {
            panic!("Product name has invalid length");
        }
    }

    fn ensure_quantity(quantity: usize) {
        if quantity == 0 {
            panic!("Quantity cannot be zero");
        }
    }

    fn ensure_unit_price(unit_price: u32) {
        if unit_price == 0 {
            panic!("Unit price cannot be zero");
        }
    }
}
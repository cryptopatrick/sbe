#[derive(Debug, PartialEq)]
pub struct Order {
    pub order_id: u64,
    pub quantity: u32,
    pub price: f64,
    pub version: u16,
}

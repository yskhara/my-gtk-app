#[derive(Debug)]
#[derive(Default)]
pub struct ReceiptEntity {
    pub id: u32,
    pub datetime: i64,
    pub store_key: u32,
    pub currency_id: u32,
    pub paid_amount: u32,
    pub payment_method_key: u32,
}

#[derive(Debug)]
pub struct ReceiptItemEntity {
    pub receipt_key: u32,
    pub id: u32,
    pub name: String,
    pub unit_price: u32,
    pub unit: u32,
    pub tax_rate_percent: u32,
    pub ext_tax: bool,
    pub category_key: u32
}

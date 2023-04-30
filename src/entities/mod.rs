pub trait EntityColumn {}

#[derive(Debug, Default)]
pub struct ReceiptEntity {
    pub id: u32,
    pub datetime: i64,
    pub store_key: u32,
    pub currency_id: u32,
    pub paid_amount: u32,
    pub payment_method_key: u32,
}

pub enum ReceiptEntityColumn {
    Id,
    Datetime,
    StoreKey,
    CurrencyKey,
    PaidAmount,
    PaymentMethodKey,
}

impl EntityColumn for ReceiptEntityColumn {}

impl ReceiptEntityColumn {
    pub fn to_string(&self) -> &str {
        match self {
            ReceiptEntityColumn::Id => "id",
            ReceiptEntityColumn::Datetime => "datetime",
            ReceiptEntityColumn::StoreKey => "store_key",
            ReceiptEntityColumn::CurrencyKey => "currency_key",
            ReceiptEntityColumn::PaidAmount => "paid_amount",
            ReceiptEntityColumn::PaymentMethodKey => "payment_method_key",
        }
    }

    pub fn from_string(string: &str) -> Option<Self> {
        match string {
            "id" => Some(ReceiptEntityColumn::Id),
            "datetime" => Some(ReceiptEntityColumn::Datetime),
            "store_key" => Some(ReceiptEntityColumn::StoreKey),
            "currency_key" => Some(ReceiptEntityColumn::CurrencyKey),
            "paid_amount" => Some(ReceiptEntityColumn::PaidAmount),
            "payment_method_key" => Some(ReceiptEntityColumn::PaymentMethodKey),
            _ => None,
        }
    }
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
    pub category_key: u32,
}

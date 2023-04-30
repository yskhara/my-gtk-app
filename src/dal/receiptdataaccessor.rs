use super::DataAccessor;

use std::{sync::Mutex, time::Instant};

use crate::entities::ReceiptEntity;
use once_cell::sync::Lazy;
use rusqlite::Connection;

// data access layer

pub enum ReceiptEntityColumn {
    Id,
    Datetime,
    StoreKey,
    CurrencyKey,
    PaidAmount,
    PaymentMethodKey,
}
impl super::EntityColumn for ReceiptEntityColumn{}

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

pub enum SortOrder {
    Ascending,
    Descending,
}

pub struct ReceiptDataAccessor {}

impl DataAccessor for ReceiptDataAccessor {

}

impl ReceiptDataAccessor{
    pub fn get_receipt(receipt_id: u32) -> Result<ReceiptEntity, rusqlite::Error> {
        let query = "SELECT * FROM receipt WHERE id=?1;";

        super::CONNECTION
            .lock()
            .unwrap()
            .query_row(query, [receipt_id], |row| {
                let id = row.get::<&str, u32>("id")?;
                let datetime = row.get::<&str, i64>("datetime")?;
                let store_key = row.get::<&str, u32>("store_key")?;
                let currency_key = row.get::<&str, u32>("currency_key")?;
                let paid_amount = row.get::<&str, u32>("paid_amount")?;
                let payment_method_key = row.get::<&str, u32>("payment_method_key")?;
                Ok(ReceiptEntity {
                    id: id,
                    datetime: datetime,
                    store_key: store_key,
                    currency_id: currency_key,
                    paid_amount: paid_amount,
                    payment_method_key: payment_method_key,
                })
            })
    }


}

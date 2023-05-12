use std::sync::Mutex;

use once_cell::sync::Lazy;
use rusqlite::Connection;

pub mod prelude;
pub mod receiptlistitem;
pub mod receiptliststore;
mod sqlentity;
pub mod sqlliststore;
mod sqlliststoreworker;
pub mod testdatagenerator;
pub mod incomeliststore;

pub enum SortOrder {
    Ascending,
    Descending,
}

impl SortOrder {
    pub fn to_str(&self) -> &str {
        match self {
            SortOrder::Ascending => "ASC",
            SortOrder::Descending => "DESC",
        }
    }
}

impl Default for SortOrder {
    fn default() -> Self {
        SortOrder::Ascending
    }
}

static CONNECTION: Lazy<Mutex<Connection>> = Lazy::new(|| {
    println!("Opening SQLite connection...");
    Mutex::new(Connection::open_in_memory().unwrap())
});

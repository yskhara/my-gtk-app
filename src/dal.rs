//pub mod receiptdataaccessor;
pub mod testdatagenerator;

use std::{sync::Mutex, time::Instant};

use crate::entities::ReceiptEntity;
use once_cell::sync::Lazy;
use rusqlite::Connection;

// data access layer

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

// Trait implemented by all "ReceiptDataAccessor"s
trait ReceiptDataAccessorImpl {
    fn fetch_entities(
        &self,
        offset: u32,
        row_count: Option<u32>,
        sort_by: &Vec<(std::string::String, SortOrder)>,
    ) -> Result<Vec<ReceiptEntity>, Box<dyn std::error::Error>>;
}

trait ReceiptSqlDataAccessorImpl: ReceiptDataAccessorImpl {}

struct SqlDataAccessor {
    table_name: &'static str,
}

trait SqlDataAccessorImpl {
    fn fetch_rows(
        &self,
        offset: u32,
        row_count: Option<u32>,
        sort_by: &Vec<(std::string::String, SortOrder)>,
    ) -> Result<Vec<rusqlite::Row>, Box<dyn std::error::Error>>;
}

pub struct ReceiptSqlDataAccessor {
    //table_name: &'static str,
    data_accessor: SqlDataAccessor,
}

impl SqlDataAccessor {
    fn fetch_rows(
        &self,
        offset: u32,
        row_count: Option<u32>,
        sort_by: &Vec<(std::string::String, SortOrder)>,
    ) -> Result<Vec<&rusqlite::Row>, Box<dyn std::error::Error>> {
        let mut query = format!("SELECT * FROM {}", self.table_name);

        if sort_by.len() > 0 {
            query += " ORDER BY";
            for i in 0..sort_by.len() {
                let (column, order) = &sort_by[i];
                query += &format!(" {} {}", column.to_string(), order.to_str());
                if i < sort_by.len() - 1 {
                    query += &",";
                }
            }
        }

        if let Some(row_count) = row_count {
            query += &format!(" LIMIT {}, {}", offset.to_string(), row_count.to_string());
        }

        query += ";";

        println!("{:}", query);
        let mut rows = vec![];
        for row in CONNECTION
            .lock()?
            .prepare(&query)?
            .query_map([], |r| Ok(r))?
        {
            rows.push(row?);
        }
        Ok(rows)
    }
}

impl ReceiptSqlDataAccessor {
    pub fn new(table_name: &str) -> Self {
        ReceiptSqlDataAccessor {
            data_accessor: SqlDataAccessor {
                table_name: table_name,
            },
        }
    }
}

impl ReceiptDataAccessorImpl for ReceiptSqlDataAccessor {
    fn fetch_entities(
        &self,
        offset: u32,
        row_count: Option<u32>,
        sort_by: &Vec<(std::string::String, SortOrder)>,
    ) -> Result<Vec<ReceiptEntity>, Box<dyn std::error::Error>> {
        let rows = self.data_accessor.fetch_rows(offset, row_count, sort_by)?;
        let mut fetched = vec![];
        for row in rows {
            fetched.push(ReceiptEntity::try_new(row)?);
        }
        dbg!(fetched.len());
        Ok(fetched)
    }
}

pub struct DataAccessor {
    table_name: String,
}

impl crate::entities::ReceiptEntity {
    pub fn try_new(row: &rusqlite::Row) -> Result<Self, rusqlite::Error> {
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
    }
}

impl DataAccessor {
    pub fn set_table(&mut self, table_name: String) {
        self.table_name = table_name;
        unimplemented!();
    }

    pub fn fetch_rows(
        offset: u32,
        row_count: Option<u32>,
        sort_by: &Vec<(std::string::String, SortOrder)>,
    ) -> Result<Vec<ReceiptEntity>, Box<dyn std::error::Error>> {
        let mut query = std::string::String::from("SELECT * FROM receipt");

        if sort_by.len() > 0 {
            query += " ORDER BY";
            for i in 0..sort_by.len() {
                let (column, order) = &sort_by[i];
                query += &format!(" {} {}", column.to_string(), order.to_str());
                if i < sort_by.len() - 1 {
                    query += &",";
                }
            }
        }

        if let Some(row_count) = row_count {
            query += &format!(" LIMIT {}, {}", offset.to_string(), row_count.to_string());
        }

        query += ";";

        println!("{:}", query);

        let mut vec = vec![];
        for entity in CONNECTION
            .lock()?
            .prepare(&query)?
            .query_map([], |row| ReceiptEntity::try_new(row))?
        {
            if let Ok(entity) = entity {
                vec.push(entity)
            }
        }

        println!("{:} items loaded.", vec.len());
        Ok(vec)
    }
}

pub static CONNECTION: Lazy<Mutex<Connection>> = Lazy::new(|| {
    println!("initializing");
    Mutex::new(Connection::open_in_memory().unwrap())
});

pub fn _get_receipts() -> Vec<ReceiptEntity> {
    //let connection = sqlite::open(":memory:").unwrap();

    let _query = "
    CREATE TABLE Purchase (ReceiptId INTEGER, Id INTEGER, Description TEXT, UnitPrice REAL, Count INTEGER, TaxRate REAL, TaxExcluded BOOL, Usage INTEGER, PRIMARY KEY(ReceiptId, Id));
    INSERT INTO  Purchase (ReceiptId, Id, Description, UnitPrice, Count, TaxRate, TaxExcluded, Usage) VALUES (1001, 1, 'Item 001', 85, 1, 0.08, 0, 1);
    INSERT INTO  Purchase (ReceiptId, Id, Description, UnitPrice, Count, TaxRate, TaxExcluded, Usage) VALUES (1002, 1, 'Item 002', 2280, 1, 0.08, 1, 2);
    INSERT INTO  Purchase (ReceiptId, Id, Description, UnitPrice, Count, TaxRate, TaxExcluded, Usage) VALUES (1002, 2, 'Item 003', 69, 1, 0.08, 1, 2);
    INSERT INTO  Purchase (ReceiptId, Id, Description, UnitPrice, Count, TaxRate, TaxExcluded, Usage) VALUES (1002, 3, 'Item 004', 198, 1, 0.08, 1, 2);
    INSERT INTO  Purchase (ReceiptId, Id, Description, UnitPrice, Count, TaxRate, TaxExcluded, Usage) VALUES (1002, 4, 'Item 005', 198, 1, 0.08, 1, 2);

    CREATE TABLE PaymentMethod (Id INTEGER PRIMARY KEY AUTOINCREMENT, Description TEXT, IsCredit BOOL);
    INSERT INTO  PaymentMethod VALUES (1, 'V15A', FALSE);
    INSERT INTO  PaymentMethod VALUES (2, 'M@5terC@rd', FALSE);

    CREATE TABLE Store (id INTEGER PRIMARY KEY AUTOINCREMENT, description TEXT);
    INSERT INTO  Store VALUES (1, 'A store');
    INSERT INTO  Store VALUES (2, 'Another store');

    CREATE TABLE users (name TEXT, age INTEGER);
    INSERT INTO users VALUES ('Alice', 42);
    INSERT INTO users VALUES ('Bob', 69);
";
    let mut receipt_list = vec![];
    let connection = CONNECTION.lock().unwrap();

    //connection.execute(query, ()).unwrap();

    let query = "SELECT * FROM receipt ";
    let mut statement = connection.prepare(query).unwrap();

    let entity_iter = statement
        .query_map([], |row| {
            let e = ReceiptEntity {
                id: row.get::<&str, u32>("id").unwrap(),
                datetime: row.get::<&str, i64>("datetime").unwrap(),
                store_key: 0,
                currency_id: 0,
                paid_amount: 0,
                payment_method_key: 0,
            };
            //println!("{:#?}", &e);
            Ok(e)
        })
        .unwrap();

    for e in entity_iter {
        receipt_list.push(e.unwrap());
    }

    receipt_list
}

pub fn add_receipt() {
    let query = "INSERT INTO receipt (datetime, store_key, currency_key, paid_amount, payment_method_key) VALUES (1725269353, 2, 392, 2964, 1);";
    CONNECTION.lock().unwrap().execute(query, ()).unwrap();
}

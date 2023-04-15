pub mod testdatagenerator;

use std::sync::Mutex;

use crate::entities::ReceiptEntity;
use once_cell::sync::Lazy;
use rusqlite::Connection;

// data access layer

static CONNECTION: Lazy<Mutex<Connection>> = Lazy::new(|| {
    println!("initializing");
    Mutex::new(Connection::open_in_memory().unwrap())
});

pub fn get_receipts() -> Vec<ReceiptEntity> {
    //let connection = sqlite::open(":memory:").unwrap();

    let query = "
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
                id: row.get(0).unwrap(),
                datetime: row.get(1).unwrap(),
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

pub fn get_receipt_count() -> u32 {
    let query = "SELECT COUNT(id) from receipt;";
    match CONNECTION
        .lock()
        .unwrap()
        .query_row(query, [], |row| row.get::<usize, u32>(0))
    {
        Ok(data) => {
            println!("{:}", data);
            data
        }
        Err(err) => {
            println!("An error detected: {:?}", err);
            0
        }
    }
}

pub fn get_receipt(position: u32) -> Result<ReceiptEntity, rusqlite::Error> {
    let query = "SELECT * FROM receipt WHERE id=?1;";

    CONNECTION
        .lock()
        .unwrap()
        .query_row(query, [position], |row| {
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

use rand::{distributions::Uniform, prelude::Distribution, Rng};

use super::*;

pub fn generate_test_receipt_data() {
    let connection = CONNECTION.lock().unwrap();
    let query = "
    DROP TABLE IF EXISTS receipt;
    CREATE TABLE receipt (id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL, datetime INTEGER, store_key INTEGER, currency_key INTEGER, paid_amount INTEGER, payment_method_key INTEGER);
";
    connection.execute_batch(query).unwrap();

    let query = "INSERT INTO receipt (datetime, store_key, currency_key, paid_amount, payment_method_key) VALUES (:datetime, :store_key, 392, :paid_amount, :payment_method_key);";
    let mut statement = connection.prepare(query).unwrap();

    let date_start = 1577804400;
    let date_end = 1672498799;
    let num_entry = 100_000;
    let num_store = 100_000;

    let mut rng = rand::thread_rng();
    let between = Uniform::from(0..(num_store - 1));

    for i in 0..(num_entry - 1) {
        statement
            .execute(&[
                (
                    ":datetime",
                    &(date_start + ((date_end - date_start) as i64 * i / num_entry)),
                ),
                (":store_key", &between.sample(&mut rng)),
                (":paid_amount", &2500),
                (":payment_method_key", &2500),
            ])
            .unwrap();
    }
}

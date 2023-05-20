use std::{sync::Mutex, time::Instant};

use crate::{database::receiptlistitem::ReceiptEntityObject, entities::ReceiptEntity};
use once_cell::sync::Lazy;
use rusqlite::Connection;

use super::SortOrder;
use std::collections::HashMap;

// data access layer

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
    ) -> Vec<HashMap<String, String>>;
}

pub struct SqlRow {}

pub struct ReceiptSqlDataAccessor {
    //table_name: &'static str,
    data_accessor: SqlDataAccessor,
}

impl SqlDataAccessorImpl for SqlDataAccessor {
    fn fetch_rows(
        &self,
        offset: u32,
        row_count: Option<u32>,
        sort_by: &Vec<(std::string::String, SortOrder)>,
    ) -> Vec<HashMap<String, String>> {
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
        let con = super::CONNECTION.lock().unwrap();
        let mut stmt = con.prepare(&query).unwrap();
        let mut columns: Vec<String> = vec![];
        for k in stmt.column_names() {
            columns.push(String::from(k));
        }
        for row in stmt
            .query_map([], |r| {
                let mut map = std::collections::HashMap::<String, String>::new();
                for k in &columns {
                    map.insert(String::from(k), r.get_unwrap(String::from(k).as_str()));
                }
                Ok(map)
            })
            .unwrap()
        {
            rows.push(row.unwrap());
        }
        rows
    }
}

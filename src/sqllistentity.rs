use glib::{Object, ParamSpec, ParamSpecInt64, ParamSpecUInt, Value};
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use once_cell::sync::Lazy;

mod imp {
    use super::*;
    use std::{cell::RefCell, collections::HashMap};

    // Object holding the state
    #[derive(Default)]
    pub struct SqlListEntity {
        columns: RefCell<HashMap<String, String>>,
    }

    // The central trait for subclassing a GObject
    #[glib::object_subclass]
    impl ObjectSubclass for SqlListEntity {
        const NAME: &'static str = "MercurySqlListEntity";
        type Type = super::SqlListEntity;
    }

    impl SqlListEntity {
        pub fn get(&self, key: &str) -> Option<&String> {
            self.columns.borrow().get(key)
        }

        pub fn get_mut(&self, key: &str) -> Option<&mut String> {
            self.columns.borrow().get_mut(key)
        }

        pub fn set(&self, key: &str, value: &str) {
            let value = String::from(value);
            self.columns
                .borrow()
                .entry(String::from(key))
                .and_modify(|v| *v = value)
                .or_insert(value);
        }
    }

    // Trait shared by all GObjects
    impl ObjectImpl for SqlListEntity {
        fn constructed(&self) {
            // Call "constructed" on parent
            self.parent_constructed();
            self.columns.replace(HashMap::new());
        }
    }
}

glib::wrapper! {
    pub struct SqlListEntity(ObjectSubclass<imp::SqlListEntity>);
}

// let id = row.get::<&str, u32>("id")?;
// let datetime = row.get::<&str, i64>("datetime")?;
// let store_key = row.get::<&str, u32>("store_key")?;
// let currency_key = row.get::<&str, u32>("currency_key")?;
// let paid_amount = row.get::<&str, u32>("paid_amount")?;
// let payment_method_key = row.get::<&str, u32>("payment_method_key")?;
// Ok(ReceiptEntity {
//     id: id,
//     datetime: datetime,
//     store_key: store_key,
//     currency_id: currency_key,
//     paid_amount: paid_amount,
//     payment_method_key: payment_method_key,
// })

impl SqlListEntity {
    pub fn new(row: &rusqlite::Row) -> Self {
        let obj: Self = Object::builder().build();
        for c in row. {
            obj.imp().set(c.0, c.1);
        }
        obj
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.imp().get(key)
    }

    pub fn get_mut(&self, key: &str) -> Option<&mut String> {
        self.imp().get_mut(key)
    }
}

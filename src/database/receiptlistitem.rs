use super::sqlentity::EntityFromSql;
use super::sqlentity::*;
use crate::entities::ReceiptEntityColumn;
use glib::{Object, ParamSpec, ParamSpecInt64, ParamSpecUInt, Value};
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use once_cell::sync::Lazy;
use std::cell::Cell;

mod imp {

    use super::*;

    // Object holding the state
    #[derive(Default)]
    pub struct ReceiptEntityObject {
        pub id: Cell<u32>,
        pub datetime: Cell<i64>,
        pub paid_amount: Cell<u32>,
    }

    // The central trait for subclassing a GObject
    #[glib::object_subclass]
    impl ObjectSubclass for ReceiptEntityObject {
        const NAME: &'static str = "MercuryReceiptEntityObject";
        type Type = super::ReceiptEntityObject;
        type ParentType = SqlEntity;
    }

    // impl ReceiptEntityObject {
    //     pub fn set_id(&self, id: u32) {
    //         self.id.replace(id);
    //     }

    //     pub fn set_datetime(&self, datetime: i64) {
    //         self.datetime.replace(datetime);
    //     }
    // }

    // Trait shared by all GObjects
    impl ObjectImpl for ReceiptEntityObject {
        fn properties() -> &'static [ParamSpec] {
            static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
                vec![
                    ParamSpecUInt::builder("id").build(),
                    ParamSpecInt64::builder("datetime").build(),
                    ParamSpecUInt::builder("storekey").build(),
                    ParamSpecUInt::builder("currencyid").build(),
                    ParamSpecUInt::builder("paidamount").build(),
                    ParamSpecUInt::builder("paymentmethodkey").build(),
                ]
            });
            PROPERTIES.as_ref()
        }

        fn set_property(&self, _id: usize, value: &Value, pspec: &ParamSpec) {
            match pspec.name() {
                "id" => self
                    .id
                    .set(value.get().expect("The value needs to be of type `u32`.")),
                "datetime" => self
                    .datetime
                    .set(value.get().expect("The value needs to be of type `i64`.")),
                // "storekey" => {
                //     self.entity.borrow_mut().store_key =
                //         value.get().expect("The value needs to be of type `u32`.")
                // }
                // "currencyid" => {
                //     self.entity.borrow_mut().currency_id =
                //         value.get().expect("The value needs to be of type `u32`.")
                // }
                "paidamount" => self
                    .paid_amount
                    .set(value.get().expect("The value needs to be of type `u32`.")),
                // "paymentmethodkey" => {
                //     self.entity.borrow_mut().payment_method_key =
                //         value.get().expect("The value needs to be of type `u32`.")
                // }
                _ => unimplemented!(),
            }
        }

        fn property(&self, _id: usize, pspec: &ParamSpec) -> Value {
            match pspec.name() {
                "id" => self.id.get().to_value(),
                "datetime" => self.datetime.get().to_value(),
                // "storekey" => self.entity.borrow().store_key.to_value(),
                // "currencyid" => self.entity.borrow().currency_id.to_value(),
                "paidamount" => self.paid_amount.get().to_value(),
                // "paymentmethodkey" => self.entity.borrow().payment_method_key.to_value(),
                _ => unimplemented!(),
            }
        }
    }

    impl SqlEntityImpl for ReceiptEntityObject {}
}

glib::wrapper! {
    pub struct ReceiptEntityObject(ObjectSubclass<imp::ReceiptEntityObject>) @extends SqlEntity;
}

impl ReceiptEntityObject {
    pub fn new() -> Self {
        Object::builder().build()
    }
}

impl EntityFromSql for ReceiptEntityObject {
    fn try_new_from_row(row: &rusqlite::Row) -> Result<Self, rusqlite::Error> {
        let obj: Self = Object::builder().build();
        obj.imp()
            .id
            .set(row.get(ReceiptEntityColumn::Id.to_string())?);
        obj.imp()
            .datetime
            .set(row.get(ReceiptEntityColumn::Datetime.to_string())?);
        obj.imp()
            .paid_amount
            .set(row.get(ReceiptEntityColumn::PaidAmount.to_string())?);
        Ok(obj)
    }
}

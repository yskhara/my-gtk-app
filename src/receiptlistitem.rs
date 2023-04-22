use glib::{Object, ParamSpec, ParamSpecInt64, ParamSpecUInt, Value};
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use once_cell::sync::Lazy;

use crate::entities::ReceiptEntity;

mod imp {
    use super::*;
    use std::{cell::RefCell, rc::Rc, sync::Arc};

    // Object holding the state
    #[derive(Default)]
    pub struct ReceiptEntityObject {
        entity: Arc<RefCell<ReceiptEntity>>,
    }

    // The central trait for subclassing a GObject
    #[glib::object_subclass]
    impl ObjectSubclass for ReceiptEntityObject {
        const NAME: &'static str = "MercuryReceiptEntityObject";
        type Type = super::ReceiptEntityObject;
    }

    impl ReceiptEntityObject {
        pub fn set_entity(&self, entity: ReceiptEntity) {
            self.entity.replace(entity);
        }
    }

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
                "id" => {
                    self.entity.borrow_mut().id =
                        value.get().expect("The value needs to be of type `u32`.")
                }
                "datetime" => {
                    self.entity.borrow_mut().datetime =
                        value.get().expect("The value needs to be of type `i64`.")
                }
                "storekey" => {
                    self.entity.borrow_mut().store_key =
                        value.get().expect("The value needs to be of type `u32`.")
                }
                "currencyid" => {
                    self.entity.borrow_mut().currency_id =
                        value.get().expect("The value needs to be of type `u32`.")
                }
                "paidamount" => {
                    self.entity.borrow_mut().paid_amount =
                        value.get().expect("The value needs to be of type `u32`.")
                }
                "paymentmethodkey" => {
                    self.entity.borrow_mut().payment_method_key =
                        value.get().expect("The value needs to be of type `u32`.")
                }
                _ => unimplemented!(),
            }
        }

        fn property(&self, _id: usize, pspec: &ParamSpec) -> Value {
            match pspec.name() {
                "id" => self.entity.borrow().id.to_value(),
                "datetime" => self.entity.borrow().datetime.to_value(),
                "storekey" => self.entity.borrow().store_key.to_value(),
                "currencyid" => self.entity.borrow().currency_id.to_value(),
                "paidamount" => self.entity.borrow().paid_amount.to_value(),
                "paymentmethodkey" => self.entity.borrow().payment_method_key.to_value(),
                _ => unimplemented!(),
            }
        }
    }
}

glib::wrapper! {
    pub struct ReceiptEntityObject(ObjectSubclass<imp::ReceiptEntityObject>);
}

impl ReceiptEntityObject {
    pub fn new(entity: ReceiptEntity) -> Self {
        let obj: Self = Object::builder().build();
        obj.imp().set_entity(entity);
        obj
    }
}

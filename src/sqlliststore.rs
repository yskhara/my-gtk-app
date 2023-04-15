use gtk::{gio, glib, prelude::ObjectExt};

mod imp {
    use crate::dal;
    use crate::receiptlistitem::ReceiptEntityObject;
    use glib::{BindingFlags, ParamSpec, Properties, Value};
    use gtk::prelude::*;
    use gtk::subclass::prelude::*;
    use gtk::{gio, glib};

    // Object holding the state
    #[derive(Default)]
    pub struct SqlListStore {
        list: Vec<u32>,
    }

    // The central trait for subclassing a GObject
    #[glib::object_subclass]
    impl ObjectSubclass for SqlListStore {
        const NAME: &'static str = "SqlListStore";
        type Type = super::SqlListStore;
        type Interfaces = (gio::ListModel,);
        //type ParentType = glib::Object;
    }

    // Trait shared by all GObjects
    impl ObjectImpl for SqlListStore {}

    // Trait shared by all ListModels
    impl ListModelImpl for SqlListStore {
        fn item_type(&self) -> glib::types::Type {
            ReceiptEntityObject::static_type()
        }
        fn n_items(&self) -> u32 {
            dal::get_receipt_count()
        }
        fn item(&self, position: u32) -> Option<glib::Object> {
            match dal::get_receipt(position) {
                Ok(entity) => Some(ReceiptEntityObject::new(entity).upcast()),
                Err(e) => {
                    println!("{:?}", e);
                    None
                }
            }
        }
    }
}

glib::wrapper! {
    pub struct SqlListStore(ObjectSubclass<imp::SqlListStore>)
        @implements gio::ListModel;
}

impl SqlListStore {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }

    pub fn set_connection(&self) {
        //self.inner
    }
}

impl Default for SqlListStore {
    fn default() -> Self {
        Self::new()
    }
}


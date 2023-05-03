use gtk::subclass::prelude::*;
use gtk::{gio, glib};

use crate::sqlliststore::{SqlListStore, SqlListStoreExt};
use gtk::prelude::*;

mod imp {
    use crate::dal;
    use crate::entities::ReceiptEntity;
    use crate::receiptlistitem::ReceiptEntityObject;
    use crate::sqlliststore::SqlListStoreImpl;

    use super::*;
    use std::cell::RefCell;
    use std::ops::Deref;

    // ANCHOR: object
    // Object holding the state
    #[derive(Default)]
    pub struct ReceiptListStore {}
    // ANCHOR_END: object

    // ANCHOR: subclass
    // The central trait for subclassing a GObject
    #[glib::object_subclass]
    impl ObjectSubclass for ReceiptListStore {
        // `NAME` needs to match `class` attribute of template
        const NAME: &'static str = "MercuryReceiptListStore";
        type Type = super::ReceiptListStore;
        type ParentType = SqlListStore;
        type Interfaces = (gio::ListModel,);
    }
    // ANCHOR_END: subclass

    // ANCHOR: object_impl
    // Trait shared by all GObjects
    impl ObjectImpl for ReceiptListStore {
        fn constructed(&self) {
            // Call "constructed" on parent
            self.parent_constructed();
        }
    }
    // ANCHOR_END: object_impl
    impl ReceiptListStore {}

    // Trait shared by all ListModels
    impl ListModelImpl for ReceiptListStore {
        fn item_type(&self) -> glib::Type {
            todo!()
        }

        fn n_items(&self) -> u32 {
            todo!()
        }

        fn item(&self, position: u32) -> Option<glib::Object> {
            todo!()
        }
    }

    // Trait shared by all SqlListStores
    impl SqlListStoreImpl for ReceiptListStore {}
}

glib::wrapper! {
    pub struct ReceiptListStore(ObjectSubclass<imp::ReceiptListStore>)
        @extends SqlListStore,
        @implements gio::ListModel;
}

impl ReceiptListStore {
    pub fn new(
        table_name: &str,
        item_type: glib::types::Type,
        sorter: Option<gtk::Sorter>,
    ) -> Self {
        let obj: Self = glib::Object::builder().build();
        obj.set_sorter(sorter);
        obj
    }
}

impl Default for ReceiptListStore {
    fn default() -> Self {
        Self::new("", glib::Object::static_type(), None)
    }
}

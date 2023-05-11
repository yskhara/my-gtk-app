use gtk::subclass::prelude::*;
use gtk::{gio, glib};

use crate::sqlliststore::{SqlListStore, SqlListStoreExt};
use gtk::prelude::*;

mod imp {
    use crate::receiptlistitem::ReceiptEntityObject;
    use crate::sqlliststore::{SqlListStoreWorker, SqlListStoreImplManual};

    use super::*;
    use std::cell::RefCell;

    // ANCHOR: object
    // Object holding the state
    #[derive(Default)]
    pub struct ReceiptListStore {
        worker: RefCell<SqlListStoreWorker<<Self as SqlListStoreImplManual>::Entity>>,
        is_fetch_scheduled: RefCell<bool>,
    }
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
    impl SqlListStoreImplManual for ReceiptListStore {
        type Entity = ReceiptEntityObject;

        fn borrow_worker(&self) -> std::cell::Ref<SqlListStoreWorker<Self::Entity>>
        {
            self.worker.borrow()
        }

        fn borrow_worker_mut(&self) -> std::cell::RefMut<SqlListStoreWorker<Self::Entity>>
        {
            self.worker.borrow_mut()
        }

        fn get_is_fetch_scheduled(&self) -> bool {
            *self.is_fetch_scheduled.borrow()
        }

        fn set_is_fetch_scheduled(&self, is_fetch_scheduled: bool) {
            *self.is_fetch_scheduled.borrow_mut() = is_fetch_scheduled;
        }
    }
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

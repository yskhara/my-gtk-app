use super::receiptlistitem::ReceiptEntityObject;
use super::sqlliststore::*;
use super::sqlliststoreworker::SqlListStoreWorker;
use super::sqlliststore::SqlListStoreImpl;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gio, glib};
use std::cell::RefCell;

mod imp {
    use super::*;

    // ANCHOR: object
    // Object holding the state
    #[derive(Default)]
    pub struct IncomeListStore {
        worker: RefCell<SqlListStoreWorker<<Self as SqlListStoreImplManual>::Entity>>,
        is_fetch_scheduled: RefCell<bool>,
    }
    // ANCHOR_END: object

    // ANCHOR: subclass
    // The central trait for subclassing a GObject
    #[glib::object_subclass]
    impl ObjectSubclass for IncomeListStore {
        // `NAME` needs to match `class` attribute of template
        const NAME: &'static str = "MercuryIncomeListStore";
        type Type = super::IncomeListStore;
        //type ParentType = SqlListStore;
        type Interfaces = (gio::ListModel,);
    }
    // ANCHOR_END: subclass

    // ANCHOR: object_impl
    // Trait shared by all GObjects
    impl ObjectImpl for IncomeListStore {
        fn constructed(&self) {
            // Call "constructed" on parent
            self.parent_constructed();
            self.worker.borrow_mut().set_table_name(Self::TABLE_NAME.to_string());
            self.worker.borrow_mut().update_sort_by();
            self.begin_polling();
        }
    }
    // ANCHOR_END: object_impl
    impl IncomeListStore {}

    // Trait shared by all ListModels
    impl ListModelImpl for IncomeListStore {
        fn item_type(&self) -> glib::types::Type {
            <Self as SqlListStoreImplManual>::Entity::static_type()
        }

        fn n_items(&self) -> u32 {
            self.worker.borrow().n_items()
        }

        fn item(&self, position: u32) -> Option<glib::Object> {
            self.worker.borrow_mut().item(position)
        }
    }

    // Trait shared by all SqlListStores
    impl SqlListStoreImplManual for IncomeListStore {
        type Entity = ReceiptEntityObject;
        const TABLE_NAME: &'static str = "income";

        fn borrow_worker(&self) -> std::cell::Ref<SqlListStoreWorker<Self::Entity>> {
            self.worker.borrow()
        }

        fn borrow_worker_mut(&self) -> std::cell::RefMut<SqlListStoreWorker<Self::Entity>> {
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
    pub struct IncomeListStore(ObjectSubclass<imp::IncomeListStore>)
        //@extends SqlListStore,
        @implements gio::ListModel;
}

impl IncomeListStore {
    pub fn new(sorter: Option<gtk::Sorter>) -> Self {
        let obj: Self = glib::Object::builder().build();
        obj.set_sorter(sorter);
        obj
    }
}

impl Default for IncomeListStore {
    fn default() -> Self {
        Self::new(None)
    }
}

impl SqlListStoreExtManual for IncomeListStore {}

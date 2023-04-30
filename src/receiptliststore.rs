use gtk::subclass::prelude::*;
use gtk::{gio, glib};

use crate::sqlliststore::SqlListStore;
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
        obj.imp()
            .parent_items_changed
            .replace(Some(std::boxed::Box::new(
                glib::clone!(@weak obj => move |position, removed, added| {
                    obj.items_changed(position, removed, added);
                }),
            )));
        obj
    }

    pub fn on_sorter_changed(&self, sorter: &gtk::Sorter, _: gtk::SorterChange) {
        let (position, removed, added) = self.imp().on_sorter_changed();
        self.items_changed(position, removed, added);
    }

    pub fn set_sorter(&self, sorter: Option<gtk::Sorter>) {
        // FIXME: dont just unwrap; care for None.
        if let Some(sorter) = sorter.clone() {
            sorter.connect_changed(glib::clone!(@weak self as sf => move |sorter, change| {
                sf.on_sorter_changed(sorter, change)
            }));
        }
        self.imp().sorter.replace(sorter);
    }

    pub fn get_sorter(&self) -> Option<gtk::Sorter> {
        self.imp().sorter.borrow().clone()
    }
}

impl Default for ReceiptListStore {
    fn default() -> Self {
        Self::new("", glib::Object::static_type(), None)
    }
}

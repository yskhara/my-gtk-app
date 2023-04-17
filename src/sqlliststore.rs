use std::borrow::Borrow;

use gtk::{
    gio, glib, prelude::ObjectExt, subclass::prelude::ObjectSubclassIsExt, traits::SorterExt,
};

mod imp {
    use std::cell::RefCell;

    use crate::dal;
    use crate::receiptlistitem::ReceiptEntityObject;
    use glib::{BindingFlags, ParamSpec, Properties, Value};
    use gtk::prelude::*;
    use gtk::subclass::prelude::*;
    use gtk::{gio, glib};

    // Object holding the state
    #[derive(Default)]
    pub struct SqlListStore {
        index_cache: RefCell<Vec<u32>>,
        pub sorter: RefCell<Option<gtk::Sorter>>,
    }

    // The central trait for subclassing a GObject
    #[glib::object_subclass]
    impl ObjectSubclass for SqlListStore {
        const NAME: &'static str = "SqlListStore";
        type Type = super::SqlListStore;
        type Interfaces = (gio::ListModel,);
    }

    impl SqlListStore {
        pub fn on_sorter_changed(&self, sorter: &gtk::Sorter, _: gtk::SorterChange) {
            println!("sorter was changed: ");
            println!("{:?}", sorter.downcast::<gtk::ColumnViewSorter>());
            //self.update_index_cache();
        }

        pub fn update_index_cache(&self) {
            let mut sort_by = None;
            if let Some(sorter) = self.sorter.borrow().clone() {
                if let Ok(sorter) = sorter.downcast::<gtk::NumericSorter>() {
                    let order = match sorter.sort_order() {
                        gtk::SortType::Ascending => dal::SortOrder::Ascending,
                        gtk::SortType::Descending => dal::SortOrder::Descending,
                        _ => dal::SortOrder::Ascending,
                    };
                    sort_by = Some(vec![(dal::ReceiptEntityColumn::Id, order)]);
                }
            }
            self.index_cache
                .replace(dal::get_receipts_id(sort_by).unwrap());
        }
    }

    // Trait shared by all GObjects
    impl ObjectImpl for SqlListStore {
        fn constructed(&self) {
            // Call "constructed" on parent
            self.parent_constructed();

            self.update_index_cache();
        }
    }

    // Trait shared by all ListModels
    impl ListModelImpl for SqlListStore {
        fn item_type(&self) -> glib::types::Type {
            ReceiptEntityObject::static_type()
        }

        fn n_items(&self) -> u32 {
            dal::get_receipt_count()
        }

        fn item(&self, position: u32) -> Option<glib::Object> {
            match dal::get_receipt(self.index_cache.borrow()[position as usize]) {
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
    pub fn new(sorter: Option<gtk::Sorter>) -> Self {
        let obj: Self = glib::Object::builder().build();
        obj.set_sorter(sorter);
        obj
    }

    pub fn set_sorter(&self, sorter: Option<gtk::Sorter>) {
        // FIXME: dont just unwrap; care for None.
        let imp = self.imp();
        if let Some(sorter) = sorter.clone() {
            sorter.connect_changed(glib::clone!(@weak imp => move |sorter, change| {
                imp.on_sorter_changed(sorter, change)
            }));
        }
        self.imp().sorter.replace(sorter);
    }

    pub fn get_sorter(&self) -> Option<gtk::Sorter> {
        self.imp().sorter.borrow().clone()
    }
}

impl Default for SqlListStore {
    fn default() -> Self {
        Self::new(None)
    }
}

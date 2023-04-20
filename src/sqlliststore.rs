use gtk::{
    gio,
    glib,
    prelude::{Cast, ListModelExt},
    subclass::prelude::ObjectSubclassIsExt,
    traits::SorterExt,
};

mod imp {
    use std::cell::RefCell;

    use crate::dal;
    use crate::receiptlistitem::ReceiptEntityObject;
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
        const NAME: &'static str = "MercurySqlListStore";
        type Type = super::SqlListStore;
        type Interfaces = (gio::ListModel,);
    }

    impl SqlListStore {
        pub fn update_index_cache(&self) -> (u32, u32, u32) {
            let mut sort_by = vec![];
            if let Some(sorter) = self.sorter.borrow().clone() {
                if let Ok(sorter) = sorter.downcast::<gtk::ColumnViewSorter>() {
                    for position in 0..sorter.n_sort_columns() {
                        let (column, order) = sorter.nth_sort_column(position);
                        if let Some(column) = column {
                            if let Some(column) =
                                dal::ReceiptEntityColumn::from_string(column.id().unwrap().as_str())
                            {
                                let order = match order {
                                    gtk::SortType::Ascending => dal::SortOrder::Ascending,
                                    gtk::SortType::Descending => dal::SortOrder::Descending,
                                    _ => dal::SortOrder::Ascending,
                                };
                                sort_by.push((column, order));
                            }
                        }
                    }
                }
            }
            self.index_cache.replace(
                dal::get_receipts_id(if sort_by.len() == 0 {
                    None
                } else {
                    Some(sort_by)
                })
                .unwrap(),
            );
            let len: u32 = {
                let c = self.index_cache.borrow();
                u32::try_from(c.len()).unwrap()
            };
            (0, len, len)
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
            // TODO: load several entries at once
            // consider using "call this function the next time gtk is idle" feature.
            // I think I saw an example of it somewhere in the SortListModel implementation.
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
    pub struct SqlListStore(ObjectSubclass<imp::SqlListStore>)//,subclass::basic::ClassStruct<imp::SqlListStore>>)
        @implements gio::ListModel;
        //match fn {
        //    type_ => || imp::SqlListStore::static_type().into_glib(),
        //}
}

impl SqlListStore {
    pub fn new(sorter: Option<gtk::Sorter>) -> Self {
        let obj: Self = glib::Object::builder().build();
        obj.set_sorter(sorter);
        obj
    }

    pub fn on_sorter_changed(&self, sorter: &gtk::Sorter, _: gtk::SorterChange) {
        println!("sorter was changed: ");
        println!(
            "{:?}",
            sorter
                .clone()
                .downcast::<gtk::ColumnViewSorter>()
                .unwrap()
                .nth_sort_column(0)
        );
        println!(
            "{:?}",
            sorter
                .clone()
                .downcast::<gtk::ColumnViewSorter>()
                .unwrap()
                .nth_sort_column(1)
        );
        let (position, removed, added) = self.imp().update_index_cache();
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

impl Default for SqlListStore {
    fn default() -> Self {
        Self::new(None)
    }
}

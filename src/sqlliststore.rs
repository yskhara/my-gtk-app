use gtk::{
    gio, glib,
    prelude::{Cast, ListModelExt},
    subclass::prelude::ObjectSubclassIsExt,
    traits::SorterExt,
};

mod imp {
    use std::cell::RefCell;
    use std::collections::HashMap;
    use std::time::Instant;

    use crate::dal::{self, ReceiptEntityColumn};
    use crate::entities::ReceiptEntity;
    use crate::receiptlistitem::ReceiptEntityObject;
    use gtk::prelude::*;
    use gtk::subclass::prelude::*;
    use gtk::{gio, glib};

    // Object holding the state
    #[derive(Default)]
    pub struct SqlListStore {
        //index_cache: RefCell<Vec<u32>>,
        //object_cache: RefCell<HashMap<u32, ReceiptEntityObject>>,
        object_cache: RefCell<Vec<u32>>,
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
        fn get_sort_by(&self) -> Vec<(dal::ReceiptEntityColumn, dal::SortOrder)> {
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
            sort_by
        }

        pub fn update_index_cache(&self) -> (u32, u32, u32) {
            let sort_by = self.get_sort_by();
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

        fn update_items(&self) {
            if let Ok(entities) = dal::get_receipts(Some(self.get_sort_by())) {
                
            }
        }
    }

    // Trait shared by all GObjects
    impl ObjectImpl for SqlListStore {
        fn constructed(&self) {
            // Call "constructed" on parent
            self.parent_constructed();

            // initialize object cache
            let id_list = dal::get_receipts_id(None).unwrap();
            let mut map = HashMap::with_capacity(id_list.len());
            for id in id_list {
                map.entry(id)
                    .or_insert(ReceiptEntityObject::new(ReceiptEntity::default()));
            }
            self.object_cache.replace(map);

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
            //let start = Instant::now();
            let receipt_id = self.index_cache.borrow()[position as usize];
            /*let res = match dal::get_receipt(receipt_id) {
                Ok(entity) => {
                    self.object_cache.borrow_mut().entry(receipt_id).and_modify(|obj| {obj.clone().set_entity(entity);}); // without thisline sorting all 100_000 items takes ~2sec.
                }
                Err(e) => {
                    println!("{:?}", e);
                    //None
                }
            };*/
            //println!("item() called: position: {:}; took {:?}.", position, start.elapsed());
            //res
            Some(self.object_cache.borrow()[&receipt_id].clone().upcast())
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

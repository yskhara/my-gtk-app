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
        object_cache: RefCell<Vec<ReceiptEntityObject>>,
        pub sorter: RefCell<Option<gtk::Sorter>>,
        sort_by: RefCell<Option<Vec<(dal::ReceiptEntityColumn, dal::SortOrder)>>>,
        pub parent_items_changed: RefCell<Option<Box<dyn Fn(u32, u32, u32)>>>,
        fetching_more: RefCell<bool>,
    }

    // The central trait for subclassing a GObject
    #[glib::object_subclass]
    impl ObjectSubclass for SqlListStore {
        const NAME: &'static str = "MercurySqlListStore";
        type Type = super::SqlListStore;
        type Interfaces = (gio::ListModel,);
    }

    impl SqlListStore {
        const DEFAULT_FETCH_LENGTH: u32 = 100;
        const DEFAULT_FETCH_MARGIN: u32 = 10;

        fn update_sort_by(&self) {
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
            if sort_by.len() == 0 {
                self.sort_by.replace(None);
            } else {
                self.sort_by.replace(Some(sort_by));
            }
        }

        pub fn on_sorter_changed(&self) -> (u32, u32, u32) {
            self.update_sort_by();
            // TODO: do not fetch here; just remove all items and report (0, <n_items()>, 0).
            let removed = u32::try_from(self.object_cache.borrow().len()).unwrap();
            self.object_cache.replace(vec![]);
            let added = u32::try_from(self.fetch_items()).unwrap();
            // positon, removed, added
            (0, removed, added)
        }

        fn fetch_items(&self) -> u32 {
            if let Ok(entities) = dal::get_receipts(
                Some(Self::DEFAULT_FETCH_LENGTH),
                Some(self.n_items()),
                &self.sort_by.borrow(),
            ) {
                let added = entities.len();
                for entity in entities {
                    self.object_cache
                        .borrow_mut()
                        .push(ReceiptEntityObject::new(entity));
                }
                added.try_into().unwrap()
            } else {
                0
            }
        }
    }

    // Trait shared by all GObjects
    impl ObjectImpl for SqlListStore {
        fn constructed(&self) {
            // Call "constructed" on parent
            self.parent_constructed();

            // initialize object cache with empty objects
            // let id_list = dal::get_receipts_id(None).unwrap();
            // let mut vec = Vec::new();
            // for index in 0..Self::DEFAULT_FETCH_LENGTH {
            //     vec.push(ReceiptEntityObject::new(ReceiptEntity::default()));
            // }
            self.object_cache.replace(vec![]);
            self.update_sort_by();
            self.fetch_items();
        }
    }

    // Trait shared by all ListModels
    impl ListModelImpl for SqlListStore {
        fn item_type(&self) -> glib::types::Type {
            ReceiptEntityObject::static_type()
        }

        fn n_items(&self) -> u32 {
            let r = TryInto::<u32>::try_into(self.object_cache.borrow().len()).unwrap();
            println!("n_items: {:}", r);
            r
        }

        fn item(&self, position: u32) -> Option<glib::Object> {
            if position >= (self.n_items() - Self::DEFAULT_FETCH_MARGIN) {
                // FIXME: do not fetch more when a fetch is already scheduled.
                // FIXME: 
                if !(*self.fetching_more.borrow()) {
                    self.fetching_more.replace(true);
                    println!("fetching more...");
                    glib::source::idle_add_local_once(glib::clone!(@weak self as sf => move || {
                        let pos = sf.n_items();
                        let added = sf.fetch_items();
                        sf.fetching_more.replace(false);
                        println!("fetching complete. {:?}", (pos, 0, added));
                        if let Some(f) = sf.parent_items_changed.borrow().as_deref() {
                            f(pos, 0, added);
                        };
                    }));
                }
            }

            println!("item requested for pos: {:}", position);

            Some(
                self.object_cache.borrow()[position as usize]
                    .clone()
                    .upcast(),
            )
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

impl Default for SqlListStore {
    fn default() -> Self {
        Self::new(None)
    }
}

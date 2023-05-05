use gtk::glib::clone::*;
use gtk::glib::subclass::types::*;
use gtk::glib::translate::ToGlibPtr;
use gtk::{gio, glib, prelude::*, subclass::prelude::*, traits::SorterExt};

mod ffi {
    use super::*;
    pub type SqlListStore = <super::imp::SqlListStore as ObjectSubclass>::Instance;

    #[repr(C)]
    pub struct SqlListStoreClass {
        pub parent_class: glib::gobject_ffi::GObjectClass,
        pub items_changed: Option<
            unsafe extern "C" fn(*mut SqlListStore, position: u32, removed: u32, added: u32),
        >,
    }

    unsafe impl ClassStruct for SqlListStoreClass {
        type Type = super::imp::SqlListStore;
    }

    #[no_mangle]
    pub unsafe extern "C" fn items_changed(
        this: *mut SqlListStore,
        position: u32,
        removed: u32,
        added: u32,
    ) {
        (*this).imp().obj().items_changed(position, removed, added)
    }
}

mod imp {
    use crate::dal;
    use crate::entities::ReceiptEntity;
    use crate::receiptlistitem::ReceiptEntityObject;
    use crate::sqlliststore::SqlListStoreImplExt;
    use gtk::prelude::*;
    use gtk::subclass::prelude::*;
    use gtk::{gio, glib};
    use std::cell::RefCell;
    use std::ops::Deref;

    use super::SqlListStoreImpl;

    // Object holding the state
    #[derive(Default)]
    pub struct SqlListStore {
        //index_cache: RefCell<Vec<u32>>,
        //object_cache: RefCell<HashMap<u32, ReceiptEntityObject>>,
        object_cache: RefCell<Vec<ReceiptEntityObject>>,
        pub sorter: RefCell<Option<gtk::Sorter>>,
        sort_by: RefCell<Vec<(String, dal::SortOrder)>>,
        //pub parent_items_changed: RefCell<Option<Box<dyn Fn(u32, u32, u32)>>>,
        fetching_more: RefCell<bool>,
        max_accessed_position: RefCell<u32>,
        //dao: Rc<RefCell<T>>,
        item_type: Option<glib::types::Type>,
    }

    // The central trait for subclassing a GObject
    #[glib::object_subclass]
    impl ObjectSubclass for SqlListStore {
        const NAME: &'static str = "MercurySqlListStore";
        type Type = super::SqlListStore;
        type Interfaces = (gio::ListModel,);
        type Class = super::ffi::SqlListStoreClass;
    
        fn class_init(klass: &mut Self::Class) {
            klass.items_changed = Some(super::ffi::items_changed);
        }
    }

    impl SqlListStore {
        const DEFAULT_FETCH_LENGTH: u32 = 100;
        const DEFAULT_FETCH_MARGIN: u32 = 1;

        fn update_sort_by(&self) {
            self.sort_by.borrow_mut().clear();

            if let Some(sorter) = self.sorter.borrow().clone() {
                if let Ok(sorter) = sorter.downcast::<gtk::ColumnViewSorter>() {
                    for position in 0..sorter.n_sort_columns() {
                        let (column, order) = sorter.nth_sort_column(position);
                        if let Some(column) = column {
                            if let Some(column_id) = column.id() {
                                let order = match order {
                                    gtk::SortType::Ascending => dal::SortOrder::Ascending,
                                    gtk::SortType::Descending => dal::SortOrder::Descending,
                                    _ => dal::SortOrder::Ascending,
                                };
                                self.sort_by
                                    .borrow_mut()
                                    .push((String::from(column_id.as_str()), order));
                            }
                        }
                    }
                }
            }
        }

        pub fn on_sorter_changed(&self) -> (u32, u32, u32) {
            self.update_sort_by();
            let removed = self.n_items_internal();
            self.object_cache.replace(vec![]);
            self.schedule_fetch_more();
            // positon, removed, added
            (0, removed, 0)
        }

        fn fetch_more(&self) -> u32 {
            match dal::DataAccessor::fetch_entities::<ReceiptEntityObject>(
                self.n_items_internal(),
                Some(Self::DEFAULT_FETCH_LENGTH),
                self.sort_by.borrow().deref(),
            ) {
                Ok(entities) => {
                    let added = entities.len();
                    /*rows.map(|r| r.try_into()).collect();
                    for entity in rows {
                        self.object_cache
                            .borrow_mut()
                            .push(ReceiptEntityObject::new(entity));
                    }*/
                    added.try_into().unwrap()
                }
                Err(e) => {
                    dbg!(e);
                    0
                }
            }
        }

        fn schedule_fetch_more(&self) {
            if !(*self.fetching_more.borrow()) {
                self.fetching_more.replace(true);
                println!("fetching more...");
                glib::source::idle_add_local_once(glib::clone!(@weak self as sf => move || {
                    let pos = sf.n_items_internal();
                    let added = sf.fetch_more();
                    sf.fetching_more.replace(false);
                    println!("fetching complete. {:?}", (pos, 0, added));
                    sf.items_changed(pos, 0, added);
                    println!("call to items_changed returned.");
                }));
            }
        }

        fn n_items_internal(&self) -> u32 {
            TryInto::<u32>::try_into(self.object_cache.borrow().len()).unwrap()
        }
    }

    // Trait shared by all GObjects
    impl ObjectImpl for SqlListStore {
        fn constructed(&self) {
            // Call "constructed" on parent
            self.parent_constructed();
            self.object_cache.replace(vec![]);
            self.sort_by.replace(vec![]);
            self.update_sort_by();
            self.schedule_fetch_more();
        }
    }

    // Trait shared by all ListModels
    impl ListModelImpl for SqlListStore {
        fn item_type(&self) -> glib::types::Type {
            ReceiptEntityObject::static_type()
        }

        fn n_items(&self) -> u32 {
            let r = self.n_items_internal();
            println!("n_items: {:}", r);
            r
        }

        fn item(&self, position: u32) -> Option<glib::Object> {
            println!("item requested for pos: {:}", position);
            if position > *self.max_accessed_position.borrow() {
                *self.max_accessed_position.borrow_mut() = position;
            }

            if (self.n_items_internal() < Self::DEFAULT_FETCH_MARGIN)
                || (position >= (self.n_items_internal() - Self::DEFAULT_FETCH_MARGIN))
            {
                // FIXME: do not fetch more when a fetch is already scheduled.
                // FIXME:
                if !(*self.fetching_more.borrow()) {
                    self.fetching_more.replace(true);
                    println!("fetching more...");
                    glib::source::idle_add_local_once(glib::clone!(@weak self as sf => move || {
                        let pos = sf.n_items_internal();
                        let added = sf.fetch_more();
                        sf.fetching_more.replace(false);
                        println!("fetching complete. {:?}", (pos, 0, added));
                        sf.items_changed(pos, 0, added);
                    }));
                }
            }

            if TryInto::<usize>::try_into(position).unwrap() >= self.object_cache.borrow().len() {
                None
            } else {
                Some(
                    self.object_cache.borrow()[position as usize]
                        .clone()
                        .upcast(),
                )
            }
        }
    }

    impl SqlListStoreImpl for SqlListStore {}
}

glib::wrapper! {
    pub struct SqlListStore(ObjectSubclass<imp::SqlListStore>)//,subclass::basic::ClassStruct<imp::SqlListStore>>)
        @implements gio::ListModel;
}

impl SqlListStore {
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

impl Default for SqlListStore {
    fn default() -> Self {
        Self::new("", glib::Object::static_type(), None)
    }
}

pub trait SqlListStoreExt: IsA<SqlListStore> + 'static {
    fn get_sorter(&self) -> Option<gtk::Sorter>;
    fn set_sorter(&self, sorter: Option<gtk::Sorter>);
    fn on_sorter_changed(&self, sorter: &gtk::Sorter, _: gtk::SorterChange);
}

impl<O: IsA<SqlListStore>> SqlListStoreExt for O {
    fn get_sorter(&self) -> Option<gtk::Sorter> {
        let this = self.as_ref();
        this.imp().sorter.borrow().clone()
    }

    fn set_sorter(&self, sorter: Option<gtk::Sorter>) {
        let this = self.as_ref();
        if let Some(sorter) = sorter.clone() {
            sorter.connect_changed(glib::clone!(@weak this => move |sorter, change| {
                this.on_sorter_changed(sorter, change);
            }));
        }
        this.imp().sorter.replace(sorter);
    }

    fn on_sorter_changed(&self, sorter: &gtk::Sorter, _: gtk::SorterChange) {
        let this = self.as_ref();
        let (position, removed, added) = this.imp().on_sorter_changed();
        this.items_changed(position, removed, added);
    }
}

pub trait SqlListStoreImpl: SqlListStoreImplExt + ObjectImpl {
    fn items_changed(&self, position: u32, removed: u32, added: u32) {
        self.parent_items_changed(position, removed, added);
    }
}

pub trait SqlListStoreImplExt {
    fn parent_items_changed(&self, position: u32, removed: u32, added: u32);
}

impl<T: SqlListStoreImpl> SqlListStoreImplExt for T {
    fn parent_items_changed(&self, position: u32, removed: u32, added: u32) {
        println!("calling parent_items_changed.");
        unsafe {
            let klass = self.as_ref().class();
            (klass.as_ref().set_a.unwrap())(
                self.as_ref().imp().instance().as_ptr() as *mut ffi::Foo,
                value,
            );
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::SqlListStoreClass;
            if let Some(f) = (*parent_class).items_changed {
                f(
                    self.obj()
                        .unsafe_cast_ref::<SqlListStore>()
                        .to_glib_none()
                        .0,
                    position,
                    removed,
                    added,
                )
            }
        }
    }
}

unsafe impl<T: SqlListStoreImpl> IsSubclassable<T> for SqlListStore {}

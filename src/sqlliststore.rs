use gtk::gio::ListModel;
use gtk::glib::subclass::types::*;
use gtk::{gio, glib, prelude::*, subclass::prelude::*, traits::SorterExt};

use crate::dal;

pub struct SqlListStoreWorker<E>
where
    E: dal::EntityFromSql + IsA<glib::Object>,
{
    object_cache: Vec<E>,
    sorter: Option<gtk::Sorter>,
    sort_by: Vec<(String, dal::SortOrder)>,
    max_accessed_position: u32,
    fetch_margin: u32,
    fetch_amount: u32,
}

impl<E> SqlListStoreWorker<E>
where
    E: dal::EntityFromSql + IsA<glib::Object>,
{
    const DEFAULT_FETCH_MARGIN: u32 = 200;
    const DEFAULT_FETCH_AMOUNT: u32 = 200;

    pub fn n_items(&self) -> u32 {
        TryInto::<u32>::try_into(self.object_cache.len()).unwrap()
    }

    pub fn clear_items(&mut self) -> u32 {
        let n_items = self.n_items();
        self.object_cache.clear();
        self.max_accessed_position = 0;
        n_items
    }

    pub fn fetch_more_if_necessary(&mut self) -> u32 {
        let n_items = self.n_items();
        if self.max_accessed_position + self.fetch_margin >= n_items {
            let n_items_to_fetch = u32::max(
                self.fetch_amount,
                self.max_accessed_position + self.fetch_margin - n_items + 1,
            );

            // FIXME: fetching takes too long when entities are sorted by datetime column
            match crate::dal::DataAccessor::fetch_entities::<E>(
                self.n_items(),
                Some(n_items_to_fetch),
                &self.sort_by,
            ) {
                Ok(mut entities) => {
                    let added = entities.len();
                    self.object_cache.append(&mut entities);
                    added.try_into().unwrap()
                }
                Err(e) => {
                    dbg!(e);
                    0
                }
            }
        } else {
            0
        }
    }

    pub fn item(&mut self, position: u32) -> Option<glib::Object> {
        //dbg!(position);
        if position > self.max_accessed_position {
            self.max_accessed_position = position;
        }

        if let Some(item) = self.object_cache.get(position as usize) {
            Some(item.clone().upcast())
        } else {
            None
        }
    }

    pub fn update_sort_by(&mut self) {
        self.sort_by.clear();

        if let Some(sorter) = self.sorter.clone() {
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
                            self.sort_by.push((String::from(column_id.as_str()), order));
                        }
                    }
                }
            }
        }
    }

    pub fn set_sorter(&mut self, sorter: Option<gtk::Sorter>) {
        self.sorter = sorter;
    }
}

impl<E> Default for SqlListStoreWorker<E>
where
    E: dal::EntityFromSql + IsA<glib::Object>,
{
    fn default() -> Self {
        Self {
            object_cache: Default::default(),
            sorter: Default::default(),
            sort_by: Default::default(),
            max_accessed_position: Default::default(),
            fetch_margin: Self::DEFAULT_FETCH_MARGIN,
            fetch_amount: Self::DEFAULT_FETCH_AMOUNT,
        }
    }
}

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
    use crate::receiptlistitem::ReceiptEntityObject;
    use gtk::prelude::*;
    use gtk::subclass::prelude::*;
    use gtk::{gio, glib};
    use std::cell::RefCell;

    use super::{SqlListStoreImpl, SqlListStoreImplManual, SqlListStoreWorker};

    // Object holding the state
    #[derive(Default)]
    pub struct SqlListStore {
        worker: RefCell<SqlListStoreWorker<<Self as SqlListStoreImplManual>::Entity>>,
        is_fetch_scheduled: RefCell<bool>,
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

    impl SqlListStore {}

    // Trait shared by all GObjects
    impl ObjectImpl for SqlListStore {
        fn constructed(&self) {
            // Call "constructed" on parent
            self.parent_constructed();
            self.worker.borrow_mut().update_sort_by();
            self.begin_polling();
        }
    }

    // Trait shared by all ListModels
    impl ListModelImpl for SqlListStore {
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

    impl SqlListStoreImplManual for SqlListStore {
        type Entity = ReceiptEntityObject;

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
    pub struct SqlListStore(ObjectSubclass<imp::SqlListStore>)
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
    fn set_sorter(&self, sorter: Option<gtk::Sorter>);
    fn on_sorter_changed(&self, sorter: &gtk::Sorter, _: gtk::SorterChange);
}

impl<O: IsA<SqlListStore>> SqlListStoreExt for O {
    fn set_sorter(&self, sorter: Option<gtk::Sorter>) {
        if let Some(sorter) = sorter.clone() {
            let sf = self.borrow();
            sorter.connect_changed(glib::clone!(@weak sf => move |sorter, sorter_change| {
                sf.on_sorter_changed(sorter, sorter_change);
            }));
        }
        self.as_ref().imp().borrow_worker_mut().set_sorter(sorter);
    }

    fn on_sorter_changed(&self, _sorter: &gtk::Sorter, _: gtk::SorterChange) {
        let this = self.as_ref();
        this.imp().on_sorter_changed();
    }
}

pub(crate) trait SqlListStoreImplManual:
    ListModelImpl + ObjectImpl + ObjectSubclass
{
    type Entity: crate::dal::EntityFromSql + IsA<glib::Object>;

    fn borrow_worker(&self) -> std::cell::Ref<SqlListStoreWorker<Self::Entity>>;
    fn borrow_worker_mut(&self) -> std::cell::RefMut<SqlListStoreWorker<Self::Entity>>;
    fn get_is_fetch_scheduled(&self) -> bool;
    fn set_is_fetch_scheduled(&self, is_fetch_scheduled: bool);
}
pub(crate) trait SqlListStoreImpl: ListModelImpl + ObjectImpl + ObjectSubclass {
    fn on_sorter_changed(&self);
    fn schedule_fetch_more(&self);
    fn begin_polling(&self);
}

impl<T> SqlListStoreImpl for T
where
    T: SqlListStoreImplManual + ListModelImpl + 'static,
    <T as ObjectSubclass>::Type: IsA<ListModel>,
{
    fn on_sorter_changed(&self) {
        self.borrow_worker_mut().update_sort_by();
        let removed = self.borrow_worker_mut().clear_items();
        // positon, removed, added
        self.obj().items_changed(0, removed, 0);
        self.schedule_fetch_more();
    }

    fn schedule_fetch_more(&self) {
        if !self.get_is_fetch_scheduled() {
            self.set_is_fetch_scheduled(true);
            let sf = self.ref_counted();
            glib::source::idle_add_local_once(glib::clone!(@weak sf => move || {
                let pos = sf.n_items();
                let added = sf.borrow_worker_mut().fetch_more_if_necessary();
                if added > 0 {
                    println!("fetching complete. {:?}", (pos, 0, added));
                    //sf.items_changed(pos, 0, added);
                    sf.obj().items_changed(pos, 0, added);
                    println!("call to items_changed returned.");
                }
                sf.set_is_fetch_scheduled(false);
            }));
        }
    }

    fn begin_polling(&self) {
        let sf = self.ref_counted();
        glib::source::timeout_add_local(
            std::time::Duration::from_millis(100),
            glib::clone!(@weak sf => @default-return glib::source::Continue(false), move || {
                sf.schedule_fetch_more();
                glib::source::Continue(true)
            }),
        );
        println!("polling started");
    }
}

unsafe impl<T: SqlListStoreImpl> IsSubclassable<T> for SqlListStore {}

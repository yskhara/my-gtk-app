use super::sqlentity::EntityFromSql;
use super::sqlliststoreworker::SqlListStoreWorker;
use gtk::gio::ListModel;
use gtk::glib::subclass::types::*;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gio, glib, traits::SorterExt};
use std::cell::RefCell;

mod imp {
    use super::*;

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
    }

    impl SqlListStore {}

    // Trait shared by all GObjects
    impl ObjectImpl for SqlListStore {
        fn constructed(&self) {
            // Call "constructed" on parent
            self.parent_constructed();
            self.worker
                .borrow_mut()
                .set_table_name(Self::TABLE_NAME.to_string());
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

    // Trait shared by all SqlListStores
    impl SqlListStoreImplManual for SqlListStore {
        type Entity = crate::database::receiptlistitem::ReceiptEntityObject;
        const TABLE_NAME: &'static str = "receipt";

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
    pub fn new(sorter: Option<gtk::Sorter>) -> Self {
        let obj: Self = glib::Object::builder().build();
        obj.set_sorter(sorter);
        obj
    }
}

impl Default for SqlListStore {
    fn default() -> Self {
        Self::new(None)
    }
}

pub trait SqlListStoreExtManual {}
impl SqlListStoreExtManual for SqlListStore {}

pub trait SqlListStoreExt:
ObjectType + glib::object::ObjectSubclassIs + SqlListStoreExtManual + 'static
{
    fn set_sorter(&self, sorter: Option<gtk::Sorter>);
}

impl<O> SqlListStoreExt for O
where
    O: ObjectType + glib::object::ObjectSubclassIs + SqlListStoreExtManual, <O as glib::object::ObjectSubclassIs>::Subclass: ObjectSubclass<Type = O> + SqlListStoreImpl
{
    fn set_sorter(&self, sorter: Option<gtk::Sorter>) {
        if let Some(sorter) = sorter.clone() {
            let imp = self.imp().ref_counted();
            sorter.connect_changed(glib::clone!(@weak imp => move |_, _| {
                imp.on_sorter_changed();
            }));
        }
        self.imp().borrow_worker_mut().set_sorter(sorter);
    }
}

pub(crate) trait SqlListStoreImplManual:
    ListModelImpl + ObjectImpl + ObjectSubclass
{
    type Entity: EntityFromSql + IsA<glib::Object>;
    const TABLE_NAME: &'static str;

    fn borrow_worker(&self) -> std::cell::Ref<SqlListStoreWorker<Self::Entity>>;
    fn borrow_worker_mut(&self) -> std::cell::RefMut<SqlListStoreWorker<Self::Entity>>;
    fn get_is_fetch_scheduled(&self) -> bool;
    fn set_is_fetch_scheduled(&self, is_fetch_scheduled: bool);
}
pub(crate) trait SqlListStoreImpl: SqlListStoreImplManual + ListModelImpl + ObjectImpl + ObjectSubclass {
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
        println!("sorter changed.");
        self.borrow_worker_mut().update_sort_by();
        let removed = self.borrow_worker_mut().clear_items();
        // positon, removed, added
        self.obj().items_changed(0, removed, 0);
        println!("{} items removed.", removed);
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

use gtk::glib::{self, Object};
use gtk::prelude::*;
use gtk::subclass::prelude::*;

mod imp {
    use super::*;

    // Object holding the state
    #[derive(Default)]
    pub struct SqlEntity {}

    // The central trait for subclassing a GObject
    #[glib::object_subclass]
    impl ObjectSubclass for SqlEntity {
        const NAME: &'static str = "MercurySqlEntity";
        type Type = super::SqlEntity;
    }

    // Trait shared by all GObjects
    impl ObjectImpl for SqlEntity {}
    impl SqlEntityImpl for SqlEntity {}
}

glib::wrapper! {
    pub struct SqlEntity(ObjectSubclass<imp::SqlEntity>);
}

impl SqlEntity {
    pub fn new() -> Self {
        Object::builder().build()
    }
}

pub trait SqlEntityImpl: ObjectImpl {}
unsafe impl<T: SqlEntityImpl> IsSubclassable<T> for SqlEntity {}

impl EntityFromSql for SqlEntity {
    fn try_new_from_row(_row: &rusqlite::Row) -> Result<Self, rusqlite::Error> {
        let obj: Self = Object::builder().build();
        Ok(obj)
    }
}

pub trait EntityFromSql: ObjectExt {
    fn try_new_from_row(row: &rusqlite::Row) -> Result<Self, rusqlite::Error>;
}

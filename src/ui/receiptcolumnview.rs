use chrono::{DateTime, NaiveDateTime, Utc};
use glib::subclass::InitializingObject;
use gtk::subclass::prelude::*;
use gtk::{
    gio, glib, prelude::*, Button, ColumnView, CompositeTemplate, Label, SignalListItemFactory,
    SingleSelection,
};
use num_format::{Locale, ToFormattedString};

use crate::database::prelude::*;

// ANCHOR: object
// Object holding the state
#[derive(Default)]
pub struct ReceiptColumnView {
}
// ANCHOR_END: object

// ANCHOR: subclass
// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for ReceiptColumnView {
    // `NAME` needs to match `class` attribute of template
    const NAME: &'static str = "MyGtkAppReceiptColumnView";
    type Type = super::ReceiptColumnView;
    type ParentType = gtk::Window;
}
// ANCHOR_END: subclass

// ANCHOR: object_impl
// Trait shared by all GObjects
impl ObjectImpl for ReceiptColumnView {
    fn constructed(&self) {
        // Call "constructed" on parent
        self.parent_constructed();

        self.update_receipt_list();
    }
}
// ANCHOR_END: object_impl

#[gtk::template_callbacks]
impl ReceiptColumnView {
    fn update_receipt_list(&self) {
    }

    #[template_callback]
    fn button_add_entry_click_handler(&self, _button: &gtk::Button) {
        //dal::add_receipt();
        self.update_receipt_list();
    }

    #[template_callback]
    fn column_text_setup_handler(_factory: &SignalListItemFactory, item: &glib::Object) {
        let item = item.downcast_ref::<gtk::ListItem>().unwrap();
        let label = Label::new(Some(""));
        item.set_child(Some(&label));

        //println!("setup()");
        //self.item_list.push(item);
    }

    #[template_callback]
    fn column_id_bind_handler(_factory: &SignalListItemFactory, item: &glib::Object) {
        let item = item.downcast_ref::<gtk::ListItem>().unwrap();
        let child = item.child().unwrap().downcast::<Label>().unwrap();
        let entry = item
            .item()
            .unwrap()
            .downcast::<ReceiptEntityObject>()
            .unwrap();
        child.set_text(&entry.property::<u32>("id").to_string());
        //entry.bind_property("id", &child, "label").build();
        //child.bind_property("source_property", target, target_property)
    }

    #[template_callback]
    fn column_datetime_bind_handler(_factory: &SignalListItemFactory, item: &glib::Object) {
        let item = item.downcast_ref::<gtk::ListItem>().unwrap();
        let child = item.child().unwrap().downcast::<Label>().unwrap();
        let entry = item
            .item()
            .unwrap()
            .downcast::<ReceiptEntityObject>()
            .unwrap();
        child.set_text(
            &DateTime::<Utc>::from_utc(
                NaiveDateTime::from_timestamp_opt(entry.property::<i64>("datetime"), 0).unwrap(),
                Utc,
            )
            .format("%Y-%m-%d %H:%m:%S")
            .to_string(),
        );
    }

    #[template_callback]
    fn column_amount_bind_handler(_factory: &SignalListItemFactory, item: &glib::Object) {
        let item = item.downcast_ref::<gtk::ListItem>().unwrap();
        let child = item.child().unwrap().downcast::<Label>().unwrap();
        let entry = item
            .item()
            .unwrap()
            .downcast::<ReceiptEntityObject>()
            .unwrap();
        child.set_text(format!("JPY {}", &entry.property::<u32>("paid-amount").to_formatted_string(&Locale::en)).as_str());
        //entry.bind_property("id", &child, "label").build();
        //child.bind_property("source_property", target, target_property)
    }

    #[template_callback]
    fn column_store_bind_handler(_factory: &SignalListItemFactory, item: &glib::Object) {
        let item = item.downcast_ref::<gtk::ListItem>().unwrap();
        let child = item.child().unwrap().downcast::<Label>().unwrap();
        let entry = item
            .item()
            .unwrap()
            .downcast::<ReceiptEntityObject>()
            .unwrap();
        child.set_text(&entry.property::<u32>("store-key").to_string());
        //entry.bind_property("id", &child, "label").build();
        //child.bind_property("source_property", target, target_property)
    }

    #[template_callback]
    fn column_method_bind_handler(_factory: &SignalListItemFactory, item: &glib::Object) {
        let item = item.downcast_ref::<gtk::ListItem>().unwrap();
        let child = item.child().unwrap().downcast::<Label>().unwrap();
        let entry = item
            .item()
            .unwrap()
            .downcast::<ReceiptEntityObject>()
            .unwrap();
        child.set_text(&entry.property::<u32>("payment-method-key").to_string());
        //entry.bind_property("id", &child, "label").build();
        //child.bind_property("source_property", target, target_property)
    }
}

// Trait shared by all widgets
impl WidgetImpl for ReceiptColumnView {}

// Trait shared by all windows
impl WindowImpl for ReceiptColumnView {}

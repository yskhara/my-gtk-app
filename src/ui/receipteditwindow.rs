use chrono::{DateTime, NaiveDateTime, Utc};
use glib::subclass::InitializingObject;
use gtk::subclass::prelude::*;
use gtk::{
    gio, glib, prelude::*, Button, ColumnView, CompositeTemplate, Label, SignalListItemFactory,
    SingleSelection,
};

use crate::dal;
use crate::receiptlistitem::ReceiptEntityObject;

// ANCHOR: object
// Object holding the state
#[derive(CompositeTemplate, Default)]
#[template(resource = "/org/gtk_rs/example/receipt-edit-window.ui")]
pub struct ReceiptEditWindow {
    #[template_child]
    pub button: TemplateChild<Button>,
}
// ANCHOR_END: object

// ANCHOR: subclass
// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for ReceiptEditWindow {
    // `NAME` needs to match `class` attribute of template
    const NAME: &'static str = "MyGtkAppReceiptEditWindow";
    type Type = super::ReceiptEditWindow;
    type ParentType = gtk::Window;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
        klass.bind_template_callbacks();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}
// ANCHOR_END: subclass

// ANCHOR: object_impl
// Trait shared by all GObjects
impl ObjectImpl for ReceiptEditWindow {
    fn constructed(&self) {
        // Call "constructed" on parent
        self.parent_constructed();

        // Connect to "clicked" signal of `button`
        self.button.connect_clicked(move |button| {
            // Set the label to "Hello World!" after the button has been clicked on
            button.set_label("Hello World!");
            dal::add_receipt();
        });

        self.update_receipt_list();
    }
}
// ANCHOR_END: object_impl

#[gtk::template_callbacks]
impl ReceiptEditWindow {
    fn update_receipt_list(&self) {
    }

    #[template_callback]
    fn button_add_entry_click_handler(&self, _button: &gtk::Button) {
        dal::add_receipt();
        self.update_receipt_list();
    }

    #[template_callback]
    fn column_text_setup_handler(&self, item: &gtk::ListItem) {
        let item = item.downcast_ref::<gtk::ListItem>().unwrap();
        let label = Label::new(Some(""));
        item.set_child(Some(&label));

        println!("setup()");
        //self.item_list.push(item);
    }

    #[template_callback]
    fn column_id_bind_handler(_factory: &SignalListItemFactory, item: &gtk::ListItem) {
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
    fn column_datetime_bind_handler(_factory: &SignalListItemFactory, item: &gtk::ListItem) {
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
}

// Trait shared by all widgets
impl WidgetImpl for ReceiptEditWindow {}

// Trait shared by all windows
impl WindowImpl for ReceiptEditWindow {}

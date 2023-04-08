use std::cell::Cell;

use chrono::{DateTime, NaiveDateTime, Utc};
use glib::subclass::InitializingObject;
use gtk::subclass::prelude::*;
use gtk::{
    gio, glib, prelude::*, Button, ColumnView, CompositeTemplate, Label, SignalListItemFactory,
    SingleSelection,
};

use super::ReceiptEditWindow;
use crate::dal;
use crate::receiptlistitem::ReceiptEntityObject;

// ANCHOR: object
// Object holding the state
#[derive(CompositeTemplate, Default)]
#[template(resource = "/org/gtk_rs/example/main-window.ui")]
pub struct MainWindow {
    #[template_child]
    pub button: TemplateChild<Button>,
    #[template_child]
    pub receipt_list_view: TemplateChild<ColumnView>,
    //item_list: Vec<&gtk::ListItem>,
    //model: gio::ListStore,
    //selection_model: SingleSelection,
    //receipt_edit_window: Cell<ReceiptEditWindow>,
}
// ANCHOR_END: object

// ANCHOR: subclass
// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for MainWindow {
    // `NAME` needs to match `class` attribute of template
    const NAME: &'static str = "MyGtkAppWindow";
    type Type = super::MainWindow;
    type ParentType = gtk::ApplicationWindow;

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
impl ObjectImpl for MainWindow {
    fn constructed(&self) {
        // Call "constructed" on parent
        self.parent_constructed();

        // Connect to "clicked" signal of `button`
        self.button.connect_clicked(move |button| {
            // Set the label to "Hello World!" after the button has been clicked on
            button.set_label("Hello World!");
            dal::add_receipt();
        });

        self.receipt_list_view
            .connect_activate(move |_receipt_item_view, pos| {
                println!("{}", pos);
            });

        // Create new model
        let model = gio::ListStore::new(ReceiptEntityObject::static_type());
        let selection_model = SingleSelection::new(Some(model));
        self.receipt_list_view.set_model(Some(&selection_model));

        self.update_receipt_list();
    }
}
// ANCHOR_END: object_impl

#[gtk::template_callbacks]
impl MainWindow {
    fn update_receipt_list(&self) {
        // Create a `Vec<IntegerObject>` with numbers from 0 to 100_000
        let vector: Vec<ReceiptEntityObject> = dal::get_receipts()
            .into_iter()
            .map(ReceiptEntityObject::new)
            .collect();

        let model = self.receipt_list_view.get().model();
        let model = model.and_downcast_ref::<SingleSelection>().unwrap().model();
        let model = model.and_downcast_ref::<gio::ListStore>().unwrap();
        // Add the vector to the model
        model.remove_all();
        model.extend_from_slice(&vector);
        //let list_view = ListView::new(Some(selection_model), Some(factory));
    }

    #[template_callback]
    async fn button_add_entry_click_handler(&self, button: &gtk::Button) {
        // TODO: use mutex to block multiple calls at the same time
        static MUTEX: once_cell::sync::Lazy<std::sync::Arc<std::sync::Mutex<i32>>> =
            once_cell::sync::Lazy::new(|| std::sync::Arc::new(std::sync::Mutex::new(1)));
        // The long running operation runs now in a separate thread
        println!("Obtaining a mutex lock...");
        let _lock = MUTEX.lock().unwrap();
        println!("A mutex lock obtained. Sleeping for 5 seconds...");
        //let button = button.clone();
        //button.set_sensitive(false);
        //let five_seconds = std::time::Duration::from_secs(5);
        for i in 1..5
        {
            println!("{:}", i);
            glib::timeout_future_seconds(1).await;
        }
        println!("5");
        //button.set_sensitive(true);
        println!("Done. Releasing the lock.");

        //dal::add_receipt();
        //self.update_receipt_list();
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
impl WidgetImpl for MainWindow {}

// Trait shared by all windows
impl WindowImpl for MainWindow {}

// Trait shared by all application windows
impl ApplicationWindowImpl for MainWindow {}

use std::time::Instant;

use chrono::{DateTime, NaiveDateTime, Utc};
use glib::subclass::InitializingObject;
use gtk::subclass::prelude::*;
use gtk::{
    gio, glib, prelude::*, Button, ColumnView, CompositeTemplate, Label,
    SignalListItemFactory, SingleSelection, NoSelection,
};

use crate::entities::{ReceiptEntity, self};
use crate::receiptlistitem::ReceiptEntityObject;
use crate::dal;

use crate::sqlliststore::SqlListStore;

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

        let col1factory = gtk::SignalListItemFactory::new();
        let col2factory = gtk::SignalListItemFactory::new();
        col1factory.connect_setup(Self::column_text_setup_handler);
        col2factory.connect_setup(Self::column_text_setup_handler);
        col1factory.connect_bind(Self::column_id_bind_handler);
        col2factory.connect_bind(Self::column_datetime_bind_handler);

        let col1 = gtk::ColumnViewColumn::new(Some("ID"), Some(col1factory));
        let col2 = gtk::ColumnViewColumn::new(Some("Date"), Some(col2factory));

        // Create new model
        //let model = gio::ListStore::new(ReceiptEntityObject::static_type());
        //let start = Instant::now();
        //let receipt_id = dal::get_receipts_id(None).unwrap();
        //let cnt = receipt_id.len();
        //for i in receipt_id {
        //    model.append(&ReceiptEntityObject::new(dal::get_receipt(i).unwrap()));
        //}
        //println!("Collected {:} receipts; Took {:?}.", cnt, start.elapsed());
        //let model = gtk::SortListModel::new(Some(model), Some(sorter.clone()));

        let sorter = self.receipt_list_view.sorter().unwrap();
        let model = SqlListStore::new("receipt", ReceiptEntityObject::static_type(), Some(sorter.clone()));
        //let model = SqlListStore::new(None);
        println!("{:?}", model);
        println!(
            "is \"model\" a gio::ListModel? : {:?}",
            model.is::<gio::ListModel>()
        );
        let model = SingleSelection::new(Some(model));
        //let model = NoSelection::new(Some(model));
        self.receipt_list_view.set_model(Some(&model));

        let sorter = gtk::NumericSorter::new(None::<gtk::Expression>);
        
        col1.set_sorter(Some(&sorter));
        col1.set_id(Some(entities::ReceiptEntityColumn::Id.to_string()));
        col2.set_sorter(Some(&sorter));
        col2.set_id(Some(entities::ReceiptEntityColumn::Datetime.to_string()));

        self.receipt_list_view.append_column(&col1);
        self.receipt_list_view.append_column(&col2);

        // for column in self.receipt_list_view.columns().into_iter(){
        //     if let Ok(column) = column {
        //         let column = column.downcast::<ColumnViewColumn>().unwrap();
        //         println!("{:?}", column);
        //         column.set_sorter(Some(&sorter));
        //     }
        // }

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
    }
}
// ANCHOR_END: object_impl

#[gtk::template_callbacks]
impl MainWindow {
    fn get_liststore(&self) -> Result<gio::ListStore, ()> {
        if let Some(model) = self
            .receipt_list_view
            .get()
            .model()
            .and_downcast_ref::<SingleSelection>()
        {
            if let Some(model) = model.model().and_downcast_ref::<gio::ListStore>() {
                Ok(model.clone())
            } else {
                Err(())
            }
        } else {
            Err(())
        }
    }

    #[template_callback]
    async fn button_add_entry_click_handler(&self, _button: &gtk::Button) {
        static MUTEX: once_cell::sync::Lazy<std::sync::Arc<std::sync::Mutex<i32>>> =
            once_cell::sync::Lazy::new(|| std::sync::Arc::new(std::sync::Mutex::new(1)));
        println!("Obtaining a mutex lock...");
        let lock = MUTEX.try_lock();
        if let Ok(_mutex) = lock {
            println!("A mutex lock obtained. Sleeping for 5 seconds...");
            //let button = button.clone();
            //button.set_sensitive(false);
            //let five_seconds = std::time::Duration::from_secs(5);
            for i in 1..5 {
                println!("{:}", i);
                //glib::timeout_future_seconds(1).await;
            }
            println!("5");
            //button.set_sensitive(true);
            println!("Done. Releasing the lock.");

            dal::add_receipt();
        } else {
            println!("Failed to obtain a lock. Returning...")
        }
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
}

// Trait shared by all widgets
impl WidgetImpl for MainWindow {}

// Trait shared by all windows
impl WindowImpl for MainWindow {}

// Trait shared by all application windows
impl ApplicationWindowImpl for MainWindow {}

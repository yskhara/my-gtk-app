mod mainwindow;
mod receiptcolumnview;
mod receipteditwindow;

use gtk::{gio, glib};

glib::wrapper! {
    pub struct MainWindow(ObjectSubclass<mainwindow::MainWindow>)
        @extends gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
            gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl MainWindow {
    pub fn new(app: &gtk::Application) -> Self {
        glib::Object::builder().property("application", app).build()
    }
}

glib::wrapper! {
    pub struct ReceiptEditWindow(ObjectSubclass<receipteditwindow::ReceiptEditWindow>)
        @extends gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
            gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl ReceiptEditWindow {
    pub fn new(app: &gtk::Application) -> Self {
        glib::Object::builder().property("application", app).build()
    }
}

glib::wrapper! {
    pub struct ReceiptColumnView(ObjectSubclass<receiptcolumnview::ReceiptColumnView>)
        @extends gtk::ColumnView, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Scrollable;
}

impl ReceiptColumnView {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }
}

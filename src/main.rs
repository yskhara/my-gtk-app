mod ui;
mod dal;
mod entities;
mod receiptlistitem;
mod sqlliststore;

use dal::testdatagenerator;
use gtk::prelude::*;
use gtk::{gio, glib, Application, Button};
use ui::MainWindow;

const APP_ID: &str = "org.gtk_rs.HelloWorld2";

fn main() -> glib::ExitCode {
    // Register and include resources
    gio::resources_register_include!("my-gtk-app.gresource")
        .expect("Failed to register resources.");

    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

fn button_clicked(button: &Button) {
    // Set the label to "Hello World!" after the button has been clicked on
    button.set_label("Hello World!");


}

fn build_ui(app: &Application) {
    testdatagenerator::generate_test_receipt_data();

    let window = MainWindow::new(app);
    window.present();
}

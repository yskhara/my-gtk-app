fn main() {
    glib_build_tools::compile_resources(
        &["resources/ui", "resources/ui/receipttableview"],
        "resources/resources.gresource.xml",
        "my-gtk-app.gresource",
    );

    println!("cargo:rustc-link-search=native=lib");
}

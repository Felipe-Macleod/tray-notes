use gtk::{prelude::*, ScrolledWindow, TextView};
use gtk::{glib, Application, ApplicationWindow};

const APP_ID: &str = "br.dev.macleod.traynotes";

fn main() -> glib::ExitCode {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

fn build_ui(app: &Application) {
    let text_field = TextView::builder()
        .monospace(true)
        .build();
    let scroll = ScrolledWindow::builder()
        .hexpand(true)
        .vexpand(true)
        .margin_top(6)
        .margin_bottom(6)
        .margin_start(6)
        .margin_end(6)
        .child(&text_field)
        .build();
    // Create a window and set the title
    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .default_height(600)
        .default_width(400)
        .child(&scroll)
        .build();

    // Present window
    window.present();
}
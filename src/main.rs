use gtk::gio::{ActionEntry, Settings};
use gtk::glib::clone;
use gtk::{prelude::*, FileChooserDialog, ScrolledWindow, TextBuffer, TextView};
use gtk::{glib, Application, ApplicationWindow};

const APP_ID: &str = "br.dev.macleod.traynotes";

fn main() -> glib::ExitCode {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_startup(setup_shortcuts);
    app.connect_activate(build_ui);
    
    // Run the application
    app.run()
}

fn build_ui(app: &Application) {
    let buffer = build_buffer();
    let text_field = build_text_field(&buffer);
    
    // Create a window and set the title
    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .default_height(600)
        .default_width(500)
        .child(&text_field)
        .build();

    // Create file chooser dialog
    let dialog = build_file_chooser(clone!(@weak window as win => move |file_content| {
        win.set_title(Some("Teste"));
        buffer.set_text(file_content);
    }));

    // Add action "open-file" to `window`
    let action_open_file = ActionEntry::builder("open-file")
        .activate(move |_, _,_| {
            dialog.show();
        })
        .build();
    window.add_action_entries([action_open_file]);

    // Present window
    window.present();
}

fn build_text_field(buffer: &TextBuffer) -> ScrolledWindow {
    let text_field = TextView::builder()
        .monospace(true)
        .buffer(buffer)
        .top_margin(6)
        .bottom_margin(6)
        .left_margin(12)
        .right_margin(6)
        .build();

    let scroll = ScrolledWindow::builder()
        .hexpand(true)
        .vexpand(true)
        .child(&text_field)
        .build();

    return scroll;
}

fn build_file_chooser<F>(f: F) -> FileChooserDialog where F: Fn(&str) + 'static {
    let dialog = FileChooserDialog::builder()
        .title("Open Note")
        .modal(true)
        .build();
    dialog.add_button("Open", gtk::ResponseType::Accept);
    dialog.connect_response(move |dialog: &FileChooserDialog, res_type| {
        match res_type {
            gtk::ResponseType::Accept => {
                let file_path = dialog.file().unwrap().path().unwrap();
                let file_content = std::fs::read_to_string(file_path.clone()).unwrap();
                f(file_content.as_str());
                dialog.close();
            },
            _ => ()
        }
    });

    return dialog;
}

fn build_buffer() -> TextBuffer {
    let settings = Settings::new(APP_ID);
    let content = settings.string("opened-file");

    let buffer = TextBuffer::builder().text(content).build();

    buffer.connect_changed(move |field| {
        let buff_start = field.start_iter();
        let buff_end = field.end_iter();
        let buff_content = field.text(&buff_start, &buff_end, true);
        
        settings.set_string("opened-file", &buff_content)
            .expect("Não foi possível salvar.");
    });

    return buffer;
}

fn setup_shortcuts(app: &Application) {
    app.set_accels_for_action("win.open-file", &["<Ctrl>O"]);
}
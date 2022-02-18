//use gtk4 as gtk;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button};

fn main() {
    let application = Application::builder()
        .application_id("com.philolog.matriarch-settings")
        .build();

    application.connect_activate(|app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Moog Matriarch Global Settings")
            .default_width(350)
            .default_height(70)
            .build();

        let button = Button::with_label("Click me!");
        button.connect_clicked(|_| {
            eprintln!("Clicked!");
        });
        window.set_child(Some(&button));

        window.show();
    });

    application.run();
}

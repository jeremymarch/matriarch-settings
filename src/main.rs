//use gtk4 as gtk;
//https://kornel.ski/rust-sys-crate
//https://github.com/chris-zen/coremidi/blob/master/examples/receive.rs

use gtk::prelude::*;
use gtk::{Application, ApplicationWindow };
use coremidi;
use std::env;

fn main() {
    let application = Application::builder()
        .application_id("com.philolog.matriarch-settings")
        .build();

    application.connect_activate(|app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Moog Matriarch Global Settings")
            .default_width(550)
            .default_height(400)
            .build();
        
        let button = gtk::Button::with_label("Connect");
        button.connect_clicked(|_| {
            eprintln!("Connect clicked!");
        });

        let dd = gtk::DropDown::from_strings(&["abc", "def"]);

        let list_box = gtk::ListBox::new();
        for number in 0..=74 {
            let label = gtk::Label::new(Some(&number.to_string()));
            list_box.append(&label);
        }

        let scrolled_window = gtk::ScrolledWindow::builder()
            .hscrollbar_policy(gtk::PolicyType::Never) // Disable horizontal scrolling
            .min_content_width(360)
            .vexpand(true)
            .child(&list_box)
            .build();

            let hbox: gtk::Box = gtk::Box::new(gtk::Orientation::Horizontal, 0);
            hbox.set_homogeneous(true);
            hbox.prepend(&button);
            hbox.append(&dd);

        let vbox: gtk::Box = gtk::Box::new(gtk::Orientation::Vertical, 0);
        vbox.set_homogeneous(false);
        vbox.prepend(&hbox);
        vbox.append(&scrolled_window);

        window.set_child(Some(&vbox));
        window.show();
    });
    
/* 
    let source_index = 0;//get_source_index();
    println!("Source index: {}", source_index);

    let source = coremidi::Source::from_index(source_index).unwrap();
    println!("Source display name: {}", source.display_name().unwrap());

    let client = coremidi::Client::new("Example Client").unwrap();

    let callback = |packet_list: &coremidi::PacketList| {
        println!("{}", packet_list);
    };

    let input_port = client.input_port("Example Port", callback).unwrap();
    input_port.connect_source(&source).unwrap();

    */
/* 
    let mut input_line = String::new();
    println!("Press Enter to Finish");
    std::io::stdin()
        .read_line(&mut input_line)
        .expect("Failed to read line");
*/
    application.run();

    //input_port.disconnect_source(&source).unwrap();
}

fn get_source_index() -> usize {
    let mut args_iter = env::args();
    let tool_name = args_iter
        .next()
        .and_then(|path| {
            path.split(std::path::MAIN_SEPARATOR)
                .last()
                .map(|v| v.to_string())
        })
        .unwrap_or_else(|| "receive".to_string());

    match args_iter.next() {
        Some(arg) => match arg.parse::<usize>() {
            Ok(index) => {
                if index >= coremidi::Sources::count() {
                    println!("Source index out of range: {}", index);
                    std::process::exit(-1);
                }
                index
            }
            Err(_) => {
                println!("Wrong source index: {}", arg);
                std::process::exit(-1);
            }
        },
        None => {
            println!("Usage: {} <source-index>", tool_name);
            println!();
            println!("Available Sources:");
            print_sources();
            std::process::exit(-1);
        }
    }
}

fn print_sources() {
    for (i, source) in coremidi::Sources.into_iter().enumerate() {
        if let Some(display_name) = source.display_name() {
            println!("[{}] {}", i, display_name)
        }
    }
}

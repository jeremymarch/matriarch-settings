//use gtk4 as gtk;
//https://kornel.ski/rust-sys-crate
//https://github.com/chris-zen/coremidi/blob/master/examples/receive.rs
//https://github.com/gtk-rs/examples/blob/master/src/bin/listbox_model.rs
//https://users.rust-lang.org/t/how-to-right-justify-numeric-data-in-gtk-rs-list-store/30538

use gtk::prelude::*;
use gtk::{Application, ApplicationWindow };
use coremidi; //or https://github.com/Boddlnagg/midir
//use std::env;

fn main() {
    let application = Application::builder()
        .application_id("com.philolog.matriarch-settings")
        .build();
    

    let mut matriarch_index:Option<usize> = None;
    let v = get_sources();
    if v.len() > 0 {
        for (idx, i) in v.iter().enumerate() {
            if i == "Moog Matriarch" {
                matriarch_index = Some(idx);
            }
        }
    }
    application.connect_activate(move |app| {
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


        
       
        let combo = gtk::ComboBoxText::new();
        if v.len() == 0 {
            combo.append_text("No midi devices connected");
        }
        else {
            for (_idx, i) in v.iter().enumerate() {
                combo.append_text(&i);
            }
        }
        combo.set_active(Some(0));

        
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
            hbox.append(&combo);

        let vbox: gtk::Box = gtk::Box::new(gtk::Orientation::Vertical, 0);
        vbox.set_homogeneous(false);
        vbox.prepend(&hbox);
        vbox.append(&scrolled_window);

        window.set_child(Some(&vbox));
        window.show();
    });
    

    let source;
    let input_port;
    let client;
    match matriarch_index {
        Some(matriarch_index) => {
            
            //println!("Source index: {}", matriarch_index);

            source = coremidi::Source::from_index(matriarch_index);
            match source {
                Some(ref source) => { 
                    //println!("Source display name: {}", source.display_name().unwrap());

                    client = coremidi::Client::new("Matriarch Settings Client").unwrap();

                    let callback = |packet_list: &coremidi::PacketList| {
                        println!("{}", packet_list);
                    };

                    input_port = client.input_port("Matriarch Settings Port", callback);
                    match input_port {
                        Ok(ref input_port) => {
                            input_port.connect_source(&source).unwrap();
                        },
                        Err(_input_port) => {
                            println!("input port not created");
                            std::process::exit(1);
                        }
                    }
                },
                None => {
                    println!("source port not created");
                    std::process::exit(1);
                }
            }
        },
        None => {
            println!("source index not set");
            input_port = Err(0);
            source = None;
        }
    }
    application.run();
    
    if input_port.is_ok() && source.is_some() {
        input_port.unwrap().disconnect_source(&source.unwrap()).unwrap();
    }
    
}
/*
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
            std::process::exit(-1);
        }
    }
}
*/
/* 
fn print_sources() {
    for (i, source) in coremidi::Sources.into_iter().enumerate() {
        if let Some(display_name) = source.display_name() {
            println!("[{}] {}", i, display_name)
        }
    }
}
*/
fn get_sources() -> Vec<String> {
    let mut v = Vec::new();
    for (_i, source) in coremidi::Sources.into_iter().enumerate() {
        if let Some(display_name) = source.display_name() {
            v.push(display_name);
            //println!("[{}] {}", i, display_name)
        }
    }
    v
}


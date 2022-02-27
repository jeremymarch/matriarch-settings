//use gtk4 as gtk;
//https://kornel.ski/rust-sys-crate
//https://github.com/chris-zen/coremidi/blob/master/examples/receive.rs
//https://github.com/gtk-rs/examples/blob/master/src/bin/listbox_model.rs
//https://users.rust-lang.org/t/how-to-right-justify-numeric-data-in-gtk-rs-list-store/30538

use gtk::prelude::*;
use gtk::{Application, ApplicationWindow };
use gtk::TreeView;
use gtk::ListStore;
use gtk::TreeViewColumn;
use gtk::glib::{ToValue, Type, Value};
use gtk::gio;

//#[derive(Debug, Clone)]
struct MatriarchParam {
    id:u32,
    name:String,
    value:String,
}

use coremidi; //or https://github.com/Boddlnagg/midir
//use std::env;

fn main() {
    let application = Application::builder()
        .application_id("com.philolog.matriarch-settings")
        .build();

    let mut params = [
        MatriarchParam {id:0, name:"Unit ID".to_string(), value:"".to_string()},
        MatriarchParam {id:1, name:"Tuning Scale".to_string(), value:"".to_string()},
        MatriarchParam {id:2, name:"Knob Mode".to_string(), value:"".to_string()},
        MatriarchParam {id:3, name:"Note Priority".to_string(), value:"".to_string()},
        MatriarchParam {id:4, name:"Transmit Program Change".to_string(), value:"".to_string()},
        MatriarchParam {id:5, name:"Receive Program Change".to_string(), value:"".to_string()},
        MatriarchParam {id:6, name:"MIDI Input Ports".to_string(), value:"".to_string()},
        MatriarchParam {id:7, name:"MIDI Output Ports".to_string(), value:"".to_string()},
        MatriarchParam {id:8, name:"MIDI Echo USB In".to_string(), value:"".to_string()},
        MatriarchParam {id:9, name:"MIDI Echo DIN In".to_string(), value:"".to_string()},
        MatriarchParam {id:10, name:"MIDI Channel In".to_string(), value:"".to_string()},
        MatriarchParam {id:11, name:"MIDI Channel Out".to_string(), value:"".to_string()},
        MatriarchParam {id:12, name:"MIDI Out Filter - Keys".to_string(), value:"".to_string()},
        MatriarchParam {id:13, name:"MIDI Out Filter - Wheels".to_string(), value:"".to_string()},
        MatriarchParam {id:14, name:"MIDI Out Filter - Panel".to_string(), value:"".to_string()},
        MatriarchParam {id:15, name:"Output 14-bit MIDI CCs".to_string(), value:"".to_string()},

        MatriarchParam {id:16, name:"Local Control: Keys".to_string(), value:"".to_string()},
        MatriarchParam {id:17, name:"Local Control: Wheels".to_string(), value:"".to_string()},
        MatriarchParam {id:18, name:"Local Control: Panel".to_string(), value:"".to_string()},
        MatriarchParam {id:19, name:"Local Control: Arp/Seq".to_string(), value:"".to_string()},
        MatriarchParam {id:20, name:"Sequence Transpose Mode".to_string(), value:"".to_string()},
        MatriarchParam {id:21, name:"Arp/Seq Keyed Timing Reset".to_string(), value:"".to_string()},
        MatriarchParam {id:22, name:"Arp FW/BW Repeats".to_string(), value:"".to_string()},
        MatriarchParam {id:23, name:"Arp/Seq Swing".to_string(), value:"".to_string()},
        MatriarchParam {id:24, name:"Sequence Keyboard Control".to_string(), value:"".to_string()},
        MatriarchParam {id:25, name:"Delay Sequence Change".to_string(), value:"".to_string()},
        MatriarchParam {id:26, name:"Sequence Latch Restart".to_string(), value:"".to_string()},
        MatriarchParam {id:27, name:"Arp/Seq Clock Input Mode".to_string(), value:"".to_string()},
        MatriarchParam {id:28, name:"Arp/Seq Clock Output".to_string(), value:"".to_string()},
        MatriarchParam {id:29, name:"Arp MIDI Output".to_string(), value:"".to_string()},
        MatriarchParam {id:30, name:"Send MIDI Clock".to_string(), value:"".to_string()},

        MatriarchParam {id:31, name:"Send MIDI Start/Stop".to_string(), value:"".to_string()},
        MatriarchParam {id:32, name:"Follow MIDI Clock".to_string(), value:"".to_string()},
        MatriarchParam {id:33, name:"Follow MIDI Start/Stop".to_string(), value:"".to_string()},
        MatriarchParam {id:34, name:"Follow Song Position Pointer".to_string(), value:"".to_string()},
        MatriarchParam {id:35, name:"Clock Input PPQN Index".to_string(), value:"".to_string()},
        MatriarchParam {id:36, name:"Clock Output PPQN Index".to_string(), value:"".to_string()},
        MatriarchParam {id:37, name:"Pitch Bend Range (Semitones)".to_string(), value:"".to_string()},
        MatriarchParam {id:38, name:"Keyboard Octave Transpose".to_string(), value:"".to_string()},
        MatriarchParam {id:39, name:"Delayed Keyboard Octave Transpose".to_string(), value:"".to_string()},
        MatriarchParam {id:40, name:"Glide Type".to_string(), value:"".to_string()},
        MatriarchParam {id:41, name:"Gated Glide".to_string(), value:"".to_string()},
        MatriarchParam {id:42, name:"Legato Glide".to_string(), value:"".to_string()},
        MatriarchParam {id:43, name:"Osc 2 Freq Knob Range".to_string(), value:"".to_string()},
        MatriarchParam {id:44, name:"Osc 3 Freq Knob Range".to_string(), value:"".to_string()},
        MatriarchParam {id:45, name:"Osc 4 Freq Knob Range".to_string(), value:"".to_string()},

        MatriarchParam {id:46, name:"Hard Sync Enable".to_string(), value:"".to_string()},
        MatriarchParam {id:47, name:"Osc 2 Hard Sync".to_string(), value:"".to_string()},
        MatriarchParam {id:48, name:"Osc 3 Hard Sync".to_string(), value:"".to_string()},
        MatriarchParam {id:49, name:"Osc 4 Hard Sync".to_string(), value:"".to_string()},
        MatriarchParam {id:50, name:"Delay Ping Pong".to_string(), value:"".to_string()},
        MatriarchParam {id:51, name:"Delay Sync".to_string(), value:"".to_string()},
        MatriarchParam {id:52, name:"Delay Filter Brightness".to_string(), value:"".to_string()},
        MatriarchParam {id:53, name:"Delay CV Sync-Bend".to_string(), value:"".to_string()},
        MatriarchParam {id:54, name:"Tap-Tempo Clock Division Persistence".to_string(), value:"".to_string()},
        MatriarchParam {id:55, name:"Paraphony Mode".to_string(), value:"".to_string()},
        MatriarchParam {id:56, name:"Paraphonic Unison".to_string(), value:"".to_string()},
        MatriarchParam {id:57, name:"Multi Trig".to_string(), value:"".to_string()},
        MatriarchParam {id:58, name:"Pitch Variance".to_string(), value:"".to_string()},
        MatriarchParam {id:59, name:"KB CV OUT Range".to_string(), value:"".to_string()},
        MatriarchParam {id:60, name:"Arp/Seq CV OUT Range".to_string(), value:"".to_string()},

        MatriarchParam {id:61, name:"KB VEL OUT Range".to_string(), value:"".to_string()},
        MatriarchParam {id:62, name:"Arp/Seq VEL OUT Range".to_string(), value:"".to_string()},
        MatriarchParam {id:63, name:"KB AT OUT Range".to_string(), value:"".to_string()},
        MatriarchParam {id:64, name:"MOD WHL OUT Range".to_string(), value:"".to_string()},
        MatriarchParam {id:65, name:"KB GATE OUT Range".to_string(), value:"".to_string()},
        MatriarchParam {id:66, name:"Arp/Seq GATE OUT Range".to_string(), value:"".to_string()},
        MatriarchParam {id:67, name:"Round-Robin Mode".to_string(), value:"".to_string()},
        MatriarchParam {id:68, name:"Restore Stolen Voices".to_string(), value:"".to_string()},
        MatriarchParam {id:69, name:"Update Unison on Note-Off".to_string(), value:"".to_string()},
        MatriarchParam {id:70, name:"Mod Oscillator Square Wave Polarity".to_string(), value:"".to_string()},
        MatriarchParam {id:71, name:"Noise Filter Cutoff".to_string(), value:"".to_string()},
        MatriarchParam {id:72, name:"Arp/Seq Random Repeats".to_string(), value:"".to_string()},
        MatriarchParam {id:73, name:"ARP/SEQ CV OUT Mirrors KB CV".to_string(), value:"".to_string()},
        MatriarchParam {id:74, name:"KB CV OUT Mirrors ARP/SEQ CV".to_string(), value:"".to_string()},
    ];
    

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

/*  
        let view_list = gtk::ListBox::new();
        for number in 0..=74 {
            let label = gtk::Label::new(Some(&number.to_string()));
            view_list.append(&label);
        }
*/

        let view_list = TreeView::new();
        {
            let types_inside_columns = &[gtk::glib::Type::U32, gtk::glib::Type::STRING];
            let model_list_of_data = ListStore::new(types_inside_columns);
            /*for liczba in 0..10 {
                //let array_of_data = &[(0, 2), (1, 3)];
                model_list_of_data.insert_with_values(None, &[(0, &2), (1, &"blah")]);
            }*/
            for p in &params {
                //let array_of_data = &[(0, 2), (1, 3)];
                model_list_of_data.insert_with_values(None, &[(0, &p.id), (1, &p.name)]);
            }
            view_list.set_model(Some(&model_list_of_data));
            let object_to_render_cells: gtk::CellRendererText = gtk::CellRendererText::new();
            object_to_render_cells.set_visible(true);
            let view_column = TreeViewColumn::new();
            view_column.set_expand(false);
            view_column.set_visible(true);
            view_column.set_title("ID");
            view_column.pack_start(&object_to_render_cells, true);
            view_column.add_attribute(&object_to_render_cells, "text", 0);
            view_list.append_column(&view_column);
            // second column
            let object_to_render_cells_2: gtk::CellRendererText = gtk::CellRendererText::new();
            object_to_render_cells_2.set_visible(true);
            let view_column_2 = TreeViewColumn::new();
            view_column_2.set_expand(true);
            view_column_2.set_visible(true);
            view_column_2.set_title("Name");
            view_column_2.pack_start(&object_to_render_cells, true);
            view_column_2.add_attribute(&object_to_render_cells, "text", 1);
            view_list.append_column(&view_column_2);
        }
        view_list.expand_all();

/* 
        let model = gio::ListStore::new(gio::AppInfo::static_type());
        gio::AppInfo::all().iter().for_each(|app_info| {
            model.append(app_info);
        });
        let view_list = gtk::ListView::new(Some(&model),None);
*/
        let scrolled_window = gtk::ScrolledWindow::builder()
            .hscrollbar_policy(gtk::PolicyType::Never) // Disable horizontal scrolling
            .min_content_width(360)
            .vexpand(true)
            .child(&view_list)
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


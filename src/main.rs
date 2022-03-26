//use gtk4 as gtk;
//https://kornel.ski/rust-sys-crate
//https://github.com/chris-zen/coremidi/blob/master/examples/receive.rs
//https://github.com/gtk-rs/examples/blob/master/src/bin/listbox_model.rs
//https://users.rust-lang.org/t/how-to-right-justify-numeric-data-in-gtk-rs-list-store/30538

use gtk::prelude::*;
use gtk::{Application, ApplicationWindow };
use gtk::TreeView;
//use gtk::ListView;
use gtk::ListStore;
use gtk::TreeViewColumn;
use gtk::glib;


struct ListOptions {
    options:Vec<String>
}

struct RangeOptions {
    min:u32,
    max:u32,
}

trait GenericOptions
{
   fn get_options(&self) -> Vec<String>;
}

impl GenericOptions for RangeOptions
{
   fn get_options(&self) -> Vec<String>
   {
       vec![self.min.to_string(), self.max.to_string()]
   }
}

impl GenericOptions for ListOptions
{
   fn get_options(&self) -> Vec<String>
   {
      self.options.to_owned()
   }
}

//#[derive(Debug, Clone)]
struct MatriarchParam<T: GenericOptions> {
    id:u32,
    name:String,
    value:String,
    options:T,
    default_value:String
}

use coremidi; //or https://github.com/Boddlnagg/midir
//use std::env;

fn main() {
    let application = Application::builder()
        .application_id("com.philolog.matriarch-settings")
        .build();

    let mut params = [
        MatriarchParam {id:0, name:"Unit ID".to_string(), value:"".to_string(), options:ListOptions {options:vec!["1".to_string(), "2".to_string()]}, default_value:"0 (Default)".to_string()},
        MatriarchParam {id:1, name:"Tuning Scale".to_string(), value:"".to_string(), options:ListOptions {options:vec!["3".to_string(), "4".to_string()]}, default_value:"0 (Default - 12-TET)".to_string()},
        /* 
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
        */
    ];

    /*
    let params = [
    new Param(0, 'Unit ID', Options(Range(0, 15), 0)),
    new Param(1, 'Tuning Scale', Options(Range(0, 31), 0, '12-TET')),
    new Param(2, 'Knob Mode', Options(['Snap', 'Pass-Thru', 'Relative'], 0), 'Actual default different to documented'),
    new Param(3, 'Note Priority', Options(['Low', 'High', 'Last Note'], 2)),
    new Param(4, 'Transmit Program Change', Options(['Off', 'On'], 0)),
    new Param(5, 'Receive Program Change', Options(['Off', 'On'], 1)),
    new Param(6, 'MIDI Input Ports',
        Options(['none', 'DIN only', 'USB only', 'Both DIN and USB'], 3)),
    new Param(7, 'MIDI Output Ports',
        Options(['none', 'DIN only', 'USB only', 'Both DIN and USB'], 3)),
    new Param(8, 'MIDI Echo USB In',
        Options(['Off', 'Echo USB In to DIN Out', 'Echo USB In to USB Out',
            'Echo USB In to Both DIN and USB Out'], 0)),
    new Param(9, 'MIDI Echo DIN In',
        Options(['Off', 'Echo DIN In to DIN Out', 'Echo DIN In to USB Out',
            'Echo DIN In to Both DIN and USB Out'], 0)),
    new Param(10, 'MIDI Channel In', Options(Range(1, 16).map(i => 'Channel ' + i), 0)),
    new Param(11, 'MIDI Channel Out', Options(Range(1, 16).map(i => 'Channel ' + i), 0)),
    new Param(12, 'MIDI Out Filter - Keys', Options(['Off', 'On'], 1)),
    new Param(13, 'MIDI Out Filter - Wheels', Options(['Off', 'On'], 1)),
    new Param(14, 'MIDI Out Filter - Panel', Options(['Off', 'On'], 1)),
    new Param(15, 'Output 14-bit MIDI CCs', Options(['Off', 'On'], 0)),
    new Param(16, 'Local Control: Keys', Options(['Off', 'On'], 1)),
    new Param(17, 'Local Control: Wheels', Options(['Off', 'On'], 1)),
    new Param(18, 'Local Control: Panel', Options(['Off', 'On'], 0), 'Actual default different to documented'),
    new Param(19, 'Local Control: Arp/Seq', Options(['Off', 'On'], 1)),
    new Param(20, 'Sequence Transpose Mode',
        Options(['Relative to First Note', 'Relative to Middle C'], 0)),
    new Param(21, 'Arp/Seq Keyed Timing Reset', Options(['Off', 'On'], 1), 'Actual default different to documented'),
    new Param(22, 'Arp FW/BW Repeats',
        Options(["Don't Repeat end notes", 'Repeat end notes'], 1)),
    new Param(23, 'Arp/Seq Swing', Slider(0, 16383, 8192)),
    new Param(24, 'Sequence Keyboard Control', Options(['Off', 'On'], 1)),
    new Param(25, 'Delay Sequence Change', Options(['Off', 'On'], 0)),
    new Param(26, 'Sequence Latch Restart', Options(['Off', 'On'], 1), 'Actual default different to documented'),
    new Param(27, 'Arp/Seq Clock Input Mode',
        Options(['Clock', 'Step-Advance Trigger'], 0)),
    new Param(28, 'Arp/Seq Clock Output',
        Options(['Always', 'Only When Playing'], 1)),
    new Param(29, 'Arp MIDI Output', Options(['Off', 'On'], 1)),
    new Param(30, 'Send MIDI Clock', Options(['Off', 'Only When Playing'], 0)),
    new Param(31, 'Send MIDI Start/Stop', Options(['Off', 'On'], 0)),
    new Param(32, 'Follow MIDI Clock', Options(['Off', 'On'], 1)),
    new Param(33, 'Follow MIDI Start/Stop', Options(['Off', 'On'], 1)),
    new Param(34, 'Follow Song Position Pointer', Options(['Off', 'On'], 1)),
    new Param(35, 'Clock Input PPQN Index',
        Options(Range(0, 15), 3, 'sixteenth notes [4PPQN]')),
    new Param(36, 'Clock Output PPQN Index',
        Options(Range(0, 15), 3, 'sixteenth notes [4PPQN]')),
    new Param(37, 'Pitch Bend Range (Semitones)',
        Options(Range(0, 12), 2)),
    new Param(38, 'Keyboard Octave Transpose',
        Options(['-2', '-1', '0', '1', '2'], 2, 'no transpose')),
    new Param(39, 'Delayed Keyboard Octave Transpose', Options(['Off', 'On'], 1)),
    new Param(40, 'Glide Type',
        Options(['Linear Constant Rate', 'Linear Constant Time', 'Exponential'], 0)),
    new Param(41, 'Gated Glide', Options(['Off', 'On'], 1)),
    new Param(42, 'Legato Glide', Options(['Off', 'On'], 0), 'Actual default different to documented'),
    new Param(43, 'Osc 2 Freq Knob Range',
        Options(Range(0, 24).map(i => i + ' Semitones'), 7)),
    new Param(44, 'Osc 3 Freq Knob Range',
        Options(Range(0, 24).map(i => i + ' Semitones'), 7)),
    new Param(45, 'Osc 4 Freq Knob Range',
        Options(Range(0, 24).map(i => i + ' Semitones'), 7)),
    new Param(46, 'Hard Sync Enable', Options(['Off', 'On'], 0)),
    new Param(47, 'Osc 2 Hard Sync', Options(['Off', 'On'], 0)),
    new Param(48, 'Osc 3 Hard Sync', Options(['Off', 'On'], 0)),
    new Param(49, 'Osc 4 Hard Sync', Options(['Off', 'On'], 0)),
    new Param(50, 'Delay Ping Pong', Options(['Off', 'On'], 0)),
    new Param(51, 'Delay Sync', Options(['Off', 'On'], 0)),
    new Param(52, 'Delay Filter Brightness', Options(['Dark', 'Bright'], 1)),
    new Param(53, 'Delay CV Sync-Bend', Options(['Off', 'On'], 0)),
    new Param(54, 'Tap-Tempo Clock Division Persistence', Options(['Off', 'On'], 0)),
    new Param(55, 'Paraphony Mode', Options(['Mono', 'Duo', 'Quad'], 2), 'Actual default different to documented'),
    new Param(56, 'Paraphonic Unison', Options(['Off', 'On'], 0)),
    new Param(57, 'Multi Trig', Options(['Off', 'On'], 0)),
    new Param(58, 'Pitch Variance',
        Options(Range(0, 400).map(i => '± ' + (i / 10) + ' cents'), 0)),
    new Param(59, 'KB CV OUT Range', Options(['-5V to +5V', '0V to 10V'], 0)),
    new Param(60, 'Arp/Seq CV OUT Range', Options(['-5V to +5V', '0V to 10V'], 0)),
    new Param(61, 'KB VEL OUT Range', Options(['-5V to +5V', '0V to 10V'], 0)),
    new Param(62, 'Arp/Seq VEL OUT Range', Options(['-5V to +5V', '0V to 10V'], 0)),
    new Param(63, 'KB AT OUT Range', Options(['-5V to +5V', '0V to 10V'], 0)),
    new Param(64, 'MOD WHL OUT Range', Options(['-5V to +5V', '0V to 10V'], 0)),
    new Param(65, 'KB GATE OUT Range', Options(['-5V to +5V', '0V to 10V'], 0)),
    new Param(66, 'Arp/Seq GATE OUT Range', Options(['-5V to +5V', '0V to 10V'], 0)),
    new Param(67, 'Round-Robin Mode',
        Options(['Off', 'First-Note Reset', 'On'], 1)),
    new Param(68, 'Restore Stolen Voices', Options(['Off', 'On'], 0)),
    new Param(69, 'Update Unison on Note-Off', Options(['Off', 'On'], 0)),
    new Param(70, 'Mod Oscillator Square Wave Polarity',
        Options(['Unipolar', 'Bipolar'], 1)),
    new Param(71, 'Noise Filter Cutoff', Slider(0, 16383, 0), 'Actual default different to documented'),
    new Param(72, 'Arp/Seq Random Repeats',
        Options(['no repeating notes/steps in RND direction',
            'allow repeating notes (true random)'], 1)),
    new Param(73, 'ARP/SEQ CV OUT Mirrors KB CV', Options(['Off', 'On'], 0)),
    new Param(74, 'KB CV OUT Mirrors ARP/SEQ CV', Options(['Off', 'On'], 0)),
    */
    

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

        // change to list?
        let view_list = TreeView::new();
        {
            let types_inside_columns = &[gtk::glib::Type::U32, gtk::glib::Type::STRING, gtk::ListStore::static_type(), gtk::glib::Type::STRING];
            let model_list_of_data = ListStore::new(types_inside_columns);

            for p in &params {
                let model_for_combo = ListStore::new(&[gtk::glib::Type::STRING]);
                for o in &p.options.get_options() {
                    model_for_combo.insert_with_values(None, &[(0, &o)]);
                }
                model_list_of_data.insert_with_values(None, &[(0, &p.id), (1, &p.name), (2, &model_for_combo), (3, &p.default_value)]);
            }

            view_list.set_model(Some(&model_list_of_data));

            // first column
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

            // third column
            let object_to_render_cells_3: gtk::CellRendererCombo = gtk::CellRendererCombo::new();
            object_to_render_cells_3.set_visible(true);
            object_to_render_cells_3.set_editable(true);
            object_to_render_cells_3.set_has_entry(false);

            // set column 3 of list model to selected value from combo so that it will be displayed once selected
            object_to_render_cells_3.connect_changed( gtk::glib::clone!( @weak model_list_of_data => move |_cell, list_path, combo_selected_iter| { 
                if let Some(list_iter) = model_list_of_data.iter(&list_path) {
                    if let Ok(combo_model) = model_list_of_data.get_value(&list_iter, 2).get::<ListStore>() {
                        if let Ok(combo_selected_value) = combo_model.get_value(&combo_selected_iter, 0).get::<String>() {
                            model_list_of_data.set_value(&list_iter, 3, &combo_selected_value.to_value() ); 
                        }
                    }
                }
            } ) );
            // use the combo model for the options
            // object_to_render_cells_3.set_model(Some(&model_for_combo)); //only set model here if same model for each row
            // display the options of the first column in the combo model
            object_to_render_cells_3.set_text_column(0);

            let view_column_3 = TreeViewColumn::new();
            view_column_3.set_expand(true);
            view_column_3.set_visible(true);
            view_column_3.set_title("Value");
            view_column_3.pack_start(&object_to_render_cells_3, true);

            // set model and text for where to get the selected value (column 3)
            // the combo data for each row is in the second column in the tree model
            view_column_3.add_attribute(&object_to_render_cells_3, "model", 2);
            // set selected value here in "changed" signal to display it
            view_column_3.add_attribute(&object_to_render_cells_3, "text", 3); 

            view_list.append_column(&view_column_3);
        }

        view_list.expand_all();

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


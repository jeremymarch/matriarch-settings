//use gtk4 as gtk;
//https://kornel.ski/rust-sys-crate
//https://github.com/chris-zen/coremidi/blob/master/examples/receive.rs
//https://github.com/gtk-rs/examples/blob/master/src/bin/listbox_model.rs
//https://users.rust-lang.org/t/how-to-right-justify-numeric-data-in-gtk-rs-list-store/30538

//scrolling on combobox
//https://mail.gnome.org/archives/gtk-list/2016-December/msg00035.html
//https://stackoverflow.com/questions/873328/how-do-i-put-a-scrollbar-inside-of-a-gtk-comboboxentry

//https://stackoverflow.com/questions/66510406/gtk-rs-how-to-update-view-from-another-thread
//https://coaxion.net/blog/2019/02/mpsc-channel-api-for-painless-usage-of-threads-with-gtk-in-rust/

//https://stackoverflow.com/questions/53216593/vec-of-generics-of-different-concrete-types
//https://github.com/rust-lang/rfcs/pull/2289 is needed to have generic struct member

/*
set up new mpsc to send from connect_changed to function
use mutex
figure out problem with thread_local
*/

//use std::sync::Arc;
//use std::sync::Mutex;
use std::rc::Rc;
use crate::glib::clone;

use gtk::prelude::*;
use gtk::{Application};
use gtk::TreeView;
use gtk::ListStore;
use gtk::TreeViewColumn;
use gtk::glib;
use gtk::CssProvider;
use gtk::TreePath;
use crate::glib::Value;
use gtk::StyleContext;
use gtk::gdk::Display;
use gtk::Label;
//use adw::prelude::*;
use std::cell::RefCell;
use std::sync::mpsc;

use adw::{ApplicationWindow, HeaderBar};

use std::error::Error;
use midir::{ Ignore, MidiInput, MidiOutput, MidiOutputConnection, MidiInputConnection };
use std::thread::sleep;
use std::time::Duration;


struct ParamListOption {
    id: u32,
    name: String,
    options: Vec<String>,
    default_index:usize, //index in options vector
    default_mesg:String,
    note:String,
}

struct ParamRangeOption {
    id: u32,
    name: String,
    min: u32,
    max: u32,
    default_index:usize, //index in min/max range
    default_mesg:String,
    note:String,
    prefix:String,
    suffix:String,
}

trait GenericOptions {
    fn get_options(&self) -> Vec<String>;
    fn get_name(&self) -> String;
    fn get_id(&self) -> u32;
    fn get_default_index(&self) -> usize;
}

impl GenericOptions for ParamRangeOption {
    fn get_options(&self) -> Vec<String> {
        let mut v = Vec::new();

        for (idx, i) in (self.min..=self.max).enumerate() {
            //let val:f32 = if self.id == 58 { i as f32 / 10 as f32 } else { i as f32 };
            if idx == self.default_index {
                v.push(format!("{}{}{} (Default{})", self.prefix, i, self.suffix, self.default_mesg));
            }
            else {
                v.push(format!("{}{}{}", self.prefix, i, self.suffix));
            }
        }
        v
    }
    fn get_name(&self) -> String {
        self.name.to_owned()
    }
    fn get_id(&self) -> u32 {
        self.id
    }
    fn get_default_index(&self) -> usize {
        self.default_index
    }
}

impl GenericOptions for ParamListOption {
    fn get_options(&self) -> Vec<String> {
        let a = self.options.to_owned();
        a.into_iter().enumerate().map(|(idx, r)| { if idx == self.default_index { format!("{} (Default{})", r, self.default_mesg) } else { r } }).collect::<Vec<String>>()
    }
    fn get_name(&self) -> String {
        self.name.to_owned()
    }
    fn get_id(&self) -> u32 {
        self.id
    }
    fn get_default_index(&self) -> usize {
        self.default_index
    }
}

struct UIModel {
    list_store:ListStore,
    label:Label,
    sources:gtk::ComboBoxText,
}
/* 
struct Midi {
    input: MidiInput,
    output: MidiOutput,
    input_port: MidiInputConnection,
    output_port: MidiOutputConnection,
}
*/
thread_local!(
    static GLOBAL_RX: RefCell<Option<mpsc::Receiver<Vec<u8>>>> = RefCell::new(None);

    static GLOBAL_UIMODEL: RefCell<Option<UIModel>> = RefCell::new(None);

    //static GLOBAL_MIDI_OUT: RefCell<Option<&'static mut MidiOutputConnection>> = RefCell::new(None);
);

const MAX_PARAM: usize = 75;

fn main() {
    
    let application = Application::builder()
        .application_id("com.philolog.matriarch-settings")
        .build();

        application.connect_startup(|_| {
            adw::init();
            load_css();
        });

    let params:[Box<dyn GenericOptions>; MAX_PARAM] = [
        Box::new(ParamRangeOption {
            id:0, 
            name:"Unit ID".to_string(), 
            min:0,
            max:15,
            default_index:0,
            default_mesg: "".to_string(),
            note:"".to_string(),
            prefix:"".to_string(),
            suffix:"".to_string()}),
        Box::new(ParamRangeOption {
            id:1, 
            name:"Tuning Scale".to_string(), 
            min: 0,
            max: 31, 
            default_index:0,
            default_mesg: " - 12-TET".to_string(),
            note:"".to_string(),
            prefix:"".to_string(),
            suffix:"".to_string()}),
        Box::new(ParamListOption {
            id:2, 
            name:"Knob Mode".to_string(), 
            options:vec!["Snap".to_string(), "Pass-Thru".to_string(), "Relative".to_string()],
            default_index:0,
            default_mesg: "".to_string(),
            note: "Actual default different to documented".to_string()}),
        Box::new(ParamListOption {
            id:3, 
            name:"Note Priority".to_string(), 
            options:vec!["Low".to_string(), "High".to_string(), "Last Note".to_string()],
            default_index:2,
            default_mesg: "".to_string(),
            note: "".to_string()}),
        Box::new(ParamListOption {
            id:4, 
            name:"Transmit Program Change".to_string(), 
            options:vec!["Off".to_string(), "On".to_string()],
            default_index:0,
            default_mesg: "".to_string(),
            note: "".to_string()}),
        Box::new(ParamListOption {
            id:5, 
            name:"Receive Program Change".to_string(), 
            options:vec!["Off".to_string(), "On".to_string()],
            default_index:1,
            default_mesg: "".to_string(),
            note: "".to_string()}),
        Box::new(ParamListOption {
            id:6, 
            name:"MIDI Input Ports".to_string(), 
            options:vec!["none".to_string(), "DIN only".to_string(), "USB only".to_string(), "Both DIN and USB".to_string()],
            default_index:3,
            default_mesg: "".to_string(),
            note: "".to_string()}),
        Box::new(ParamListOption {
            id:7, 
            name:"MIDI Output Ports".to_string(), 
            options:vec!["none".to_string(), "DIN only".to_string(), "USB only".to_string(), "Both DIN and USB".to_string()],
            default_index:3,
            default_mesg: "".to_string(),
            note: "".to_string()}),
        Box::new(ParamListOption {
            id:8, 
            name:"MIDI Echo USB In".to_string(), 
            options:vec!["Off".to_string(), "Echo USB In to DIN Out".to_string(), "Echo USB In to USB Out".to_string(), "Echo DIN In to Both DIN and USB Out".to_string()],
            default_index:0,
            default_mesg: "".to_string(),
            note: "".to_string()}),
        Box::new(ParamListOption {
            id:9, 
            name:"MIDI Echo DIN In".to_string(), 
            options:vec!["Off".to_string(), "Echo DIN In to DIN Out".to_string(), "Echo DIN In to USB Out".to_string(), "Echo DIN In to Both DIN and USB Out".to_string()],
            default_index:0,
            default_mesg: "".to_string(),
            note: "".to_string()}),
        Box::new(ParamRangeOption {
            id:10, 
            name:"MIDI Channel In".to_string(), 
            min:1,
            max:16,
            default_index:0,
            default_mesg: "".to_string(),
            note: "".to_string(),
            prefix:"Channel ".to_string(),
            suffix:"".to_string()}),
        Box::new(ParamRangeOption {
            id:11, 
            name:"MIDI Channel Out".to_string(), 
            min:1,
            max:16,
            default_index:0,
            default_mesg: "".to_string(),
            note: "".to_string(),
            prefix:"Channel ".to_string(),
            suffix:"".to_string()}),
        Box::new(ParamListOption {
            id:12, 
            name:"MIDI Out Filter - Keys".to_string(), 
            options:vec!["Off".to_string(), "On".to_string()],
            default_index:1,
            default_mesg: "".to_string(),
            note: "".to_string()}),
        Box::new(ParamListOption {
            id:13, 
            name:"MIDI Out Filter - Wheels".to_string(), 
            options:vec!["Off".to_string(), "On".to_string()],
            default_index:1,
            default_mesg: "".to_string(),
            note: "".to_string()}),
        Box::new(ParamListOption {
            id:14, 
            name:"MIDI Out Filter - Panel".to_string(), 
            options:vec!["Off".to_string(), "On".to_string()],
            default_index:1,
            default_mesg: "".to_string(),
            note: "".to_string()}),
        Box::new(ParamListOption {
            id:15, 
            name:"Output 14-bit MIDI CCs".to_string(), 
            options:vec!["Off".to_string(), "On".to_string()],
            default_index:0,
            default_mesg: "".to_string(),
            note: "".to_string()}),
        Box::new(ParamListOption {
            id:16, 
            name:"Local Control: Keys".to_string(), 
            options:vec!["Off".to_string(), "On".to_string()],
            default_index:1,
            default_mesg: "".to_string(),
            note: "".to_string()}),
        Box::new(ParamListOption {
            id:17, 
            name:"Local Control: Wheels".to_string(), 
            options:vec!["Off".to_string(), "On".to_string()],
            default_index:1,
            default_mesg: "".to_string(),
            note: "".to_string()}),
        Box::new(ParamListOption {
            id:18, 
            name:"Local Control: Panel".to_string(), 
            options:vec!["Off".to_string(), "On".to_string()],
            default_index:0,
            default_mesg: "".to_string(),
            note: "Actual default different to documented".to_string()}),
        Box::new(ParamListOption {
            id:19, 
            name:"Local Control: Arp/Seq".to_string(), 
            options:vec!["Off".to_string(), "On".to_string()],
            default_index:1,
            default_mesg: "".to_string(),
            note: "".to_string()}),
        Box::new(ParamListOption {
            id:20, 
            name:"Sequence Transpose Mode".to_string(), 
            options:vec!["Relative to First Note".to_string(), "Relative to Middle C".to_string()],
            default_index:0,
            default_mesg: "".to_string(),
            note: "".to_string()}),
        Box::new(ParamListOption {
            id:21, 
            name:"Arp/Seq Keyed Timing Reset".to_string(), 
            options:vec!["Off".to_string(), "On".to_string()],
            default_index:1,
            default_mesg: "".to_string(),
            note: "Actual default different to documented".to_string()}),
        Box::new(ParamListOption {
            id:22, 
            name:"Arp FW/BW Repeats".to_string(),
            options:vec!["Don't Repeat end notes".to_string(), "Repeat end notes".to_string()],
            default_index:1,
            default_mesg: "".to_string(),
            note: "".to_string()}),
        Box::new(ParamListOption {
            id:23, 
            name:"Arp/Seq Swing".to_string(),
            options:vec!["Slider".to_string(), "Slider".to_string()],
            default_index:0,
            default_mesg: "".to_string(),
            note: "".to_string()}), //Slider(0, 16383, 8192)
            /* new Param(23, "Arp/Seq Swing", Slider(0, 16383, 8192)), */
        Box::new(ParamListOption {
            id:24, 
            name:"Sequence Keyboard Control".to_string(),
            options:vec!["Off".to_string(), "On".to_string()],
            default_index:1,
            default_mesg: "".to_string(),
            note: "".to_string()}),
        Box::new(ParamListOption {
            id:25, 
            name:"Delay Sequence Change".to_string(),
            options:vec!["Off".to_string(), "On".to_string()],
            default_index:0,
            default_mesg: "".to_string(),
            note: "".to_string()}),
        Box::new(ParamListOption {
            id:26, 
            name:"Sequence Latch Restart".to_string(),
            options:vec!["Off".to_string(), "On".to_string()],
            default_index:1,
            default_mesg: "".to_string(),
            note: "Actual default different to documented".to_string()}),
        Box::new(ParamListOption {
            id:27, 
            name:"Arp/Seq Clock Input Mode".to_string(),
            options:vec!["Clock".to_string(), "Step-Advance Trigger".to_string()],
            default_index:0,
            default_mesg: "".to_string(),
            note: "".to_string()}),
        Box::new(ParamListOption {
            id:28, 
            name:"Arp/Seq Clock Output".to_string(),
            options:vec!["Always".to_string(), "Only When Playing".to_string()],
            default_index:1,
            default_mesg: "".to_string(),
            note: "".to_string()}),
        Box::new(ParamListOption {
            id:29, 
            name:"Arp MIDI Output".to_string(),
            options:vec!["Off".to_string(), "On".to_string()],
            default_index:1,
            default_mesg: "".to_string(),
            note: "".to_string()}),
        Box::new(ParamListOption {
            id:30, 
            name:"Send MIDI Clock".to_string(),
            options:vec!["Off".to_string(), "On When Playing".to_string()],
            default_index:0,
            default_mesg: "".to_string(),
            note: "".to_string()}),
        Box::new(ParamListOption {
            id:31, 
            name:"Send MIDI Start/Stop".to_string(),
            options:vec!["Off".to_string(), "On".to_string()],
            default_index:0,
            default_mesg: "".to_string(),
            note: "".to_string()}),
        Box::new(ParamListOption {
            id:32, 
            name:"Follow MIDI Clock".to_string(),
            options:vec!["Off".to_string(), "On".to_string()],
            default_index:1,
            default_mesg: "".to_string(),
            note: "".to_string()}),
        Box::new(ParamListOption {
            id:33, 
            name:"Follow MIDI Start/Stop".to_string(),
            options:vec!["Off".to_string(), "On".to_string()],
            default_index:1,
            default_mesg: "".to_string(),
            note: "".to_string()}),
        Box::new(ParamListOption {
            id:34, 
            name:"Follow Song Position Pointer".to_string(),
            options:vec!["Off".to_string(), "On".to_string()],
            default_index:1,
            default_mesg: "".to_string(),
            note: "".to_string()}),
        Box::new(ParamRangeOption {
            id:35, 
            name:"Clock Input PPQN Index".to_string(), 
            min:0,
            max:15,
            default_index:3,
            default_mesg: " sixteenth notes".to_string(),
            note: "".to_string(),
            prefix:"".to_string(),
            suffix:" PPQN".to_string()}),
        Box::new(ParamRangeOption {
            id:36, 
            name:"Clock Output PPQN Index".to_string(), 
            min:0,
            max:15,
            default_index:3,
            default_mesg: " sixteenth notes".to_string(),
            note: "".to_string(),
            prefix:"".to_string(),
            suffix:" PPQN".to_string()}),
        Box::new(ParamRangeOption {
            id:37, 
            name:"Pitch Bend Range (Semitones)".to_string(), 
            min:0,
            max:12,
            default_index:2,
            default_mesg: " sixteenth notes".to_string(),
            note: "".to_string(),
            prefix:"".to_string(),
            suffix:" PPQN".to_string()}),
        Box::new(ParamListOption {
            id:38, 
            name:"Keyboard Octave Transpose".to_string(),
            options:vec!["-2".to_string(), "-1".to_string(), "0".to_string(), "1".to_string(), "2".to_string()],
            default_index:2,
            default_mesg: " no transpose".to_string(),
            note: "".to_string()}),
        Box::new(ParamListOption {
            id:39, 
            name:"Delayed Keyboard Octave Transpose".to_string(),
            options:vec!["Off".to_string(), "On".to_string()],
            default_index:1,
            default_mesg: "".to_string(),
            note: "".to_string()}),
        Box::new(ParamListOption {
            id:40, 
            name:"Glide Type".to_string(),
            options:vec!["Linear Constant Rate".to_string(), "Linear Constant Time".to_string(), "Exponential".to_string()],
            default_index:0,
            default_mesg: "".to_string(),
            note: "".to_string()}),
        Box::new(ParamListOption {
            id:41, 
            name:"Gated Glid".to_string(),
            options:vec!["Linear Constant Rate".to_string(), "Linear Constant Time".to_string(), "Exponential".to_string()],
            default_index:1,
            default_mesg: "".to_string(),
            note: "".to_string()}),
        Box::new(ParamListOption {
            id:42, 
            name:"Legato Glide".to_string(),
            options:vec!["Linear Constant Rate".to_string(), "Linear Constant Time".to_string(), "Exponential".to_string()],
            default_index:0,
            default_mesg: "".to_string(),
            note: "Actual default different to documented".to_string()}),
        Box::new(ParamRangeOption {
            id:43, 
            name:"Osc 2 Freq Knob Range".to_string(), 
            min:0,
            max:24,
            default_index:7,
            default_mesg: "".to_string(),
            note: "".to_string(),
            prefix:"".to_string(),
            suffix:" Semitones".to_string()}),
        Box::new(ParamRangeOption {
            id:44, 
            name:"Osc 3 Freq Knob Range".to_string(), 
            min:0,
            max:24,
            default_index:7,
            default_mesg: "".to_string(),
            note: "".to_string(),
            prefix:"".to_string(),
            suffix:" Semitones".to_string()}),
        Box::new(ParamRangeOption {
            id:45, 
            name:"Osc 4 Freq Knob Range".to_string(), 
            min:0,
            max:24,
            default_index:7,
            default_mesg: "".to_string(),
            note: "".to_string(),
            prefix:"".to_string(),
            suffix:" Semitones".to_string()}),
        Box::new(ParamListOption {
            id:46, 
            name:"Hard Sync Enable".to_string(),
            options:vec!["Off".to_string(), "On".to_string()],
            default_index:0,
            default_mesg: "".to_string(),
            note: "".to_string()}),
        Box::new(ParamListOption {
            id:47, 
            name:"Osc 2 Hard Sync".to_string(),
            options:vec!["Off".to_string(), "On".to_string()],
            default_index:0,
            default_mesg: "".to_string(),
            note: "".to_string()}),
        Box::new(ParamListOption {
            id:48, 
            name:"Osc 3 Hard Sync".to_string(),
            options:vec!["Off".to_string(), "On".to_string()],
            default_index:0,
            default_mesg: "".to_string(),
            note: "".to_string()}),
        Box::new(ParamListOption {
            id:49, 
            name:"Osc 4 Hard Sync".to_string(),
            options:vec!["Off".to_string(), "On".to_string()],
            default_index:0,
            default_mesg: "".to_string(),
            note: "".to_string()}),
        Box::new(ParamListOption {
            id:50, 
            name:"Delay Ping Pong".to_string(),
            options:vec!["Off".to_string(), "On".to_string()],
            default_index:0,
            default_mesg: "".to_string(),
            note: "".to_string()}),
        Box::new(ParamListOption {
            id:51, 
            name:"Delay Sync".to_string(),
            options:vec!["Off".to_string(), "On".to_string()],
            default_index:0,
            default_mesg: "".to_string(),
            note: "".to_string()}),
        Box::new(ParamListOption {
            id:52, 
            name:"Delay Filter Brightness".to_string(),
            options:vec!["Dark".to_string(), "Bright".to_string()],
            default_index:1,
            default_mesg: "".to_string(),
            note: "".to_string()}),
        Box::new(ParamListOption {
            id:53, 
            name:"Delay CV Sync-Bend".to_string(),
            options:vec!["Off".to_string(), "On".to_string()],
            default_index:0,
            default_mesg: "".to_string(),
            note: "".to_string()}),
        Box::new(ParamListOption {
            id:54, 
            name:"Tap-Tempo Clock Division Persistence".to_string(),
            options:vec!["Off".to_string(), "On".to_string()],
            default_index:0,
            default_mesg: "".to_string(),
            note: "".to_string()}),
        Box::new(ParamListOption {
            id:55, 
            name:"Paraphony Mode".to_string(),
            options:vec!["Mono".to_string(), "Duo".to_string(), "Quad".to_string()],
            default_index:2,
            default_mesg: "".to_string(),
            note: "Actual default different to documented".to_string()}),
        Box::new(ParamListOption {
            id:56, 
            name:"Paraphonic Unison".to_string(),
            options:vec!["Off".to_string(), "On".to_string()],
            default_index:0,
            default_mesg: "".to_string(),
            note: "".to_string()}),
        Box::new(ParamListOption {
            id:57, 
            name:"Multi Trig".to_string(),
            options:vec!["Off".to_string(), "On".to_string()],
            default_index:0,
            default_mesg: "".to_string(),
            note: "".to_string()}),
        Box::new(ParamRangeOption {
            id:58, 
            name:"Pitch Variance".to_string(), 
            min:0,
            max:400,
            default_index:7,
            default_mesg: "".to_string(),
            note: "".to_string(),
            prefix:"± ".to_string(),
            suffix:" cents".to_string()}),
    /* new Param(58, "Pitch Variance", Options(Range(0, 400).map(i => "± " + (i / 10) + " cents"), 0)),*/
        Box::new(ParamListOption {
            id:59, 
            name:"KB CV OUT Range".to_string(),
            options:vec!["-5V to +5V".to_string(), "0V to 10V".to_string()],
            default_index:0,
            default_mesg: "".to_string(),
            note: "".to_string()}),
        Box::new(ParamListOption {
            id:60, 
            name:"Arp/Seq CV OUT Range".to_string(),
            options:vec!["-5V to +5V".to_string(), "0V to 10V".to_string()],
            default_index:0,
            default_mesg: "".to_string(),
            note: "".to_string()}),
        Box::new(ParamListOption {
            id:61, 
            name:"KB VEL OUT Range".to_string(),
            options:vec!["-5V to +5V".to_string(), "0V to 10V".to_string()],
            default_index:0,
            default_mesg: "".to_string(),
            note: "".to_string()}),
        Box::new(ParamListOption {
            id:62, 
            name:"Arp/Seq VEL OUT Range".to_string(),
            options:vec!["-5V to +5V".to_string(), "0V to 10V".to_string()],
            default_index:0,
            default_mesg: "".to_string(),
            note: "".to_string()}),
        Box::new(ParamListOption {
            id:63, 
            name:"KB AT OUT Range".to_string(),
            options:vec!["-5V to +5V".to_string(), "0V to 10V".to_string()],
            default_index:0,
            default_mesg: "".to_string(),
            note: "".to_string()}),
        Box::new(ParamListOption {
            id:64, 
            name:"MOD WHL OUT Range".to_string(),
            options:vec!["-5V to +5V".to_string(), "0V to 10V".to_string()],
            default_index:0,
            default_mesg: "".to_string(),
            note: "".to_string()}),
        Box::new(ParamListOption {
            id:65, 
            name:"KB GATE OUT Range".to_string(),
            options:vec!["-5V to +5V".to_string(), "0V to 10V".to_string()],
            default_index:0,
            default_mesg: "".to_string(),
            note: "".to_string()}),
        Box::new(ParamListOption {
            id:66, 
            name:"Arp/Seq GATE OUT Range".to_string(),
            options:vec!["-5V to +5V".to_string(), "0V to 10V".to_string()],
            default_index:0,
            default_mesg: "".to_string(),
            note: "".to_string()}),
        Box::new(ParamListOption {
            id:67, 
            name:"Round-Robin Mode".to_string(),
            options:vec!["Off".to_string(), "First-Note Reset".to_string(), "On".to_string()],
            default_index:1,
            default_mesg: "".to_string(),
            note: "".to_string()}),
        Box::new(ParamListOption {
            id:68, 
            name:"Restore Stolen Voices".to_string(),
            options:vec!["Off".to_string(), "On".to_string()],
            default_index:0,
            default_mesg: "".to_string(),
            note: "".to_string()}),
        Box::new(ParamListOption {
            id:69, 
            name:"Update Unison on Note-Off".to_string(),
            options:vec!["Off".to_string(), "On".to_string()],
            default_index:0,
            default_mesg: "".to_string(),
            note: "".to_string()}),
        Box::new(ParamListOption {
            id:70, 
            name:"Mod Oscillator Square Wave Polarity".to_string(),
            options:vec!["Unipolar".to_string(), "Bipolar".to_string()],
            default_index:1,
            default_mesg: "".to_string(),
            note: "".to_string()}),
        Box::new(ParamListOption {
            id:71, 
            name:"Noise Filter Cutoff".to_string(),
            options:vec!["Slider".to_string(), "Slider".to_string()],
            default_index:0,
            default_mesg: "".to_string(),
            note: "Actual default different to documented".to_string()}), //Slider(0, 16383, 0)
    /* new Param(71, "Noise Filter Cutoff", Slider(0, 16383, 0), "Actual default different to documented"),*/
        Box::new(ParamListOption {
            id:72, 
            name:"Arp/Seq Random Repeats".to_string(),
            options:vec!["no repeating notes/steps in RND direction".to_string(), "allow repeating notes (true random)".to_string()],
            default_index:1,
            default_mesg: "".to_string(),
            note: "".to_string()}),
        Box::new(ParamListOption {
            id:73, 
            name:"ARP/SEQ CV OUT Mirrors KB CV".to_string(),
            options:vec!["Off".to_string(), "On".to_string()],
            default_index:0,
            default_mesg: "".to_string(),
            note: "".to_string()}),
        Box::new(ParamListOption {
            id:74, 
            name:"KB CV OUT Mirrors ARP/SEQ CV".to_string(),
            options:vec!["Off".to_string(), "On".to_string()],
            default_index:0,
            default_mesg: "".to_string(),
            note: "".to_string()}),
    ];

    //let data = Arc::new(Mutex::new(0i32));

    let (tx, rx) = mpsc::channel();
    GLOBAL_RX.with(|global| {
        *global.borrow_mut() = Some(rx);
    });

    let midi_out = MidiOutput::new("Matriarch Settings Output").unwrap();
    let out_ports = midi_out.ports();
    let mut conn_out = Rc::new(RefCell::new(None)); //https://github.com/gtk-rs/examples/issues/115

    if !out_ports.is_empty() {
        let sources = get_out_sources(&midi_out);
        for (i, name) in sources.iter().enumerate() {
            if name.to_lowercase().contains("moog matriarch") {
                let out_port = &out_ports[i];

                if let Ok(out) = midi_out.connect(out_port, "Matriarch Settings Output Connection") {
                    conn_out = Rc::new(RefCell::new(Some(out)));
                }
                break;
            }
        }
    }

    let mut midi_in = MidiInput::new("midir reading input").unwrap();
    midi_in.ignore(Ignore::None);
    let sources = get_in_sources(&midi_in);
    for a in &sources {
        println!("{}", a);
    }

    let in_ports = midi_in.ports();
    let mut selected_port:Option<u32> = None;
    let _conn_in; //declare here for scope
    if !in_ports.is_empty() {
        let sources = get_in_sources(&midi_in);
        for (i, name) in sources.iter().enumerate() {
            if name.to_lowercase().contains("moog matriarch") {
                selected_port = Some(i as u32);
                let in_port = &in_ports[i];

                _conn_in = midi_in.connect(in_port, "midir-read-input", move |stamp, message, tx| {
                    println!("received: {}: {:02X?} (len = {})", stamp, message, message.len());

                    tx.send(message.to_vec()).unwrap();
                    
                    glib::source::idle_add(|| { // tell ui thread to read from channel
                        check_for_new_message();
                        glib::source::Continue(false)
                    });

                }, tx).unwrap();
                break;
            }
        }
    }

    application.connect_activate(clone!(@weak conn_out => move |app| {
            
        let button = gtk::Button::with_label("Connect");
        button.connect_clicked(|_| {
            println!("Connect clicked!");
        });

        let combo = gtk::ComboBoxText::new();
        if sources.is_empty() {
            combo.append_text("No midi devices connected");
        }
        else {
            for i in &sources {
                combo.append_text(i);
            }
        }

        combo.set_active(selected_port);
        combo.connect_changed( /*gtk::glib::clone!( @weak model_list_of_data as l, @weak conn_out => move */|combo_selected_iter| { 
            println!("combo changed");
           /*  if let Some(list_iter) = l.iter(&list_path) {
                if let Ok(combo_model) = l.get_value(&list_iter, 2).get::<ListStore>() {
                    if let Ok(combo_selected_value) = combo_model.get_value(combo_selected_iter, 0).get::<String>() {
                        l.set_value(&list_iter, 3, &combo_selected_value.to_value() ); 

                        let param_id = list_path.indices()[0];
                        let param_index = combo_model.path(combo_selected_iter).indices()[0];
                        //if let Some(ref mut xx) = conn_out {
                            param_changed(conn_out, param_id.try_into().unwrap(), param_index, &combo_selected_value);
                        //}
                    }
                }
            }*/
        } /*)*/ );


        let view_list = TreeView::new();
        
        let types_inside_columns = &[gtk::glib::Type::U32, gtk::glib::Type::STRING, gtk::ListStore::static_type(), gtk::glib::Type::STRING];
        let model_list_of_data = ListStore::new(types_inside_columns);

        for p in &params {
            let model_for_combo = ListStore::new(&[gtk::glib::Type::STRING]);
            for o in &p.get_options() {
                model_for_combo.insert_with_values(None, &[(0, &o)]);
            }
            model_list_of_data.insert_with_values(None, 
                &[
                    ( 0, &p.get_id() ), 
                    ( 1, &p.get_name() ), 
                    ( 2, &model_for_combo ), 
                    ( 3, &p.get_options()[ p.get_default_index() ] )
                ]);
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
        object_to_render_cells_3.set_has_entry(false); //whether it also has a text entry besides the combo options

        // set column 3 of list model to selected value from combo so that it will be displayed once selected
        object_to_render_cells_3.connect_changed( gtk::glib::clone!( @weak model_list_of_data as l, @weak conn_out => move |_cell, list_path, combo_selected_iter| { 
            if let Some(list_iter) = l.iter(&list_path) {
                if let Ok(combo_model) = l.get_value(&list_iter, 2).get::<ListStore>() {
                    if let Ok(combo_selected_value) = combo_model.get_value(combo_selected_iter, 0).get::<String>() {
                        l.set_value(&list_iter, 3, &combo_selected_value.to_value() ); 

                        let param_id = list_path.indices()[0];
                        let param_index = combo_model.path(combo_selected_iter).indices()[0];
                        //if let Some(ref mut xx) = conn_out {
                            param_changed(conn_out, param_id.try_into().unwrap(), param_index, &combo_selected_value);
                        //}
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
        
        view_list.expand_all();

        let scrolled_window = gtk::ScrolledWindow::builder()
            .hscrollbar_policy(gtk::PolicyType::Never) // Disable horizontal scrolling
            .min_content_width(360)
            .vexpand(true)
            .child(&view_list)
            .build();

            let hbox: gtk::Box = gtk::Box::new(gtk::Orientation::Horizontal, 4);
            hbox.set_homogeneous(true);
            hbox.prepend(&button);
            hbox.append(&combo);

        let vbox: gtk::Box = gtk::Box::new(gtk::Orientation::Vertical, 4);
        vbox.set_homogeneous(false);
        vbox.append(
            &HeaderBar::builder()
                .title_widget(&adw::WindowTitle::new("Moog Matriarch Global Settings", ""))
                .build(),
        );
        vbox.append(&hbox);
        vbox.append(&scrolled_window);

        let midi_received_label = gtk::Label::new(Some(""));
        //entry.set_xalign(0.0);
        midi_received_label.set_halign(gtk::Align::End);
        //entry.set_justify(gtk::Justification::Right);
        midi_received_label.set_margin_top(2);
        midi_received_label.set_margin_bottom(4);
        midi_received_label.set_margin_start(8);
        midi_received_label.set_margin_end(8);
        vbox.append(&midi_received_label);

        GLOBAL_UIMODEL.with(|global| {
            *global.borrow_mut() = Some(UIModel {list_store:model_list_of_data, label:midi_received_label, sources:combo});
        });

        //https://github.com/gtk-rs/gtk4-rs/blob/9a70b149ca0aad042e7bf0cec3bcd8c781eb62a4/gtk4/README.md
        glib::timeout_add_local(Duration::from_millis(5000), clone!(@weak conn_out => @default-return glib::Continue(true), move || {
            //let a = get_in_sources(&midi_in);
            /*
            widgets.main_view.progress.set_fraction(0.0);
            widgets
                .view_stack
                .set_visible_child(&widgets.main_view.container);
                */
                println!("check");
            glib::Continue(true)
        }) );

        let window = ApplicationWindow::builder()
            .application(app)
            .title("Moog Matriarch Global Settings")
            .default_width(550)
            .default_height(400)
            .content(&vbox)
            .build();
        window.show();


    }));
    
    if let Some(a) = &mut *conn_out.borrow_mut() {
        for p in 0..MAX_PARAM {
            sleep(Duration::from_millis(200));
            match read_param(a, p.try_into().unwrap() ) {
                Ok(_a) => (),
                Err(_e) => panic!("error reading param"),
            }
        }
    }
    
    application.run();
}

fn update_param_row(list: &ListStore, param_row:i32, param_value:i32) {
    println!("Updating ui for param {} to {}.", param_row, param_value);
    //if param_row > -1 && param_row < 23 { //this is already guarded by whether the row_iter exists
        let row_path = TreePath::from_indices(&[param_row]);
        if let Some(row_iter) = list.iter(&row_path) {

            if let Ok(combo_liststore) = list.get_value(&row_iter, 2).get::<ListStore>() {
                let combo_path = TreePath::from_indices(&[param_value]);
                if let Some(combo_iter) = combo_liststore.iter(&combo_path) {
                    if let Ok(combo_value) = combo_liststore.get_value(&combo_iter, 0).get::<Value>() {
                        list.set_value(&row_iter, 3, &combo_value );
                    }
                }
            }
        }
    //}
}

fn check_for_new_message() {
    GLOBAL_RX.with(|global| {
        //println!("checking for message...");
        if let Some(rx) = &*global.borrow() {
            let received: Vec<u8> = rx.recv().unwrap();
            //ui.main_buffer.set_text(&received);
            println!("passed message: {:02X?}", received);

            GLOBAL_UIMODEL.with(|global| {
                if let Some(uimodel) = &*global.borrow() {
                    uimodel.label.set_text(&format!("Received: {:02X?}", received));

                    if !received.is_empty() && received.len() >= 7 && received[0] == 0xF0 && received[received.len() - 1] == 0xF7 { //if sysex
                        let param_row = received[4];
                
                        let msb = received[5];
                        let lsb = received[6];
                        let param_value:i32 = (128 * msb as i32 + lsb as i32).into();
        
                        //let param_row:i32 = 1;
                        //let param_value:i32 = 9;
                        if param_row != 23 && param_row != 71 { //ignore sliders for now
                            update_param_row(&uimodel.list_store, param_row.into(), param_value.into());
                        }
                    }
                }
            });
        }
    });
}

fn param_changed(conn_out: Rc<RefCell<Option<MidiOutputConnection>>>, id: u8, param_index: i32, value: &str) {
    println!("changed row index: {}, combo index: {}, value: {:?}", id, param_index, value);
    GLOBAL_UIMODEL.with(|global| {
        if let Some(uimodel) = &*global.borrow() {
            uimodel.label.set_text(""); //clear
            match set_param(conn_out, id, param_index) {
                Ok(msg) => {
                    uimodel.label.set_text(&format!("Set param: {:02X?}", msg));
                },
                Err(e) => {
                    println!("Error setting param: {:?}", e);
                }
            }
        }
    });
}

fn set_param(conn_out: Rc<RefCell<Option<MidiOutputConnection>>>, param_id: u8, value: i32) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut msb = 0;
    let mut lsb = value;
    if value > 128 { 
        msb = value / 128; 
        lsb = value % 128; 
    }
    let msg:[u8; 17] = [0xf0, 0x04, 0x17, 0x23, param_id, msb.try_into().unwrap(), lsb.try_into().unwrap(), 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x7f, 0xf7];
    if let Some(out) = &mut *conn_out.borrow_mut() {
        println!("Sending set param: {:02X?}", msg);
        out.send(&msg)?;
    }
    Ok(msg.to_vec())
}

fn read_param(conn_out: &mut MidiOutputConnection, param_id: u8) -> Result<(), Box<dyn Error>> {
    let msg: [u8; 17] = [0xf0, 0x04, 0x17, 0x3e, param_id, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x7f, 0xf7];
    println!("Request param: {}",  param_id);
    //sleep(Duration::from_millis(200));
    conn_out.send(&msg)?;

    Ok(())
}

fn load_css() {
    // Load the CSS file and add it to the provider
    let provider = CssProvider::new();
    provider.load_from_path("style.css");//or: /User/jeremy/Documents/code/matriarch-settings/style.css
    //provider.load_from_data(include_bytes!("style.css"));

    // Add the provider to the default screen
    StyleContext::add_provider_for_display(
        &Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

fn get_in_sources(input: &MidiInput/* , output: MidiOutput*/) -> Vec<String> {
    let mut v = Vec::new();
    //println!("Available input ports:");
    for (_i, p) in input.ports().iter().enumerate() {
        if let Ok(port) = input.port_name(p) {
            v.push(port);
            //println!("{}: {}", i, port);
        }
    }
    v
}

fn get_out_sources(output: &MidiOutput/* , output: MidiOutput*/) -> Vec<String> {
    let mut v = Vec::new();
    //println!("Available input ports:");
    for (_i, p) in output.ports().iter().enumerate() {
        if let Ok(port) = output.port_name(p) {
            v.push(port);
            //println!("{}: {}", i, port);
        }
    }
    v
}

//use gtk4 as gtk;
//https://kornel.ski/rust-sys-crate
//https://github.com/chris-zen/coremidi/blob/master/examples/receive.rs
//https://github.com/gtk-rs/examples/blob/master/src/bin/listbox_model.rs
//https://users.rust-lang.org/t/how-to-right-justify-numeric-data-in-gtk-rs-list-store/30538

//scrolling on combobox
//https://mail.gnome.org/archives/gtk-list/2016-December/msg00035.html
//https://stackoverflow.com/questions/873328/how-do-i-put-a-scrollbar-inside-of-a-gtk-comboboxentry

//https://stackoverflow.com/questions/66510406/gtk-rs-how-to-update-view-from-another-thread

use gtk::prelude::*;
use gtk::{Application};
use gtk::TreeView;
//use gtk::ListView;
use gtk::ListStore;
use gtk::TreeViewColumn;
use gtk::glib;
use gtk::CssProvider;
use gtk::StyleContext;
use gtk::gdk::Display;
//use adw::prelude::*;

use adw::{ApplicationWindow, HeaderBar};

struct ParamListOption {
    id: u32,
    name: String,
    value:String,
    options: Vec<String>,
    default_index:usize, //index in options vector
    default_mesg:String,
    note:String,
}

struct ParamRangeOption {
    id: u32,
    name: String,
    value:String,
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
        let mut idx:usize = 0;
        for i in self.min..=self.max {
            if idx == self.default_index {
                v.push(format!("{}{}{} (Default{})", self.prefix, i, self.suffix, self.default_mesg));
            }
            else {
                v.push(format!("{}{}{}", self.prefix, i, self.suffix));
            }
            idx += 1;
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
        a.into_iter().enumerate().map(|(idx, r)| { if idx == self.default_index { format!("{} (Default{})", r, self.default_mesg) } else { r.to_owned() } }).collect::<Vec<String>>()
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

//use coremidi; //or https://github.com/Boddlnagg/midir
//use std::error::Error;
use midir::{MidiInput, Ignore};
//use std::env;

use std::sync::Arc;

fn main() {
    let application = Application::builder()
        .application_id("com.philolog.matriarch-settings")
        .build();

        application.connect_startup(|_| {
            load_css(); 
            adw::init();
        });

        

    //https://stackoverflow.com/questions/53216593/vec-of-generics-of-different-concrete-types
    //https://github.com/rust-lang/rfcs/pull/2289 is needed to have generic struct member
    let mut params:[Box<dyn GenericOptions>; 23] = [
        Box::new(ParamRangeOption {
            id:0, 
            name:"Unit ID".to_string(), 
            value:"".to_string(), 
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
            value:"".to_string(), 
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
            value:"".to_string(), 
            options:vec!["Snap".to_string(), "Pass-Thru".to_string(), "Relative".to_string()],
            default_index:0,
            default_mesg: "".to_string(),
            note: "Actual default different to documented".to_string()}),
        Box::new(ParamListOption {
            id:3, 
            name:"Note Priority".to_string(), 
            value:"".to_string(), 
            options:vec!["Low".to_string(), "High".to_string(), "Last Note".to_string()],
            default_index:2,
            default_mesg: "".to_string(),
            note: "".to_string()}),
        Box::new(ParamListOption {
            id:4, 
            name:"Transmit Program Change".to_string(), 
            value:"".to_string(), 
            options:vec!["Off".to_string(), "On".to_string()],
            default_index:0,
            default_mesg: "".to_string(),
            note: "".to_string()}),
        Box::new(ParamListOption {
            id:5, 
            name:"Receive Program Change".to_string(), 
            value:"".to_string(), 
            options:vec!["Off".to_string(), "On".to_string()],
            default_index:1,
            default_mesg: "".to_string(),
            note: "".to_string()}),
        Box::new(ParamListOption {
            id:6, 
            name:"MIDI Input Ports".to_string(), 
            value:"".to_string(), 
            options:vec!["none".to_string(), "DIN only".to_string(), "USB only".to_string(), "Both DIN and USB".to_string()],
            default_index:3,
            default_mesg: "".to_string(),
            note: "".to_string()}),
        Box::new(ParamListOption {
            id:7, 
            name:"MIDI Output Ports".to_string(), 
            value:"".to_string(), 
            options:vec!["none".to_string(), "DIN only".to_string(), "USB only".to_string(), "Both DIN and USB".to_string()],
            default_index:3,
            default_mesg: "".to_string(),
            note: "".to_string()}),
        Box::new(ParamListOption {
            id:8, 
            name:"MIDI Echo USB In".to_string(), 
            value:"".to_string(), 
            options:vec!["Off".to_string(), "Echo USB In to DIN Out".to_string(), "Echo USB In to USB Out".to_string(), "Echo DIN In to Both DIN and USB Out".to_string()],
            default_index:0,
            default_mesg: "".to_string(),
            note: "".to_string()}),
        Box::new(ParamListOption {
            id:9, 
            name:"MIDI Echo DIN In".to_string(), 
            value:"".to_string(), 
            options:vec!["Off".to_string(), "Echo DIN In to DIN Out".to_string(), "Echo DIN In to USB Out".to_string(), "Echo DIN In to Both DIN and USB Out".to_string()],
            default_index:0,
            default_mesg: "".to_string(),
            note: "".to_string()}),
        Box::new(ParamRangeOption {
            id:10, 
            name:"MIDI Channel In".to_string(), 
            value:"".to_string(), 
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
            value:"".to_string(), 
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
            value:"".to_string(), 
            options:vec!["Off".to_string(), "On".to_string()],
            default_index:1,
            default_mesg: "".to_string(),
            note: "".to_string()}),
        Box::new(ParamListOption {
            id:13, 
            name:"MIDI Out Filter - Wheels".to_string(), 
            value:"".to_string(), 
            options:vec!["Off".to_string(), "On".to_string()],
            default_index:1,
            default_mesg: "".to_string(),
            note: "".to_string()}),
        Box::new(ParamListOption {
            id:14, 
            name:"MIDI Out Filter - Panel".to_string(), 
            value:"".to_string(), 
            options:vec!["Off".to_string(), "On".to_string()],
            default_index:1,
            default_mesg: "".to_string(),
            note: "".to_string()}),
        Box::new(ParamListOption {
            id:15, 
            name:"Output 14-bit MIDI CCs".to_string(), 
            value:"".to_string(), 
            options:vec!["Off".to_string(), "On".to_string()],
            default_index:0,
            default_mesg: "".to_string(),
            note: "".to_string()}),
        Box::new(ParamListOption {
            id:16, 
            name:"Local Control: Keys".to_string(), 
            value:"".to_string(), 
            options:vec!["Off".to_string(), "On".to_string()],
            default_index:1,
            default_mesg: "".to_string(),
            note: "".to_string()}),
        Box::new(ParamListOption {
            id:17, 
            name:"Local Control: Wheels".to_string(), 
            value:"".to_string(), 
            options:vec!["Off".to_string(), "On".to_string()],
            default_index:1,
            default_mesg: "".to_string(),
            note: "".to_string()}),
        Box::new(ParamListOption {
            id:18, 
            name:"Local Control: Panel".to_string(), 
            value:"".to_string(), 
            options:vec!["Off".to_string(), "On".to_string()],
            default_index:0,
            default_mesg: "".to_string(),
            note: "Actual default different to documented".to_string()}),
        Box::new(ParamListOption {
            id:19, 
            name:"Local Control: Arp/Seq".to_string(), 
            value:"".to_string(), 
            options:vec!["Off".to_string(), "On".to_string()],
            default_index:1,
            default_mesg: "".to_string(),
            note: "".to_string()}),
        Box::new(ParamListOption {
            id:20, 
            name:"Sequence Transpose Mode".to_string(), 
            value:"".to_string(), 
            options:vec!["Relative to First Note".to_string(), "Relative to Middle C".to_string()],
            default_index:0,
            default_mesg: "".to_string(),
            note: "".to_string()}),
        Box::new(ParamListOption {
            id:21, 
            name:"Arp/Seq Keyed Timing Reset".to_string(), 
            value:"".to_string(), 
            options:vec!["Off".to_string(), "On".to_string()],
            default_index:1,
            default_mesg: "".to_string(),
            note: "Actual default different to documented".to_string()}),
        Box::new(ParamListOption {
            id:22, 
            name:"Arp FW/BW Repeats".to_string(), 
            value:"".to_string(), 
            options:vec!["Don't Repeat end notes".to_string(), "Repeat end notes".to_string()],
            default_index:1,
            default_mesg: "".to_string(),
            note: "".to_string()}),

                /*
    new Param(23, "Arp/Seq Swing", Slider(0, 16383, 8192)),
    new Param(24, "Sequence Keyboard Control", Options(["Off", "On"], 1)),
    new Param(25, "Delay Sequence Change", Options(["Off", "On"], 0)),
    new Param(26, "Sequence Latch Restart", Options(["Off", "On"], 1), "Actual default different to documented"),
    new Param(27, "Arp/Seq Clock Input Mode",
        Options(["Clock", "Step-Advance Trigger"], 0)),
    new Param(28, "Arp/Seq Clock Output",
        Options(["Always", "Only When Playing"], 1)),
    new Param(29, "Arp MIDI Output", Options(["Off", "On"], 1)),
    new Param(30, "Send MIDI Clock", Options(["Off", "Only When Playing"], 0)),
    new Param(31, "Send MIDI Start/Stop", Options(["Off", "On"], 0)),
    new Param(32, "Follow MIDI Clock", Options(["Off", "On"], 1)),
    new Param(33, "Follow MIDI Start/Stop", Options(["Off", "On"], 1)),
    new Param(34, "Follow Song Position Pointer", Options(["Off", "On"], 1)),
    new Param(35, "Clock Input PPQN Index",
        Options(Range(0, 15), 3, "sixteenth notes [4PPQN]")),
    new Param(36, "Clock Output PPQN Index",
        Options(Range(0, 15), 3, "sixteenth notes [4PPQN]")),
    new Param(37, "Pitch Bend Range (Semitones)",
        Options(Range(0, 12), 2)),
    new Param(38, "Keyboard Octave Transpose",
        Options(["-2", "-1", "0", "1", "2"], 2, "no transpose")),
    new Param(39, "Delayed Keyboard Octave Transpose", Options(["Off", "On"], 1)),
    new Param(40, "Glide Type",
        Options(["Linear Constant Rate", "Linear Constant Time", "Exponential"], 0)),
    new Param(41, "Gated Glide", Options(["Off", "On"], 1)),
    new Param(42, "Legato Glide", Options(["Off", "On"], 0), "Actual default different to documented"),
    new Param(43, "Osc 2 Freq Knob Range",
        Options(Range(0, 24).map(i => i + " Semitones"), 7)),
    new Param(44, "Osc 3 Freq Knob Range",
        Options(Range(0, 24).map(i => i + " Semitones"), 7)),
    new Param(45, "Osc 4 Freq Knob Range",
        Options(Range(0, 24).map(i => i + " Semitones"), 7)),
    new Param(46, "Hard Sync Enable", Options(["Off", "On"], 0)),
    new Param(47, "Osc 2 Hard Sync", Options(["Off", "On"], 0)),
    new Param(48, "Osc 3 Hard Sync", Options(["Off", "On"], 0)),
    new Param(49, "Osc 4 Hard Sync", Options(["Off", "On"], 0)),
    new Param(50, "Delay Ping Pong", Options(["Off", "On"], 0)),
    new Param(51, "Delay Sync", Options(["Off", "On"], 0)),
    new Param(52, "Delay Filter Brightness", Options(["Dark", "Bright"], 1)),
    new Param(53, "Delay CV Sync-Bend", Options(["Off", "On"], 0)),
    new Param(54, "Tap-Tempo Clock Division Persistence", Options(["Off", "On"], 0)),
    new Param(55, "Paraphony Mode", Options(["Mono", "Duo", "Quad"], 2), "Actual default different to documented"),
    new Param(56, "Paraphonic Unison", Options(["Off", "On"], 0)),
    new Param(57, "Multi Trig", Options(["Off", "On"], 0)),
    new Param(58, "Pitch Variance",
        Options(Range(0, 400).map(i => "± " + (i / 10) + " cents"), 0)),
    new Param(59, "KB CV OUT Range", Options(["-5V to +5V", "0V to 10V"], 0)),
    new Param(60, "Arp/Seq CV OUT Range", Options(["-5V to +5V", "0V to 10V"], 0)),
    new Param(61, "KB VEL OUT Range", Options(["-5V to +5V", "0V to 10V"], 0)),
    new Param(62, "Arp/Seq VEL OUT Range", Options(["-5V to +5V", "0V to 10V"], 0)),
    new Param(63, "KB AT OUT Range", Options(["-5V to +5V", "0V to 10V"], 0)),
    new Param(64, "MOD WHL OUT Range", Options(["-5V to +5V", "0V to 10V"], 0)),
    new Param(65, "KB GATE OUT Range", Options(["-5V to +5V", "0V to 10V"], 0)),
    new Param(66, "Arp/Seq GATE OUT Range", Options(["-5V to +5V", "0V to 10V"], 0)),
    new Param(67, "Round-Robin Mode",
        Options(["Off", "First-Note Reset", "On"], 1)),
    new Param(68, "Restore Stolen Voices", Options(["Off", "On"], 0)),
    new Param(69, "Update Unison on Note-Off", Options(["Off", "On"], 0)),
    new Param(70, "Mod Oscillator Square Wave Polarity",
        Options(["Unipolar", "Bipolar"], 1)),
    new Param(71, "Noise Filter Cutoff", Slider(0, 16383, 0), "Actual default different to documented"),
    new Param(72, "Arp/Seq Random Repeats",
        Options(["no repeating notes/steps in RND direction",
            "allow repeating notes (true random)"], 1)),
    new Param(73, "ARP/SEQ CV OUT Mirrors KB CV", Options(["Off", "On"], 0)),
    new Param(74, "KB CV OUT Mirrors ARP/SEQ CV", Options(["Off", "On"], 0)),
    */

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
    new Param(0, "Unit ID", Options(Range(0, 15), 0)),
    new Param(1, "Tuning Scale", Options(Range(0, 31), 0, "12-TET")),
    new Param(2, "Knob Mode", Options(["Snap", "Pass-Thru", "Relative"], 0), "Actual default different to documented"),
    new Param(3, "Note Priority", Options(["Low", "High", "Last Note"], 2)),
    new Param(4, "Transmit Program Change", Options(["Off", "On"], 0)),
    new Param(5, "Receive Program Change", Options(["Off", "On"], 1)),
    new Param(6, "MIDI Input Ports",
        Options(["none", "DIN only", "USB only", "Both DIN and USB"], 3)),
    new Param(7, "MIDI Output Ports",
        Options(["none", "DIN only", "USB only", "Both DIN and USB"], 3)),
    new Param(8, "MIDI Echo USB In",
        Options(["Off", "Echo USB In to DIN Out", "Echo USB In to USB Out",
            "Echo USB In to Both DIN and USB Out"], 0)),
    new Param(9, "MIDI Echo DIN In",
        Options(["Off", "Echo DIN In to DIN Out", "Echo DIN In to USB Out",
            "Echo DIN In to Both DIN and USB Out"], 0)),
    new Param(10, "MIDI Channel In", Options(Range(1, 16).map(i => "Channel " + i), 0)),
    new Param(11, "MIDI Channel Out", Options(Range(1, 16).map(i => "Channel " + i), 0)),
    new Param(12, "MIDI Out Filter - Keys", Options(["Off", "On"], 1)),
    new Param(13, "MIDI Out Filter - Wheels", Options(["Off", "On"], 1)),
    new Param(14, "MIDI Out Filter - Panel", Options(["Off", "On"], 1)),
    new Param(15, "Output 14-bit MIDI CCs", Options(["Off", "On"], 0)),
    new Param(16, "Local Control: Keys", Options(["Off", "On"], 1)),
    new Param(17, "Local Control: Wheels", Options(["Off", "On"], 1)),
    new Param(18, "Local Control: Panel", Options(["Off", "On"], 0), "Actual default different to documented"),
    new Param(19, "Local Control: Arp/Seq", Options(["Off", "On"], 1)),
    new Param(20, "Sequence Transpose Mode",
        Options(["Relative to First Note", "Relative to Middle C"], 0)),
    new Param(21, "Arp/Seq Keyed Timing Reset", Options(["Off", "On"], 1), "Actual default different to documented"),
    new Param(22, "Arp FW/BW Repeats",
        Options(["Don"t Repeat end notes", "Repeat end notes"], 1)),
    new Param(23, "Arp/Seq Swing", Slider(0, 16383, 8192)),
    new Param(24, "Sequence Keyboard Control", Options(["Off", "On"], 1)),
    new Param(25, "Delay Sequence Change", Options(["Off", "On"], 0)),
    new Param(26, "Sequence Latch Restart", Options(["Off", "On"], 1), "Actual default different to documented"),
    new Param(27, "Arp/Seq Clock Input Mode",
        Options(["Clock", "Step-Advance Trigger"], 0)),
    new Param(28, "Arp/Seq Clock Output",
        Options(["Always", "Only When Playing"], 1)),
    new Param(29, "Arp MIDI Output", Options(["Off", "On"], 1)),
    new Param(30, "Send MIDI Clock", Options(["Off", "Only When Playing"], 0)),
    new Param(31, "Send MIDI Start/Stop", Options(["Off", "On"], 0)),
    new Param(32, "Follow MIDI Clock", Options(["Off", "On"], 1)),
    new Param(33, "Follow MIDI Start/Stop", Options(["Off", "On"], 1)),
    new Param(34, "Follow Song Position Pointer", Options(["Off", "On"], 1)),
    new Param(35, "Clock Input PPQN Index",
        Options(Range(0, 15), 3, "sixteenth notes [4PPQN]")),
    new Param(36, "Clock Output PPQN Index",
        Options(Range(0, 15), 3, "sixteenth notes [4PPQN]")),
    new Param(37, "Pitch Bend Range (Semitones)",
        Options(Range(0, 12), 2)),
    new Param(38, "Keyboard Octave Transpose",
        Options(["-2", "-1", "0", "1", "2"], 2, "no transpose")),
    new Param(39, "Delayed Keyboard Octave Transpose", Options(["Off", "On"], 1)),
    new Param(40, "Glide Type",
        Options(["Linear Constant Rate", "Linear Constant Time", "Exponential"], 0)),
    new Param(41, "Gated Glide", Options(["Off", "On"], 1)),
    new Param(42, "Legato Glide", Options(["Off", "On"], 0), "Actual default different to documented"),
    new Param(43, "Osc 2 Freq Knob Range",
        Options(Range(0, 24).map(i => i + " Semitones"), 7)),
    new Param(44, "Osc 3 Freq Knob Range",
        Options(Range(0, 24).map(i => i + " Semitones"), 7)),
    new Param(45, "Osc 4 Freq Knob Range",
        Options(Range(0, 24).map(i => i + " Semitones"), 7)),
    new Param(46, "Hard Sync Enable", Options(["Off", "On"], 0)),
    new Param(47, "Osc 2 Hard Sync", Options(["Off", "On"], 0)),
    new Param(48, "Osc 3 Hard Sync", Options(["Off", "On"], 0)),
    new Param(49, "Osc 4 Hard Sync", Options(["Off", "On"], 0)),
    new Param(50, "Delay Ping Pong", Options(["Off", "On"], 0)),
    new Param(51, "Delay Sync", Options(["Off", "On"], 0)),
    new Param(52, "Delay Filter Brightness", Options(["Dark", "Bright"], 1)),
    new Param(53, "Delay CV Sync-Bend", Options(["Off", "On"], 0)),
    new Param(54, "Tap-Tempo Clock Division Persistence", Options(["Off", "On"], 0)),
    new Param(55, "Paraphony Mode", Options(["Mono", "Duo", "Quad"], 2), "Actual default different to documented"),
    new Param(56, "Paraphonic Unison", Options(["Off", "On"], 0)),
    new Param(57, "Multi Trig", Options(["Off", "On"], 0)),
    new Param(58, "Pitch Variance",
        Options(Range(0, 400).map(i => "± " + (i / 10) + " cents"), 0)),
    new Param(59, "KB CV OUT Range", Options(["-5V to +5V", "0V to 10V"], 0)),
    new Param(60, "Arp/Seq CV OUT Range", Options(["-5V to +5V", "0V to 10V"], 0)),
    new Param(61, "KB VEL OUT Range", Options(["-5V to +5V", "0V to 10V"], 0)),
    new Param(62, "Arp/Seq VEL OUT Range", Options(["-5V to +5V", "0V to 10V"], 0)),
    new Param(63, "KB AT OUT Range", Options(["-5V to +5V", "0V to 10V"], 0)),
    new Param(64, "MOD WHL OUT Range", Options(["-5V to +5V", "0V to 10V"], 0)),
    new Param(65, "KB GATE OUT Range", Options(["-5V to +5V", "0V to 10V"], 0)),
    new Param(66, "Arp/Seq GATE OUT Range", Options(["-5V to +5V", "0V to 10V"], 0)),
    new Param(67, "Round-Robin Mode",
        Options(["Off", "First-Note Reset", "On"], 1)),
    new Param(68, "Restore Stolen Voices", Options(["Off", "On"], 0)),
    new Param(69, "Update Unison on Note-Off", Options(["Off", "On"], 0)),
    new Param(70, "Mod Oscillator Square Wave Polarity",
        Options(["Unipolar", "Bipolar"], 1)),
    new Param(71, "Noise Filter Cutoff", Slider(0, 16383, 0), "Actual default different to documented"),
    new Param(72, "Arp/Seq Random Repeats",
        Options(["no repeating notes/steps in RND direction",
            "allow repeating notes (true random)"], 1)),
    new Param(73, "ARP/SEQ CV OUT Mirrors KB CV", Options(["Off", "On"], 0)),
    new Param(74, "KB CV OUT Mirrors ARP/SEQ CV", Options(["Off", "On"], 0)),
    */

    /* 
    let mut matriarch_index:Option<usize> = None;
    let v = get_sources();
    if v.len() > 0 {
        for (idx, i) in v.iter().enumerate() {
            if i == "Moog Matriarch" {
                matriarch_index = Some(idx);
            }
        }
    }
    */
    

    application.connect_activate(move |app| {
        
        let button = gtk::Button::with_label("Connect");
        button.connect_clicked(|_| {
            eprintln!("Connect clicked!");
        });

        let combo = gtk::ComboBoxText::new();
        /*
        if v.len() == 0 {
            combo.append_text("No midi devices connected");
        }
        else {
            for (_idx, i) in v.iter().enumerate() {
                combo.append_text(&i);
            }
        }
        combo.set_active(Some(0));
        */

        let view_list = TreeView::new();
        {
            let types_inside_columns = &[gtk::glib::Type::U32, gtk::glib::Type::STRING, gtk::ListStore::static_type(), gtk::glib::Type::STRING];
            let model_list_of_data = ListStore::new(types_inside_columns);

            for p in &params {
                let model_for_combo = ListStore::new(&[gtk::glib::Type::STRING]);
                for o in &p.get_options() {
                    model_for_combo.insert_with_values(None, &[(0, &o)]);
                }
                model_list_of_data.insert_with_values(None, &[(0, &p.get_id()), (1, &p.get_name()), (2, &model_for_combo), (3, &p.get_options()[p.get_default_index()])]);
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
            object_to_render_cells_3.connect_changed( gtk::glib::clone!( @weak model_list_of_data => move |_cell, list_path, combo_selected_iter| { 
                if let Some(list_iter) = model_list_of_data.iter(&list_path) {
                    if let Ok(combo_model) = model_list_of_data.get_value(&list_iter, 2).get::<ListStore>() {
                        if let Ok(combo_selected_value) = combo_model.get_value(&combo_selected_iter, 0).get::<String>() {
                            model_list_of_data.set_value(&list_iter, 3, &combo_selected_value.to_value() ); 

                            let param_id = list_path.indices()[0];
                            let param_index = combo_model.path(combo_selected_iter).indices()[0];

                            param_changed(param_id, param_index, &combo_selected_value.to_string());
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

        let window = ApplicationWindow::builder()
            .application(app)
            .title("Moog Matriarch Global Settings")
            .default_width(550)
            .default_height(400)
            .content(&vbox)
            .build();
        window.show();

        let mut input = String::new();
    
        let mut midi_in = MidiInput::new("midir reading input").unwrap();
        midi_in.ignore(Ignore::None);
        
        // Get an input port (read from console if multiple are available)
        let in_ports = midi_in.ports();
        if in_ports.len() > 0 {
            let in_port = &in_ports[0];
            /* 
            let in_port = match in_ports.len() {
                0 => panic!("no ports"),
                _ => {
                    println!("Choosing the only available input port: {}", midi_in.port_name(&in_ports[0]).unwrap());
                    &in_ports[0]
                },
                
                _ => {
                    println!("\nAvailable input ports:");
                    for (i, p) in in_ports.iter().enumerate() {
                        println!("{}: {}", i, midi_in.port_name(p).unwrap());
                    }
                    print!("Please select input port: ");
    
                    stdout().flush()?;
                    let mut input = String::new();
                    stdin().read_line(&mut input)?;
                    in_ports.get(input.trim().parse::<usize>()?)
                            .ok_or("invalid input port selected")?
                }
            };
            */
            //println!("\nOpening connection");
            //let in_port_name = midi_in.port_name(in_port).unwrap();
            
            // _conn_in needs to be a named parameter, because it needs to be kept alive until the end of the scope
            let log_all_bytes:Vec<Vec<u8>> = Vec::new();
            let _conn_in = midi_in.connect(in_port, "midir-read-input", move |stamp, message, log| {
                println!("{}: {:?} (len = {})", stamp, message, message.len());
                log.push(message.to_vec());
            }, log_all_bytes).unwrap();
        }
    });
    
    /* 
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
    }*/

    application.run();
    /* 
    if input_port.is_ok() && source.is_some() {
        input_port.unwrap().disconnect_source(&source.unwrap()).unwrap();
    }*/
    
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

fn param_changed(id: i32, param_index: i32, value: &str) {
    println!("changed row index: {}, combo index: {}, value: {:?}", id, param_index, value);
    set_param(id, param_index);
}

fn set_param(param_id: i32, value: i32) {
    let mut msb = 0;
    let mut lsb = value;
    if value > 128 { 
        //msb = parseInt(value / 128); 
        lsb = value % 128; 
    }
    let msg:Vec<i32> = vec![0xf0, 0x04, 0x17, 0x23, param_id, msb, lsb, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x7f, 0xf7];
    //midi_out.send(msg);
}

fn read_param(param_id: i32) {
    let msg:Vec<i32> = vec![0xf0, 0x04, 0x17, 0x3e, param_id, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x7f, 0xf7];
    println!("Sending read request for Parameter {}",  param_id);
    //params_waiting[param_id] = true;
    //midi_out.send(msg);
}

fn update_cell(param_id:i32, value_index:i32) {
    //get value from params array
    //set text column of combocellrenderer
    //let text_value = params[param_id].get_options()[value_index];
    //model_list_of_data.set_value(&list_iter, 3, text_value ); 
}

/* 
function set_param(param_id, value) {
    let msb = 0;
    let lsb = value;
    if(value > 128) { msb = parseInt(value / 128); lsb = value % 128; }
    let msg = [0xf0, 0x04, 0x17, 0x23, param_id, msb, lsb, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x7f, 0xf7]
    midi_out.send(msg);
}

function read_param(param_id) {
    let msg = [0xf0, 0x04, 0x17, 0x3e, param_id, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x7f, 0xf7]
    console.log('Sending read request for Parameter ' + param_id);
    params_waiting[param_id] = true;
    midi_out.send(msg);
}
*/
pub struct ParamListOption {
    id: u8,
    name: String,
    options: Vec<String>,
    default_index: usize, //index in options vector
    default_mesg: String,
    note: String,
}

pub struct ParamRangeOption {
    id: u8,
    name: String,
    min: u32,
    max: u32,
    default_index: usize, //index in min/max range
    default_mesg: String,
    note: String,
    prefix: String,
    suffix: String,
}

pub trait GenericOptions {
    fn get_options(&self) -> Vec<String>;
    fn get_name(&self) -> String;
    fn get_id(&self) -> u8;
    fn get_default_index(&self) -> usize;
}

impl GenericOptions for ParamRangeOption {
    fn get_options(&self) -> Vec<String> {
        let mut v = Vec::new();

        for (idx, i) in (self.min..=self.max).enumerate() {
            //divide range of param 58 by 10 to get cents
            let val: f32 = if self.id == 58 {
                i as f32 / 10_f32
            } else {
                i as f32
            };
            if idx == self.default_index {
                v.push(format!(
                    "{}{}{} (Default{})",
                    self.prefix,
                    val,
                    self.suffix,
                    if !self.default_mesg.is_empty() {
                        format!(" – {}", self.default_mesg)
                    } else {
                        String::from("")
                    }
                ));
            } else {
                v.push(format!("{}{}{}", self.prefix, val, self.suffix));
            }
        }
        v
    }
    fn get_name(&self) -> String {
        self.name.to_owned()
    }
    fn get_id(&self) -> u8 {
        self.id
    }
    fn get_default_index(&self) -> usize {
        self.default_index
    }
}

impl GenericOptions for ParamListOption {
    fn get_options(&self) -> Vec<String> {
        let a = self.options.to_owned();
        a.into_iter()
            .enumerate()
            .map(|(idx, r)| {
                if idx == self.default_index {
                    format!(
                        "{} (Default{})",
                        r,
                        if !self.default_mesg.is_empty() {
                            format!(" – {}", self.default_mesg)
                        } else {
                            String::from("")
                        }
                    )
                } else {
                    r
                }
            })
            .collect::<Vec<String>>()
    }
    fn get_name(&self) -> String {
        self.name.to_owned()
    }
    fn get_id(&self) -> u8 {
        self.id
    }
    fn get_default_index(&self) -> usize {
        self.default_index
    }
}

pub const PARAM_COUNT: i32 = 75;

pub fn get_params() -> Vec<Box<dyn GenericOptions>> {
    vec![
        Box::new(ParamRangeOption {
            id: 0,
            name: "Unit ID".to_string(),
            min: 0,
            max: 15,
            default_index: 0,
            default_mesg: "".to_string(),
            note: "".to_string(),
            prefix: "".to_string(),
            suffix: "".to_string(),
        }),
        Box::new(ParamRangeOption {
            id: 1,
            name: "Tuning Scale".to_string(),
            min: 0,
            max: 31,
            default_index: 0,
            default_mesg: "12-TET".to_string(),
            note: "".to_string(),
            prefix: "".to_string(),
            suffix: "".to_string(),
        }),
        Box::new(ParamListOption {
            id: 2,
            name: "Knob Mode".to_string(),
            options: vec![
                "Snap".to_string(),
                "Pass-Thru".to_string(),
                "Relative".to_string(),
            ],
            default_index: 0,
            default_mesg: "".to_string(),
            note: "Actual default different to documented".to_string(),
        }),
        Box::new(ParamListOption {
            id: 3,
            name: "Note Priority".to_string(),
            options: vec![
                "Low".to_string(),
                "High".to_string(),
                "Last Note".to_string(),
            ],
            default_index: 2,
            default_mesg: "".to_string(),
            note: "".to_string(),
        }),
        Box::new(ParamListOption {
            id: 4,
            name: "Transmit Program Change".to_string(),
            options: vec!["Off".to_string(), "On".to_string()],
            default_index: 0,
            default_mesg: "".to_string(),
            note: "".to_string(),
        }),
        Box::new(ParamListOption {
            id: 5,
            name: "Receive Program Change".to_string(),
            options: vec!["Off".to_string(), "On".to_string()],
            default_index: 1,
            default_mesg: "".to_string(),
            note: "".to_string(),
        }),
        Box::new(ParamListOption {
            id: 6,
            name: "MIDI Input Ports".to_string(),
            options: vec![
                "none".to_string(),
                "DIN only".to_string(),
                "USB only".to_string(),
                "Both DIN and USB".to_string(),
            ],
            default_index: 3,
            default_mesg: "".to_string(),
            note: "".to_string(),
        }),
        Box::new(ParamListOption {
            id: 7,
            name: "MIDI Output Ports".to_string(),
            options: vec![
                "none".to_string(),
                "DIN only".to_string(),
                "USB only".to_string(),
                "Both DIN and USB".to_string(),
            ],
            default_index: 3,
            default_mesg: "".to_string(),
            note: "".to_string(),
        }),
        Box::new(ParamListOption {
            id: 8,
            name: "MIDI Echo USB In".to_string(),
            options: vec![
                "Off".to_string(),
                "Echo USB In to DIN Out".to_string(),
                "Echo USB In to USB Out".to_string(),
                "Echo DIN In to Both DIN and USB Out".to_string(),
            ],
            default_index: 0,
            default_mesg: "".to_string(),
            note: "".to_string(),
        }),
        Box::new(ParamListOption {
            id: 9,
            name: "MIDI Echo DIN In".to_string(),
            options: vec![
                "Off".to_string(),
                "Echo DIN In to DIN Out".to_string(),
                "Echo DIN In to USB Out".to_string(),
                "Echo DIN In to Both DIN and USB Out".to_string(),
            ],
            default_index: 0,
            default_mesg: "".to_string(),
            note: "".to_string(),
        }),
        Box::new(ParamRangeOption {
            id: 10,
            name: "MIDI Channel In".to_string(),
            min: 1,
            max: 16,
            default_index: 0,
            default_mesg: "".to_string(),
            note: "".to_string(),
            prefix: "Channel ".to_string(),
            suffix: "".to_string(),
        }),
        Box::new(ParamRangeOption {
            id: 11,
            name: "MIDI Channel Out".to_string(),
            min: 1,
            max: 16,
            default_index: 0,
            default_mesg: "".to_string(),
            note: "".to_string(),
            prefix: "Channel ".to_string(),
            suffix: "".to_string(),
        }),
        Box::new(ParamListOption {
            id: 12,
            name: "MIDI Out Filter - Keys".to_string(),
            options: vec!["Off".to_string(), "On".to_string()],
            default_index: 1,
            default_mesg: "".to_string(),
            note: "".to_string(),
        }),
        Box::new(ParamListOption {
            id: 13,
            name: "MIDI Out Filter - Wheels".to_string(),
            options: vec!["Off".to_string(), "On".to_string()],
            default_index: 1,
            default_mesg: "".to_string(),
            note: "".to_string(),
        }),
        Box::new(ParamListOption {
            id: 14,
            name: "MIDI Out Filter - Panel".to_string(),
            options: vec!["Off".to_string(), "On".to_string()],
            default_index: 1,
            default_mesg: "".to_string(),
            note: "".to_string(),
        }),
        Box::new(ParamListOption {
            id: 15,
            name: "Output 14-bit MIDI CCs".to_string(),
            options: vec!["Off".to_string(), "On".to_string()],
            default_index: 0,
            default_mesg: "".to_string(),
            note: "".to_string(),
        }),
        Box::new(ParamListOption {
            id: 16,
            name: "Local Control: Keys".to_string(),
            options: vec!["Off".to_string(), "On".to_string()],
            default_index: 1,
            default_mesg: "".to_string(),
            note: "".to_string(),
        }),
        Box::new(ParamListOption {
            id: 17,
            name: "Local Control: Wheels".to_string(),
            options: vec!["Off".to_string(), "On".to_string()],
            default_index: 1,
            default_mesg: "".to_string(),
            note: "".to_string(),
        }),
        Box::new(ParamListOption {
            id: 18,
            name: "Local Control: Panel".to_string(),
            options: vec!["Off".to_string(), "On".to_string()],
            default_index: 0,
            default_mesg: "".to_string(),
            note: "Actual default different to documented".to_string(),
        }),
        Box::new(ParamListOption {
            id: 19,
            name: "Local Control: Arp/Seq".to_string(),
            options: vec!["Off".to_string(), "On".to_string()],
            default_index: 1,
            default_mesg: "".to_string(),
            note: "".to_string(),
        }),
        Box::new(ParamListOption {
            id: 20,
            name: "Sequence Transpose Mode".to_string(),
            options: vec![
                "Relative to First Note".to_string(),
                "Relative to Middle C".to_string(),
            ],
            default_index: 0,
            default_mesg: "".to_string(),
            note: "".to_string(),
        }),
        Box::new(ParamListOption {
            id: 21,
            name: "Arp/Seq Keyed Timing Reset".to_string(),
            options: vec!["Off".to_string(), "On".to_string()],
            default_index: 1,
            default_mesg: "".to_string(),
            note: "Actual default different to documented".to_string(),
        }),
        Box::new(ParamListOption {
            id: 22,
            name: "Arp FW/BW Repeats".to_string(),
            options: vec![
                "Don't Repeat end notes".to_string(),
                "Repeat end notes".to_string(),
            ],
            default_index: 1,
            default_mesg: "".to_string(),
            note: "".to_string(),
        }),
        Box::new(ParamListOption {
            id: 23,
            name: "Arp/Seq Swing".to_string(),
            options: vec!["Slider".to_string(), "Slider".to_string()],
            default_index: 0,
            default_mesg: "".to_string(),
            note: "".to_string(),
        }), //Slider(819, 15565, 8192)
        /* new Param(23, 'Arp/Seq Swing', Slider(819, 15565, 8192)), */
        Box::new(ParamListOption {
            id: 24,
            name: "Sequence Keyboard Control".to_string(),
            options: vec!["Off".to_string(), "On".to_string()],
            default_index: 1,
            default_mesg: "".to_string(),
            note: "".to_string(),
        }),
        Box::new(ParamListOption {
            id: 25,
            name: "Delay Sequence Change".to_string(),
            options: vec!["Off".to_string(), "On".to_string()],
            default_index: 0,
            default_mesg: "".to_string(),
            note: "".to_string(),
        }),
        Box::new(ParamListOption {
            id: 26,
            name: "Sequence Latch Restart".to_string(),
            options: vec!["Off".to_string(), "On".to_string()],
            default_index: 1,
            default_mesg: "".to_string(),
            note: "Actual default different to documented".to_string(),
        }),
        Box::new(ParamListOption {
            id: 27,
            name: "Arp/Seq Clock Input Mode".to_string(),
            options: vec!["Clock".to_string(), "Step-Advance Trigger".to_string()],
            default_index: 0,
            default_mesg: "".to_string(),
            note: "".to_string(),
        }),
        Box::new(ParamListOption {
            id: 28,
            name: "Arp/Seq Clock Output".to_string(),
            options: vec!["Always".to_string(), "Only When Playing".to_string()],
            default_index: 1,
            default_mesg: "".to_string(),
            note: "".to_string(),
        }),
        Box::new(ParamListOption {
            id: 29,
            name: "Arp MIDI Output".to_string(),
            options: vec!["Off".to_string(), "On".to_string()],
            default_index: 1,
            default_mesg: "".to_string(),
            note: "".to_string(),
        }),
        Box::new(ParamListOption {
            id: 30,
            name: "Send MIDI Clock".to_string(),
            options: vec!["Off".to_string(), "On When Playing".to_string()],
            default_index: 0,
            default_mesg: "".to_string(),
            note: "".to_string(),
        }),
        Box::new(ParamListOption {
            id: 31,
            name: "Send MIDI Start/Stop".to_string(),
            options: vec!["Off".to_string(), "On".to_string()],
            default_index: 0,
            default_mesg: "".to_string(),
            note: "".to_string(),
        }),
        Box::new(ParamListOption {
            id: 32,
            name: "Follow MIDI Clock".to_string(),
            options: vec!["Off".to_string(), "On".to_string()],
            default_index: 1,
            default_mesg: "".to_string(),
            note: "".to_string(),
        }),
        Box::new(ParamListOption {
            id: 33,
            name: "Follow MIDI Start/Stop".to_string(),
            options: vec!["Off".to_string(), "On".to_string()],
            default_index: 1,
            default_mesg: "".to_string(),
            note: "".to_string(),
        }),
        Box::new(ParamListOption {
            id: 34,
            name: "Follow Song Position Pointer".to_string(),
            options: vec!["Off".to_string(), "On".to_string()],
            default_index: 1,
            default_mesg: "".to_string(),
            note: "".to_string(),
        }),
        Box::new(ParamListOption {
            id: 35,
            name: "Clock Input PPQN Index".to_string(),
            options: vec![
                "1 PPQN".to_string(),
                "2 PPQN".to_string(),
                "3 PPQN".to_string(),
                "4 PPQN".to_string(),
                "5 PPQN".to_string(),
                "6 PPQN".to_string(),
                "7 PPQN".to_string(),
                "8 PPQN".to_string(),
                "9 PPQN".to_string(),
                "10 PPQN".to_string(),
                "11 PPQN".to_string(),
                "12 PPQN".to_string(),
                "24 PPQN".to_string(),
                "48 PPQN".to_string(),
            ],
            default_index: 3,
            default_mesg: "sixteenth notes".to_string(),
            note: "".to_string(),
        }),
        Box::new(ParamListOption {
            id: 36,
            name: "Clock Output PPQN Index".to_string(),
            options: vec![
                "1 PPQN".to_string(),
                "2 PPQN".to_string(),
                "3 PPQN".to_string(),
                "4 PPQN".to_string(),
                "5 PPQN".to_string(),
                "6 PPQN".to_string(),
                "7 PPQN".to_string(),
                "8 PPQN".to_string(),
                "9 PPQN".to_string(),
                "10 PPQN".to_string(),
                "11 PPQN".to_string(),
                "12 PPQN".to_string(),
                "24 PPQN".to_string(),
                "48 PPQN".to_string(),
            ],
            default_index: 3,
            default_mesg: "sixteenth notes".to_string(),
            note: "".to_string(),
        }),
        Box::new(ParamRangeOption {
            id: 37,
            name: "Pitch Bend Range (Semitones)".to_string(),
            min: 0,
            max: 12,
            default_index: 2,
            default_mesg: "".to_string(),
            note: "".to_string(),
            prefix: "".to_string(),
            suffix: "".to_string(),
        }),
        Box::new(ParamListOption {
            id: 38,
            name: "Keyboard Octave Transpose".to_string(),
            options: vec![
                "-2".to_string(),
                "-1".to_string(),
                "0".to_string(),
                "1".to_string(),
                "2".to_string(),
            ],
            default_index: 2,
            default_mesg: "no transpose".to_string(),
            note: "".to_string(),
        }),
        Box::new(ParamListOption {
            id: 39,
            name: "Delayed Keyboard Octave Transpose".to_string(),
            options: vec!["Off".to_string(), "On".to_string()],
            default_index: 1,
            default_mesg: "".to_string(),
            note: "".to_string(),
        }),
        Box::new(ParamListOption {
            id: 40,
            name: "Glide Type".to_string(),
            options: vec![
                "Linear Constant Rate".to_string(),
                "Linear Constant Time".to_string(),
                "Exponential".to_string(),
            ],
            default_index: 0,
            default_mesg: "".to_string(),
            note: "".to_string(),
        }),
        Box::new(ParamListOption {
            id: 41,
            name: "Gated Glide".to_string(),
            options: vec![
                "Linear Constant Rate".to_string(),
                "Linear Constant Time".to_string(),
                "Exponential".to_string(),
            ],
            default_index: 1,
            default_mesg: "".to_string(),
            note: "".to_string(),
        }),
        Box::new(ParamListOption {
            id: 42,
            name: "Legato Glide".to_string(),
            options: vec![
                "Linear Constant Rate".to_string(),
                "Linear Constant Time".to_string(),
                "Exponential".to_string(),
            ],
            default_index: 0,
            default_mesg: "".to_string(),
            note: "Actual default different to documented".to_string(),
        }),
        Box::new(ParamRangeOption {
            id: 43,
            name: "Osc 2 Freq Knob Range".to_string(),
            min: 0,
            max: 24,
            default_index: 7,
            default_mesg: "".to_string(),
            note: "".to_string(),
            prefix: "".to_string(),
            suffix: " Semitones".to_string(),
        }),
        Box::new(ParamRangeOption {
            id: 44,
            name: "Osc 3 Freq Knob Range".to_string(),
            min: 0,
            max: 24,
            default_index: 7,
            default_mesg: "".to_string(),
            note: "".to_string(),
            prefix: "".to_string(),
            suffix: " Semitones".to_string(),
        }),
        Box::new(ParamRangeOption {
            id: 45,
            name: "Osc 4 Freq Knob Range".to_string(),
            min: 0,
            max: 24,
            default_index: 7,
            default_mesg: "".to_string(),
            note: "".to_string(),
            prefix: "".to_string(),
            suffix: " Semitones".to_string(),
        }),
        Box::new(ParamListOption {
            id: 46,
            name: "Hard Sync Enable".to_string(),
            options: vec!["Off".to_string(), "On".to_string()],
            default_index: 0,
            default_mesg: "".to_string(),
            note: "".to_string(),
        }),
        Box::new(ParamListOption {
            id: 47,
            name: "Osc 2 Hard Sync".to_string(),
            options: vec!["Off".to_string(), "On".to_string()],
            default_index: 0,
            default_mesg: "".to_string(),
            note: "".to_string(),
        }),
        Box::new(ParamListOption {
            id: 48,
            name: "Osc 3 Hard Sync".to_string(),
            options: vec!["Off".to_string(), "On".to_string()],
            default_index: 0,
            default_mesg: "".to_string(),
            note: "".to_string(),
        }),
        Box::new(ParamListOption {
            id: 49,
            name: "Osc 4 Hard Sync".to_string(),
            options: vec!["Off".to_string(), "On".to_string()],
            default_index: 0,
            default_mesg: "".to_string(),
            note: "".to_string(),
        }),
        Box::new(ParamListOption {
            id: 50,
            name: "Delay Ping Pong".to_string(),
            options: vec!["Off".to_string(), "On".to_string()],
            default_index: 0,
            default_mesg: "".to_string(),
            note: "".to_string(),
        }),
        Box::new(ParamListOption {
            id: 51,
            name: "Delay Sync".to_string(),
            options: vec!["Off".to_string(), "On".to_string()],
            default_index: 0,
            default_mesg: "".to_string(),
            note: "".to_string(),
        }),
        Box::new(ParamListOption {
            id: 52,
            name: "Delay Filter Brightness".to_string(),
            options: vec!["Dark".to_string(), "Bright".to_string()],
            default_index: 1,
            default_mesg: "".to_string(),
            note: "".to_string(),
        }),
        Box::new(ParamListOption {
            id: 53,
            name: "Delay CV Sync-Bend".to_string(),
            options: vec!["Off".to_string(), "On".to_string()],
            default_index: 0,
            default_mesg: "".to_string(),
            note: "".to_string(),
        }),
        Box::new(ParamListOption {
            id: 54,
            name: "Tap-Tempo Clock Division Persistence".to_string(),
            options: vec!["Off".to_string(), "On".to_string()],
            default_index: 0,
            default_mesg: "".to_string(),
            note: "".to_string(),
        }),
        Box::new(ParamListOption {
            id: 55,
            name: "Paraphony Mode".to_string(),
            options: vec!["Mono".to_string(), "Duo".to_string(), "Quad".to_string()],
            default_index: 2,
            default_mesg: "".to_string(),
            note: "Actual default different to documented".to_string(),
        }),
        Box::new(ParamListOption {
            id: 56,
            name: "Paraphonic Unison".to_string(),
            options: vec!["Off".to_string(), "On".to_string()],
            default_index: 0,
            default_mesg: "".to_string(),
            note: "".to_string(),
        }),
        Box::new(ParamListOption {
            id: 57,
            name: "Multi Trig".to_string(),
            options: vec!["Off".to_string(), "On".to_string()],
            default_index: 0,
            default_mesg: "".to_string(),
            note: "".to_string(),
        }),
        Box::new(ParamRangeOption {
            id: 58,
            name: "Pitch Variance".to_string(),
            min: 0,   // divide by 10 to get actual value
            max: 400, // divide by 10 to get actual value
            default_index: 0,
            default_mesg: "".to_string(),
            note: "".to_string(),
            prefix: "± ".to_string(),
            suffix: " cents".to_string(),
        }),
        Box::new(ParamListOption {
            id: 59,
            name: "KB CV OUT Range".to_string(),
            options: vec!["-5V to +5V".to_string(), "0V to 10V".to_string()],
            default_index: 0,
            default_mesg: "".to_string(),
            note: "".to_string(),
        }),
        Box::new(ParamListOption {
            id: 60,
            name: "Arp/Seq CV OUT Range".to_string(),
            options: vec!["-5V to +5V".to_string(), "0V to 10V".to_string()],
            default_index: 0,
            default_mesg: "".to_string(),
            note: "".to_string(),
        }),
        Box::new(ParamListOption {
            id: 61,
            name: "KB VEL OUT Range".to_string(),
            options: vec!["-5V to +5V".to_string(), "0V to 10V".to_string()],
            default_index: 0,
            default_mesg: "".to_string(),
            note: "".to_string(),
        }),
        Box::new(ParamListOption {
            id: 62,
            name: "Arp/Seq VEL OUT Range".to_string(),
            options: vec!["-5V to +5V".to_string(), "0V to 10V".to_string()],
            default_index: 0,
            default_mesg: "".to_string(),
            note: "".to_string(),
        }),
        Box::new(ParamListOption {
            id: 63,
            name: "KB AT OUT Range".to_string(),
            options: vec!["-5V to +5V".to_string(), "0V to 10V".to_string()],
            default_index: 0,
            default_mesg: "".to_string(),
            note: "".to_string(),
        }),
        Box::new(ParamListOption {
            id: 64,
            name: "MOD WHL OUT Range".to_string(),
            options: vec!["-5V to +5V".to_string(), "0V to 10V".to_string()],
            default_index: 0,
            default_mesg: "".to_string(),
            note: "".to_string(),
        }),
        Box::new(ParamListOption {
            id: 65,
            name: "KB GATE OUT Range".to_string(),
            options: vec!["-5V to +5V".to_string(), "0V to 10V".to_string()],
            default_index: 0,
            default_mesg: "".to_string(),
            note: "".to_string(),
        }),
        Box::new(ParamListOption {
            id: 66,
            name: "Arp/Seq GATE OUT Range".to_string(),
            options: vec!["-5V to +5V".to_string(), "0V to 10V".to_string()],
            default_index: 0,
            default_mesg: "".to_string(),
            note: "".to_string(),
        }),
        Box::new(ParamListOption {
            id: 67,
            name: "Round-Robin Mode".to_string(),
            options: vec![
                "Off".to_string(),
                "First-Note Reset".to_string(),
                "On".to_string(),
            ],
            default_index: 1,
            default_mesg: "".to_string(),
            note: "".to_string(),
        }),
        Box::new(ParamListOption {
            id: 68,
            name: "Restore Stolen Voices".to_string(),
            options: vec!["Off".to_string(), "On".to_string()],
            default_index: 0,
            default_mesg: "".to_string(),
            note: "".to_string(),
        }),
        Box::new(ParamListOption {
            id: 69,
            name: "Update Unison on Note-Off".to_string(),
            options: vec!["Off".to_string(), "On".to_string()],
            default_index: 0,
            default_mesg: "".to_string(),
            note: "".to_string(),
        }),
        Box::new(ParamListOption {
            id: 70,
            name: "Mod Oscillator Square Wave Polarity".to_string(),
            options: vec!["Unipolar".to_string(), "Bipolar".to_string()],
            default_index: 1,
            default_mesg: "".to_string(),
            note: "".to_string(),
        }),
        Box::new(ParamListOption {
            id: 71,
            name: "Noise Filter Cutoff".to_string(),
            options: vec!["Slider".to_string(), "Slider".to_string()],
            default_index: 0,
            default_mesg: "".to_string(),
            note: "Actual default different to documented".to_string(),
        }), //Slider(0, 16383, 0)
        /* new Param(71, "Noise Filter Cutoff", Slider(0, 16383, 0), "Actual default different to documented"),*/
        Box::new(ParamListOption {
            id: 72,
            name: "Arp/Seq Random Repeats".to_string(),
            options: vec![
                "no repeating notes/steps in RND direction".to_string(),
                "allow repeating notes (true random)".to_string(),
            ],
            default_index: 1,
            default_mesg: "".to_string(),
            note: "".to_string(),
        }),
        Box::new(ParamListOption {
            id: 73,
            name: "ARP/SEQ CV OUT Mirrors KB CV".to_string(),
            options: vec!["Off".to_string(), "On".to_string()],
            default_index: 0,
            default_mesg: "".to_string(),
            note: "".to_string(),
        }),
        Box::new(ParamListOption {
            id: 74,
            name: "KB CV OUT Mirrors ARP/SEQ CV".to_string(),
            options: vec!["Off".to_string(), "On".to_string()],
            default_index: 0,
            default_mesg: "".to_string(),
            note: "".to_string(),
        }),
    ]
}

use std::{
    collections::VecDeque,
    fmt::Debug,
    fs::File,
    io::prelude::*,
    path::{Path, PathBuf},
    rc::Rc,
};

use logos::Logos;

use crate::{
    error::Result,
    sfz::{types::OpcodeMap, Header, Opcode, SfzToken},
};

/// Represents the SFZ instrument parsed
///
/// All units in the sfz format are in real-world values:
/// Frequencies are expressed in Hertz, pitches in cents,
/// amplitudes in percentage and volumes in decibels.
///
/// Notes are expressed in MIDI Note Numbers, or in note names according to the
/// International Pitch Notation (IPN) convention. According to this rules,
/// middle C in the keyboard is C4 and the MIDI note number 60.
///
#[derive(Debug)]
pub struct Instrument {
    pub regions: Vec<OpcodeMap>,
    pub default_path: PathBuf,
    control_codes: OpcodeMap,
}

// constructors:
// - new
// - from_file
// - from_sfz
//
// - add_opcode
// - add_opcode_global
// - add_opcode_to_group
// - add_opcode_to_region
// - groups
// - regions
// - regions_in
// - new_group
// - new_region
// - set_region_group
//
impl Instrument {
    /// Creates an Instrument via loading and parsing some SFZ code in a file
    ///
    pub fn from_file(sfz_path: &Path) -> Result<Self> {
        // open sfz file, and read it into sfz_text
        let mut sfz_file = File::open(&sfz_path)?;
        let mut sfz_text = String::new();
        sfz_file.read_to_string(&mut sfz_text)?;

        Ok(Self::from_sfz(&sfz_text, sfz_path.parent().unwrap()))
    }

    /// Creates an Instrument via parsing some SFZ code in a string
    ///
    /// sfz_path would be the root location from where to find the samples
    /// and default_path opcode value is appended to it.
    ///
    pub fn from_sfz(sfz: &str, sfz_path: &Path) -> Self {
        // Global = 0
        // Master = 1
        // Group = 2
        // Region = 3

        let mut default_path = sfz_path.to_path_buf();

        struct BuilderData {
            regions: Vec<OpcodeMap>,
            is_control: bool,
            control_codes: OpcodeMap,
            levels: VecDeque<Rc<OpcodeMap>>,
            current: Option<OpcodeMap>,
        }

        impl BuilderData {
            pub fn step_to_level(&mut self, lvl: usize) {
                if self.levels.len() == 3 {
                    let top = self.current.take().unwrap();
                    self.regions.push(top);
                }

                while self.levels.len() > lvl {
                    self.levels.pop_front();
                }
                if self.levels.len() == lvl {
                    let top = match self.levels.front() {
                        None => None,
                        Some(top) => Some(top.clone()),
                    };
                    self.current.insert(OpcodeMap::new(top));
                }
                while self.levels.len() < lvl {
                    let top = Rc::new(self.current.take().unwrap());

                    self.levels.push_front(top.clone());
                    self.current.insert(OpcodeMap::new(Some(top)));
                }
            }
        }

        let mut data = BuilderData {
            regions: Vec::new(),

            is_control: false,
            control_codes: OpcodeMap::new(None),

            levels: VecDeque::<Rc<OpcodeMap>>::new(),
            current: Some(OpcodeMap::new(None)),
        };

        let lex = SfzToken::lexer(&sfz);
        for t in lex {
            match &t {
                SfzToken::Header(h) => {
                    match h {
                        Header::Control => {
                            data.is_control = true;
                        }
                        Header::Global => {
                            data.is_control = false;
                            data.step_to_level(0);
                        }
                        Header::Master => {
                            data.is_control = false;
                            data.step_to_level(1);
                        }
                        Header::Group => {
                            data.is_control = false;
                            data.step_to_level(2);
                        }
                        Header::Region => {
                            data.is_control = false;
                            data.step_to_level(3);
                        }
                        // TBD
                        Header::Curve => {
                            // println!("\n<curve>")
                        }
                        // TBD
                        Header::Effect => {
                            // println!("\n<effect>")
                        }
                        _ => (),
                    }
                }

                SfzToken::Opcode(o) => {
                    if data.is_control {
                        if let Opcode::default_path(path) = o {
                            default_path = sfz_path.join(path.clone());
                        }
                        data.control_codes.add_opcode(o.clone());
                    } else {
                        data.current.as_mut().unwrap().add_opcode(o.clone());
                    }
                }
                _ => (),
            }
        }

        data.step_to_level(0);

        Self {
            default_path,
            regions: data.regions,
            control_codes: data.control_codes,
        }
    }
}

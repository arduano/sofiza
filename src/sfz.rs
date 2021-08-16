//! The basic component of an instrument is a region.
//!
//! An instrument then, is defined by one or more regions.
//! Multiple regions can be arranged in a group.
//! Groups allow entering common parameters for multiple regions.

mod headers;
mod instrument;
mod opcodes;

pub mod types;

pub use headers::Header;
pub use instrument::Instrument;
pub use opcodes::Opcode;
pub use types::{OpcodeMap, OpcodeType};

pub(crate) use opcodes::SfzToken;

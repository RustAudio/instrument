//! A crate for sharing various software instrument abstractions.
//!
//! See the [**Instrument**](./struct.Instrument.html) type.

extern crate gaussian;
extern crate panning;
extern crate pitch_calc as pitch;
extern crate rand;
extern crate time_calc as time;
extern crate utils;

pub use freq_warp::FreqWarp;
pub use instrument::Instrument;
pub use mode::Mode;
pub use note_freq::{NoteFreq, NoteFreqGenerator};
pub use voice::{NoteState, Voice};

pub mod freq_warp;
mod instrument;
pub mod mode;
pub mod note_freq;
mod source;
pub mod unit;
mod voice;

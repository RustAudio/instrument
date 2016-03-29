//! A crate for sharing various software instrument abstractions.
//!
//! See the [**Instrument**](./struct.Instrument.html) type.

extern crate gaussian;
extern crate panning;
extern crate pitch_calc as pitch;
extern crate rand;
extern crate time_calc as time;
extern crate utils;

pub use instrument::{Frames, Instrument};
pub use mode::Mode;
pub use note_freq::{NoteFreq, NoteFreqGenerator};
pub use voice::{NoteState, Voice};

mod instrument;
pub mod mode;
pub mod note_freq;
pub mod unit;
mod voice;

use pitch;
use time;

pub type LoopStartPerc = f64;
pub type LoopEndPerc = f64;
pub type AttackMs = time::calc::Ms;
pub type ReleaseMs = time::calc::Ms;

pub type Playhead = u64;
pub type LoopStart = time::calc::Samples;
pub type LoopEnd = time::calc::Samples;
pub type Attack = time::calc::Samples;
pub type Release = time::calc::Samples;
pub type LoopPlayhead = time::calc::Samples;
pub type NoteFreqMulti = f64;
pub type NoteHz = pitch::calc::Hz;
pub type NoteVelocity = f32;

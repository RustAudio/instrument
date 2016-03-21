use pitch::{self, Hz};
use rand;
use std;
use time;
use unit::NoteHz;
use voice::{NoteState, Voice};

/// Types designed to modulate the state of a Node.
pub trait NoteFreqGenerator {
    /// The note frequency generated by the NoteFreqModulator type.
    type NoteFreq: NoteFreq;

    /// Construct a new note_freq from the note_hz given by a note_event and the last voice that
    /// handled a note.
    fn generate(&self,
                note_hz: NoteHz,
                detune: f32,
                voice: Option<&Voice<Self::NoteFreq>>) -> Self::NoteFreq;
}


/// Types to be generated by `NoteFreqGenerator` types.
pub trait NoteFreq: Clone + std::fmt::Debug {
    /// Get the current Hz from the NoteFreq.
    fn hz(&self) -> pitch::calc::Hz;
    /// Calls `NoteFreq::hz` and then steps forward `Self` by one frame.
    fn next_hz(&mut self) -> pitch::calc::Hz;
}


/// A PortamentoNote generator that applies a glissando for the given number of samples.
#[derive(Copy, Clone, Debug)]
pub struct Portamento(pub time::calc::Samples);

/// A note that interpolates between to given frequencies over the given duration.
#[derive(Copy, Clone, Debug)]
pub struct PortamentoFreq {
    current_sample: time::calc::Samples,
    target_samples: time::calc::Samples,
    start_mel: pitch::calc::Mel,
    target_mel: pitch::calc::Mel,
}


/// A wrapper for switching between NoteFreqGenerators at runtime.
#[derive(Copy, Clone, Debug)]
pub enum DynamicGenerator {
    Portamento(Portamento),
    Constant,
}

/// A warpper for switching between different NoteFreqs at runtime.
#[derive(Copy, Clone, Debug)]
pub enum Dynamic {
    Portamento(PortamentoFreq),
    Constant(pitch::calc::Hz),
}


impl DynamicGenerator {
    /// Construct a default portamento.
    pub fn portamento(samples: time::calc::Samples) -> DynamicGenerator {
        DynamicGenerator::Portamento(Portamento(samples))
    }
}


/// Generate a constant `Hz` frequency.
fn generate_constant_freq(note_hz: NoteHz, detune: f32) -> pitch::calc::Hz {
    // If some detune was given, slightly detune the note_hz.
    if detune > 0.0 {
        let step_offset = rand::random::<f32>() * 2.0 * detune - detune;
        pitch::Step(Hz(note_hz).step() + step_offset).hz()
    // Otherwise, our target_hz is the given note_hz.
    } else {
        note_hz
    }
}


/// Generate a portamento frequency.
fn generate_portamento_freq(portamento_samples: time::calc::Samples,
                            note_hz: NoteHz,
                            detune: f32,
                            maybe_last_hz: Option<pitch::calc::Hz>) -> PortamentoFreq {

    // If some detune was given, slightly detune the note_hz.
    let target_hz = generate_constant_freq(note_hz, detune);

    PortamentoFreq {
        current_sample: 0,
        target_samples: portamento_samples,
        start_mel: Hz(maybe_last_hz.unwrap_or(target_hz)).mel(),
        target_mel: Hz(target_hz).mel(),
    }
}


impl NoteFreqGenerator for () {
    type NoteFreq = pitch::calc::Hz;
    fn generate(&self,
                note_hz: NoteHz,
                detune: f32,
                _voice: Option<&Voice<pitch::calc::Hz>>) -> pitch::calc::Hz {
        generate_constant_freq(note_hz, detune)
    }
}

impl NoteFreq for pitch::calc::Hz {
    fn hz(&self) -> pitch::calc::Hz { *self }
    fn next_hz(&mut self) -> pitch::calc::Hz { *self }
}


impl NoteFreqGenerator for Portamento {
    type NoteFreq = PortamentoFreq;
    fn generate(&self,
                note_hz: NoteHz,
                detune: f32,
                maybe_voice: Option<&Voice<PortamentoFreq>>) -> PortamentoFreq {

        let Portamento(duration_samples) = *self;

        // If some note is already playing, take it to use for portamento.
        let maybe_last_hz = match maybe_voice {
            Some(voice) => match voice.note.as_ref() {
                Some(&(NoteState::Playing, _, ref porta_freq, _)) => Some(porta_freq.hz()),
                _ => None,
            },
            None => None,
        };

        generate_portamento_freq(duration_samples, note_hz, detune, maybe_last_hz)
    }
}

impl NoteFreq for PortamentoFreq {
    fn hz(&self) -> pitch::calc::Hz {
        if self.current_sample < self.target_samples {
            let perc = self.current_sample as f64 / self.target_samples as f64;
            let diff_mel = self.target_mel - self.start_mel;
            let perc_diff_mel = perc * diff_mel as f64;
            let mel = self.start_mel + perc_diff_mel as pitch::calc::Mel;
            pitch::Mel(mel).hz()
        } else {
            pitch::Mel(self.target_mel).hz()
        }
    }
    fn next_hz(&mut self) -> pitch::calc::Hz {
        let hz = self.hz();
        if self.current_sample < self.target_samples {
            self.current_sample += 1;
        }
        hz
    }
}


impl NoteFreqGenerator for DynamicGenerator {
    type NoteFreq = Dynamic;
    fn generate(&self,
                note_hz: NoteHz,
                detune: f32,
                maybe_voice: Option<&Voice<Dynamic>>) -> Dynamic {
        match *self {
            DynamicGenerator::Portamento(Portamento(portamento_ms)) => {
                // If some note is already playing, take it to use for portamento.
                let maybe_last_hz = match maybe_voice {
                    Some(voice) => match voice.note.as_ref() {
                        Some(&(NoteState::Playing, _, ref porta_freq, _)) => Some(porta_freq.hz()),
                        _ => None,
                    },
                    None => None,
                };
                let freq = generate_portamento_freq(portamento_ms, note_hz, detune, maybe_last_hz);
                Dynamic::Portamento(freq)
            },
            DynamicGenerator::Constant =>
                Dynamic::Constant(generate_constant_freq(note_hz, detune)),
        }
    }
}

impl NoteFreq for Dynamic {
    fn hz(&self) -> pitch::calc::Hz {
        match *self {
            Dynamic::Portamento(ref porta) => porta.hz(),
            Dynamic::Constant(ref hz) => hz.hz(),
        }
    }
    fn next_hz(&mut self) -> pitch::calc::Hz {
        match *self {
            Dynamic::Portamento(ref mut porta) => porta.next_hz(),
            Dynamic::Constant(ref mut hz)      => hz.next_hz(),
        }
    }
}

use note_freq::NoteFreqGenerator;
use unit::{NoteHz, NoteVelocity};
use voice::{NoteState, Voice};

/// The "mode" with which the `Instrument` will handle notes.
///
/// The `Mode` manages several areas of logic:
///
/// 1. Conversion of input hz to target hz using note_freq_gen and detune.
/// 2. Distribution of new notes between voices.
/// 3. Resetting voice playheads on note-offs or voice-stealing.
pub trait Mode {

    /// Handle a `note_on` event.
    fn note_on<NFG>(&mut self,
                    note_hz: NoteHz,
                    note_velocity: NoteVelocity,
                    detune: f32,
                    note_freq_gen: &NFG,
                    voices: &mut [Voice<NFG::NoteFreq>])
        where NFG: NoteFreqGenerator;

    /// Handle a `note_off` event.
    fn note_off<NFG>(&mut self,
                     note_hz: NoteHz,
                     detune: f32,
                     note_freq_gen: &NFG,
                     voices: &mut [Voice<NFG::NoteFreq>])
        where NFG: NoteFreqGenerator;

    /// Handle a `stop` event.
    fn stop(&mut self) {}

}


/// Monophonic playback.
#[derive(Clone, Debug, PartialEq)]
pub struct Mono(pub MonoKind, pub Vec<NoteHz>);

/// The state of monophony.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum MonoKind {
    /// New notes will reset the voice's playheads
    Retrigger,
    /// If a note is already playing, new notes will not reset the voice's playheads.
    /// A stack of notes is kept - if a NoteOff occurs on the current note, it is replaced with the
    /// note at the top of the stack if there is one. The stacked notes are reset if the voice
    /// becomes inactive.
    Legato,
}

/// Polyphonic playback.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Poly;

/// The mode in which the Synth will handle notes.
#[derive(Clone, Debug, PartialEq)]
pub enum Dynamic {
    /// Single voice (normal or legato) with a stack of fallback notes.
    Mono(Mono),
    /// Multiple voices.
    Poly(Poly),
}


/// Does the given `hz` match the `target_hz`?
fn does_hz_match(hz: NoteHz, target_hz: NoteHz) -> bool {
    const HZ_VARIANCE: NoteHz = 0.25;
    let (min_hz, max_hz) = (target_hz - HZ_VARIANCE, target_hz + HZ_VARIANCE);
    hz > min_hz && hz < max_hz
}

/// Is the given `voice` currently playing a note that matches the `target_hz`?
fn does_voice_match<NF>(voice: &Voice<NF>, target_hz: NoteHz) -> bool {
    match voice.note {
        Some((NoteState::Playing, voice_note_hz, _, _)) =>
            does_hz_match(voice_note_hz, target_hz),
        _ => false,
    }
}


impl Mono {
    /// Construct a default Retrigger mono mode.
    pub fn retrigger() -> Mono {
        Mono(MonoKind::Retrigger, Vec::with_capacity(16))
    }
    /// construct a default Legato mono mode.
    pub fn legato() -> Mono {
        Mono(MonoKind::Legato, Vec::with_capacity(16))
    }
}


impl Dynamic {
    /// Construct a default Retrigger mono mode.
    pub fn retrigger() -> Dynamic {
        Dynamic::Mono(Mono::retrigger())
    }
    /// Construct a default Legato mono mode.
    pub fn legato() -> Dynamic {
        Dynamic::Mono(Mono::legato())
    }
    /// Construct a default Poly mode.
    pub fn poly() -> Dynamic {
        Dynamic::Poly(Poly)
    }
}


impl Mode for Mono {

    /// Handle a note_on event.
    fn note_on<NFG>(&mut self,
                    note_hz: NoteHz,
                    note_vel: NoteVelocity,
                    detune: f32,
                    note_freq_gen: &NFG,
                    voices: &mut [Voice<NFG::NoteFreq>])
        where NFG: NoteFreqGenerator,
    {
        // To ensure that we don't double-stack notes when multiple `note_on`s are given for the
        // same note, we first release the note if it exists.
        self.note_off(note_hz, detune, note_freq_gen, voices);

        let Mono(kind, ref mut notes) = *self;

        // If a note was already playing, move it onto the stack
        if let Some((NoteState::Playing, hz, _, _)) = voices[0].note {
            notes.push(hz);

            // If in Retrigger mode, reset the playheads.
            if let MonoKind::Retrigger = kind {
                for voice in voices.iter_mut() {
                    voice.reset_playhead();
                }
            }
        }
        // Otherwise if there were no notes currently playing, reset the playheads anyway.
        else {
            notes.clear();
            for voice in voices.iter_mut() {
                voice.reset_playhead();
            }
        }

        // Generate a unique NoteFreq and trigger note_on for each voice.
        for voice in voices.iter_mut() {
            let freq = note_freq_gen.generate(note_hz, detune, Some(voice));
            voice.note_on(note_hz, freq, note_vel);
        }
    }

    /// Handle a note_off event.
    fn note_off<NFG>(&mut self,
                     note_hz: NoteHz,
                     detune: f32,
                     note_freq_gen: &NFG,
                     voices: &mut [Voice<NFG::NoteFreq>])
        where NFG: NoteFreqGenerator,
    {
        let Mono(kind, ref mut notes) = *self;

        if does_voice_match(&mut voices[0], note_hz) {
            if let Some((_, _, _, vel)) = voices[0].note {
                // If there's a note still on the stack, fall back to it.
                if let Some(old_hz) = notes.pop() {

                    if let MonoKind::Retrigger = kind {
                        for voice in voices.iter_mut() {
                            voice.reset_playhead();
                        }
                    }

                    // Play the popped stack note on all voices.
                    for voice in voices.iter_mut() {
                        let freq = note_freq_gen.generate(old_hz, detune, Some(voice));
                        voice.note_on(old_hz, freq, vel);
                    }
                    return;
                }
            }
            for voice in voices.iter_mut() {
                voice.note_off();
            }
        } else {
            // If any notes in the note stack match the given note_off, remove them.
            for i in (0..notes.len()).rev() {
                if does_hz_match(notes[i], note_hz) {
                    notes.remove(i);
                }
            }
        }

    }

    /// Handle a stop event.
    fn stop(&mut self) {
        let Mono(_, ref mut notes) = *self;
        notes.clear();
    }

}


impl Mode for Poly {

    fn note_on<NFG>(&mut self,
                    note_hz: NoteHz,
                    note_vel: NoteVelocity,
                    detune: f32,
                    note_freq_gen: &NFG,
                    voices: &mut [Voice<NFG::NoteFreq>])
        where NFG: NoteFreqGenerator,
    {

        // Construct the new CurrentFreq for the new note.
        let freq = {
            // First, determine the current hz of the last note played if there is one.
            let mut active = voices.iter().filter(|voice| voice.note.is_some());
            
            // Find the most recent voice.
            let maybe_newest_voice = active.next().map(|voice| {
                active.fold(voice, |newest, voice| {
                    if voice.playhead < newest.playhead { voice }
                    else { newest }
                })
            });

            note_freq_gen.generate(note_hz, detune, maybe_newest_voice)
        };

        // Find the right voice to play the note.
        let mut oldest = None;
        let mut max_sample_count: u64 = 0;
        for voice in voices.iter_mut() {
            if voice.note.is_none() {
                voice.reset_playhead();
                voice.note_on(note_hz, freq, note_vel);
                return;
            }
            else if voice.playhead >= max_sample_count {
                max_sample_count = voice.playhead;
                oldest = Some(voice);
            }
        }
        if let Some(voice) = oldest {
            voice.reset_playhead();
            voice.note_on(note_hz, freq, note_vel);
        }

    }

    fn note_off<NFG>(&mut self,
                     note_hz: NoteHz,
                     _detune: f32,
                     _note_freq_gen: &NFG,
                     voices: &mut [Voice<NFG::NoteFreq>])
        where NFG: NoteFreqGenerator,
    {

        let maybe_voice = voices.iter_mut().fold(None, |maybe_current_match, voice| {
            if does_voice_match(voice, note_hz) {
                match maybe_current_match {
                    None => return Some(voice),
                    Some(ref current_match) => if voice.playhead >= current_match.playhead {
                        return Some(voice)
                    },
                }
            }
            maybe_current_match
        });

        if let Some(voice) = maybe_voice {
            voice.note_off();
        }
    }

}


impl Mode for Dynamic {

    /// Handle a note_on event.
    fn note_on<NFG>(&mut self,
                    note_hz: NoteHz,
                    note_vel: NoteVelocity,
                    detune: f32,
                    note_freq_gen: &NFG,
                    voices: &mut [Voice<NFG::NoteFreq>]) where NFG: NoteFreqGenerator {
        match *self {
            Dynamic::Mono(ref mut mono) =>
                mono.note_on(note_hz, note_vel, detune, note_freq_gen, voices),
            Dynamic::Poly(ref mut poly) =>
                poly.note_on(note_hz, note_vel, detune, note_freq_gen, voices),
        }
    }

    fn note_off<NFG>(&mut self,
                     note_hz: NoteHz,
                     detune: f32,
                     note_freq_gen: &NFG,
                     voices: &mut [Voice<NFG::NoteFreq>]) where NFG: NoteFreqGenerator {
        match *self {
            Dynamic::Mono(ref mut mono) =>
                mono.note_off(note_hz, detune, note_freq_gen, voices),
            Dynamic::Poly(ref mut poly) =>
                poly.note_off(note_hz, detune, note_freq_gen, voices),
        }
    }

    fn stop(&mut self) {
        match *self {
            Dynamic::Mono(ref mut mono) => mono.stop(),
            Dynamic::Poly(ref mut poly) => poly.stop(),
        }
    }

}


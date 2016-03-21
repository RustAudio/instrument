use mode::Mode;
use note_freq::{NoteFreq, NoteFreqGenerator};
use pitch;
use std;
use time;
use voice::Voice;
use unit::{NoteHz, NoteVelocity};


/// A performable `Instrument` type that converts `Note` events into a sequence of voices, each
/// with their own unique amplitude and frequency per frame. This is useful for driving the
/// playback of instrument types like synthesisers or samplers.
///
/// `Instrument` handles the following logic:
///
/// - Playback mode: Legato, Retrigger or Polyphonic.
/// - Note on detuning.
/// - Note on "interoplation" / frequency generation: Legato or Constant.
/// - Sustained note warping: 
/// - Multi-channel audio processing.
#[derive(Clone, Debug)]
pub struct Instrument<M, NFG>
    where NFG: NoteFreqGenerator,
{
    /// The mode of note playback.
    pub mode: M,
    /// The stack of `Voice`s used by the Instrument.
    /// - If the Instrument is in Mono mode, it will play one voice at a time.
    /// - If the Instrument is in Poly mode, it will play all voices at once.
    pub voices: Vec<Voice<NFG::NoteFreq>>,
    /// The amount each voice's note_on should be detuned.
    pub detune: f32,
    /// Note on "interoplation" / frequency generation: Legato or Constant.
    pub note_freq_gen: NFG,
    /// A duration in frames over which the amplitude of each note will fade in after `note_on`.
    pub attack_ms: time::Ms,
    /// A duration in frames over which the amplitude of each note will fade out after `note_off`.
    pub release_ms: time::Ms,
    /// Is the playback currently paused?
    pub is_paused: bool,
}

/// Yields the amplitude and frequency of each voice for a single frame.
pub struct FramePerVoice<'a, NF: 'a> {
    attack: u64,
    release: u64,
    voices: std::slice::IterMut<'a, Voice<NF>>,
}


impl<M, NFG> Instrument<M, NFG>
    where NFG: NoteFreqGenerator,
{

    /// Construct a new `Instrument` of the given mode using the given note frequency generator.
    pub fn new(mode: M, note_freq_gen: NFG) -> Self {
        Instrument {
            mode: mode,
            voices: vec![Voice::new()],
            detune: 0.0,
            note_freq_gen: note_freq_gen,
            attack_ms: time::Ms(0.0),
            release_ms: time::Ms(0.0),
            is_paused: false,
        }
    }

    /// Build the Instrument with the given number of voices.
    pub fn num_voices(mut self, num_voices: usize) -> Self {
        self.set_num_voices(num_voices);
        self
    }

    /// Set the note fades for the `Instrument` in frames.
    pub fn fade(mut self, attack: time::Ms, release: time::Ms) -> Self {
        self.attack_ms = attack;
        self.release_ms = release;
        self
    }

    /// Set the attack.
    pub fn attack(mut self, attack: time::Ms) -> Self {
        self.attack_ms = attack;
        self
    }

    /// Set the release.
    pub fn release(mut self, release: time::Ms) -> Self {
        self.release_ms = release;
        self
    }

    /// Set the Instrument's note_on detune amount.
    pub fn detune(mut self, detune: f32) -> Self {
        self.detune = detune;
        self
    }

    /// Convert `Self` into a new `Instrument` with the given NoteFreqGenerator.
    ///
    /// Generates new `NoteFreq`s for each of the currently active `Voice`s.
    pub fn note_freq_generator(mut self, generator: NFG) -> Self {
        self.note_freq_gen = generator;
        self
    }

    /// Set the number of voices that the Instrument shall use.
    pub fn set_num_voices(&mut self, num_voices: usize) {
        if num_voices == 0 {
            println!("A Synth must have at least one voice, but the requested number is 0.");
        } else {
            let len = self.voices.len();
            if len < num_voices {
                let last_voice = self.voices[len-1].clone();
                self.voices.extend(std::iter::repeat(last_voice).take(num_voices - len));
            } else if len > num_voices {
                self.voices.truncate(num_voices);
            }
        }
    }

    /// Return whether or not there are any currently active voices.
    pub fn is_active(&self) -> bool {
        if self.is_paused { return false }
        self.voices.iter().any(|voice| voice.note.is_some())
    }

    /// Begin playback of a note. Instrument will try to use a free `Voice` to do this.
    ///
    /// If no `Voice`s are free, the one playing the oldest note will be chosen to play the new
    /// note instead.
    #[inline]
    pub fn note_on<T>(&mut self, note_hz: T, note_vel: NoteVelocity)
        where M: Mode,
              T: Into<pitch::Hz>
    {
        let Instrument { detune, ref note_freq_gen, ref mut mode, ref mut voices, .. } = *self;
        mode.note_on(note_hz.into().hz(), note_vel, detune, note_freq_gen, voices);
    }

    /// Stop playback of the note that was triggered with the matching frequency.
    #[inline]
    pub fn note_off<T>(&mut self, note_hz: T)
        where M: Mode,
              T: Into<pitch::Hz>
    {
        let Instrument { detune, ref note_freq_gen,  ref mut mode, ref mut voices, .. } = *self;
        mode.note_off(note_hz.into().hz(), detune, note_freq_gen, voices);
    }

    /// Pause playback.
    #[inline]
    pub fn pause(&mut self) {
        self.is_paused = true;
    }

    /// Unpause playback.
    #[inline]
    pub fn unpause(&mut self) {
        self.is_paused = false;
    }

    /// Stop playback and clear the current notes.
    #[inline]
    pub fn stop(&mut self)
        where M: Mode,
    {
        self.mode.stop();
        for voice in self.voices.iter_mut() {
            voice.stop();
        }
    }

    /// Produces an Iterator that yields the amplitude and frequency of each voice for the next
    /// frame.
    #[inline]
    pub fn frams_per_voice(&mut self, sample_hz: time::SampleHz) -> FramePerVoice<NFG::NoteFreq> {
        FramePerVoice {
            attack: self.attack_ms.samples(sample_hz) as u64,
            release: self.release_ms.samples(sample_hz) as u64,
            voices: self.voices.iter_mut(),
        }
    }

}


///// Iterators


impl<'a, NF> FramePerVoice<'a, NF>
    where NF: NoteFreq,
{
    /// The velocity and frequency in hertz of the next `Voice` at the current frame.
    ///
    /// Returns `Some(None)` if the voice exists but is not currently playing a note.
    ///
    /// Returns `None` if there are no more voices for the current frame.
    #[inline]
    pub fn next_voice_vel_hz(&mut self) -> Option<Option<(NoteVelocity, NoteHz)>> {
        let FramePerVoice { ref mut voices, attack, release } = *self;
        voices.next().map(|voice| voice.next_vel_hz(attack, release).map(|(vel, hz)| {
            (vel, hz)
        }))
    }
}

impl<'a, NF> Iterator for FramePerVoice<'a, NF>
    where NF: NoteFreq,
{
    type Item = Option<(NoteVelocity, NoteHz)>;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.next_voice_vel_hz()
    }
}

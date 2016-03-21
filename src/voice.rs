use note_freq::NoteFreq;
use unit::{NoteHz, NoteVelocity, Playhead};

/// A single Voice. A Synth may consist of any number of Voices.
#[derive(Clone, Debug)]
pub struct Voice<NF> {
    /// Data for a note, if there is one currently being played.
    ///
    /// `Playhead` represents the number of frames played since `note` became `Some`.
    pub note: Option<(NoteState, NoteHz, NF, NoteVelocity)>,
    /// Number of frames played since the beginning of the note.
    pub playhead: Playhead,
}

/// The current state of the Voice's note playback.
#[derive(Copy, Clone, Debug)]
pub enum NoteState {
    /// The note is current playing.
    Playing,
    /// The note has been released and is fading out.
    Released(Playhead),
}


impl<NF> Voice<NF> {

    /// Constructor for a Voice.
    pub fn new() -> Self {
        Voice {
            note: None,
            playhead: 0,
        }
    }

    /// Reset the voice's playheads.
    #[inline]
    pub fn reset_playhead(&mut self) {
        self.playhead = 0;
    }

    /// Trigger playback with the given note, resetting all playheads.
    #[inline]
    pub fn note_on(&mut self, hz: NoteHz, freq: NF, vel: NoteVelocity) {
        self.note = Some((NoteState::Playing, hz, freq, vel));
    }

    /// Release playback of the current not eif there is one.
    #[inline]
    pub fn note_off(&mut self) {
        if let Some(&mut(ref mut state, _, _, _)) = self.note.as_mut() {
            *state = NoteState::Released(0);
        }
    }

    /// Stop playback of the current note if there is one and reset the playheads.
    #[inline]
    pub fn stop(&mut self) {
        self.note = None;
        self.playhead = 0;
    }

    /// The velocity and frequency of the voice for the next frame.
    #[inline]
    pub fn next_vel_hz(&mut self, attack: u64, release: u64) -> Option<(NoteVelocity, NoteHz)>
        where NF: NoteFreq,
    {
        // Calculates the current attack amplitude, steps forward the playhead and returns the amp.
        fn next_attack_amp(playhead: &mut u64, attack: u64) -> f32 {
            if *playhead < attack {
                let amp = *playhead as f32 / attack as f32;
                *playhead += 1;
                amp
            } else {
                1.0
            }
        }

        let Voice { ref mut note, ref mut playhead } = *self;
        match note.as_mut() {
            Some(&mut(ref mut state, _, ref mut freq, vel)) => match *state {
                NoteState::Playing => {
                    let attack_amp = next_attack_amp(playhead, attack);
                    let vel = vel * attack_amp;
                    return Some((vel, freq.next_hz()));
                },
                NoteState::Released(ref mut release_playhead) if *release_playhead < release => {
                    let attack_amp = next_attack_amp(playhead, attack);
                    let release_amp = (release - *release_playhead) as f32 / release as f32;
                    *release_playhead += 1;
                    let vel = vel * attack_amp * release_amp;
                    return Some((vel, freq.next_hz()));
                },
                _ => (),
            },
            None => return None,
        }

        // The `NoteState::Released` playhead is out of range, thus the note is finished.
        *note = None;
        None
    }

}


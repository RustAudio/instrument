
/// Some source of samples for the instrument.
pub trait Source {
    /// Source state that is to be associatd with each unique playing voice.
    type Voice;
}

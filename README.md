# instrument [![Build Status](https://travis-ci.org/RustAudio/instrument.svg?branch=master)](https://travis-ci.org/RustAudio/instrument) [![Crates.io](https://img.shields.io/crates/v/instrument.svg)](https://crates.io/crates/instrument) [![Crates.io](https://img.shields.io/crates/l/instrument.svg)](https://github.com/RustAudio/instrument/blob/master/LICENSE)

A foundational type for performable musical instruments.

The `Instrument` type takes discrete `note_on` and `note_off` events as inputs
and returns a `Frames` iterator yielding a amplitude/frequency value pair per
`Voice` per `Frame` as an output.

`Instrument` supports multiple note handling `Mode`s including **n** voice
polyphony and **n** voice unison monophony (both retrigger and legato).
Note-on effects such as detuning and legato are also supported.

The `instrument` crate is used by:
- [synth](https://github.com/RustAudio/synth)
- [sampler](https://github.com/RustAudio/sampler)

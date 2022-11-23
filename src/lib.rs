#![no_std]

/// re-exports
pub use midi_types;
pub use midi_convert;

pub mod constants;

pub use {
    midi_device::{MidiClass, MidiReadError, MidiClassInvalidArgs},
    event_packet::{UsbMidiEventPacket, MidiPacketParsingError},
    cable_number::{CableNumber, InvalidCableNumber},
    packet_reader::MidiPacketBufferReader,
};

mod code_index_number;
mod packet_reader;
mod cable_number;
mod midi_device;
mod event_packet;

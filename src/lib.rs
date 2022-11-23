#![no_std]

/// re-export midi_types
pub use midi_convert::midi_types;

pub mod constants;

pub use {
    cable_number::{CableNumber, InvalidCableNumber},
    event_packet::{MidiPacketParsingError, UsbMidiEventPacket},
    midi_device::{MidiClass, MidiClassInvalidArgs, MidiReadError},
    packet_reader::MidiPacketBufferReader,
};

mod cable_number;
mod code_index_number;
mod event_packet;
mod midi_device;
mod packet_reader;

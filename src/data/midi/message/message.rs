
use crate::data::midi::channel::Channel;
use crate::data::midi::notes::Note;
use crate::data::byte::u7::U7;
use crate::data::midi::message::raw::{Raw,Payload};
use crate::data::byte::from_traits::FromClamped;
use core::convert::TryFrom;
use crate::data::usb_midi::usb_midi_event_packet::MidiPacketParsingError;

type Velocity = U7;

/// Represents midi messages
/// Note: not current exhaustive and SysEx messages end up
/// being a confusing case. So are currently note implemented 
/// they are sort-of unbounded
#[derive(Debug)]
pub enum Message {
    NoteOff(Channel,Note,Velocity),
    NoteOn(Channel,Note,Velocity),
    PolyphonicAftertouch(Channel,Note,U7),
    ProgramChange(Channel,U7),
    ChannelAftertouch(Channel,U7),
    PitchWheelChange(Channel,U7,U7)
}

const NOTE_OFF_MASK             :u8 = 0b1000_0000;
const NOTE_ON_MASK              :u8 = 0b1001_0000;
const POLYPHONIC_MASK           :u8 = 0b1010_0000;
const PROGRAM_MASK              :u8 = 0b1100_0000;
const CHANNEL_AFTERTOUCH_MASK   :u8 = 0b1101_0000;
const PITCH_BEND_MASK           :u8 = 0b1110_0000;

impl From<Message> for Raw {
    fn from(value:Message) -> Raw {
        match value {
            Message::NoteOn(chan,note,vel)  =>  {
                let payload = Payload::DoubleByte(note.into(),vel);
                let status =  NOTE_ON_MASK | u8::from(chan);
                Raw { status, payload  }
            },
            Message::NoteOff(chan,note,vel) => {
                let payload = Payload::DoubleByte(note.into(),vel);
                let status = NOTE_OFF_MASK | u8::from(chan);
                Raw {status, payload}
            },
            Message::PolyphonicAftertouch(chan,note,pressure) => {
                let payload = Payload::DoubleByte(note.into(),pressure);
                let status = POLYPHONIC_MASK | u8::from(chan);
                Raw {status, payload}
            },
            Message::ProgramChange(chan,program) => {
                let payload = Payload::SingleByte(program);
                let status = PROGRAM_MASK | u8::from(chan);
                Raw {status, payload}
            },
            Message::ChannelAftertouch(chan,pressure) => {
                let payload = Payload::SingleByte(pressure);
                let status = CHANNEL_AFTERTOUCH_MASK | u8::from(chan);
                Raw {status, payload}
            },
            Message::PitchWheelChange(chan,lsb,msb) => {
                let payload = Payload::DoubleByte(lsb,msb);
                let status = PITCH_BEND_MASK | u8::from(chan);
                Raw {status , payload}
            }
                
        }
    }
}

impl<'a> TryFrom<&'a [u8]> for Message {
    type Error = MidiPacketParsingError;
    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        let status_byte = data[0];
        let event_type = status_byte & 0b1111_0000;
        let channel_bytes = (status_byte) & 0b0000_1111;

        let channel = Channel::try_from(channel_bytes).ok().unwrap();

        match event_type {
            NOTE_OFF_MASK => Ok(Message::NoteOff(channel, get_note(data)?, get_velocity(data))),
            NOTE_ON_MASK => Ok(Message::NoteOn(channel, get_note(data)?, get_velocity(data))),
            _ => Err(MidiPacketParsingError::InvalidEventType(event_type))
        }
    }
}

fn get_note(data: &[u8]) -> Result<Note, MidiPacketParsingError> {
    let note_byte = data[1];
    match Note::try_from(note_byte) {
        Ok(note) => Ok(note),
        Err(_) => Err(MidiPacketParsingError::InvalidNote(note_byte))
    }
}

fn get_velocity(data: &[u8]) -> U7 {
    U7::from_clamped(data[2])
}
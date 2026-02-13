use super::model::{Midi, MidiTrack, TrackEvent, MidiEvent, SystemMessage, ChannelMessage, MetaMessage};

pub struct MidiWriter {
    data: Vec<u8>
}

impl MidiWriter
{
    pub fn new() -> Self
    {
        Self { data: Vec::new() }
    }

    pub fn into_bytes(self) -> Vec<u8>
    {
        self.data
    }

    fn write_u8(&mut self, value: u8)
    {
        self.data.push(value);
    }

    fn write_u16(&mut self, value: u16)
    {
        self.data.extend_from_slice(&value.to_be_bytes());
    }

    fn write_u32(&mut self, value: u32)
    {
        self.data.extend_from_slice(&value.to_be_bytes());
    }

    fn write_bytes(&mut self, bytes: &[u8])
    {
        self.data.extend_from_slice(bytes);
    }

    fn write_vlq(&mut self, mut value: u32)
    {
        let mut buffer = [0u8; 4];
        let mut index = 4;

        buffer[3] = (value & 0x7F) as u8;
        index -= 1;
        value >>= 7;

        while value > 0 {
            buffer[index - 1] = ((value & 0x7F) as u8) | 0x80;
            value >>= 7;
            index -= 1;
        }

        self.write_bytes(&buffer[index..]);
    }

    pub fn write_midi(&mut self, midi: &Midi)
    {
        // Write MIDI header
        self.write_bytes(b"MThd");
        self.write_u32(6); // Header length
        self.write_u16(midi.format);
        self.write_u16(midi.tracks.len() as u16);
        self.write_u16(midi.time_division);

        // Write each track
        for track in &midi.tracks {
            self.write_track(track);
        }
    }

    fn write_track(&mut self, track: &MidiTrack)
    {
        self.write_bytes(b"MTrk");

        let length_pos = self.data.len();
        self.write_u32(0); // Placeholder for track length

        for event in &track.events {
            self.write_event(event);
        }

        let end_pos = self.data.len();
        let track_length = (end_pos - length_pos) as u32;

        self.data[length_pos..length_pos+4].copy_from_slice(&track_length.to_be_bytes());
    }

    fn write_event(&mut self, event: &TrackEvent)
    {
        self.write_vlq(event.delta_time);

        match &event.event
        {
            MidiEvent::Channel { channel, message } => 
            {
                self.write_channel_event(*channel, message);
            },
            MidiEvent::System(SystemMessage::Meta(meta)) => 
            {
                self.write_meta_event(meta);
            },
            MidiEvent::System(SystemMessage::SysEx(data)) =>
            {
                self.write_u8(0xF0);
                self.write_vlq(data.len() as u32);
                self.write_bytes(data);
            }
        }
    }

    fn write_channel_event(&mut self, channel: u8, message: &ChannelMessage)
    {
        match message
        {
            ChannelMessage::NoteOff { note, velocity } =>
            {
                self.write_u8(0x80 | (channel));
                self.write_u8(*note);
                self.write_u8(*velocity);
            }

            ChannelMessage::NoteOn { note, velocity } =>
            {
                self.write_u8(0x90 | (channel));
                self.write_u8(*note);
                self.write_u8(*velocity);
            },

            ChannelMessage::PolyPressure { note, pressure } =>
            {
                self.write_u8(0xA0 | (channel));
                self.write_u8(*note);
                self.write_u8(*pressure);
            },

            ChannelMessage::ControlChange { controller, value } =>
            {
                self.write_u8(0xB0 | (channel));
                self.write_u8(*controller);
                self.write_u8(*value);
            },

            ChannelMessage::ProgramChange { program } =>
            {
                self.write_u8(0xC0 | (channel));
                self.write_u8(*program);
            },

            ChannelMessage::ChannelPressure { pressure } =>
            {
                self.write_u8(0xD0 | (channel));
                self.write_u8(*pressure);
            },

            ChannelMessage::PitchBend { value } =>
            {
                let raw = value.to_raw();
                let lsb = (raw & 0x7F) as u8;
                let msb = ((raw >> 7) & 0x7F) as u8;

                self.write_u8(0xE0 | (channel));
                self.write_u8(lsb);
                self.write_u8(msb);
            }
        }
    }

    fn write_meta_event(&mut self, meta: &MetaMessage)
    {
        self.write_u8(0xFF);

        match meta
        {
            MetaMessage::Tempo(tempo) =>
            {
                self.write_u8(0x51);
                self.write_u8(3);
                self.write_u8((tempo >> 16) as u8);
                self.write_u8((tempo >> 8) as u8);
                self.write_u8(*tempo as u8);
            },

            MetaMessage::TrackName(name) =>
            {
                let bytes = name.as_bytes();
                self.write_u8(0x03);
                self.write_vlq(bytes.len() as u32);
                self.write_bytes(bytes);
            },

            MetaMessage::Raw { meta_type, data } =>
            {
                self.write_u8(*meta_type);
                self.write_vlq(data.len() as u32);
                self.write_bytes(data);
            },

            MetaMessage::TimeSignature { numerator, denominator, clocks_per_click, notes_per_quarter } =>
            {
                self.write_u8(0x58);
                self.write_u8(4);
                self.write_u8(*numerator);

                let dd = match *denominator
                {
                    1 => 0,
                    2 => 1,
                    4 => 2,
                    8 => 3,
                    16 => 4,
                    32 => 5,
                    64 => 6,
                    128 => 7,

                    _ => panic!("Unsupported denominator: {}", denominator),
                };

                self.write_u8(dd);
                self.write_u8(*clocks_per_click);
                self.write_u8(*notes_per_quarter);
            }
        }
    }
}
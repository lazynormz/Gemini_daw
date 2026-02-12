use super::model::*;
use super::error::MidiParseError;
use super::reader::MidiParser;

pub fn parse_midi_track(mut reader: &mut MidiParser, track_end: usize) -> Result<MidiTrack, MidiParseError> {

    let mut events: Vec<TrackEvent> = Vec::new();
    let mut running_status: Option<u8> = None;

    while reader.position < track_end
    {
        let delta_time = reader.read_vlq()?;

        let next_byte = reader.peek_u8_be()?;

        let status = if next_byte & 0x80 != 0
        {
            let status_byte = reader.read_u8_be()?;

            if status_byte < 0xF0
            {
                running_status = Some(status_byte);
            }
            else 
            {
                running_status = None;
            };

            status_byte
        }
        else {
            match running_status {
                Some(prev_status) => prev_status,
                None => return Err(MidiParseError::RunningStatusWithoutPrevious),
            }
        };

        let event = parse_midi_event(&mut reader, status)?;
        events.push(TrackEvent { delta_time, event });
    }

    Ok(MidiTrack {
        events: events,
    })
}

pub fn parse_meta_event(reader: &mut MidiParser) -> Result<MidiEvent, MidiParseError> {
    let meta_type = reader.read_u8_be()?;
    let length = reader.read_vlq()? as usize;

    let mut data = Vec::with_capacity(length);

    for _ in 0..length {
        data.push(reader.read_u8_be()?);
    }

    let meta = match meta_type
    {
        0x2F => MetaMessage::Raw { meta_type, data },
        0x51 => {
            if data.len() != 3
            {
                return Err(MidiParseError::InvalidMetaLength(meta_type))
            }

            let tempo = 
                ((data[0] as u32) << 16) | 
                ((data[1] as u32) << 8) |
                (data[2] as u32);

            MetaMessage::Tempo(tempo)
        },
        0x03 => 
        {
            let name = String::from_utf8_lossy(&data).to_string();
            MetaMessage::TrackName(name)
        }
        _ => MetaMessage::Raw { meta_type, data },
    };

    Ok(MidiEvent::System(SystemMessage::Meta(meta)))
}

pub fn parse_sysex_event(reader: &mut MidiParser) -> Result<MidiEvent, MidiParseError>
{
    let length = reader.read_vlq()? as usize;

    let mut data = Vec::with_capacity(length);

    for _ in 0..length
    {
        data.push(reader.read_u8_be()?);
    }

    Ok(MidiEvent::System(SystemMessage::SysEx(data)))
}

pub fn parse_midi_event(reader: &mut MidiParser, status: u8) -> Result<MidiEvent, MidiParseError>
{ 
    if status >=0xF0
    {
        match status
        {
            0xFF => return parse_meta_event(reader),
            0xF0 | 0xF7 => return parse_sysex_event(reader),
            _ => return Err(MidiParseError::InvalidFormat(status as u16)),
        }
    }

    let event_type = status & 0xF0;
    let channel = status & 0x0F;

    let channel_message = match event_type {
        0x80 => {
            let note = reader.read_u8_be()?;
            let velocity = reader.read_u8_be()?;
            ChannelMessage::NoteOff { note, velocity }
        },
        0x90 => {
            let note = reader.read_u8_be()?;
            let velocity = reader.read_u8_be()?;
            ChannelMessage::NoteOn { note, velocity }
        },
        0xA0 => {
            let note = reader.read_u8_be()?;
            let pressure = reader.read_u8_be()?;
            ChannelMessage::PolyPressure { note, pressure }
        },
        0xB0 => {
            let controller = reader.read_u8_be()?;
            let value = reader.read_u8_be()?;
            ChannelMessage::ControlChange { controller, value }
        },
        0xC0 => {
            let program = reader.read_u8_be()?;
            ChannelMessage::ProgramChange { program }
        },
        0xD0 => {
            let pressure = reader.read_u8_be()?;
            ChannelMessage::ChannelPressure { pressure }
        },
        0xE0 => {
            let low_byte = reader.read_u8_be()?;
            let high_byte = reader.read_u8_be()?;
            let value = ((high_byte as i16) << 7) | (low_byte as i16);
            ChannelMessage::PitchBend { value: PitchBend::from_raw(value)}
        },
        _ => return Err(MidiParseError::InvalidFormat(event_type as u16)), 
    };

    Ok(MidiEvent::Channel {
        channel: channel,
        message: channel_message,
    })
}

pub fn parse_midi_file(data: &[u8]) -> Result<Midi, MidiParseError> {
    let mut reader = MidiParser::new(data);

    // Header parsing
    reader.expect_bytes(b"MThd")?;
    let header_length = reader.read_u32_be()?;
    if header_length != 6 {
        return Err(MidiParseError::InvalidHeader);
    }

    let format = reader.read_u16_be()?;
    let amount_of_tracks = reader.read_u16_be()?;
    let time_division = reader.read_u16_be()?;

    if format != 0
    {
        return Err(MidiParseError::InvalidFormat(format));
    }

    if amount_of_tracks == 0
    {
        return Err(MidiParseError::InvalidTrackCount(amount_of_tracks));
    }

    // Track parsing
    reader.expect_bytes(b"MTrk")?;
    let track_length = reader.read_u32_be()? as usize;

    let track_end = reader.position + track_length;
    let track = parse_midi_track(&mut reader, track_end)?;


    // Placeholder implementation
    Ok(Midi {
        format,
        tracks: vec![track],
        time_division,
    })
}

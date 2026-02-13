
#[allow(dead_code)]
#[derive(Debug)]
pub struct PitchBend(i16);

#[allow(dead_code)]
impl PitchBend {
    pub const MIN: i16 = -8192;
    pub const MAX: i16 = 8191;
    pub const CENTER: i16 = 8192;

    pub fn from_raw(raw: i16) -> Self {
        PitchBend(raw - PitchBend::CENTER)
    }

    pub fn to_raw(&self) -> i16 {
        self.0 + PitchBend::CENTER
    }

    pub fn value(&self) -> i16 {
        self.0
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum MidiEvent {
    Channel {
        channel: u8,
        message: ChannelMessage
    },
    System(SystemMessage),
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum ChannelMessage {
    NoteOn { note: u8, velocity: u8 },
    NoteOff { note: u8, velocity: u8 },
    PolyPressure { note: u8, pressure: u8 },
    ControlChange { controller: u8, value: u8 },
    ProgramChange { program: u8 },
    ChannelPressure { pressure: u8 },
    PitchBend { value: PitchBend },
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum MetaMessage
{
    TrackName(String),
    Tempo(u32),
    TimeSignature { numerator: u8, denominator: u8, clocks_per_click: u8, notes_per_quarter: u8 },
    Raw { meta_type: u8, data: Vec<u8> },
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum SystemMessage
{
    SysEx(Vec<u8>),
    Meta(MetaMessage),
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct TrackEvent {
    pub delta_time: u32,
    pub event: MidiEvent,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct MidiTrack {
    pub events: Vec<TrackEvent>
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Midi {
    pub format: u16,
    pub tracks: Vec<MidiTrack>,
    pub time_division: u16,
}
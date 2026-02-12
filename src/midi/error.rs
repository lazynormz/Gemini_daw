
#[allow(dead_code)]
#[derive(Debug)]
pub enum MidiParseError {
    InvalidHeader,
    InvalidTrackHeader(usize),
    UnexpectedEndOfData,
    InvalidFormat(u16),
    InvalidTrackCount(u16),
    InvalidMetaLength(u8),
    RunningStatusWithoutPrevious,
}
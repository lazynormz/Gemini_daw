mod io;
use io::midi::model::Midi;
use io::midi::parser::parse_midi_file;

fn dump_midi(midi: &Midi) {
    println!("MIDI Format: {}", midi.format);
    println!("Time Division: {}", midi.time_division);
    println!("Tracks: {}", midi.tracks.len());

    for (i, track) in midi.tracks.iter().enumerate() {
        println!("  Track {}: {} events", i + 1, track.events.len());
        for event in &track.events {
            println!("Delta Time: {}, Event: {:?}", event.delta_time, event.event);
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let midi_path = String::from("./midi files/multi_track.mid");
    let midi_data = std::fs::read(midi_path)?;

    let _midi = parse_midi_file(&midi_data);

    match _midi {
        Ok(midi) => {
            dump_midi(&midi);
            write_midi_file(&midi, "./midi files/test_out.mid")?;
        }
        Err(_error) => println!("Failed to parse MIDI file: {:#?}", _error),
    }

    Ok(())
}

fn write_midi_file(midi: &Midi, path: &str) -> std::io::Result<()> {
    let mut writer = io::midi::writer::MidiWriter::new();
    writer.write_midi(midi);
    std::fs::write(path, writer.into_bytes())
}

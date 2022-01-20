use itertools::Itertools;

use mumuse::midi::{self, MidiSend};
use mumuse::music::note::Note;
use mumuse::music::scale::Scale;

pub fn get_notes(n: usize) -> Vec<Note> {
    Scale::major(Note::try_from("C3").unwrap()).two(n).notes
}

fn main() {
    // Open Midi output port connection
    let mut conn_out = midi::get_output_connection("Virtual Midi Bus 1".to_string());
    let n = 4;

    // THUMB UP DOWN
    let thumb_up_down = get_notes(n)
        .into_iter()
        .take(n)
        .chain(get_notes(n).into_iter().take(n - 1).rev())
        .intersperse(get_notes(n)[0])
        .skip(1)
        .take(4 * (n - 2));

    thumb_up_down
        .clone()
        .cycle()
        .take(16)
        .for_each(|n| n.send_midi(&mut conn_out, 150, 64));

    println!(
        "\"tinyNotation: 4/4 {}\"",
        thumb_up_down
            .clone()
            .cycle()
            .take(16)
            .fold(String::new(), |acc, note| {
                let octave_str = (0..note.octave - 3).map(|_| "'").collect::<String>();
                acc + &format!("{:?}{octave_str}4", &note.letter).to_lowercase() + " "
            })
    );
}

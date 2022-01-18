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

    // PINKY UP
    let pinky_up = get_notes(n)
        .into_iter()
        .take(n)
        .intersperse(get_notes(n)[n-1])
        .take(2*(n-1));

    pinky_up.cycle().take(16).for_each(|n| n.send_midi(&mut conn_out, 150, 64)); 

}
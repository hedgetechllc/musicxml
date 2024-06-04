use musicxml::*;

fn main() {
  // Read a MusicXML file of one score type as a different score type
  let score = read_score_timewise("./musicxml/tests/Grande Valse Brillante.musicxml");
  println! {"{:?}", score};
}

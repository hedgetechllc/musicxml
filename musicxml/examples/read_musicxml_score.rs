use musicxml::*;

fn main() {
  // Read and parse a MusicXML file
  let score = read_score_partwise("./musicxml/tests/Grande Valse Brillante.musicxml");
  println! {"{:?}", score};
}

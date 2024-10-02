use musicxml::*;

fn main() {
  // Read and parse a compressed MusicXML file
  let score = read_score_partwise("./musicxml/tests/Grande Valse Brillante.mxl");
  println! {"{score:?}"};
}

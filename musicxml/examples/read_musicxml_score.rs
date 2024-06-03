use musicxml::*;

fn main() {
  let score = read_score_partwise("./musicxml/tests/Grande Valse Brillante.musicxml");
  println! {"{:?}", score};
}

use musicxml::*;

fn main() {
  // Read, parse, and rewrite a MusicXML file
  let output_path = "./target/Grande Valse Brillante.musicxml";
  let score =
    read_score_partwise("./musicxml/tests/Grande Valse Brillante.musicxml").expect("Failed to read input test file");
  println!(
    "Writing to '{}'...{}",
    output_path,
    if write_partwise_score(output_path, &score, false, false).is_ok() {
      "SUCCESS"
    } else {
      "FAILED"
    }
  );
}

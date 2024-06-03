.PHONY: all clean lib format unit

all:
	$(error You must specify one of the following targets: clean docs lib testunit test_EXAMPLE testfull format)

clean:
	@rm -rf pkg target

docs:
	RUSTDOCFLAGS="--extend-css musicxml/assets/docs.css" cargo doc --no-deps --release
	cp -r musicxml/assets/* target/doc/musicxml/
	mv target/doc/musicxml/fonts target/doc/

lib:
	cargo build --lib --release

testunit:
	cargo test --features debug -- --nocapture

test_musicxml_deserializer:
	cargo run --release --features debug --example read_musicxml_score

test_musicxml_serializer:
	cargo run --release --features debug --example write_musicxml_score

test_mxl_deserializer:
	cargo run --release --features debug --example read_mxl_score

test_mxl_serializer:
	cargo run --release --features debug --example write_mxl_score

test_conversion:
	cargo run --release --features debug --example convert_score_types

testfull:
	cargo run --release --features debug --example read_convert_write

format:
	cargo fmt

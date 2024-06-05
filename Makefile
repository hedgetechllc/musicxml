.PHONY: all clean docs lib format publish testunit

all:
	$(error You must specify one of the following targets: clean docs lib format publish testunit test_EXAMPLE)

clean:
	@rm -rf pkg target

docs:
	RUSTDOCFLAGS="--extend-css musicxml/assets/docs.css" cargo doc --workspace --no-deps --release --exclude musicxml_internal --exclude musicxml_macros
	cp -r musicxml/assets/* target/doc/musicxml/
	mv target/doc/musicxml/fonts target/doc/

lib:
	cargo build --lib --release

format:
	cargo fmt

publish:
	cargo install cargo-smart-release
	cargo smart-release --update-crates-index --changelog-without commit-statistics musicxml --execute

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

test_read_convert1:
	cargo run --release --features debug --example read_convert_score1

test_read_convert2:
	cargo run --release --features debug --example read_convert_score2

test_write_convert1:
	cargo run --release --features debug --example write_convert_score1

test_write_convert2:
	cargo run --release --features debug --example write_convert_score2

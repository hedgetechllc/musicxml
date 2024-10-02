.PHONY: all clean docs lib format publish checknostd testunit

all:
	$(error You must specify one of the following targets: clean docs lib format publish checknostd check testunit test_EXAMPLE)

clean:
	cd ensure_no_std && cargo clean && rm -rf Cargo.lock
	cargo clean
	@rm -rf pkg target*

docs:
	RUSTDOCFLAGS="--extend-css musicxml/assets/docs.css" cargo doc --workspace --no-deps --release --exclude musicxml_internal --exclude musicxml_macros
	cp -r musicxml/assets/* target/doc/musicxml/
	mv target/doc/musicxml/fonts target/doc/

lib:
	cargo build --lib --release

format:
	cargo fmt

publish:
	cargo publish -p musicxml_internal
	cargo publish -p musicxml_macros
	cargo publish -p musicxml

checknostd:
	cd ensure_no_std && cargo rustc -- -C link-arg=-nostartfiles

check:
	cargo clippy -- -W clippy::all -W clippy::correctness -W clippy::suspicious -W clippy::complexity -W clippy::perf -W clippy::style -W clippy::pedantic -W clippy::panic -A clippy::doc_markdown -A clippy::wildcard_imports -A clippy::module_name_repetitions -D warnings

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

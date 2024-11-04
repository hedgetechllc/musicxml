# Changelog

## v1.1.0

### Added

* Library has been made `no_std` compatible
* `std` feature (enabled by default) allows direct file reading/writing
* Public functionality to allow parsing to/from data buffer in addition to files
* Internal zip file reader/writer with no external dependencies
* Error documentation

### Fixed

* Removed dependency on `zip` crate
* Switched from `libflate` to `miniz_oxide` for DEFLATE implementation
* Fixed incorrect `32nd` duration parsing string
* Removed all naked `unwrap()` calls
* Lots of clippy cleanups

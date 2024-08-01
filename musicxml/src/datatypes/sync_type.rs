use alloc::string::String;
use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use musicxml_macros::{DatatypeDeserialize, DatatypeSerialize};

/// Specifies the style that a score-following application should use to synchronize an accompaniment with a performer.
#[derive(Debug, PartialEq, Eq, DatatypeDeserialize, DatatypeSerialize)]
pub enum SyncType {
  /// No synchronization to the performer.
  None,
  /// Synchronization is based on the performer tempo rather than individual events in the score.
  Tempo,
  /// Combines the tempo and event approaches, giving more weight to tempo.
  #[rename("mostly-tempo")]
  MostlyTempo,
  /// Combines the tempo and event approaches, giving more weight to performed events.
  #[rename("mostly-event")]
  MostlyEvent,
  /// Synchronization follows the performance of individual events in the score rather than the performer tempo.
  Event,
  /// Provides the strictest synchronization by not being forgiving of missing performed events.
  #[rename("always-event")]
  AlwaysEvent,
}

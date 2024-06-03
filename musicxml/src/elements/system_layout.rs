use super::{SystemDistance, SystemDividers, SystemMargins, TopSystemDistance};
use musicxml_internal::*;
use musicxml_macros::*;

/// Contents of the [SystemLayout] element.
#[derive(Debug, Default, PartialEq, Eq, ContentDeserialize, ContentSerialize)]
pub struct SystemLayoutContents {
  /// The [SystemMargins] element specifies the left and right margins of the system.
  pub system_margins: Option<SystemMargins>,
  /// The [SystemDistance] element specifies the vertical distance between this system and the previous system.
  pub system_distance: Option<SystemDistance>,
  /// The [TopSystemDistance] element specifies the vertical distance from the top of the page to the top of the system.
  pub top_system_distance: Option<TopSystemDistance>,
  /// The [SystemDividers] element specifies the presence or absence of dividers before the system.
  pub system_dividers: Option<SystemDividers>,
}

/// A system is a group of staves that are read and played simultaneously.
///
/// The [SystemLayout] element includes left and right margins and the vertical distance from the previous system.
///
/// Sometimes the sum of measure widths in a system may not equal the system width specified by the layout elements due to roundoff or other errors.
/// The behavior when reading MusicXML files in these cases is application-dependent. For instance, applications may find that the system layout data
/// is more reliable than the sum of the measure widths, and adjust the measure widths accordingly.
///
/// When used in the [Defaults][super::Defaults] element, the [SystemLayout] element defines a default appearance for all systems in the score.
/// If no [SystemLayout] element is present in the [Defaults][super::Defaults] element, default system layout values are chosen by the application.
///
/// When used in the [Print][super::Print] element, the [SystemLayout] element affects the appearance of the current system only. All other systems
/// use the default values as determined by the [Defaults][super::Defaults] element. If any child elements are missing from the [SystemLayout] element
/// in a [Print][super::Print] element, the values determined by the [Defaults][super::Defaults] element are used there as well. This type of
/// [SystemLayout] element need only be read from or written to the first visible part in the score.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("system-layout")]
pub struct SystemLayout {
  /// Element-specific attributes
  pub attributes: (),
  #[flatten]
  /// Element-specific content
  pub content: SystemLayoutContents,
}

use super::{
  Accent, BreathMark, Caesura, DetachedLegato, Doit, Falloff, OtherArticulation, Plop, Scoop, SoftAccent, Spiccato,
  Staccatissimo, Staccato, Stress, StrongAccent, Tenuto, Unstress,
};
use crate::datatypes::Id;
use musicxml_internal::*;
use musicxml_macros::*;

/// Attributes pertaining to the [Articulations] element.
#[derive(Debug, Default, PartialEq, Eq, AttributeDeserialize, AttributeSerialize)]
pub struct ArticulationsAttributes {
  /// Specifies an ID that is unique to the entire document.
  pub id: Option<Id>,
}

#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub enum ArticulationsType {
  Accent(Accent),
  #[rename("strong-accent")]
  StrongAccent(StrongAccent),
  Staccato(Staccato),
  Tenuto(Tenuto),
  #[rename("detached-legato")]
  DetachedLegato(DetachedLegato),
  Staccatissimo(Staccatissimo),
  Spiccato(Spiccato),
  Scoop(Scoop),
  Plop(Plop),
  Doit(Doit),
  Falloff(Falloff),
  #[rename("breath-mark")]
  BreathMark(BreathMark),
  Caesura(Caesura),
  Stress(Stress),
  Unstress(Unstress),
  #[rename("soft-accent")]
  SoftAccent(SoftAccent),
  #[rename("other-articulation")]
  OtherArticulation(OtherArticulation),
}

/// The [Articulations] element groups together articulations and accents.
/// 
/// ![Articulations](articulations.png)
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Articulations {
  /// Element-specific attributes
  pub attributes: ArticulationsAttributes,
  /// Element-specific content
  pub content: Vec<ArticulationsType>,
}

#[cfg(test)]
mod articulations_tests {
  use super::*;
  use crate::datatypes::{BreathMarkValue, CaesuraValue, Tenths};
  use crate::elements::{
    Accent, AccentAttributes, BreathMark, BreathMarkAttributes, Caesura, CaesuraAttributes, DetachedLegato,
    DetachedLegatoAttributes, Doit, DoitAttributes, Falloff, FalloffAttributes, OtherArticulation, Plop,
    PlopAttributes, Scoop, ScoopAttributes, SoftAccent, SoftAccentAttributes, Spiccato, SpiccatoAttributes,
    Staccatissimo, StaccatissimoAttributes, Staccato, StaccatoAttributes, Stress, StressAttributes, StrongAccent,
    StrongAccentAttributes, Tenuto, TenutoAttributes, Unstress, UnstressAttributes,
  };
  use crate::parser::parse_from_xml_str;

  #[test]
  fn test1() {
    let result = parse_from_xml_str::<Articulations>(
      "<articulations id=\"test\">
        <accent/>
        <strong-accent default-x=\"0.1\"/>
        <staccato/>
        <tenuto/>
        <detached-legato/>
        <staccatissimo/>
        <spiccato/>
        <scoop/>
        <plop/>
        <doit/>
        <falloff/>
        <breath-mark>salzedo</breath-mark>
        <caesura>curved</caesura>
        <stress/>
        <unstress/>
        <soft-accent/>
      </articulations>",
    );
    assert_eq!(
      result.unwrap(),
      Articulations {
        attributes: ArticulationsAttributes {
          id: Some(Id(String::from("test")))
        },
        content: vec![
          ArticulationsType::Accent(Accent {
            attributes: AccentAttributes { ..Default::default() },
            content: ()
          }),
          ArticulationsType::StrongAccent(StrongAccent {
            attributes: StrongAccentAttributes {
              default_x: Some(Tenths(0.1)),
              ..Default::default()
            },
            content: ()
          }),
          ArticulationsType::Staccato(Staccato {
            attributes: StaccatoAttributes { ..Default::default() },
            content: ()
          }),
          ArticulationsType::Tenuto(Tenuto {
            attributes: TenutoAttributes { ..Default::default() },
            content: ()
          }),
          ArticulationsType::DetachedLegato(DetachedLegato {
            attributes: DetachedLegatoAttributes { ..Default::default() },
            content: ()
          }),
          ArticulationsType::Staccatissimo(Staccatissimo {
            attributes: StaccatissimoAttributes { ..Default::default() },
            content: ()
          }),
          ArticulationsType::Spiccato(Spiccato {
            attributes: SpiccatoAttributes { ..Default::default() },
            content: ()
          }),
          ArticulationsType::Scoop(Scoop {
            attributes: ScoopAttributes { ..Default::default() },
            content: ()
          }),
          ArticulationsType::Plop(Plop {
            attributes: PlopAttributes { ..Default::default() },
            content: ()
          }),
          ArticulationsType::Doit(Doit {
            attributes: DoitAttributes { ..Default::default() },
            content: ()
          }),
          ArticulationsType::Falloff(Falloff {
            attributes: FalloffAttributes { ..Default::default() },
            content: ()
          }),
          ArticulationsType::BreathMark(BreathMark {
            attributes: BreathMarkAttributes { ..Default::default() },
            content: BreathMarkValue::Salzedo
          }),
          ArticulationsType::Caesura(Caesura {
            attributes: CaesuraAttributes { ..Default::default() },
            content: CaesuraValue::Curved
          }),
          ArticulationsType::Stress(Stress {
            attributes: StressAttributes { ..Default::default() },
            content: ()
          }),
          ArticulationsType::Unstress(Unstress {
            attributes: UnstressAttributes { ..Default::default() },
            content: ()
          }),
          ArticulationsType::SoftAccent(SoftAccent {
            attributes: SoftAccentAttributes { ..Default::default() },
            content: ()
          }),
        ],
      }
    );
  }

  #[test]
  fn test2() {
    let result = parse_from_xml_str::<Articulations>("<articulations></articulations>");
    assert_eq!(
      result.unwrap(),
      Articulations {
        attributes: Default::default(),
        content: Vec::new(),
      }
    );
  }

  #[test]
  fn test3() {
    let result = parse_from_xml_str::<Articulations>(
      "<articulations>
        <tenuto/>
        <other-articulation>Other Test</other-articulation>
      </articulations>",
    );
    assert_eq!(
      result.unwrap(),
      Articulations {
        attributes: Default::default(),
        content: vec![
          ArticulationsType::Tenuto(Tenuto {
            attributes: TenutoAttributes { ..Default::default() },
            content: ()
          }),
          ArticulationsType::OtherArticulation(OtherArticulation {
            attributes: Default::default(),
            content: String::from("Other Test")
          }),
        ],
      }
    );
  }
}

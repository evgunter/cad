//! **Every arm of [`SectionUnbuilt`] ends on one recourse**, rendered the
//! way the viewer draws a failed Boolean (`render(Reading::Build)` under
//! [`BooleanError::GermSectionUnbuilt`]) and held to the refusal standard
//! (`test_utils::refusal::problems`). The arms are walked by their
//! compiler-derived kind, so an arm added without a sample is a compile
//! error here, and a [`geom_brep::WindsRefusal`] is sampled at every
//! variant.

use super::{BooleanError, SectionUnbuilt, SectionUnbuiltKind};
use geom::{PieceWalkRefusal, SurfaceKind};
use geom_brep::recourse::Reading;
use strum::{IntoEnumIterator as _, VariantArray as _};
use test_utils::refusal::Admission;

fn samples(kind: SectionUnbuiltKind) -> Vec<(String, SectionUnbuilt)> {
    let one = |e| vec![(format!("{kind:?}"), e)];
    match kind {
        SectionUnbuiltKind::Refused => one(SectionUnbuilt::Refused(
            geom_brep::SectionError::DegenerateOperand {
                what: "a cone's half-angle is not definitely positive",
            },
        )),
        SectionUnbuiltKind::Unminted => geom_brep::NoCarrier::VARIANTS
            .iter()
            .map(|&why| (format!("Unminted({why:?})"), SectionUnbuilt::Unminted(why)))
            .collect(),
        SectionUnbuiltKind::NoLane => one(SectionUnbuilt::NoLane { scalar: "Interval" }),
        SectionUnbuiltKind::March => one(SectionUnbuilt::March(
            geom_brep::SsiError::InvalidMarchTol { value: -1.0 },
        )),
        SectionUnbuiltKind::Winds => geom_brep::WindsRefusal::VARIANTS
            .iter()
            .map(|&why| (format!("Winds({why:?})"), SectionUnbuilt::Winds(why)))
            .collect(),
        SectionUnbuiltKind::PadShort => vec![
            (
                "PadShort".to_owned(),
                SectionUnbuilt::PadShort {
                    pad: 1e-7,
                    hull_sup: 3e-7,
                },
            ),
            (
                "PadShort (NaN bound)".to_owned(),
                SectionUnbuilt::PadShort {
                    pad: 1e-7,
                    hull_sup: f64::NAN,
                },
            ),
        ],
        SectionUnbuiltKind::TubeUnread => one(SectionUnbuilt::TubeUnread),
        SectionUnbuiltKind::Seat => one(SectionUnbuilt::Seat(
            geom_core::spline::KnotAlgebraError::KnotNotPresent { u: 0.5 },
        )),
        SectionUnbuiltKind::Unwalked => PieceWalkRefusal::VARIANTS
            .iter()
            .map(|&why| (format!("Unwalked({why:?})"), SectionUnbuilt::Unwalked(why)))
            .collect(),
    }
}

/// **The spans of a door-ended arm's text a filed row owns**, admitted
/// the shared way (`test_utils::refusal::problems_admitting`): an
/// admission whose span the text no longer holds is itself a problem.
const ADMITTED: &[Admission<'static>] = &[Admission {
    row: "March",
    span: "ssi:",
    filed: "work/ssiarith/ssi-refusals-reach-the-boolean-under-their-stage-prefix.md",
}];

/// **The door-ended arms a filed row owns a missing recourse for.** An
/// admission masks a span of text, and a missing recourse has none, so
/// these are named apart: the row must still state no recourse, or its
/// entry is a problem.
const NO_RECOURSE_FILED: &[(&str, &str)] = &[(
    "Refused",
    "work/germtail/a-section-refused-at-the-glue-door-ends-on-no-recourse.md",
)];

#[test]
fn every_section_unbuilt_arm_ends_on_one_recourse() {
    let mut problems = Vec::new();
    let mut names = Vec::new();
    for (name, why) in SectionUnbuiltKind::iter().flat_map(samples) {
        let text = BooleanError::GermSectionUnbuilt {
            a_face: crate::FaceKey::default(),
            a_kind: SurfaceKind::Torus,
            b_face: crate::FaceKey::default(),
            b_kind: SurfaceKind::Cylinder,
            why,
        }
        .render(Reading::Build);
        let filed = NO_RECOURSE_FILED.iter().find(|(row, _)| *row == name);
        let found = test_utils::refusal::problems_admitting(&name, &text, &[], false, ADMITTED);
        let unstated = format!("{name} states no recourse: {text}");
        match filed {
            Some((_, item)) if !found.contains(&unstated) => problems.push(format!(
                "{name} now states a recourse ({item}): drop its entry: {text}"
            )),
            Some(_) => problems.extend(found.into_iter().filter(|p| *p != unstated)),
            None => problems.extend(found),
        }
        names.push(name);
    }
    for (row, item) in NO_RECOURSE_FILED {
        if !names.iter().any(|n| n == row) {
            problems.push(format!("the entry for {row} names no sampled row ({item})"));
        }
    }
    problems.extend(test_utils::refusal::unclaimed_admissions(
        ADMITTED,
        names.iter().map(String::as_str),
    ));
    assert!(problems.is_empty(), "{}", problems.join("\n"));
}

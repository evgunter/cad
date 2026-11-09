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

fn samples(kind: SectionUnbuiltKind) -> Vec<(String, SectionUnbuilt)> {
    let one = |e| vec![(format!("{kind:?}"), e)];
    match kind {
        SectionUnbuiltKind::Refused => one(SectionUnbuilt::Refused(
            geom_brep::SectionError::DegenerateOperand {
                what: "a cone's half-angle is not definitely positive",
            },
        )),
        SectionUnbuiltKind::Tangent => one(SectionUnbuilt::Tangent(
            "a component of the section is tangent",
        )),
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

/// **The shortfalls of a door-ended arm's payload that a filed row owns**:
/// the row's name, a fragment of the problem, and the item that fixes it.
/// An entry whose problem no longer turns up is itself a problem, so the
/// list cannot outlive what it names.
const FILED: &[(&str, &str, &str)] = &[
    (
        "Refused",
        "states no recourse",
        "work/germtail/a-section-refused-at-the-glue-door-ends-on-no-recourse.md",
    ),
    (
        "March",
        "stage prefix \"ssi:\"",
        "work/ssiarith/ssi-refusals-reach-the-boolean-under-their-stage-prefix.md",
    ),
];

#[test]
fn every_section_unbuilt_arm_ends_on_one_recourse() {
    let mut problems = Vec::new();
    let mut claimed = vec![false; FILED.len()];
    for (name, why) in SectionUnbuiltKind::iter().flat_map(samples) {
        let text = BooleanError::GermSectionUnbuilt {
            a_face: crate::FaceKey::default(),
            a_kind: SurfaceKind::Torus,
            b_face: crate::FaceKey::default(),
            b_kind: SurfaceKind::Cylinder,
            why,
        }
        .render(Reading::Build);
        for problem in test_utils::refusal::problems(&name, &text, &[], false) {
            match FILED
                .iter()
                .position(|(row, what, _)| *row == name && problem.contains(what))
            {
                Some(k) => claimed[k] = true,
                None => problems.push(problem),
            }
        }
    }
    for ((row, what, filed), claimed) in FILED.iter().zip(claimed) {
        if !claimed {
            problems.push(format!("{row} no longer {what} ({filed}): drop its entry"));
        }
    }
    assert!(problems.is_empty(), "{}", problems.join("\n"));
}

//! **Every arm of [`SectionUnbuilt`] ends on one recourse**, rendered the
//! way the viewer draws a failed Boolean (`render(Reading::Build)` under
//! [`BooleanError::GermSectionUnbuilt`]) and held to the refusal standard
//! (`test_utils::refusal`). The arms, and the carry's refusal under them,
//! are walked by their compiler-derived kinds, so an arm added without a
//! sample is a compile error here; every payload enum that states its own
//! sentence and recourse is sampled at every variant.

#![allow(clippy::expect_used)] // one fixed band, well-formed by construction

use super::{BooleanError, SectionUnbuilt, SectionUnbuiltKind};
use geom::{PieceWalkRefusal, SurfaceKind};
use geom_brep::recourse::Reading;
use geom_brep::{CarryError, CarryErrorKind, NoCarrier, TubeRefusal, WindsRefusal};
use geom_core::spline::{KnotAlgebraError, KnotVectorIssue, SplineError};
use strum::{IntoEnumIterator as _, VariantArray as _};
use test_utils::refusal::Admission;

fn each<T: Copy + core::fmt::Debug>(
    what: &str,
    all: &[T],
    carry: impl Fn(T) -> CarryError,
) -> Vec<(String, SectionUnbuilt)> {
    all.iter()
        .map(|&v| (format!("{what}({v:?})"), SectionUnbuilt::Carry(carry(v))))
        .collect()
}

fn carried(kind: CarryErrorKind) -> Vec<(String, SectionUnbuilt)> {
    let one = |e| vec![(format!("{kind:?}"), SectionUnbuilt::Carry(e))];
    match kind {
        CarryErrorKind::Unminted => each("Unminted", NoCarrier::VARIANTS, CarryError::Unminted),
        CarryErrorKind::NoLane => one(CarryError::NoLane { scalar: "Interval" }),
        CarryErrorKind::March => one(CarryError::March(geom_brep::SsiError::InvalidMarchTol {
            value: -1.0,
        })),
        CarryErrorKind::Winds => each("Winds", WindsRefusal::VARIANTS, CarryError::Winds),
        CarryErrorKind::Escalated => one(CarryError::Escalated(geom_core::Indeterminate {
            margin: geom_core::MarginDiag::value(5e-9),
            band: geom_core::Band::new(1e-9, 1e-8).expect("a well-formed band"),
            predicate: Some("ssi_tube_graph"),
            terminal_sliver: false,
        })),
        CarryErrorKind::PadShort => vec![
            (
                "PadShort".to_owned(),
                SectionUnbuilt::Carry(CarryError::PadShort {
                    pad: 1e-7,
                    hull_sup: 3e-7,
                }),
            ),
            (
                "PadShort (NaN bound)".to_owned(),
                SectionUnbuilt::Carry(CarryError::PadShort {
                    pad: 1e-7,
                    hull_sup: f64::NAN,
                }),
            ),
        ],
        CarryErrorKind::TubeUnread => {
            each("TubeUnread", TubeRefusal::VARIANTS, CarryError::TubeUnread)
        }
        CarryErrorKind::Desync => one(CarryError::Desync("a branch's winding is read per operand")),
        CarryErrorKind::Unwalked => {
            each("Unwalked", PieceWalkRefusal::VARIANTS, CarryError::Unwalked)
        }
        CarryErrorKind::Seat => vec![
            (
                "Seat".to_owned(),
                SectionUnbuilt::Carry(CarryError::Seat(KnotAlgebraError::KnotNotPresent {
                    u: 0.5,
                })),
            ),
            (
                "Seat (structure)".to_owned(),
                SectionUnbuilt::Carry(CarryError::Seat(KnotAlgebraError::Structure(
                    SplineError::KnotVectorInvalid {
                        reason: KnotVectorIssue::Decreasing { index: 3 },
                    },
                ))),
            ),
        ],
    }
}

fn samples(kind: SectionUnbuiltKind) -> Vec<(String, SectionUnbuilt)> {
    match kind {
        SectionUnbuiltKind::Refused => vec![(
            "Refused".to_owned(),
            SectionUnbuilt::Refused(geom_brep::SectionError::DegenerateOperand {
                what: "a cone's half-angle is not definitely positive",
            }),
        )],
        SectionUnbuiltKind::Carry => CarryErrorKind::iter().flat_map(carried).collect(),
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

/// **A seat refused on its knots' structure keeps the structure's fact**
/// for the defect report, and states the knot API's own recourse nowhere.
#[test]
fn a_refused_seat_keeps_the_structures_fact() {
    let issue = KnotVectorIssue::Decreasing { index: 3 };
    let text = CarryError::Seat(KnotAlgebraError::Structure(
        SplineError::KnotVectorInvalid { reason: issue },
    ))
    .render(Reading::Build);
    assert!(
        text.contains(&issue.fact().to_string())
            && text.ends_with(geom_core::KERNEL_DEFECT_ENDING)
            && text.matches("Recourse:").count() == 0,
        "{text}"
    );
}

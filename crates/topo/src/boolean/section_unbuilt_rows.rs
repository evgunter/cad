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
use geom_brep::{
    CarryError, CarryErrorKind, CarryRefusal, CarryRefusalKind, LoopTubeRefusal, NoCarrier,
    WindsRefusal,
};
use geom_core::spline::{Built, Fact as _, KnotAlgebraError, KnotVectorIssue, SplineError};
use strum::{IntoEnumIterator as _, VariantArray as _};
use test_utils::refusal::Admission;

fn each<T: Copy + core::fmt::Debug>(
    what: &str,
    all: &[T],
    carry: impl Fn(T) -> CarryRefusal,
) -> Vec<(String, SectionUnbuilt)> {
    all.iter()
        .map(|&v| (format!("{what}({v:?})"), SectionUnbuilt::Carry(carry(v))))
        .collect()
}

fn carried(kind: CarryRefusalKind) -> Vec<(String, SectionUnbuilt)> {
    let one = |e| vec![(format!("{kind:?}"), SectionUnbuilt::Carry(e))];
    match kind {
        CarryRefusalKind::Unminted => each("Unminted", NoCarrier::VARIANTS, CarryRefusal::Unminted),
        CarryRefusalKind::NoLane => one(CarryRefusal::NoLane { scalar: "Interval" }),
        CarryRefusalKind::March => one(CarryRefusal::March(geom_brep::SsiError::InvalidMarchTol {
            value: -1.0,
        })),
        CarryRefusalKind::Winds => each("Winds", WindsRefusal::VARIANTS, CarryRefusal::Winds),
        CarryRefusalKind::PadShort => vec![
            (
                "PadShort".to_owned(),
                SectionUnbuilt::Carry(CarryRefusal::PadShort {
                    pad: 1e-7,
                    hull_sup: 3e-7,
                }),
            ),
            (
                "PadShort (NaN bound)".to_owned(),
                SectionUnbuilt::Carry(CarryRefusal::PadShort {
                    pad: 1e-7,
                    hull_sup: f64::NAN,
                }),
            ),
        ],
        CarryRefusalKind::TubeUnread => each(
            "TubeUnread",
            LoopTubeRefusal::VARIANTS,
            CarryRefusal::TubeUnread,
        ),
        CarryRefusalKind::Unwalked => each(
            "Unwalked",
            PieceWalkRefusal::VARIANTS,
            CarryRefusal::Unwalked,
        ),
        CarryRefusalKind::Seat => vec![
            (
                "Seat".to_owned(),
                SectionUnbuilt::Carry(CarryRefusal::Seat(Built::new(
                    KnotAlgebraError::KnotNotPresent { u: 0.5 },
                ))),
            ),
            (
                "Seat (structure)".to_owned(),
                SectionUnbuilt::Carry(CarryRefusal::Seat(Built::new(KnotAlgebraError::Structure(
                    SplineError::KnotVectorInvalid {
                        reason: KnotVectorIssue::Decreasing { index: 3 },
                    },
                )))),
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
        SectionUnbuiltKind::Carry => CarryRefusalKind::iter().flat_map(carried).collect(),
    }
}

/// **Every arm of [`CarryError`] ends on its own recourse**, as a caller
/// of `PairSection::carry` reads it, walked by its compiler-derived kind:
/// a reading in the band on the escalation's own ending, a broken
/// invariant on the kernel's defect, and a decided refusal on its
/// payload's (the gate above holds every payload).
#[test]
fn every_carry_error_arm_ends_on_its_own_recourse() {
    let diag = geom_core::Indeterminate {
        margin: geom_core::MarginDiag::value(5e-9),
        band: geom_core::Band::new(1e-9, 1e-8).expect("a well-formed band"),
        predicate: Some("ssi_tube_graph"),
        terminal_sliver: false,
    };
    let refusal = CarryRefusal::NoLane { scalar: "Interval" };
    let mut problems = Vec::new();
    for kind in CarryErrorKind::iter() {
        let (e, ending) = match kind {
            CarryErrorKind::Refused => (
                CarryError::Refused(refusal.clone()),
                refusal.render(Reading::Build),
            ),
            CarryErrorKind::Escalated => (CarryError::Escalated(diag), diag.to_string()),
            CarryErrorKind::Desync => (
                CarryError::Desync("a component's tube pads no band"),
                geom_core::KERNEL_DEFECT_ENDING.to_owned(),
            ),
        };
        let name = format!("{kind:?}");
        let text = e.render(Reading::Build);
        if !text.ends_with(&ending) {
            problems.push(format!(
                "{name} does not end on its own ending ({ending}): {text}"
            ));
        }
        problems.extend(test_utils::refusal::problems(&name, &text, &[], false));
    }
    assert!(problems.is_empty(), "{}", problems.join("\n"));
}

/// **A decided refusal reads the same held in a [`CarryError`]**: the
/// wrapper adds nothing to its payload's text, at a run door and at the
/// import door.
#[test]
fn a_refused_carry_renders_as_its_refusal() {
    let file = geom_core::FileCoincidence::new(1e-6);
    for refusal in [
        CarryRefusal::PadShort {
            pad: 1e-7,
            hull_sup: 3e-7,
        },
        CarryRefusal::Unminted(NoCarrier::Touch),
    ] {
        let held = CarryError::Refused(refusal.clone());
        assert_eq!(
            held.render(Reading::Build),
            refusal.render(Reading::Build),
            "{refusal:?}"
        );
        assert_eq!(held.render(file), refusal.render(file), "{refusal:?}");
        assert_eq!(held.to_string(), refusal.to_string(), "{refusal:?}");
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
    let text = CarryError::Refused(CarryRefusal::Seat(Built::new(KnotAlgebraError::Structure(
        SplineError::KnotVectorInvalid { reason: issue },
    ))))
    .render(Reading::Build);
    assert!(
        text.contains(&issue.fact().to_string())
            && text.ends_with(geom_core::KERNEL_DEFECT_ENDING)
            && text.matches("Recourse:").count() == 0,
        "{text}"
    );
}

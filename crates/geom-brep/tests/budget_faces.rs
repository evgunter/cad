//! The refinement loop's four faces, pinned from outside the module
//! over a corpus rather than one instance each: every refusal the
//! corpus reaches keeps its face's own payload invariants — the two
//! faces that carry a bound carry a finite one, the cap face never
//! fires on the round budget's last round (the budget test precedes
//! the marking), `rounds` never exceeds the budget, a not-finite face
//! carries the last finite bound or none — and every message names the
//! lever its face's doc claims.
//!
//! The corpus is 2 bases x 7 deltas x 5 tolerances, and each (base, δ)
//! column is its own `#[test]`: `fit_offset_at` is seconds per cell, so
//! a corpus in one row is a row the runner cannot spread over its
//! cores and is the whole binary's wall. The columns are independent —
//! a cell's assertions read only that cell's payload — so the split
//! costs nothing but the fixture, rebuilt per column.
//!
//! What a column claims beyond its cells: that it reached at least one
//! refusal face. A column every tolerance fits is a column whose
//! payload assertions never ran, and it says so rather than passing.
//! The two faces the corpus is here for — the sample cap and a
//! not-finite bound with no finite round behind it — are claimed by
//! name on the columns that reach them.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::NurbsSurface;
use geom_brep::offset_fit::{
    OFFSET_FIT_BUDGET, OFFSET_FIT_SAMPLE_CAP, OffsetFitError, fit_offset_at,
};

use crate::shared::fixture::{bumpy_patch, quarter_cylinder};
use crate::shared::tol::band;

/// The tolerances every column sweeps.
const TOLS: [f64; 5] = [1e-15, 1e-12, 1e-9, 1e-6, 1e-3];

/// The refusal faces one column reached, one counter each.
#[derive(Default)]
struct Faces {
    budget: u32,
    cap: u32,
    /// A not-finite bound with no finite round behind it.
    not_finite_none: u32,
    /// A not-finite bound that lost a finite round.
    not_finite_lost: u32,
    stalled: u32,
}

impl Faces {
    fn total(&self) -> u32 {
        self.budget + self.cap + self.not_finite_none + self.not_finite_lost + self.stalled
    }
}

/// One (base, δ) column: every tolerance's outcome, each refusal
/// checked against its own face's payload invariants.
///
/// Returns the faces the column reached, and refuses a column that
/// reached none — five fits and nothing asserted is a column that
/// proves nothing, which is the one way this sweep can rot silently.
fn column(name: &str, base: &NurbsSurface<f64>, d: f64) -> Faces {
    let mut seen = Faces::default();
    for t in TOLS {
        match fit_offset_at(base, d, t, band()) {
            Ok(_) => {}
            Err(OffsetFitError::BudgetExhausted {
                budget,
                grid,
                achieved,
                tolerance,
            }) => {
                seen.budget += 1;
                assert_eq!(budget, OFFSET_FIT_BUDGET, "{name} d={d} t={t}");
                assert!(
                    achieved.is_finite(),
                    "{name} d={d} t={t}: BudgetExhausted carries a non-finite bound"
                );
                assert!(achieved > tolerance, "{name} d={d} t={t}");
                assert!(
                    grid.0 <= OFFSET_FIT_SAMPLE_CAP && grid.1 <= OFFSET_FIT_SAMPLE_CAP,
                    "{name} d={d} t={t}: grid past the cap"
                );
            }
            Err(OffsetFitError::SampleCapReached {
                cap,
                rounds,
                grid,
                achieved,
                tolerance,
            }) => {
                seen.cap += 1;
                assert_eq!(cap, OFFSET_FIT_SAMPLE_CAP, "{name} d={d} t={t}");
                assert!(
                    achieved.is_finite(),
                    "{name} d={d} t={t}: SampleCapReached carries a non-finite bound"
                );
                assert!(achieved > tolerance, "{name} d={d} t={t}");
                // THE TIE: the budget test runs before the marking, so
                // a cap stop can never be the budget's last round.
                assert!(
                    (rounds as usize) < OFFSET_FIT_BUDGET,
                    "{name} d={d} t={t}: the cap face fired on the budget's last round \
                     (rounds={rounds}, budget={OFFSET_FIT_BUDGET})"
                );
                assert!(
                    grid.0 <= OFFSET_FIT_SAMPLE_CAP && grid.1 <= OFFSET_FIT_SAMPLE_CAP,
                    "{name} d={d} t={t}"
                );
            }
            Err(OffsetFitError::BoundNotFinite {
                rounds,
                grid,
                d: dd,
                tolerance,
                last_finite,
            }) => {
                assert_eq!(dd, d, "{name} d={d} t={t}");
                assert_eq!(tolerance, t, "{name} d={d} t={t}");
                assert!(
                    (rounds as usize) <= OFFSET_FIT_BUDGET,
                    "{name} d={d} t={t}: rounds past the budget"
                );
                assert!(
                    grid.0 <= OFFSET_FIT_SAMPLE_CAP && grid.1 <= OFFSET_FIT_SAMPLE_CAP,
                    "{name} d={d} t={t}"
                );
                match last_finite {
                    None => seen.not_finite_none += 1,
                    Some(b) => {
                        seen.not_finite_lost += 1;
                        assert!(
                            b.is_finite() && b > tolerance,
                            "{name} d={d} t={t}: a lost bound of {b} is not a bound \
                             the loop could have refused on"
                        );
                    }
                }
            }
            Err(OffsetFitError::RefinementStalled { rounds, .. }) => {
                seen.stalled += 1;
                assert!(
                    (rounds as usize) <= OFFSET_FIT_BUDGET,
                    "{name} d={d} t={t}: rounds past the budget"
                );
            }
            Err(_) => {}
        }
    }
    eprintln!(
        "{name} d={d}: budget={} cap={} not_finite(none)={} not_finite(lost)={} stalled={}",
        seen.budget, seen.cap, seen.not_finite_none, seen.not_finite_lost, seen.stalled
    );
    assert!(
        seen.total() > 0,
        "{name} d={d}: every tolerance fitted, so the column asserted nothing about any face"
    );
    seen
}

/// Every face this column reaches keeps its own payload invariants,
/// and this is the corpus's witness of the not-finite face with no
/// finite round behind it.
#[test]
fn quarter_cylinder_at_delta_1e_8_keeps_every_faces_payload_invariants() {
    let faces = column("quarter_cylinder", &quarter_cylinder(1.0, 1.0), 1e-8);
    assert!(
        faces.not_finite_none > 0,
        "this column is the corpus's witness of a not-finite bound with no finite \
         round behind it, and it no longer reaches that face"
    );
}

/// Every face this column reaches keeps its own payload invariants.
#[test]
fn quarter_cylinder_at_delta_1e_7_keeps_every_faces_payload_invariants() {
    column("quarter_cylinder", &quarter_cylinder(1.0, 1.0), 1e-7);
}

/// Every face this column reaches keeps its own payload invariants,
/// and this is the corpus's witness of the sample-cap face.
#[test]
fn quarter_cylinder_at_delta_1e_6_keeps_every_faces_payload_invariants() {
    let faces = column("quarter_cylinder", &quarter_cylinder(1.0, 1.0), 1e-6);
    assert!(
        faces.cap > 0,
        "this column is the corpus's witness of the sample cap, and it no longer \
         reaches that face"
    );
}

/// Every face this column reaches keeps its own payload invariants.
#[test]
fn quarter_cylinder_at_delta_1e_5_keeps_every_faces_payload_invariants() {
    column("quarter_cylinder", &quarter_cylinder(1.0, 1.0), 1e-5);
}

/// Every face this column reaches keeps its own payload invariants.
#[test]
fn quarter_cylinder_at_delta_0_01_keeps_every_faces_payload_invariants() {
    column("quarter_cylinder", &quarter_cylinder(1.0, 1.0), 0.01);
}

/// Every face this column reaches keeps its own payload invariants.
#[test]
fn quarter_cylinder_at_delta_0_05_keeps_every_faces_payload_invariants() {
    column("quarter_cylinder", &quarter_cylinder(1.0, 1.0), 0.05);
}

/// Every face this column reaches keeps its own payload invariants.
#[test]
fn quarter_cylinder_at_delta_minus_0_05_keeps_every_faces_payload_invariants() {
    column("quarter_cylinder", &quarter_cylinder(1.0, 1.0), -0.05);
}

/// Every face this column reaches keeps its own payload invariants.
#[test]
fn bumpy_at_delta_1e_8_keeps_every_faces_payload_invariants() {
    column("bumpy", &bumpy_patch(), 1e-8);
}

/// Every face this column reaches keeps its own payload invariants.
#[test]
fn bumpy_at_delta_1e_7_keeps_every_faces_payload_invariants() {
    column("bumpy", &bumpy_patch(), 1e-7);
}

/// Every face this column reaches keeps its own payload invariants.
#[test]
fn bumpy_at_delta_1e_6_keeps_every_faces_payload_invariants() {
    column("bumpy", &bumpy_patch(), 1e-6);
}

/// Every face this column reaches keeps its own payload invariants.
#[test]
fn bumpy_at_delta_1e_5_keeps_every_faces_payload_invariants() {
    column("bumpy", &bumpy_patch(), 1e-5);
}

/// Every face this column reaches keeps its own payload invariants.
#[test]
fn bumpy_at_delta_0_01_keeps_every_faces_payload_invariants() {
    column("bumpy", &bumpy_patch(), 0.01);
}

/// Every face this column reaches keeps its own payload invariants.
#[test]
fn bumpy_at_delta_0_05_keeps_every_faces_payload_invariants() {
    column("bumpy", &bumpy_patch(), 0.05);
}

/// Every face this column reaches keeps its own payload invariants.
#[test]
fn bumpy_at_delta_minus_0_05_keeps_every_faces_payload_invariants() {
    column("bumpy", &bumpy_patch(), -0.05);
}

/// Each message names the lever the face's doc claims, and no other.
#[test]
fn each_faces_message_names_its_lever() {
    let b = OffsetFitError::BudgetExhausted {
        budget: OFFSET_FIT_BUDGET,
        grid: (9, 9),
        achieved: 1e-3,
        tolerance: 1e-9,
    }
    .to_string();
    assert!(b.contains("OFFSET_FIT_BUDGET"), "{b}");
    assert!(!b.contains("OFFSET_FIT_SAMPLE_CAP"), "{b}");
    let c = OffsetFitError::SampleCapReached {
        cap: OFFSET_FIT_SAMPLE_CAP,
        rounds: 5,
        grid: (41, 40),
        achieved: 1e-3,
        tolerance: 1e-9,
    }
    .to_string();
    assert!(c.contains("OFFSET_FIT_SAMPLE_CAP"), "{c}");
    assert!(
        c.contains(&format!("5 of {OFFSET_FIT_BUDGET} rounds")),
        "{c}"
    );
    let n = OffsetFitError::BoundNotFinite {
        rounds: 4,
        grid: (25, 17),
        d: 1e-7,
        tolerance: 1e-3,
        last_finite: None,
    }
    .to_string();
    assert!(
        n.contains("without any round producing a finite sup bound"),
        "{n}"
    );
    assert!(
        n.contains("neither the round budget nor the sample cap"),
        "{n}"
    );
    // The never-finite message names no constant a caller could raise.
    assert!(!n.contains("OFFSET_FIT_"), "{n}");
    let l = OffsetFitError::BoundNotFinite {
        rounds: 4,
        grid: (25, 17),
        d: 1e-7,
        tolerance: 1e-3,
        last_finite: Some(3.2e-4),
    }
    .to_string();
    assert!(l.contains("the schedule is the lever"), "{l}");
    assert!(
        l.contains("0.00032 m"),
        "the lost bound is not in the message: {l}"
    );
    assert!(!l.contains("OFFSET_FIT_"), "{l}");
    assert_ne!(n, l, "the two not-finite cases must read differently");
}

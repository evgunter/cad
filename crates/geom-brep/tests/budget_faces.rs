//! The refinement loop's four faces, pinned from outside the module
//! over a corpus rather than one instance each: every refusal the
//! corpus reaches keeps its face's own payload invariants — the two
//! faces that carry a bound carry a finite one, every face that carries
//! a bound also carries the smallest any round reached, no larger than
//! its last and on a grid no finer, the cap face never fires on the
//! round budget's last round (the budget test precedes the marking),
//! `rounds` never exceeds the budget, a not-finite face carries the
//! smallest finite bound or none — and every message names the lever
//! its face's doc claims.
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
//!
//! **Each column also pins its five OUTCOMES**, face and digits, and
//! the fourteen tables together are the corpus census: which face
//! each of the 70 requests wears and what bound it carries. Two
//! things need that and neither is served by the payload invariants
//! above, which hold whatever the numbers are.
//!
//! - **A face count stated in prose is checkable against it.** A
//!   change that moves requests between faces has to move rows here,
//!   one per request, so the count in the change's own description is
//!   read off a table rather than assembled by hand.
//! - **A bound that GREW reds here.** The cell bound is monotone on a
//!   fixed grid, but the door's is not monotone in it — a tightening
//!   reorders the refinement marking, and a different schedule is a
//!   different fitted surface (`offset_fit`'s `measure`, where the
//!   marking's cut is taken).
//!   So "every bound only goes down" is not available as an argument
//!   and the corpus has to be measured. The digits are pinned at
//!   `1e-3` relative: tight enough to red on the 1.8% the one grown
//!   request moved by, loose enough not to chase an ulp.
//!
//! The budget face is pinned by its reading too: `budget` when the last
//! round was still falling, `budget-rose` when it was the first not
//! to, since those are two different sentences to the caller.
//!
//! Re-baselining a row here is the ordinary answer when a bound
//! moves; what the table forbids is moving one silently.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::NurbsSurface;
use geom_brep::offset_fit::{
    OFFSET_FIT_BUDGET, OFFSET_FIT_SAMPLE_CAP, OffsetFitError, fit_offset_at,
};

use crate::shared::fixture::{bumpy_patch, quarter_cylinder};
use crate::shared::tol::band;

/// The tolerances every column sweeps.
const TOLS: [f64; 5] = [1e-15, 1e-12, 1e-9, 1e-6, 1e-3];

/// The best bound a face carries against the last one it carries:
/// finite, above the tolerance (a request at it would otherwise have
/// certified on the round that reached it), no larger than the last,
/// and reached on a grid no finer.
fn best_is_the_smallest(
    at: &str,
    achieved: f64,
    grid: (usize, usize),
    tolerance: f64,
    best: f64,
    best_grid: (usize, usize),
) {
    assert!(
        best.is_finite() && best > tolerance && best <= achieved,
        "{at}: the best bound {best:e} is not the smallest of a run ending on {achieved:e} \
         against {tolerance:e}"
    );
    assert!(
        best_grid.0 <= grid.0 && best_grid.1 <= grid.1,
        "{at}: the best bound's grid {best_grid:?} is finer than the last {grid:?}"
    );
}

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
/// checked against its own face's payload invariants, and each
/// outcome checked against the census row `want` pins for it — the
/// face by name, and the bound it carries to `1e-3` relative where
/// its face carries one. `f64::NAN` in a `want` entry is the face
/// that carries no number.
///
/// Returns the faces the column reached, and refuses a column that
/// reached none — five fits and nothing asserted is a column that
/// proves nothing, which is the one way this sweep can rot silently.
fn column(name: &str, base: &NurbsSurface<f64>, d: f64, want: [(&str, f64); 5]) -> Faces {
    let mut seen = Faces::default();
    // EVERY census cell that moved, not the first: a re-pin reads the
    // whole column at once, and a first-mismatch report costs one full
    // refinement sweep per cell.
    let mut moved: Vec<String> = Vec::new();
    for (i, t) in TOLS.into_iter().enumerate() {
        let got: (&str, f64) = match fit_offset_at(base, d, t, band()) {
            Ok((_, c)) => ("certified", c.hull_sup),
            Err(OffsetFitError::BudgetExhausted {
                budget,
                grid,
                achieved,
                tolerance,
                still_falling,
                best,
                best_grid,
            }) => {
                seen.budget += 1;
                let at = format!("{name} d={d} t={t}");
                best_is_the_smallest(&at, achieved, grid, tolerance, best, best_grid);
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
                if still_falling {
                    ("budget", achieved)
                } else {
                    ("budget-rose", achieved)
                }
            }
            Err(OffsetFitError::SampleCapReached {
                cap,
                rounds,
                grid,
                achieved,
                tolerance,
                best,
                best_grid,
            }) => {
                seen.cap += 1;
                let at = format!("{name} d={d} t={t}");
                best_is_the_smallest(&at, achieved, grid, tolerance, best, best_grid);
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
                ("cap", achieved)
            }
            Err(OffsetFitError::BoundNotFinite {
                rounds,
                grid,
                d: dd,
                tolerance,
                best_finite,
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
                match best_finite {
                    None => {
                        seen.not_finite_none += 1;
                        ("not-finite", f64::NAN)
                    }
                    Some(b) => {
                        seen.not_finite_lost += 1;
                        assert!(
                            b.is_finite() && b > tolerance,
                            "{name} d={d} t={t}: a lost bound of {b} is not a bound \
                             the loop could have refused on"
                        );
                        ("not-finite-lost", b)
                    }
                }
            }
            Err(OffsetFitError::RefinementStalled {
                rounds,
                grid,
                achieved,
                tolerance,
                best,
                best_grid,
            }) => {
                seen.stalled += 1;
                let at = format!("{name} d={d} t={t}");
                best_is_the_smallest(&at, achieved, grid, tolerance, best, best_grid);
                assert!(
                    (rounds as usize) <= OFFSET_FIT_BUDGET,
                    "{name} d={d} t={t}: rounds past the budget"
                );
                ("stalled", achieved)
            }
            Err(e) => panic!("{name} d={d} t={t}: the loop left the four faces: {e:?}"),
        };
        // The census row. The face is pinned by name, because which
        // face a request wears is the thing a change to the bound
        // moves and the thing a count of them is read off. The bound
        // is pinned where the face carries one, because the door's
        // sup is NOT monotone in the cell bound and a request that got
        // worse is otherwise invisible.
        assert_eq!(
            got.0, want[i].0,
            "{name} d={d} t={t}: the face moved to {} carrying {:e}",
            got.0, got.1
        );
        if want[i].1.is_finite() && (got.1 - want[i].1).abs() > want[i].1.abs() * 1e-3 {
            moved.push(format!(
                "t={t}: the {} face carries {:e}, pinned at {:e}",
                got.0, got.1, want[i].1
            ));
        }
    }
    assert!(
        moved.is_empty(),
        "{name} d={d}: {} census cell(s) moved — {}",
        moved.len(),
        moved.join("; ")
    );
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
/// the table is this column's row of the corpus census, and this is
/// the corpus's witness of the not-finite face with no finite round
/// behind it.
#[test]
fn quarter_cylinder_at_delta_1e_8_keeps_every_faces_payload_invariants() {
    let faces = column(
        "quarter_cylinder",
        &quarter_cylinder(1.0, 1.0),
        1e-8,
        [
            ("not-finite", f64::NAN),
            ("not-finite", f64::NAN),
            ("not-finite", f64::NAN),
            ("not-finite", f64::NAN),
            ("not-finite", f64::NAN),
        ],
    );
    assert!(
        faces.not_finite_none > 0,
        "this column is the corpus's witness of a not-finite bound with no finite \
         round behind it, and it no longer reaches that face"
    );
}

/// Every face this column reaches keeps its own payload invariants,
/// and the table is this column's row of the corpus census.
#[test]
fn quarter_cylinder_at_delta_1e_7_keeps_every_faces_payload_invariants() {
    column(
        "quarter_cylinder",
        &quarter_cylinder(1.0, 1.0),
        1e-7,
        [
            ("cap", 5.8549822e-7),
            ("cap", 5.8549822e-7),
            ("cap", 5.8549822e-7),
            ("certified", 5.8549822e-7),
            ("certified", 5.8549822e-7),
        ],
    );
}

/// Every face this column reaches keeps its own payload invariants,
/// the table is this column's row of the corpus census, and this is
/// the corpus's witness of the sample-cap face.
#[test]
fn quarter_cylinder_at_delta_1e_6_keeps_every_faces_payload_invariants() {
    let faces = column(
        "quarter_cylinder",
        &quarter_cylinder(1.0, 1.0),
        1e-6,
        [
            ("cap", 3.7544249e-7),
            ("cap", 3.7544249e-7),
            ("cap", 3.7544249e-7),
            ("certified", 3.7544249e-7),
            ("certified", 1.7071974e-5),
        ],
    );
    assert!(
        faces.cap > 0,
        "this column is the corpus's witness of the sample cap, and it no longer \
         reaches that face"
    );
}

/// Every face this column reaches keeps its own payload invariants,
/// and the table is this column's row of the corpus census.
#[test]
fn quarter_cylinder_at_delta_1e_5_keeps_every_faces_payload_invariants() {
    column(
        "quarter_cylinder",
        &quarter_cylinder(1.0, 1.0),
        1e-5,
        [
            ("budget", 3.6255804e-7),
            ("budget", 3.6255804e-7),
            ("budget", 3.6255804e-7),
            ("certified", 3.6255804e-7),
            ("certified", 7.1460400e-6),
        ],
    );
}

/// Every face this column reaches keeps its own payload invariants,
/// and the table is this column's row of the corpus census.
#[test]
fn quarter_cylinder_at_delta_0_01_keeps_every_faces_payload_invariants() {
    column(
        "quarter_cylinder",
        &quarter_cylinder(1.0, 1.0),
        0.01,
        [
            ("budget", 3.6726212e-7),
            ("budget", 3.6726212e-7),
            ("budget", 3.6726212e-7),
            ("certified", 3.6726212e-7),
            ("certified", 6.5181412e-5),
        ],
    );
}

/// Every face this column reaches keeps its own payload invariants,
/// and the table is this column's row of the corpus census.
#[test]
fn quarter_cylinder_at_delta_0_05_keeps_every_faces_payload_invariants() {
    column(
        "quarter_cylinder",
        &quarter_cylinder(1.0, 1.0),
        0.05,
        [
            ("budget", 3.8180980e-7),
            ("budget", 3.8180980e-7),
            ("budget", 3.8180980e-7),
            ("certified", 3.8180980e-7),
            ("certified", 6.7531657e-5),
        ],
    );
}

/// Every face this column reaches keeps its own payload invariants,
/// and the table is this column's row of the corpus census.
#[test]
fn quarter_cylinder_at_delta_minus_0_05_keeps_every_faces_payload_invariants() {
    column(
        "quarter_cylinder",
        &quarter_cylinder(1.0, 1.0),
        -0.05,
        [
            ("budget", 3.4544881e-7),
            ("budget", 3.4544881e-7),
            ("budget", 3.4544881e-7),
            ("certified", 3.4544881e-7),
            ("certified", 6.1050669e-5),
        ],
    );
}

/// Every face this column reaches keeps its own payload invariants,
/// and the table is this column's row of the corpus census.
#[test]
fn bumpy_at_delta_1e_8_keeps_every_faces_payload_invariants() {
    column(
        "bumpy",
        &bumpy_patch(),
        1e-8,
        [
            ("not-finite", f64::NAN),
            ("not-finite", f64::NAN),
            ("not-finite", f64::NAN),
            ("not-finite", f64::NAN),
            ("not-finite", f64::NAN),
        ],
    );
}

/// Every face this column reaches keeps its own payload invariants,
/// and the table is this column's row of the corpus census.
#[test]
fn bumpy_at_delta_1e_7_keeps_every_faces_payload_invariants() {
    column(
        "bumpy",
        &bumpy_patch(),
        1e-7,
        [
            ("cap", 3.3545915e-7),
            ("cap", 3.3545915e-7),
            ("cap", 3.3545915e-7),
            ("certified", 3.3545915e-7),
            ("certified", 3.3545915e-7),
        ],
    );
}

/// Every face this column reaches keeps its own payload invariants,
/// and the table is this column's row of the corpus census.
#[test]
fn bumpy_at_delta_1e_6_keeps_every_faces_payload_invariants() {
    column(
        "bumpy",
        &bumpy_patch(),
        1e-6,
        [
            ("cap", 2.4521974e-7),
            ("cap", 2.4521974e-7),
            // Re-pinned when the C9 ring became a newtype over the
            // backend (`7.6102157e-10` before): `cell_bound`'s
            // assembly loses the ring's unconditional one-step
            // outward pad per operation, so the same certificate on
            // the same cells comes in a third tighter. The FACE each
            // cell wears is unmoved, which is what this census counts.
            ("certified", 5.0593136e-10),
            ("certified", 5.0593136e-10),
            ("certified", 3.4386399e-6),
        ],
    );
}

/// Every face this column reaches keeps its own payload invariants,
/// and the table is this column's row of the corpus census.
#[test]
fn bumpy_at_delta_1e_5_keeps_every_faces_payload_invariants() {
    column(
        "bumpy",
        &bumpy_patch(),
        1e-5,
        [
            // Re-pinned for the reason the δ = 1e-6 column gives
            // (`1.6337241e-7` and `8.3373901e-10` before): the ring's
            // unconditional pad is gone from `cell_bound`, and the
            // cap face's achieved bound moves with the certified one
            // because it is the same assembly read one round short.
            ("cap", 2.5343499e-8),
            ("cap", 2.5343499e-8),
            ("certified", 5.6830092e-10),
            ("certified", 5.6830092e-10),
            ("certified", 3.0299228e-5),
        ],
    );
}

/// Every face this column reaches keeps its own payload invariants,
/// and the table is this column's row of the corpus census.
#[test]
fn bumpy_at_delta_0_01_keeps_every_faces_payload_invariants() {
    column(
        "bumpy",
        &bumpy_patch(),
        0.01,
        [
            ("cap", 1.4792482e-7),
            ("cap", 1.4792482e-7),
            ("cap", 1.4792482e-7),
            ("certified", 6.0162261e-7),
            ("certified", 1.9331467e-5),
        ],
    );
}

/// Every face this column reaches keeps its own payload invariants,
/// and the table is this column's row of the corpus census.
#[test]
fn bumpy_at_delta_0_05_keeps_every_faces_payload_invariants() {
    column(
        "bumpy",
        &bumpy_patch(),
        0.05,
        [
            ("cap", 6.5406620e-7),
            ("cap", 6.5406620e-7),
            ("cap", 6.5406620e-7),
            ("certified", 6.5406620e-7),
            ("certified", 4.0378183e-5),
        ],
    );
}

/// Every face this column reaches keeps its own payload invariants,
/// and the table is this column's row of the corpus census.
#[test]
fn bumpy_at_delta_minus_0_05_keeps_every_faces_payload_invariants() {
    column(
        "bumpy",
        &bumpy_patch(),
        -0.05,
        [
            ("cap", 6.5784386e-7),
            ("cap", 6.5784386e-7),
            ("cap", 6.5784386e-7),
            ("certified", 6.5784386e-7),
            ("certified", 4.3826144e-5),
        ],
    );
}

/// Each message names what stopped the loop and the repair the face's
/// doc claims, and no other: the budget and the cap are told apart by
/// what they name, the budget's two readings say which one it was, the
/// repair is sized to the best bound rather than the last, and the two
/// not-finite cases send the caller to two different repairs.
#[test]
fn each_faces_message_names_its_lever() {
    let budget = |still_falling| {
        OffsetFitError::BudgetExhausted {
            budget: OFFSET_FIT_BUDGET,
            grid: (9, 9),
            achieved: 2e-3,
            tolerance: 1e-9,
            still_falling,
            best: 1e-3,
            best_grid: (7, 9),
        }
        .to_string()
    };
    let (falling, rose) = (budget(true), budget(false));
    for b in [&falling, &rose] {
        assert!(
            b.contains(&format!("all {OFFSET_FIT_BUDGET} refinement rounds")),
            "{b}"
        );
        assert!(!b.contains("samples"), "{b}");
        assert!(b.contains("loosen the tolerance to 0.001 m"), "{b}");
        assert!(!b.contains("0.002"), "the last bound is named: {b}");
    }
    assert!(falling.contains("while still improving"), "{falling}");
    assert!(
        !rose.contains("improving,") && rose.contains("did not improve on the one before"),
        "{rose}"
    );
    let c = OffsetFitError::SampleCapReached {
        cap: OFFSET_FIT_SAMPLE_CAP,
        rounds: 5,
        grid: (41, 40),
        achieved: 2e-3,
        tolerance: 1e-9,
        best: 1e-3,
        best_grid: (33, 40),
    }
    .to_string();
    assert!(
        c.contains(&format!(
            "limit of {OFFSET_FIT_SAMPLE_CAP} samples per direction"
        )),
        "{c}"
    );
    assert!(!c.contains("rounds"), "{c}");
    assert!(c.contains("loosen the tolerance to 0.001 m"), "{c}");
    assert!(!c.contains("0.002"), "the last bound is named: {c}");
    let n = OffsetFitError::BoundNotFinite {
        rounds: 4,
        grid: (25, 17),
        d: 1e-7,
        tolerance: 1e-3,
        best_finite: None,
    }
    .to_string();
    assert!(n.contains("offset distance of 0.0000001 m"), "{n}");
    assert!(
        n.contains("Recourse: use an offset distance of larger magnitude"),
        "{n}"
    );
    // The never-finite message names no knob of the loop, and no
    // tolerance to loosen to: there is no bound to size one against.
    assert!(
        !n.contains("rounds") && !n.contains("samples") && !n.contains("loosen"),
        "{n}"
    );
    let l = OffsetFitError::BoundNotFinite {
        rounds: 4,
        grid: (25, 17),
        d: 1e-7,
        tolerance: 1e-3,
        best_finite: Some(3.2e-4),
    }
    .to_string();
    assert!(
        l.contains("Recourse: loosen the tolerance to 0.00032 m or more"),
        "the lost bound is not what the repair is sized to: {l}"
    );
    assert!(!l.contains("offset distance"), "{l}");
    assert_ne!(n, l, "the two not-finite cases must read differently");
}

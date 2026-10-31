//! **Straight edges kilometres long on one line read past their ends'
//! rounding, at the result gate and at validation alike.**
//!
//! The census reads two collinear edges' overlap from their ends'
//! projections on one line (`geom_core::one_line`). Where the spans are
//! long enough that the projections' rounding passes the band's room
//! (`(K − 1)·ε`), it reads each end comparison at its value less and
//! plus that rounding. Where the two bounds agree it decides; where they
//! straddle the band it escalates, and that escalation is the precision
//! limit at the edges' length, never a near-coincidence: its recourse is
//! shorter edges, never a move apart.
//!
//! The rows:
//! - two 3 km bars stacked with a half-length overlap, turned about z,
//!   whose union builds at every ε;
//! - a cube on a 6 km bar, whose union's collinear edges share vertices
//!   past the threshold, and builds at every ε;
//! - two 3 km bars meeting end to end, the second's start a few `10⁻¹¹` m
//!   back along the line: in band past the rounding at `ε = 10⁻¹²`, so
//!   the union refuses on the precision and validation of the two bars
//!   as one body says the same.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use common::meeting::{Pose, posed_box};
use common::{FaceGeometry, describe_as_intersections, prism_ops};
use geom_core::Tol;
use topo::{
    Body, BooleanDecision, BooleanError, ContactRecords, ValidationError, union,
    validate_geometric, validate_pseudomanifold,
};

fn t() -> Tol {
    Tol::witness()
}

/// The precision limit's recourse, as every door it reaches says it.
const RECOURSE: &str = "make the straight edges along that line shorter, splitting them where \
                        they run long";

/// The turn the bars stand at: about z, off every axis.
fn turned() -> Pose {
    Pose::turn("turned 0.65 rad about z", [0.0, 0.0, 1.0], 0.65, [0.0; 3])
}

/// **Two 3 km bars sharing half their length build their union.** The
/// bars `[0, 3000] × [0, 1] × [0, 1]` and `[1500, 4500] × [0, 1] × [1, 2]`,
/// turned 0.65 rad about z, meet along the top face of the first, and
/// two of their edges lie on one line along `1500` m. At `ε = 10⁻¹²`
/// their ends' projections carry more rounding than the band's room, so
/// the census reads them charged, and decides: the overlap is metres
/// past its charge. The union is one solid of volume `6000`.
#[test]
fn two_long_bars_overlapping_by_half_their_length_build_their_union() {
    // Pinned to limb 2's kernel limit at ε = 1e-12: the kilometre bars'
    // edges, outside the session box
    // (`work/offset/the-session-box-is-not-enforced-at-construction.md`),
    // measure their ends against their planes in certification arithmetic
    // at `[0, about 1e-12]`, the data's own rounding at 3 to 6 km — an
    // enclosure reaching zero, so only an upper bound, and the
    // composite's stall stands.
    test_utils::kernel_limit::pinned_row(
        geom_core::Tol::witness().eps(),
        &[1e-12],
        "two_long_bars_overlapping_by_half_their_length_build_their_union",
        || {
            let tol = t();
            let pose = turned();
            let a = posed_box(
                "the first bar",
                [(0.0, 3000.0), (0.0, 1.0), (0.0, 1.0)],
                &pose,
                tol,
            );
            let b = posed_box(
                "the second bar",
                [(1500.0, 4500.0), (0.0, 1.0), (1.0, 2.0)],
                &pose,
                tol,
            );
            let built = union(&a, &b, tol).unwrap_or_else(|e| panic!("the union refused: {e}"));
            let body = &built.body().expect("a body").body;
            let volume = topo::test_support::pinned_volume("the measured body", body, tol).volume;
            assert!((volume - 6000.0).abs() < 1e-6, "volume {volume}");
            validate_geometric(body, tol).unwrap_or_else(|e| panic!("the union is invalid: {e:?}"));
        },
    );
}

/// **A cube on a 6 km bar builds its union.** The bar
/// `[0, 6000] × [0, 1] × [0, 1]` and the cube `[2000, 2001] × [0, 1] × [1, 2]`
/// on its top, turned 0.65 rad about z: the cube splits the bar's top
/// edges, so the union holds collinear edges that share vertices, their
/// summed length past the census threshold at `ε = 10⁻¹²`. The shared
/// ends read as one point by their point keys, not by their rounded
/// coordinates, so the union is one solid of volume `6001`.
#[test]
fn a_cube_on_a_six_kilometre_bar_builds_its_union() {
    // Pinned to limb 2's kernel limit at ε = 1e-12: the kilometre bars'
    // edges, outside the session box
    // (`work/offset/the-session-box-is-not-enforced-at-construction.md`),
    // measure their ends against their planes in certification arithmetic
    // at `[0, about 1e-12]`, the data's own rounding at 3 to 6 km — an
    // enclosure reaching zero, so only an upper bound, and the
    // composite's stall stands.
    test_utils::kernel_limit::pinned_row(
        geom_core::Tol::witness().eps(),
        &[1e-12],
        "a_cube_on_a_six_kilometre_bar_builds_its_union",
        || {
            let tol = t();
            let pose = turned();
            let bar = posed_box(
                "the bar",
                [(0.0, 6000.0), (0.0, 1.0), (0.0, 1.0)],
                &pose,
                tol,
            );
            let cube = posed_box(
                "the cube",
                [(2000.0, 2001.0), (0.0, 1.0), (1.0, 2.0)],
                &pose,
                tol,
            );
            let built =
                union(&bar, &cube, tol).unwrap_or_else(|e| panic!("the union refused: {e}"));
            let body = &built.body().expect("a body").body;
            let volume = topo::test_support::pinned_volume("the measured body", body, tol).volume;
            assert!((volume - 6001.0).abs() < 1e-6, "volume {volume}");
            validate_geometric(body, tol).unwrap_or_else(|e| panic!("the union is invalid: {e:?}"));
        },
    );
}

/// The second bar's start, `back` metres before the first bar's end
/// along the line, both bars posed by [`turned`].
fn end_to_end(back: f64) -> [[(f64, f64); 3]; 2] {
    [
        [(0.0, 3000.0), (0.0, 1.0), (0.0, 1.0)],
        [(3000.0 - back, 6000.0), (0.0, 1.0), (1.0, 2.0)],
    ]
}

/// How far back the second bar starts: inside the rounding of the bars'
/// ends at `ε = 10⁻¹²` (about `2·10⁻¹¹` m at their length), and past
/// the band's zero.
const BACK: f64 = 2e-11;

/// Stands the row down where ε is wider than `10⁻¹²`: there the bars'
/// rounding is inside the band's room, so the census reads them
/// uncharged, and the overlap is a coincidence of the geometry.
fn stood_down(tol: Tol) -> bool {
    if tol.eps() > 1e-12 {
        test_utils::vacuity::stood_down(
            "eps too wide for the bars' rounding",
            "two 3 km bars' ends round inside the band's room above 1e-12, so the census reads \
             them uncharged",
        );
        return true;
    }
    false
}

/// **At the result gate, an overlap inside the rounding refuses on the
/// precision.** The union of [`end_to_end`]`(`[`BACK`]`)` refuses
/// `Escalated` under `BooleanDecision::LineSpanPrecision`, and its
/// message names the precision limit's recourse, not a move apart.
#[test]
fn an_overlap_inside_the_rounding_refuses_at_the_result_gate_on_the_precision() {
    // Pinned to limb 2's kernel limit at ε = 1e-12: the kilometre bars'
    // edges, outside the session box
    // (`work/offset/the-session-box-is-not-enforced-at-construction.md`),
    // measure their ends against their planes in certification arithmetic
    // at `[0, about 1e-12]`, the data's own rounding at 3 to 6 km — an
    // enclosure reaching zero, so only an upper bound, and the
    // composite's stall stands.
    test_utils::kernel_limit::pinned_row(
        geom_core::Tol::witness().eps(),
        &[1e-12],
        "an_overlap_inside_the_rounding_refuses_at_the_result_gate_on_the_precision",
        || {
            let tol = t();
            if stood_down(tol) {
                return;
            }
            let pose = turned();
            let [ba, bb] = end_to_end(BACK);
            let a = posed_box("the first bar", ba, &pose, tol);
            let b = posed_box("the second bar", bb, &pose, tol);
            let refused = union(&a, &b, tol).expect_err("the union refuses");
            let said = refused.to_string();
            assert!(
                matches!(
                    refused,
                    BooleanError::Escalated {
                        decision: BooleanDecision::LineSpanPrecision,
                        ..
                    }
                ),
                "{refused:?}"
            );
            assert!(said.contains(RECOURSE), "{said}");
            assert!(!said.contains("apart"), "{said}");
        },
    );
}

/// **At validation, the same two bars as one body escalate on the
/// precision.** The bars of [`end_to_end`]`(`[`BACK`]`)` built as the two
/// solids of one body, with no contact records: tier 3′'s census
/// (`validate_pseudomanifold`) escalates `CensusEscalated` on the two
/// lines their edges share, and its message names the same recourse,
/// not the coincidence menu. (The body refuses on its unrecorded
/// contacts too; the row reads only the census's precision findings.)
#[test]
fn an_overlap_inside_the_rounding_escalates_at_validation_on_the_precision() {
    // Pinned to limb 2's kernel limit at ε = 1e-12: the kilometre bars'
    // edges, outside the session box
    // (`work/offset/the-session-box-is-not-enforced-at-construction.md`),
    // measure their ends against their planes in certification arithmetic
    // at `[0, about 1e-12]`, the data's own rounding at 3 to 6 km — an
    // enclosure reaching zero, so only an upper bound, and the
    // composite's stall stands.
    test_utils::kernel_limit::pinned_row(
        geom_core::Tol::witness().eps(),
        &[1e-12],
        "an_overlap_inside_the_rounding_escalates_at_validation_on_the_precision",
        || {
            let tol = t();
            if stood_down(tol) {
                return;
            }
            let pose = turned();
            let mut body = Body::<f64>::new();
            for [(x0, x1), (y0, y1), z] in end_to_end(BACK) {
                prism_ops(
                    &mut body,
                    &[(x0, y0), (x1, y0), (x1, y1), (x0, y1)],
                    z,
                    |x, y, z| pose.at([x, y, z]),
                    FaceGeometry::Certified,
                    tol,
                );
            }
            describe_as_intersections(&mut body, tol);
            let errors = validate_pseudomanifold(&body, &ContactRecords::default(), tol)
                .expect_err("validation escalates");
            let precision: Vec<String> = errors
                .iter()
                .filter(|e| matches!(e, ValidationError::CensusEscalated { .. }))
                .map(ToString::to_string)
                .filter(|said| said.contains(RECOURSE))
                .collect();
            assert!(!precision.is_empty(), "{errors:?}");
            for said in &precision {
                assert!(!said.contains("apart"), "{said}");
            }
        },
    );
}

/// **At validation, bars that meet exactly read their touch as one point.**
/// The bars of [`end_to_end`]`(0)`, turned as above, as the two solids of
/// one body at `ε = 10⁻¹²`: the second bar's start vertices are distinct
/// vertices at the first bar's end vertices' bit-identical stored
/// coordinates. The census reads each touch through its coincidence
/// reading of the two stored vertices, not through their projections,
/// so no precision escalation is raised, as the same bars' union builds.
/// (The body still refuses on its unrecorded contacts, which is what an
/// undeclared touch of two solids is.)
#[test]
fn bars_meeting_exactly_raise_no_precision_escalation_at_validation() {
    // Pinned to limb 2's kernel limit at ε = 1e-12: the kilometre bars'
    // edges, outside the session box
    // (`work/offset/the-session-box-is-not-enforced-at-construction.md`),
    // measure their ends against their planes in certification arithmetic
    // at `[0, about 1e-12]`, the data's own rounding at 3 to 6 km — an
    // enclosure reaching zero, so only an upper bound, and the
    // composite's stall stands.
    test_utils::kernel_limit::pinned_row(
        geom_core::Tol::witness().eps(),
        &[1e-12],
        "bars_meeting_exactly_raise_no_precision_escalation_at_validation",
        || {
            let tol = t();
            if stood_down(tol) {
                return;
            }
            let pose = turned();
            let mut body = Body::<f64>::new();
            for [(x0, x1), (y0, y1), z] in end_to_end(0.0) {
                prism_ops(
                    &mut body,
                    &[(x0, y0), (x1, y0), (x1, y1), (x0, y1)],
                    z,
                    |x, y, z| pose.at([x, y, z]),
                    FaceGeometry::Certified,
                    tol,
                );
            }
            describe_as_intersections(&mut body, tol);
            let errors = validate_pseudomanifold(&body, &ContactRecords::default(), tol)
                .expect_err("the undeclared touch refuses");
            let precision: Vec<String> = errors
                .iter()
                .filter(|e| matches!(e, ValidationError::CensusEscalated { .. }))
                .map(ToString::to_string)
                .filter(|said| said.contains(RECOURSE))
                .collect();
            assert!(precision.is_empty(), "{precision:?}");
        },
    );
}

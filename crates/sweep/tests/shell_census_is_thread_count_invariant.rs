//! **The per-shell census's verdict channel on a voided body.**
//!
//! `topo::classify_shells_of` restricts the REPORTING face walk to one
//! shell's faces and sums that shell's contributions in the shell's own
//! face order. That walk is SERIAL, and deliberately so: every shell
//! the census meets is below the per-face map's break-even
//! (`work/perf/parallel-map-costs-a-fixed-price-on-a-cheap-body.md`
//! carries the numbers, and `topo::props`' `decide_faces_serially`
//! states the reason at the door). The row here pins what the census
//! records on `voided_rod`, order-free.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common::cavity::{brick, rod};
use geom_core::k_stats::Bracket;
use geom_core::{Point2, Point3, Tol};
use sweep::test_support::finished;
use topo::{Body, BooleanResult, BooleanResultKind};

/// **A brick with a rod-shaped cavity strictly inside it**: one solid,
/// two shells, and the void shell is the rod's boundary reverted — a
/// cylinder wall and two discs, every one of them a CURVED face whose
/// closed form decides predicates of its own.
///
/// **Its cavity answers in CLOSED FORM, not on the quadrature lane**,
/// and no cavity this corpus can carve does: this wall is iso-trimmed,
/// an extruded bulge's is too, and the boolean engine refuses a lofted
/// operand's spline edges wherever they meet the other operand
/// (`EdgeCarrierUnsupported`).
fn voided_rod() -> Body<f64> {
    let a = finished(
        "the brick",
        brick(Point3::new(0.0, 0.0, 0.0), Point3::new(3.0, 3.0, 3.0)),
        Tol::witness(),
    );
    let b = finished(
        "the rod",
        rod(Point2::new(1.5, 1.5), 0.5, 1.0, 2.0),
        Tol::witness(),
    );
    let BooleanResult::Body(bb) = topo::subtract(&a, &b, Tol::witness()).expect("the cut runs")
    else {
        panic!("a rod strictly inside the brick leaves a voided body")
    };
    assert_eq!(bb.kind, BooleanResultKind::Voided);
    bb.body.into_body()
}

/// **`voided_rod`'s verdicts as a SORTED multiset.** A digest that
/// hashes the verdict stream in DECISION order (`common::channels`)
/// cannot tell a verdict whose sign changed from one that merely
/// moved; this row pins each `(predicate, sign)` with its count,
/// order-free.
///
/// None of them is anchor-relative. The rod's walls are cylinder
/// faces, whose flux is the chart Green form `−∮ v du`: it takes the
/// material side from the loops' own traversal, so the rim-side
/// reading (`props_rim_side`, the sign of `lo + hi − 2·level` on
/// whichever rim the walk from `Cycle::first` meets first) and the
/// iso-rectangle premises it rests on are not run on this body at all.
/// What the Green form does run is its closure premise, which makes the
/// sum anchor-free: every loop closes on the chart at each of its edge
/// junctions (`props_chart_junction_closed`), and turns about the
/// cylinder a whole number of times, here none
/// (`props_chart_loop_turns` Positive: short of a half turn, and
/// `props_chart_loop_whole` Zero: within the slack of none) —
/// per-junction and per-loop facts, whichever edge a walk starts at.
///
/// Each shell's sums are measured once (`props_*`), and its role is read
/// off rays aimed at its faces, every decision of which is recorded
/// under the reader's family (`side_at_infinity_*`, the ray walk's rows
/// among them); no volume sign is read.
#[test]
fn voided_rods_verdicts_as_a_sorted_multiset() {
    let body = topo::test_support::finished("body", voided_rod(), Tol::witness());
    let bracket = Bracket::open();
    let _ = topo::classify_shells(&body, Tol::witness());
    let log = bracket.finish();
    let mut got: Vec<(String, usize)> = Vec::new();
    for v in &log.verdicts {
        let key = format!("{} {:?}", v.predicate, v.sign);
        match got.iter_mut().find(|(k, _)| *k == key) {
            Some((_, n)) => *n += 1,
            None => got.push((key, 1)),
        }
    }
    got.sort();
    let want: Vec<(String, usize)> = [
        ("props_chart_axis_inside Positive", 2),
        ("props_chart_junction_closed Zero", 8),
        ("props_chart_loop_turns Positive", 2),
        ("props_chart_loop_whole Zero", 2),
        ("props_circle_axis_class Positive", 4),
        ("props_cylinder_circle_off_axis Zero", 4),
        ("props_cylinder_circle_tilt Zero", 4),
        ("props_face_extent Positive", 2),
        ("props_meridian_axial Zero", 4),
        ("props_meridian_on_surface Zero", 4),
        ("props_rim_axis_parallel Zero", 4),
        ("props_rim_center_on_axis Zero", 4),
        ("props_rim_fit Zero", 4),
        ("props_rim_span_forward Positive", 4),
        ("props_rim_span_winding Positive", 4),
        ("side_at_infinity_advance Positive", 4),
        ("side_at_infinity_bool_point_in_solid_denom Positive", 2),
        ("side_at_infinity_bool_ray_cylinder_disc Positive", 2),
        ("side_at_infinity_bool_wall_turn_true Positive", 16),
        ("side_at_infinity_curve_carrier_clear Positive", 20),
        ("side_at_infinity_denom Negative", 4),
        ("side_at_infinity_denom Positive", 4),
        ("side_at_infinity_edge Positive", 12),
        ("side_at_infinity_order Negative", 2),
        ("side_at_infinity_order Positive", 1),
        ("side_at_infinity_pm_census_curved_speed_floor Positive", 6),
        ("side_at_infinity_pm_census_curved_speed_hi Positive", 6),
        ("side_at_infinity_vertex Positive", 12),
    ]
    .into_iter()
    .map(|(k, n)| (k.to_string(), n))
    .collect();
    assert_eq!(
        got, want,
        "voided_rod's verdict multiset moved: a sign changed or a predicate came or went \
         (an ORDER change alone does not reach this row)"
    );
}

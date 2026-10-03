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
use topo::{Body, BooleanResult, BooleanResultKind};

/// **A brick with a rod-shaped cavity strictly inside it**: one solid,
/// two shells, and the void shell is the rod's boundary reverted — a
/// cylinder wall and two discs, every one of them a CURVED face whose
/// closed form decides predicates of its own.
///
/// **Its cavity answers in CLOSED FORM, not on the quadrature lane**,
/// and no cavity this corpus can carve does: this wall is iso-trimmed,
/// an extruded bulge's is too, and the boolean engine refuses a lofted
/// operand outright (`CurvedEdgeUnsupported`).
fn voided_rod() -> Body<f64> {
    let a = brick(Point3::new(0.0, 0.0, 0.0), Point3::new(3.0, 3.0, 3.0));
    let b = rod(Point2::new(1.5, 1.5), 0.5, 1.0, 2.0);
    let BooleanResult::Body(bb) = topo::subtract(&a, &b, Tol::witness()).expect("the cut runs")
    else {
        panic!("a rod strictly inside the brick leaves a voided body")
    };
    assert_eq!(bb.kind, BooleanResultKind::Voided);
    bb.body
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
/// sum anchor-free: every loop closes at each of its eight edge
/// junctions (`props_loop_closed`), and each wall's loops wind the
/// cylinder zero times (`props_chart_loops_closed`) — per-junction and
/// per-face facts, whichever edge a walk starts at.
#[test]
fn voided_rods_verdicts_as_a_sorted_multiset() {
    let body = voided_rod();
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
        ("chk_shell_volume_sign Negative", 1),
        ("chk_shell_volume_sign Positive", 1),
        ("props_chart_loops_closed Zero", 2),
        ("props_circle_axis_class Positive", 4),
        ("props_face_extent Positive", 2),
        ("props_loop_closed Zero", 8),
        ("props_meridian_axial Zero", 4),
        ("props_meridian_on_surface Zero", 4),
        ("props_rim_axis_parallel Zero", 4),
        ("props_rim_center_on_axis Zero", 4),
        ("props_rim_fit Zero", 4),
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

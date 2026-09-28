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
/// ONE of the twelve predicates is ANCHOR-RELATIVE by construction,
/// and its sign is a fact about cycle order rather than about the
/// body: `props_rim_side` is the sign of `lo + hi − 2·level` on
/// whichever rim the loop walk from `Cycle::first` meets FIRST
/// (`geom_brep`'s `props/curved.rs`, `linear_rim_side`'s `side`). The
/// flux compensates (`Positive ⇒ d_u_sign`, `Negative ⇒ flip`), so
/// the readings do not depend on the anchor while that sign does —
/// the void shell here is the rod REVERTED, and `Body::revert` moves
/// every loop's anchor to its source predecessor, which is why it
/// reads `Positive` on this tree and `Negative` on one whose reversal
/// kept the anchor. The other eleven are per-rim, per-meridian or
/// per-face facts and count the same whichever rim comes first.
///
/// It was two. `props_rim_dir_group` compared each rim's traversal
/// direction against that same first rim's, through a `Margin` over
/// two values that are `±1` by construction; the direction is a
/// discrete sign now and is compared as one, so that predicate
/// records nothing and the multiset below is one row shorter.
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
        ("props_circle_axis_class Positive", 4),
        ("props_du_consistent Zero", 2),
        ("props_face_extent Positive", 2),
        ("props_meridian_axial Zero", 4),
        ("props_meridian_on_surface Zero", 4),
        ("props_rim_axis_parallel Zero", 4),
        ("props_rim_center_on_axis Zero", 4),
        ("props_rim_fit Zero", 4),
        ("props_rim_level Zero", 4),
        ("props_rim_level_group Positive", 2),
        ("props_rim_side Positive", 2),
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

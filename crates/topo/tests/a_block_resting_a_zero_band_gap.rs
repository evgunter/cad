//! **A block resting a Zero-band gap on another, through every op, in
//! both orders.** `A = [0,1]³` and a brick `B` whose bottom stands `g`
//! off `A`'s top (a gap, an overlap, or a few ulps), over three
//! footprints; and a block with a parallelepiped standing on its top,
//! the tool's bottom tilted up by `θ` about the edge it rests on.
//!
//! Where the glue door reads the pair one carrier at Zero, every op
//! builds, and its volume stands off the box arithmetic by no more than
//! the volume the glue admits: the band's zero over the glued faces
//! but the smallest (`volume_backstop`'s "What the glue admits",
//! `crates/topo/src/boolean/ops.rs`). Where it does not, an op builds
//! within rounding of the box arithmetic or escalates an in-band
//! reading. No cell refuses `ResultVolumeImplausible`: each op's
//! result is correct, so a refusal at the backstop would be a correct
//! body refused.

#![allow(clippy::expect_used, clippy::panic)]

use geom_core::{Band, Point3, Tol, Vec3};
use topo::test_support::{brick, face_facing, finished, mapped_cube};
use topo::{AtRestBody, BooleanError, BooleanResult, PlaneRelation};

/// One op's label, its outcome and the box arithmetic's volume.
type Cell = (&'static str, Result<BooleanResult<f64>, BooleanError>, f64);

/// A footprint's label and its `x` and `y` spans.
type Footprint = (&'static str, (f64, f64), (f64, f64));

/// The six ops of `a` and `b`, labelled, with the box arithmetic's
/// volume of each: `[vol(A ∪ B), vol(A ∩ B), vol(A ∖ B), vol(B ∖ A)]`.
fn six(a: &AtRestBody<f64>, b: &AtRestBody<f64>, want: [f64; 4], tol: Tol) -> [Cell; 6] {
    let [union, common, a_less, b_less] = want;
    [
        ("A ∪ B", topo::union(a, b, tol), union),
        ("B ∪ A", topo::union(b, a, tol), union),
        ("A ∩ B", topo::intersect(a, b, tol), common),
        ("B ∩ A", topo::intersect(b, a, tol), common),
        ("A ∖ B", topo::subtract(a, b, tol), a_less),
        ("B ∖ A", topo::subtract(b, a, tol), b_less),
    ]
}

/// What a pose must answer, every op: `glued` says whether the glue
/// door reads `a`'s top and `b`'s bottom one carrier at Zero, and
/// `admitted` is the volume that glue may move a correct result by.
/// Answers how many ops built.
fn holds(
    label: &str,
    a: &AtRestBody<f64>,
    b: &AtRestBody<f64>,
    want: [f64; 4],
    admitted: f64,
    tol: Tol,
) -> usize {
    let band = Band::linear(tol).expect("band");
    let (top, bottom) = (face_facing(a, 1.0), face_facing(b, -1.0));
    let glued = matches!(
        topo::flush_pair_relation(a, top, b, bottom, false, band),
        Ok(Ok(PlaneRelation::SameOpposite))
    );
    let mut built = 0;
    for (op, result, want) in six(a, b, want, tol) {
        let label = format!("{label}, {op} (glued: {glued})");
        let volume = match result {
            Ok(BooleanResult::Body(r)) => {
                topo::mass_properties(&r.body, tol)
                    .unwrap_or_else(|e| panic!("{label}: measures: {e:?}"))
                    .volume
            }
            Ok(BooleanResult::Empty) => 0.0,
            Err(e) => {
                assert!(
                    !glued,
                    "{label}: a pair the glue reads Zero builds every op: {e:?}"
                );
                assert!(
                    matches!(e, BooleanError::Escalated { .. }),
                    "{label}: an unglued pose escalates its in-band reading: {e:?}"
                );
                continue;
            }
        };
        built += 1;
        let rounding = 64.0 * f64::EPSILON * want.max(1.0);
        let slack = if glued { admitted + rounding } else { rounding };
        assert!(
            (volume - want).abs() <= slack,
            "{label}: volume {volume} against the box arithmetic's {want} \
             (off by {:e}, admitted {slack:e})",
            volume - want
        );
    }
    built
}

/// `B`'s footprints: inside `A`'s top, all of it, and half off it.
const FOOTPRINTS: [Footprint; 3] = [
    ("inside", (0.25, 0.75), (0.25, 0.75)),
    ("flush", (0.0, 1.0), (0.0, 1.0)),
    ("half off", (-0.5, 0.5), (0.25, 0.75)),
];

/// `B`'s bottom, as `(label, z)`: the row's gaps and overlaps, the same
/// as fractions of the band's zero, and `1.0` plus a few ulps.
fn bottoms(zero: f64) -> Vec<(String, f64)> {
    let mut out: Vec<(String, f64)> = [
        0.0, 1e-15, 1e-13, 1e-12, 1e-11, 1e-10, 5e-10, -1e-12, -1e-10,
    ]
    .iter()
    .map(|g| (format!("g = {g:e}"), 1.0 + g))
    .collect();
    for k in [0.25, 0.5, 0.9, 0.99, -0.5] {
        out.push((format!("g = {k}·zero"), 1.0 + k * zero));
    }
    for n in [1, 2, 4, 16] {
        out.push((format!("{n} ulp"), f64::from_bits(1.0_f64.to_bits() + n)));
    }
    out
}

/// **The row's table**: every footprint, every bottom, every op in
/// both orders. A gap inside the zero band builds every op.
#[test]
fn a_block_resting_a_zero_band_gap_builds_every_op() {
    let tol = Tol::witness();
    let zero = Band::linear(tol).expect("band").zero();
    let a = finished(
        "a",
        brick::<f64>((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol),
        tol,
    );
    for (name, x, y) in FOOTPRINTS {
        let area = (x.1 - x.0) * (y.1 - y.0);
        let lap = (x.1.min(1.0) - x.0.max(0.0)) * (y.1.min(1.0) - y.0.max(0.0));
        for (gap, z) in bottoms(zero) {
            let b = finished("b", brick::<f64>(x, y, (z, 2.0), tol), tol);
            let (vb, common) = (area * (2.0 - z), lap * (1.0 - z).max(0.0));
            let want = [1.0 + vb - common, common, 1.0 - common, vb - common];
            // The glued group is A's top and B's bottom.
            let admitted = zero * (1.0 + area - area.min(1.0));
            let label = format!("{name}, {gap}");
            let built = holds(&label, &a, &b, want, admitted, tol);
            if (z - 1.0).abs() <= zero / 2.0 {
                assert_eq!(built, 6, "{label}: a gap inside the zero band builds");
            }
        }
    }
}

/// **FLUSH's tilt poses**: a parallelepiped standing on a block's top,
/// its bottom tilted up by `θ` about the edge it rests on, opening `φ`.
/// A tilt the glue reads Zero builds every op.
#[test]
fn a_block_standing_a_zero_band_tilt_on_another_builds_every_op() {
    let tol = Tol::witness();
    let zero = Band::linear(tol).expect("band").zero();
    let block = finished(
        "block",
        brick::<f64>((0.0, 3.0), (-2.0, 2.5), (0.0, 1.0), tol),
        tol,
    );
    for theta in [2e-12, 5e-12, 0.1 * zero, 0.25 * zero] {
        for phi_deg in [2.0_f64, 5.0, 20.0, 60.0, 90.0] {
            let phi = phi_deg.to_radians();
            let (p, ea) = (Point3::new(0.5, 0.2, 1.0), Vec3::new(1.0, 0.0, 0.0));
            let eb = Vec3::new(phi.cos(), phi.sin(), theta * phi.sin());
            let tool = finished(
                "tool",
                mapped_cube::<f64>(
                    move |u, v, w| p + ea * u + eb * v + Vec3::new(0.0, 0.0, w),
                    tol,
                ),
                tol,
            );
            let vt = phi.sin();
            let want = [13.5 + vt, 0.0, 13.5, vt];
            // The glued group is the block's top and the tool's bottom.
            let admitted = zero * 13.5;
            let label = format!("θ = {theta:e}, φ = {phi_deg}°");
            let built = holds(&label, &block, &tool, want, admitted, tol);
            if theta <= 0.25 * zero {
                assert_eq!(built, 6, "{label}: a tilt inside the zero band builds");
            }
        }
    }
}

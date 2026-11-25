//! **The ball cull's rows** ([`super::culled`], `curve_face_pose_clearance`):
//! the cull decides `Apart` only where the exact oracle
//! ([`crate::curve_face::conic_oracle::exact::truth`], double-double on the
//! stored data) finds no crossing and no point of the conic within the
//! band; past it, the pose answers `Apart` where its closed rungs refuse;
//! and a far-centred pair reads the gap its stored data holds.

#![allow(clippy::panic, clippy::unwrap_used, clippy::expect_used)]

use core::f64::consts::TAU;

use geom::Curve3;
use geom_core::{Band, Point3, Tol, Vec3};

use super::{Pose, culled, pose};
use crate::curve_face::conic_oracle::exact::{Dd, truth};
use crate::curve_face::conic_oracle::{ellipse, unit};
use crate::curve_pair::Span;

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap_or_else(|e| panic!("the witness band: {e:?}"))
}

/// The four kinds the conic arms cull against.
#[derive(Clone, Copy, Debug)]
enum Kind {
    Sphere,
    Wall,
    Cone,
    Torus,
}

/// A frame at `origin` along the unit `axis`.
fn frame(origin: Point3<f64>, axis: Vec3<f64>) -> geom::Frame<f64> {
    geom::Frame::new(origin, axis, axis.orthonormal_basis().0)
}

/// A surface of `kind` about `origin` along `axis`, drawn by `draw`.
fn surface(
    kind: Kind,
    origin: Point3<f64>,
    axis: Vec3<f64>,
    mut draw: impl FnMut(f64, f64) -> f64,
) -> geom::Surface<f64> {
    let frame = frame(origin, axis);
    match kind {
        Kind::Sphere => geom::Surface::Sphere {
            frame,
            radius: draw(0.3, 3.0),
        },
        Kind::Wall => geom::Surface::Cylinder {
            frame,
            radius: draw(0.3, 3.0),
        },
        Kind::Cone => geom::Surface::Cone {
            frame,
            half_angle: draw(0.2, 1.3),
        },
        Kind::Torus => {
            let big = draw(1.2, 3.0);
            geom::Surface::Torus {
                frame,
                major_radius: big,
                minor_radius: draw(0.15, big - 0.3),
            }
        }
    }
}

/// **The cull never decides `Apart` where the conic meets the band.**
/// Random circles and ellipses, of random frame, size and place, against
/// random spheres, walls, cones and ring tori; a third of them scaled so
/// their ball reaches just short of the surface, or just past it, where
/// the margin `|g(m)| − L·ℓ` reads across the band. Wherever the cull
/// answers `true`, the exact oracle finds no crossing round the whole
/// turn, and, on the three kinds whose oracle reads the true distance,
/// no extreme within the band's zero.
///
/// Mutants, each red: any kind's `g` with its constant's sign swapped
/// (`|e| + R`, `ρ + R`, `ρ cos α + |h| sin α`, `√(·) + r`), and the
/// margin read without `L·ℓ`.
#[test]
fn the_cull_never_decides_apart_where_the_conic_meets_the_band() {
    let b = band();
    let mut rng = test_utils::fuzz::start("cull-soundness");
    let mut culls = [0_usize; 4];
    let mut crossing = [0_usize; 4];
    for case in 0..test_utils::fuzz::scaled(4000) {
        let kind = [Kind::Sphere, Kind::Wall, Kind::Cone, Kind::Torus][case % 4];
        let origin = Point3::new(
            rng.range(-3.0, 3.0),
            rng.range(-3.0, 3.0),
            rng.range(-3.0, 3.0),
        );
        let s = surface(kind, origin, unit(|lo, hi| rng.range(lo, hi)), |lo, hi| {
            rng.range(lo, hi)
        });
        let c = [
            rng.range(-5.0, 5.0),
            rng.range(-5.0, 5.0),
            rng.range(-5.0, 5.0),
        ];
        let n = unit(|lo, hi| rng.range(lo, hi));
        let u = unit(|lo, hi| rng.range(lo, hi));
        let a = rng.log_uniform(0.05, 4.0);
        let b_axis = if rng.unit() < 0.3 {
            a
        } else {
            a * rng.range(0.1, 1.0)
        };
        let carrier = ellipse(c, n.to_array(), u.to_array(), a, b_axis);
        let t0 = rng.range(0.0, TAU);
        let span = Span::new(&carrier, (t0, t0 + rng.range(0.1, TAU)));
        let Some(conic) = carrier.conic() else {
            unreachable!("an ellipse is a conic")
        };
        if !culled(&span, conic, &s, b) {
            continue;
        }
        culls[case % 4] += 1;
        let t = truth(&carrier, &s);
        assert!(
            t.roots.is_empty(),
            "{kind:?} case {case}: culled, but the oracle crosses at {:?} ({})",
            t.roots,
            test_utils::fuzz::replay()
        );
        if !matches!(kind, Kind::Cone) {
            assert!(
                t.depth > b.zero(),
                "{kind:?} case {case}: culled within the band, {:e} deep ({})",
                t.depth,
                test_utils::fuzz::replay()
            );
        }
    }
    // Against vacuity: every kind culls, and each kind's draw crosses
    // its surface too (a sign-swapped `g` culls those).
    let mut rng = test_utils::fuzz::start("cull-soundness-crossings");
    for case in 0..400 {
        let kind = [Kind::Sphere, Kind::Wall, Kind::Cone, Kind::Torus][case % 4];
        let s = surface(kind, Point3::origin(), Vec3::unit_z(), |lo, hi| {
            rng.range(lo, hi)
        });
        let carrier = ellipse(
            [
                rng.range(-2.0, 2.0),
                rng.range(-2.0, 2.0),
                rng.range(-2.0, 2.0),
            ],
            unit(|lo, hi| rng.range(lo, hi)).to_array(),
            unit(|lo, hi| rng.range(lo, hi)).to_array(),
            rng.range(0.5, 4.0),
            rng.range(0.5, 4.0),
        );
        if !truth(&carrier, &s).roots.is_empty() {
            crossing[case % 4] += 1;
        }
    }
    for (k, (&culled, &crossed)) in culls.iter().zip(&crossing).enumerate() {
        assert!(
            culled > 20 && crossed > 10,
            "kind {k}: {culled} culls, {crossed} crossing draws"
        );
    }
}

/// **A definite clearance answers `Apart` where the closed rungs refuse.**
/// A unit circle against a unit sphere and against a unit wall `2·10⁷` m
/// off: the first-harmonic lane's extremes are products of lengths of
/// order `2·10⁷`, whose rounding is past every band row, so its noise
/// meter refuses; the ball cull reads the gap from the circle's own
/// points and answers `Apart` first.
///
/// Mutant: the cull skipped (`culled` answering `false`), red: the pose
/// refuses on the meter.
#[test]
fn a_definite_clearance_answers_apart_where_the_closed_rungs_refuse() {
    let far = 2e7;
    let ring = Curve3::circle(Point3::origin(), Vec3::unit_z(), 1.0, Vec3::unit_x());
    let span = Span::new(&ring, (-1.0, 1.0));
    for s in [
        geom::Surface::Sphere {
            frame: frame(Point3::new(far, 0.0, 0.0), Vec3::unit_z()),
            radius: 1.0,
        },
        geom::Surface::Cylinder {
            frame: frame(Point3::new(far, 0.0, 0.0), Vec3::unit_z()),
            radius: 1.0,
        },
    ] {
        let got = pose(&span, &s, band());
        assert!(matches!(got, Ok(Pose::Apart)), "{s:?}: {got:?}");
    }
}

/// **A circle `10⁶` m from its sphere's centre reads the gap its stored
/// data holds.** A circle about the origin in the `xy` plane, of radius
/// `10⁶ − 1 ∓ gap`, and a unit sphere centred `10⁶` m along `x`: the
/// circle's nearest point `(ρ, 0, 0)` stands `±gap` off the sphere, the
/// stored gap read in double-double (`10⁶ − 1 − ρ`) and confirmed by the
/// exact oracle's crossings. Clear by `10·Kε`, the cull decides `Apart`;
/// crossing by any of 40 stored gaps from an ulp of `10⁶` up to `10·Kε`,
/// it never does. Read only at an ε whose rounding reach covers the
/// `10⁶`-m lever (`geom_core::tolerance::rounding_reach`), where a short
/// chain's rounding at that scale fits the band's headroom.
///
/// Mutant: the sphere's `g` read through the harmonic extremes `c₀ ∓ A₁`
/// of its residual (`(|e|² + ρ² − R²)/2R` and `ρ·|e_∥|/R`), which cancel
/// at the far centre (their rounding `u·|e|²/2R ≈ 10⁻⁴`), red: a crossing
/// pose reads clear, or the clear one does not.
#[test]
fn a_circle_far_from_its_spheres_centre_reads_its_stored_gap() {
    let b = band();
    let far = 1e6;
    let k = b.escalate() / b.zero();
    if geom_core::tolerance::rounding_reach(b.zero(), k) < 2.0 * far {
        test_utils::vacuity::stood_down(
            "the 1e6 m lever",
            "the band's rounding reach does not cover a 1e6 m lever at this ε",
        );
        return;
    }
    let sphere = geom::Surface::Sphere {
        frame: frame(Point3::new(far, 0.0, 0.0), Vec3::unit_z()),
        radius: 1.0,
    };
    let ring =
        |radius: f64| Curve3::circle(Point3::origin(), Vec3::unit_z(), radius, Vec3::unit_x());
    // The stored gap `far − 1 − ρ`, exactly.
    let stored_gap = |radius: f64| Dd::of(far).sub(Dd::of(1.0)).sub(Dd::of(radius)).to_f64();
    let gap = 10.0 * b.escalate();
    let clear = ring(far - 1.0 - gap);
    let g = stored_gap(far - 1.0 - gap);
    assert!(g > 9.0 * b.escalate(), "the stored gap {g:e}");
    assert!(
        truth(&clear, &sphere).roots.is_empty(),
        "the oracle reads it clear"
    );
    let span = Span::new(&clear, (-1e-6, 1e-6));
    let conic = clear
        .conic()
        .unwrap_or_else(|| unreachable!("a circle is a conic"));
    assert!(
        culled(&span, conic, &sphere, b),
        "clear by {g:e}: the cull decides"
    );
    let ulp = f64::EPSILON * far;
    for j in 1..=40 {
        let depth = ulp * f64::from(j) * (gap / ulp / 40.0).max(1.0);
        let crossing = ring(far - 1.0 + depth);
        assert!(stored_gap(far - 1.0 + depth) < 0.0, "a crossing pose");
        assert!(
            !truth(&crossing, &sphere).roots.is_empty(),
            "the oracle crosses"
        );
        let span = Span::new(&crossing, (-1e-6, 1e-6));
        let conic = crossing
            .conic()
            .unwrap_or_else(|| unreachable!("a circle is a conic"));
        assert!(
            !culled(&span, conic, &sphere, b),
            "crossing by {depth:e}: culled"
        );
    }
}

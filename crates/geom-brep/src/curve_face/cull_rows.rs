//! **The ball cull's rows** ([`super::pose::culled`], `curve_face_pose_clearance`):
//! the cull decides `Apart` only where the exact oracle
//! ([`crate::curve_face::conic_oracle::exact::truth`], double-double on the
//! stored data) finds no crossing and no point of the conic within the
//! band; past it, the pose answers `Apart` where its closed rungs refuse;
//! and a far-centred pair reads the gap its stored data holds.

#![allow(clippy::panic, clippy::unwrap_used, clippy::expect_used)]

use core::f64::consts::TAU;

use geom::Curve3;
use geom_core::{Band, Point3, Tol, Vec3};

use super::pose::{Pose, culled, pose};
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
/// no extreme within `Kε`, which is what `Apart` claims.
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
                t.depth > b.escalate(),
                "{kind:?} case {case}: culled within Kε, {:e} deep ({})",
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

/// A drawn circle and its wall: centre `x` and `z`, axis, `u_ref`,
/// radius, the wall's radius.
type Draw = (f64, f64, [f64; 3], [f64; 3], f64, f64);

/// **A large circle grazing a small wall within the band's escalation is
/// never culled** (`Apart` asked past `Kε`, [`super::pose::culled`]). Five
/// circles of radius 68 to 1323 m in planes all but square to a wall's
/// axis, each grazing a wall of radius 0.003 to 4.7 m from outside with
/// its residual aimed within `0.01·ε` of `Kε` at `ε = 10⁻¹²` (the
/// first-harmonic edge sweep's draws, `conic_quadric::in_band_sweep`):
/// the exact oracle reads each clear but inside `Kε`, which the cull's
/// bound, the difference of two lengths of the circle's size, does not
/// resolve. The cull hands every one on.
///
/// Mutant: the question asked past zero (`|g(m)| − L·ℓ` decided bare), red: a
/// circle's rounding reads it past the band.
#[test]
fn a_large_circle_grazing_a_wall_within_the_escalation_is_not_culled() {
    let b = Band::new(1e-12, 1e-11).unwrap_or_else(|e| panic!("the 1e-12 band: {e:?}"));
    let draws: [Draw; 5] = [
        (
            363.136_956_988_935_94,
            -8.871_150_746_191_029e-8,
            [-2.442_984_817_845_077_3e-10, 0.0, -1.0],
            [-1.0, 0.0, 2.442_984_817_845_077_3e-10],
            363.127_543_052_688_57,
            0.009_413_936_237_377_16,
        ),
        (
            1_323.480_833_810_390_8,
            -1.620_792_733_947_465_3e-13,
            [
                -1.224_646_799_147_353_2e-16,
                8.307_255_315_739_171e-11,
                -1.0,
            ],
            [-1.0, 0.0, 1.224_646_799_147_353_2e-16],
            1_323.477_704_000_797_8,
            0.003_129_809_583_038_564_4,
        ),
        (
            189.204_362_531_105_1,
            -2.316_800_225_917_064e-14,
            [
                -1.224_646_799_147_353_2e-16,
                -1.386_127_232_462_233_7e-9,
                -1.0,
            ],
            [-1.0, 0.0, 1.224_646_799_147_353_2e-16],
            189.181_095_114_943_4,
            0.023_267_416_151_701_834,
        ),
        (
            72.409_753_874_600_82,
            -8.292_267_095_612_705e-15,
            [
                -1.224_646_799_147_353_2e-16,
                2.965_405_227_249_024_6e-8,
                -1.0,
            ],
            [-1.0, 0.0, 1.224_646_799_147_352_7e-16],
            67.711_499_359_538_66,
            4.698_254_515_052_163,
        ),
        (
            182.686_514_255_106_8,
            -2.219_215_823_871_732_2e-14,
            [-1.224_646_799_147_353_2e-16, 3.382_284_766_224_552e-9, -1.0],
            [-1.0, 0.0, 1.224_646_799_147_353_2e-16],
            181.212_723_980_239_6,
            1.473_790_274_857_219_8,
        ),
    ];
    for (cx, cz, axis, u_ref, radius, r) in draws {
        let wall = geom::Surface::Cylinder {
            frame: geom::Frame::new(Point3::origin(), Vec3::unit_z(), Vec3::unit_x()),
            radius: r,
        };
        let ring = Curve3::circle(
            Point3::new(cx, 0.0, cz),
            Vec3::new(axis[0], axis[1], axis[2]),
            radius,
            Vec3::new(u_ref[0], u_ref[1], u_ref[2]),
        );
        let t = truth(&ring, &wall);
        assert!(
            t.roots.is_empty() && t.residual_depth > 0.0 && t.residual_depth < b.escalate(),
            "radius {radius}: the oracle reads it clear inside Kε: {:e}",
            t.residual_depth
        );
        let span = Span::new(&ring, (-TAU / 2.0, TAU / 2.0));
        let conic = ring
            .conic()
            .unwrap_or_else(|| unreachable!("a circle is a conic"));
        assert!(
            !culled(&span, conic, &wall, b),
            "radius {radius}, clear by {:e} inside Kε: culled",
            t.residual_depth
        );
    }
}

/// **At the session box's scale the cull never decides `Apart` within
/// `Kε`.** Each kind, the cone included, against a circle drawn tangent to
/// it from outside at a known gap, so the circle's true distance from the
/// surface is that gap by construction: radii up to 450 m, centres up to
/// 500 m off the origin, the gap from `0.2·Kε` to `3·Kε`. A sphere's
/// circle lies in a plane through its centre; a wall's square to its
/// axis; a cone's and a torus's in a meridian plane, beside a generator
/// line or a tube circle. The circle's ball about its centre then reaches
/// the surface at the gap, so the cull's bound reads the gap itself,
/// through a rounding of the box's size. Wherever the cull answers `true`,
/// the gap is past `Kε`, the oracle finds no crossing, and on the three
/// kinds whose oracle reads the true distance that distance is past `Kε`.
/// Against vacuity, each kind culls some draws and hands others on.
///
/// Mutant: the cull's question asked past zero (`− escalate` dropped),
/// red.
#[test]
fn at_the_boxs_scale_the_cull_never_decides_apart_within_k_eps() {
    let b = band();
    let k = b.escalate();
    let mut rng = test_utils::fuzz::start("cull-box-scale");
    let mut culls = [0_usize; 4];
    let mut kept = [0_usize; 4];
    for case in 0..test_utils::fuzz::scaled(2000) {
        let kind = [Kind::Sphere, Kind::Wall, Kind::Cone, Kind::Torus][case % 4];
        let o = Point3::new(
            rng.range(-500.0, 500.0),
            rng.range(-500.0, 500.0),
            rng.range(-500.0, 500.0),
        );
        let axis = unit(|lo, hi| rng.range(lo, hi));
        let (u, v) = axis.orthonormal_basis();
        let a = rng.log_uniform(0.01, 450.0);
        let gap = rng.range(0.2, 3.0) * k;
        let (s, centre, normal) = match kind {
            Kind::Sphere => {
                let r = rng.log_uniform(0.3, 450.0);
                let s = geom::Surface::Sphere {
                    frame: frame(o, axis),
                    radius: r,
                };
                (s, o + u * (r + a + gap), axis)
            }
            Kind::Wall => {
                let r = rng.log_uniform(0.3, 450.0);
                let s = geom::Surface::Cylinder {
                    frame: frame(o, axis),
                    radius: r,
                };
                (
                    s,
                    o + u * (r + a + gap) + axis * rng.range(-50.0, 50.0),
                    axis,
                )
            }
            Kind::Cone => {
                let alpha = rng.range(0.2, 1.3);
                let s = geom::Surface::Cone {
                    frame: frame(o, axis),
                    half_angle: alpha,
                };
                // Along a generator, a distance past the circle's radius
                // from the apex, then off it outward in the meridian
                // plane.
                let g = axis * alpha.cos() + u * alpha.sin();
                let w = u * alpha.cos() - axis * alpha.sin();
                let s_along = a * rng.range(1.5, 4.0);
                (s, o + g * s_along + w * (a + gap), v)
            }
            Kind::Torus => {
                let big = rng.log_uniform(1.0, 450.0);
                let small = big * rng.range(0.05, 0.9);
                let s = geom::Surface::Torus {
                    frame: frame(o, axis),
                    major_radius: big,
                    minor_radius: small,
                };
                // Outward from the tube circle at `o + u·R`, in the
                // meridian plane, away from the axis.
                let tilt = rng.range(-1.2, 1.2);
                let w = u * tilt.cos() + axis * tilt.sin();
                (s, o + u * big + w * (small + a + gap), v)
            }
        };
        let circle = Curve3::circle(centre, normal, a, normal.orthonormal_basis().0);
        let span = Span::new(&circle, (0.0, TAU));
        let conic = circle
            .conic()
            .unwrap_or_else(|| unreachable!("a circle is a conic"));
        if !culled(&span, conic, &s, b) {
            kept[case % 4] += 1;
            continue;
        }
        culls[case % 4] += 1;
        assert!(
            gap > k,
            "{kind:?} case {case}: culled at a gap of {:.3}·Kε ({})",
            gap / k,
            test_utils::fuzz::replay()
        );
        let t = truth(&circle, &s);
        assert!(
            t.roots.is_empty(),
            "{kind:?} case {case}: culled, but the oracle crosses ({})",
            test_utils::fuzz::replay()
        );
        if !matches!(kind, Kind::Cone) {
            assert!(
                t.depth > k,
                "{kind:?} case {case}: culled {:e} from the surface ({})",
                t.depth,
                test_utils::fuzz::replay()
            );
        }
    }
    for (kind, (&c, &h)) in culls.iter().zip(&kept).enumerate() {
        assert!(c > 20 && h > 20, "kind {kind}: {c} culled, {h} handed on");
    }
}

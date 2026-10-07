//! Reviewer probes for the sphere ring lane (PR 4211 review), pinned as
//! rows: every op in both member orders, each body held to tier 3, 3′
//! (or, for a two-lump result, to tier 3 alone) and its volume against
//! a slice integral of the ball and the box.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use geom_core::{Affine3, Point3, Tol, Vec3};
use sweep::test_support::{ball_poled, brick, finished};
use topo::{AtRestBody, BooleanDeclarations, BooleanError, BooleanResult};

type Bounds = [(f64, f64); 3];

fn boxed(b: Bounds) -> AtRestBody<f64> {
    finished(
        "the box",
        brick(b[0], b[1], b[2], Tol::witness()),
        Tol::witness(),
    )
}

/// The unit ball at the origin, its pole along `pole`, spun `spin`
/// about `+y` and then tilted `tilt` about `tilt_axis`.
fn ball(pole: Vec3<f64>, spin: f64, (tilt_axis, tilt): (Vec3<f64>, f64)) -> AtRestBody<f64> {
    let tol = Tol::witness();
    let o = Point3::origin();
    let at = ball_poled(1.0, Vec3::new(0.0, 0.0, 0.0), pole, tol);
    let spun = topo::transform_rigid(
        &at,
        &Affine3::rotation_about_axis(o, Vec3::new(0.0, 1.0, 0.0), spin),
        tol,
    )
    .unwrap();
    let tilted = topo::transform_rigid(
        &spun,
        &Affine3::rotation_about_axis(o, tilt_axis, tilt),
        tol,
    )
    .unwrap();
    finished("the ball", tilted, tol)
}

fn quadrant(rho: f64, x: f64, y: f64) -> f64 {
    let s = |t: f64| (rho * rho - t * t).max(0.0).sqrt();
    let arc = |t: f64| {
        let t = t.clamp(-rho, rho);
        0.5 * (t * s(t) + rho * rho * (t / rho).asin()) + 0.25 * PI * rho * rho
    };
    let x = x.clamp(-rho, rho);
    let a = s(y);
    let inner = |lo: f64, hi: f64| {
        let (lo, hi) = (lo.max(-a), hi.min(a));
        if hi > lo {
            y * (hi - lo) + arc(hi) - arc(lo)
        } else {
            0.0
        }
    };
    let outer = |lo: f64, hi: f64| {
        if y < 0.0 {
            return 0.0;
        }
        let mut total = 0.0;
        for (p, q) in [(-rho, -a), (a, rho)] {
            let (p, q) = (p.max(lo), q.min(hi));
            if q > p {
                total += 2.0 * (arc(q) - arc(p));
            }
        }
        total
    };
    inner(-rho, x) + outer(-rho, x)
}

fn simpson(f: &dyn Fn(f64) -> f64, a: f64, b: f64) -> f64 {
    fn step(
        f: &dyn Fn(f64) -> f64,
        (a, b): (f64, f64),
        (fa, fm, fb): (f64, f64, f64),
        whole: f64,
        depth: u32,
    ) -> f64 {
        let (m, h) = (0.5 * (a + b), b - a);
        let (flm, frm) = (f(0.5 * (a + m)), f(0.5 * (m + b)));
        let left = h / 12.0 * (fa + 4.0 * flm + fm);
        let right = h / 12.0 * (fm + 4.0 * frm + fb);
        if depth == 0 || (left + right - whole).abs() <= 1e-15 {
            return left + right + (left + right - whole) / 15.0;
        }
        step(f, (a, m), (fa, flm, fm), left, depth - 1)
            + step(f, (m, b), (fm, frm, fb), right, depth - 1)
    }
    let (fa, fm, fb) = (f(a), f(0.5 * (a + b)), f(b));
    step(
        f,
        (a, b),
        (fa, fm, fb),
        (b - a) / 6.0 * (fa + 4.0 * fm + fb),
        40,
    )
}

/// The volume the unit ball at the origin shares with the box `b`.
fn ball_in_box(b: Bounds) -> f64 {
    let (lo, hi) = (b[2].0.max(-1.0), b[2].1.min(1.0));
    if hi <= lo {
        return 0.0;
    }
    let area = |z: f64| {
        let rho = (1.0 - z * z).max(0.0).sqrt();
        if rho == 0.0 {
            return 0.0;
        }
        let f = |x: f64, y: f64| quadrant(rho, x, y);
        f(b[0].1, b[1].1) - f(b[0].0, b[1].1) - f(b[0].1, b[1].0) + f(b[0].0, b[1].0)
    };
    simpson(&area, lo, hi)
}

fn box_volume(b: Bounds) -> f64 {
    b.iter().map(|(lo, hi)| hi - lo).product()
}

const BALL: f64 = 4.0 / 3.0 * PI;

/// What one op yields.
#[derive(Clone, Copy, Debug)]
enum Want {
    /// A body at tiers 3 and 3′ and at its volume.
    Body,
    /// Two lumps at tier 3 and at their volume, whose tier 3′ the census
    /// cannot decide (curved solids within reach of each other:
    /// `work/contact/census-cross-solid-curved-pairs-undecidable-on-shell-results.md`).
    Lumps,
    /// The result gate's refusal of a ringed sphere face.
    Gate,
    /// Ring re-homing on a sphere decided by no reference pair.
    Homing,
}

/// `a ∪ b`, `b ∪ a`, `a ∖ b`, `b ∖ a`, `a ∩ b`, `b ∩ a` against `wants`.
fn assert_six(
    pose: &str,
    a: &AtRestBody<f64>,
    b: &AtRestBody<f64>,
    (va, vb, shared): (f64, f64, f64),
    wants: [Want; 6],
) {
    let (tol, none) = (Tol::witness(), BooleanDeclarations::none());
    let ops = [
        (
            "a ∪ b",
            va + vb - shared,
            topo::union_with(a, b, &none, tol),
        ),
        (
            "b ∪ a",
            va + vb - shared,
            topo::union_with(b, a, &none, tol),
        ),
        ("a ∖ b", va - shared, topo::subtract_with(a, b, &none, tol)),
        ("b ∖ a", vb - shared, topo::subtract_with(b, a, &none, tol)),
        ("a ∩ b", shared, topo::intersect_with(a, b, &none, tol)),
        ("b ∩ a", shared, topo::intersect_with(b, a, &none, tol)),
    ];
    for ((op, volume, out), want) in ops.into_iter().zip(wants) {
        let label = format!("{pose}, {op}");
        match (out, want) {
            (Ok(BooleanResult::Body(bb)), Want::Body | Want::Lumps) => {
                topo::validate_geometric(&bb.body, tol)
                    .unwrap_or_else(|e| panic!("{label}: tier 3: {e:?}"));
                match (
                    topo::validate_pseudomanifold(&bb.body, &bb.contacts, tol),
                    want,
                ) {
                    (Ok(()), Want::Body) => {}
                    (Err(errors), Want::Lumps)
                        if errors.iter().all(|e| {
                            matches!(e, topo::ValidationError::CensusUndecidable { .. })
                        }) => {}
                    (got, _) => panic!("{label}: tier 3′: {got:?}"),
                }
                let v = topo::mass_properties(&bb.body, tol).unwrap().volume;
                assert!(
                    (v - volume).abs() <= 1e-9 * volume.max(1.0),
                    "{label}: volume {v} against the slice integral {volume}"
                );
            }
            (Err(BooleanError::ResultInvalid { errors }), Want::Gate)
                if matches!(
                    errors.as_slice(),
                    [topo::ValidationError::VolumeUncomputable {
                        source: topo::MassPropsError::RingOnCurvedFace { .. },
                        ..
                    }]
                ) => {}
            (
                Err(BooleanError::Join(topo::SplitJoinError::RingHomingAmbiguous { .. })),
                Want::Homing,
            ) => {}
            (out, want) => panic!(
                "{label}: wanted {want:?}, got {:?}",
                out.map(|r| r.body().map(|b| b.body.faces().count()))
            ),
        }
    }
}

/// **An island that holds the far cap's pole.** The tool covers the
/// unit ball but for the cap `x > 0.6` widened by a notch; the ball is
/// poled on `y` and spun so its lune boundary is the meridian circle
/// `x = 0`, on the run's side of the section plane `x = 0.6`. The
/// outer-loop point is read by a path to the far pole and lies INSIDE
/// the inner region, so the island is the region holding that pole.
#[test]
fn an_island_holding_the_far_pole_winds_by_an_inner_outer_point() {
    use Want::{Body, Gate};
    let big: Bounds = [(-2.0, 0.6), (-2.0, 2.0), (-2.0, 2.0)];
    for (name, notch) in [
        ("notch y > 0.5", [(0.4, 3.0), (0.5, 3.0), (-0.3, 0.3)]),
        ("notch y > 0.3", [(0.3, 3.0), (0.3, 3.0), (-0.25, 0.35)]),
        ("notch y < −0.4", [(0.45, 3.0), (-3.0, -0.4), (-0.2, 0.3)]),
    ] {
        let tol = Tol::witness();
        let tool = topo::subtract(&boxed(big), &boxed(notch), tol)
            .unwrap()
            .body()
            .unwrap()
            .body
            .clone();
        let vt = topo::mass_properties(&tool, tol).unwrap().volume;
        let cut: Bounds = [(notch[0].0, 0.6), notch[1], notch[2]];
        let shared = ball_in_box(big) - ball_in_box(cut);
        for tilt in [
            (Vec3::new(1.0, 0.0, 0.0), 0.0),
            (Vec3::new(1.0, 0.0, 0.0), 0.15),
            (Vec3::new(0.0, 0.0, 1.0), -0.2),
        ] {
            for spin in [0.5 * PI, -0.5 * PI] {
                let b = ball(Vec3::new(0.0, 1.0, 0.0), spin, tilt);
                assert_six(
                    &format!("{name}, tilt {}, spin {spin:.2}", tilt.1),
                    &tool,
                    &b,
                    (vt, BALL, shared),
                    [Body, Body, Gate, Body, Gate, Gate],
                );
            }
        }
    }
}

/// **A bar through the ball** leaves two rings on its face; bar ∖ ball
/// is two lumps.
#[test]
fn a_bar_through_a_ball_winds_both_rings() {
    use Want::{Body, Gate, Lumps};
    for pole in [
        Vec3::new(0.0, 1.0, 0.0),
        Vec3::new(0.3, 0.8, 0.52).normalize(),
    ] {
        for (name, b) in [
            ("bar x", [(-2.0, 2.0), (-0.2, 0.25), (0.1, 0.4)]),
            ("bar x off", [(-2.0, 2.0), (0.3, 0.5), (-0.45, -0.2)]),
            ("bar z", [(-0.3, 0.1), (0.2, 0.45), (-2.0, 2.0)]),
        ] {
            let ball = ball(pole, 0.0, (Vec3::new(1.0, 0.0, 0.0), 0.0));
            assert_six(
                &format!("{name}, pole {pole:?}"),
                &boxed(b),
                &ball,
                (box_volume(b), BALL, ball_in_box(b)),
                [Gate, Gate, Lumps, Gate, Body, Body],
            );
        }
    }
}

/// **A sphere ring no reference pair re-homes** (review finding m1):
/// every outer-loop vertex of the divided face lies on the new face's
/// run, so every path ends on the run and the ring refuses, though it
/// lies clear of the run. Pins the refusal; it goes red when re-homing
/// reads a point off the run.
#[test]
fn a_ring_whose_every_reference_lies_on_the_run_refuses() {
    use Want::{Body, Gate, Homing};
    let pole = Vec3::new(-0.6, 0.2, 0.77).normalize();
    for b in [
        [(-2.0, 2.0), (-0.2, 0.25), (0.1, 0.4)],
        [(-0.3, 0.1), (0.2, 0.45), (-2.0, 2.0)],
    ] {
        let ball = ball(pole, 0.0, (Vec3::new(1.0, 0.0, 0.0), 0.0));
        assert_six(
            &format!("bar {b:?}"),
            &boxed(b),
            &ball,
            (box_volume(b), BALL, ball_in_box(b)),
            [Homing, Gate, Homing, Gate, Homing, Body],
        );
    }
    let b: Bounds = [
        (-0.6237172865476482, 1.233346104703386),
        (-0.5631024588741076, 0.346881547918797),
        (-0.7919341754260856, -0.23917581093071405),
    ];
    let pole = Vec3::new(0.6355369378990602, -0.7198546525528592, -0.2791094404779036);
    let ball = ball(pole, 0.0, (Vec3::new(1.0, 0.0, 0.0), 0.0));
    assert_six(
        "the random box",
        &boxed(b),
        &ball,
        (box_volume(b), BALL, ball_in_box(b)),
        [Homing; 6],
    );
}

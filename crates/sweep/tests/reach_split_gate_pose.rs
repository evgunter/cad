//! The split gate's reach test is pose-invariant: a plane clear of a
//! sphere or torus face splits the body however the body is turned, at
//! every scale, and a plane that meets the face refuses.
//!
//! Every verdict is read against an oracle that owes nothing to the
//! kernel: the face's support along the cut normal, closed form in the
//! azimuth of the body's revolution and densely sampled along its
//! meridian, and every half's volume against a slice integral of the
//! revolved profile.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use crate::revolve_common::{axis_y, validated};
use geom::SurfaceKind;
use geom_core::{Affine3, Band, Point2, Point3, Tol, UnitVec3, Vec3};
use profile::{ArcSweep, RawLoop, bulge_from_center, test_support::bulge_loop};
use sweep::{Revolution, revolve};
use topo::splitting::{SplitError, SplitPart, SplitPlane, SplitReduceError, split};
use topo::{Body, DATUM_UNIT_NORM, transform_rigid};

/// One fixture: the revolved profile about `y`, its radius at height
/// `y` (the slice integral's integrand), and the unarmed face's
/// meridian `θ ↦ (ρ, y)`, sampled over its own parameter window.
struct Fixture {
    name: &'static str,
    chain: Vec<(Point2<f64>, f64)>,
    tangent_joints: Vec<usize>,
    /// The profile's height window and its smooth pieces' breakpoints.
    breaks: Vec<f64>,
    radius_at: fn(f64, f64) -> f64,
    meridian: fn(f64, f64) -> (f64, f64),
}

fn sq(x: f64) -> f64 {
    x * x
}

/// The unit cylinder `y ∈ [0, 1]` under the cap of the sphere of radius
/// 5/4 about `(0, 1/4)`; the cap is the zone `y ≥ 1`.
fn capped(s: f64) -> Fixture {
    let (a, b) = (Point2::new(s, s), Point2::new(0.0, 1.5 * s));
    let bulge = bulge_from_center(a, b, Point2::new(0.0, 0.25 * s), ArcSweep::Ccw);
    Fixture {
        name: "capped cylinder",
        chain: vec![
            (Point2::new(0.0, 0.0), 0.0),
            (Point2::new(s, 0.0), 0.0),
            (a, bulge),
            (b, 0.0),
        ],
        tangent_joints: Vec::new(),
        breaks: vec![0.0, s, 1.5 * s],
        radius_at: |s, y| {
            if y <= s {
                s
            } else {
                (sq(1.25 * s) - sq(y - 0.25 * s)).max(0.0).sqrt()
            }
        },
        // Polar angle from `+y`, over the cap's window `cos θ ≥ 3/5`.
        meridian: |s, t| {
            let th = t * 0.6f64.acos();
            (1.25 * s * th.sin(), 0.25 * s + 1.25 * s * th.cos())
        },
    }
}

/// The same sphere below `y = 1`, flat on top: the complement zone.
fn truncated(s: f64) -> Fixture {
    let (a, b) = (Point2::new(0.0, -s), Point2::new(s, s));
    let bulge = bulge_from_center(a, b, Point2::new(0.0, 0.25 * s), ArcSweep::Ccw);
    Fixture {
        name: "truncated ball",
        chain: vec![(a, bulge), (b, 0.0), (Point2::new(0.0, s), 0.0)],
        tangent_joints: Vec::new(),
        breaks: vec![-s, s],
        radius_at: |s, y| (sq(1.25 * s) - sq(y - 0.25 * s)).max(0.0).sqrt(),
        meridian: |s, t| {
            let lo = 0.6f64.acos();
            let th = lo + t * (PI - lo);
            (1.25 * s * th.sin(), 0.25 * s + 1.25 * s * th.cos())
        },
    }
}

/// The unit cylinder with its top rim rounded by the torus of radii
/// 3/4 and 1/4 about `(0, 1)`, capped flat at `y = 5/4`.
fn rounded(s: f64) -> Fixture {
    let (a, b) = (Point2::new(s, s), Point2::new(0.75 * s, 1.25 * s));
    let bulge = bulge_from_center(a, b, Point2::new(0.75 * s, s), ArcSweep::Ccw);
    Fixture {
        name: "rounded cylinder",
        chain: vec![
            (Point2::new(0.0, 0.0), 0.0),
            (Point2::new(s, 0.0), 0.0),
            (a, bulge),
            (b, 0.0),
            (Point2::new(0.0, 1.25 * s), 0.0),
        ],
        tangent_joints: vec![2, 3],
        breaks: vec![0.0, s, 1.25 * s],
        radius_at: |s, y| {
            if y <= s {
                s
            } else {
                0.75 * s + (sq(0.25 * s) - sq(y - s)).max(0.0).sqrt()
            }
        },
        meridian: |s, t| {
            let v = t * PI / 2.0;
            (0.75 * s + 0.25 * s * v.cos(), s + 0.25 * s * v.sin())
        },
    }
}

fn build(f: &Fixture) -> Body<f64> {
    revolve(
        &validated(vec![
            bulge_loop(f.chain.clone()).with_tangent_joints(f.tangent_joints.clone()),
        ]),
        axis_y(),
        Revolution::Full,
        Tol::witness(),
    )
    .unwrap()
    .body
}

/// The face's support along the body-frame unit normal `n`, as
/// `(min, max)` of `n·p`: over the azimuth of a revolution about `y`
/// the extremes of `nₓx + n_z z` at radius `ρ` are `±ρ·√(nₓ² + n_z²)`
/// exactly, and the meridian is sampled at a step whose chord error is
/// far below the margins read.
fn face_support(f: &Fixture, s: f64, n: Vec3<f64>) -> (f64, f64) {
    let m = (sq(n.x) + sq(n.z)).sqrt();
    let (mut lo, mut hi) = (f64::INFINITY, f64::NEG_INFINITY);
    let k = 200_000;
    for i in 0..=k {
        let (rho, y) = (f.meridian)(s, i as f64 / k as f64);
        lo = lo.min(n.y * y - rho * m);
        hi = hi.max(n.y * y + rho * m);
    }
    (lo, hi)
}

/// The volume of the body's part on the `n·p > d` side, by slices
/// across `y`: each slice is a disk of radius `ρ(y)`, cut by the line
/// at distance `t = (d − n_y·y)/√(nₓ² + n_z²)` from its centre, whose
/// segment area is closed form. Composite Simpson per smooth piece.
fn half_volume(f: &Fixture, s: f64, n: Vec3<f64>, d: f64) -> f64 {
    let m = (sq(n.x) + sq(n.z)).sqrt();
    let area = |y: f64| {
        let rho = (f.radius_at)(s, y);
        let t = (d - n.y * y) / m;
        if t >= rho {
            0.0
        } else if t <= -rho {
            PI * rho * rho
        } else {
            rho * rho * (t / rho).acos() - t * (rho * rho - t * t).sqrt()
        }
    };
    // A cut square to the axis makes the slice area a step at its
    // height, which Simpson cannot integrate across: integrate the
    // whole disks on the kept side of it instead.
    let mut breaks = f.breaks.clone();
    if m < 1e-15 {
        let cut = d / n.y;
        let keep = |y: f64| if n.y > 0.0 { y >= cut } else { y <= cut };
        breaks = breaks.iter().copied().filter(|&y| keep(y)).collect();
        if cut > f.breaks[0] && cut < f.breaks[f.breaks.len() - 1] {
            breaks.push(cut);
            breaks.sort_by(f64::total_cmp);
        }
    }
    let area = |y: f64| {
        if m < 1e-15 {
            let rho = (f.radius_at)(s, y);
            PI * rho * rho
        } else {
            area(y)
        }
    };
    let mut v = 0.0;
    for w in breaks.windows(2) {
        let (a, b) = (w[0], w[1]);
        let k = 20_000;
        let h = (b - a) / k as f64;
        let mut acc = area(a) + area(b);
        for i in 1..k {
            acc += area(a + h * i as f64) * if i % 2 == 1 { 4.0 } else { 2.0 };
        }
        v += acc * h / 3.0;
    }
    v
}

fn unit(v: Vec3<f64>) -> UnitVec3<f64> {
    let band = Band::linear(Tol::witness()).unwrap();
    UnitVec3::new(v, DATUM_UNIT_NORM, band).unwrap()
}

/// The planes, in the body's own frame, as `(n, point on the plane)`:
/// cuts square to the axis at heights through and beyond the face, and
/// oblique cuts at three tilts in two azimuths through the axis.
fn planes(f: &Fixture, s: f64) -> Vec<(Vec3<f64>, Point3<f64>)> {
    let mut out = grazing(f, s);
    for c in [
        -0.9, -0.5, 0.05, 0.2, 0.3, 0.5, 0.7, 0.8, 0.9, 0.95, 0.99, 1.05, 1.2, 1.4, 1.6,
    ] {
        out.push((Vec3::new(0.0, 1.0, 0.0), Point3::new(0.0, c * s, 0.0)));
    }
    for phi in [0.3f64, 0.7, 1.1] {
        for psi in [0.0f64, 1.3] {
            let n = Vec3::new(phi.sin() * psi.cos(), phi.cos(), phi.sin() * psi.sin());
            for c in [-0.5, 0.2, 0.5, 0.8, 0.95, 1.2, 1.4] {
                out.push((n, Point3::new(0.0, c * s, 0.0)));
            }
        }
    }
    out
}

/// Planes grazing the face from its outer side and from its inner
/// side along tilted normals: `δ` beyond the face's least support
/// along `n`, and `δ` inside it, at three `δ`.
fn grazing(f: &Fixture, s: f64) -> Vec<(Vec3<f64>, Point3<f64>)> {
    let mut out = Vec::new();
    for phi in [0.0f64, 0.3, 0.7, 1.1, 1.4, 2.0, 2.8] {
        for psi in [0.0f64, 1.3] {
            let n = Vec3::new(phi.sin() * psi.cos(), phi.cos(), phi.sin() * psi.sin());
            let (lo, _) = face_support(f, s, n);
            for delta in [1e-2, 1e-4, 1e-5] {
                for d in [lo - delta * s, lo + delta * s] {
                    out.push((n, Point3::new(n.x * d, n.y * d, n.z * d)));
                }
            }
        }
    }
    out
}

#[derive(Default, Debug)]
struct Tally {
    clear_split: usize,
    clear_gate: usize,
    clear_other: usize,
    meets_split: usize,
    meets_gate: usize,
    meets_other: usize,
    near: usize,
    wrong_volume: usize,
    props_refused: usize,
}

fn poses(s: f64) -> [(&'static str, Affine3<f64>); 3] {
    [
        ("upright", Affine3::identity()),
        (
            "turned 0.3 about z",
            Affine3::rotation_about_axis(Point3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0), 0.3),
        ),
        (
            "skew",
            Affine3::translation(Vec3::new(0.3 * s, -0.7 * s, 0.2 * s))
                * Affine3::rotation_about_axis(
                    Point3::new(0.0, 0.0, 0.0),
                    Vec3::new(1.0, 2.0, 3.0),
                    1.1,
                ),
        ),
    ]
}

/// The probe: every fixture at three scales and three poses against
/// every plane, tallied against the oracle. Run with `--nocapture` to
/// read the table.
#[test]
#[ignore = "measurement probe: cargo nextest run -p sweep --run-ignored all reach_split_gate_pose::probe --no-capture"]
fn probe() {
    for s in [1e-3, 1.0, 1e3] {
        for f in [capped(s), truncated(s), rounded(s)] {
            let body = build(&f);
            for (pose, map) in poses(s) {
                let posed = transform_rigid(&body, &map, Tol::witness()).unwrap();
                let mut t = Tally::default();
                for (n, q) in planes(&f, s) {
                    let n = n * (1.0 / (n.x * n.x + n.y * n.y + n.z * n.z).sqrt());
                    let d = n.x * q.x + n.y * q.y + n.z * q.z;
                    let (lo, hi) = face_support(&f, s, n);
                    let gap = (lo - d).max(d - hi);
                    let cut = SplitPlane {
                        origin: map.transform_point(q),
                        normal: unit(map.transform_vec(n)),
                    };
                    let r = split(&posed, &cut, Tol::witness());
                    let clear = gap > 1e-6 * s;
                    let meets = gap < -1e-6 * s;
                    if !clear && !meets {
                        t.near += 1;
                    }
                    match r {
                        Ok(res) => {
                            if clear {
                                t.clear_split += 1;
                            } else if meets {
                                t.meets_split += 1;
                            }
                            for (part, sign) in [(&res.above, 1.0), (&res.below, -1.0)] {
                                let oracle = half_volume(&f, s, n * sign, d * sign);
                                let got = match part {
                                    SplitPart::Body(b) => {
                                        match topo::props::mass_properties(b, Tol::witness()) {
                                            Ok(p) => (p.volume, p.volume_pad),
                                            Err(e) => {
                                                t.props_refused += 1;
                                                println!(
                                                    "PROPS {} s={s} {pose} n={n:?} d={d} clear={clear}: {e}",
                                                    f.name
                                                );
                                                continue;
                                            }
                                        }
                                    }
                                    SplitPart::Empty => (0.0, 0.0),
                                };
                                if (got.0 - oracle).abs() > got.1 + 1e-6 * s * s * s {
                                    t.wrong_volume += 1;
                                    println!(
                                        "WRONG {} s={s} {pose} n={n:?} d={d}: {} ± {} vs {oracle}",
                                        f.name, got.0, got.1
                                    );
                                }
                            }
                        }
                        Err(SplitError::Reduce(SplitReduceError::CurvedBooleanUnsupported {
                            ..
                        })) => {
                            if clear {
                                t.clear_gate += 1;
                            } else if meets {
                                t.meets_gate += 1;
                            }
                        }
                        Err(e) => {
                            if clear {
                                t.clear_other += 1;
                                println!("OTHER clear {} s={s} {pose} n={n:?} d={d}: {e}", f.name);
                            } else if meets {
                                t.meets_other += 1;
                            }
                        }
                    }
                }
                println!("ROW | {} | {s:e} | {pose} | {t:?}", f.name);
            }
        }
    }
}

/// Split `body` posed by `map` with the body-frame plane `(n, q)`, and
/// read each half's volume against the slice integral.
fn split_posed(
    f: &Fixture,
    s: f64,
    posed: &Body<f64>,
    map: Affine3<f64>,
    (n, q): (Vec3<f64>, Point3<f64>),
    what: &str,
) -> Result<(), SplitError> {
    let n = n * (1.0 / n.norm());
    let d = n.dot(Vec3::new(q.x, q.y, q.z));
    let cut = SplitPlane {
        origin: map.transform_point(q),
        normal: unit(map.transform_vec(n)),
    };
    let result = split(posed, &cut, Tol::witness())?;
    for (part, sign, side) in [
        (&result.above, 1.0, "above"),
        (&result.below, -1.0, "below"),
    ] {
        let oracle = half_volume(f, s, n * sign, d * sign);
        let SplitPart::Body(b) = part else {
            assert!(
                oracle.abs() <= 1e-9 * s * s * s,
                "{what}, {side}: empty against the oracle {oracle}"
            );
            continue;
        };
        let p = topo::props::mass_properties(b, Tol::witness()).unwrap();
        assert!(
            (p.volume - oracle).abs() <= p.volume_pad + 1e-7 * s * s * s,
            "{what}, {side}: {} ± {} against the oracle {oracle}",
            p.volume,
            p.volume_pad
        );
    }
    Ok(())
}

/// **A cut clear of a sphere or torus face splits the body however
/// the body is turned.** Each fixture is cut square to its own axis at
/// mid-height of the cylinder, under the cap or the rounding, and along
/// a direction tilted off its axis `10⁻³` of its size (and `100 ε`)
/// clear of the face's own support there; in all three poses at unit scale, and
/// skewed at the other two scales. Both halves match the slice
/// integral.
#[test]
fn a_cut_clear_of_the_face_splits_in_every_pose() {
    let tilted = Vec3::new(
        0.7f64.sin() * 1.3f64.cos(),
        0.7f64.cos(),
        0.7f64.sin() * 1.3f64.sin(),
    );
    for (s, pose_set) in [(1.0, 0..3), (1e-3, 2..3), (1e3, 2..3)] {
        if s < 1e4 * Tol::witness().eps() {
            test_utils::vacuity::stood_down(
                &format!("scale {s}, eps = {:e}", Tol::witness().eps()),
                "the body is under 10⁴ ε across, where the cut's later stages read in band",
            );
            continue;
        }
        for f in [capped(s), rounded(s)] {
            let body = build(&f);
            let (lo, _) = face_support(&f, s, tilted);
            // Clear of the face by a thousandth of its size, and of the
            // sweep pad at every ε.
            let d = lo - 1e-3 * s - 100.0 * Tol::witness().eps();
            for (pose, map) in &poses(s)[pose_set.clone()] {
                let tol = Tol::witness();
                let posed = match transform_rigid(&body, map, tol) {
                    Ok(posed) => posed,
                    Err(e) if tol.eps() < geom_core::tolerance::DEFAULT_EPS => {
                        test_utils::vacuity::stood_down(
                            &format!("{} at scale {s}, {pose}, eps = {:e}", f.name, tol.eps()),
                            &format!("posing the body refused ({e}) before any cut"),
                        );
                        continue;
                    }
                    Err(e) => panic!("{} at scale {s}, {pose}: posing refused: {e}", f.name),
                };
                for (cut, plane) in [
                    (
                        "square",
                        (Vec3::new(0.0, 1.0, 0.0), Point3::new(0.0, 0.5 * s, 0.0)),
                    ),
                    (
                        "tilted",
                        (
                            tilted,
                            Point3::new(tilted.x * d, tilted.y * d, tilted.z * d),
                        ),
                    ),
                ] {
                    let what = format!("{} at scale {s}, {pose}, {cut}", f.name);
                    split_posed(&f, s, &posed, *map, plane, &what)
                        .unwrap_or_else(|e| panic!("{what}: {e}"));
                }
            }
        }
    }
}

/// **A cut through the face refuses naming it, however the body is
/// turned**: the gate is scoped to the plane's reach in the plane's own
/// frame, never lifted. The cuts are the clear row's tilted cut moved
/// `10⁻³` of the size INTO the face, and one across the truncated ball's
/// flank that meets its sphere face in a circle crossing no edge.
#[test]
fn a_cut_into_the_face_refuses_in_every_pose() {
    let s = 1.0;
    let tilted = Vec3::new(
        0.7f64.sin() * 1.3f64.cos(),
        0.7f64.cos(),
        0.7f64.sin() * 1.3f64.sin(),
    );
    for (f, kind) in [
        (capped(s), SurfaceKind::Sphere),
        (rounded(s), SurfaceKind::Torus),
        (truncated(s), SurfaceKind::Sphere),
    ] {
        let body = build(&f);
        let (lo, _) = face_support(&f, s, tilted);
        let d = lo + 1e-3 * s;
        for (pose, map) in poses(s) {
            let posed = transform_rigid(&body, &map, Tol::witness()).unwrap();
            let what = format!("{}, {pose}", f.name);
            let plane = (
                tilted,
                Point3::new(tilted.x * d, tilted.y * d, tilted.z * d),
            );
            match split_posed(&f, s, &posed, map, plane, &what) {
                Err(SplitError::Reduce(SplitReduceError::CurvedBooleanUnsupported {
                    face,
                    kind: k,
                })) => {
                    assert_eq!(k, kind, "{what}");
                    let surface = posed.get_face(face).unwrap().surface;
                    assert_eq!(posed.get_surface(surface).unwrap().kind(), kind, "{what}");
                }
                other => panic!("{what}: expected the gate's refusal, got {other:?}"),
            }
        }
    }
}

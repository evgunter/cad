//! S393 review lane R2 — probes against PR #2466's claims.
//!
//! 1. Outside the retired cone the door and the deleted copy agree
//!    bit for bit (claim 1); inside it they do not.
//! 2. The inflecting duct's two frames, bodies, volumes and ring
//!    centroids (claims 3 and 8), and whether every station ring is
//!    the same point set.
//! 3. The old halfway-index row logic on the new body, and the new
//!    sign-split logic on a one-way path (the anti-vacuity mutation).
//! 4. The tour's `s_duct` scene places by `Affine3::identity()`; how
//!    far is that from the door's frame on the same path?
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::cast_precision_loss,
    clippy::too_many_lines
)]

use core::f64::consts::FRAC_PI_2;

use geom::NurbsCurve3;
use geom_core::linalg::frame::path_start_frame;
use geom_core::{Affine3, Mat3, Point2, Point3, Tol, Vec3};
use sweep::skin::{segment_curve, sweep_places};
use sweep::SketchSegment;
use sweep::{Lofted, sweep_body};

use crate::common;
use common::orient::ring_centroid;
use common::{normal_start_place, quad};

/// The deleted copy, verbatim (merge base `common/mod.rs`).
fn cone_place(path: &NurbsCurve3<f64>) -> Affine3<f64> {
    let (lo, _) = path.domain();
    let d = path.deriv(lo);
    let n = d / d.norm();
    let helper = if n.z.abs() < 0.9 {
        Vec3::new(0.0, 0.0, 1.0)
    } else {
        Vec3::new(1.0, 0.0, 0.0)
    };
    let u = helper.cross(n);
    let u = u / u.norm();
    let v = n.cross(u);
    let p = path.eval(lo);
    Affine3::from_parts(Mat3::from_cols(u, v, n), Vec3::new(p.x, p.y, p.z))
}

/// The deleted TOUR copy, verbatim (merge base `skinned.rs`): the third
/// column spelled as `u × (n × u)` through `from_frame`.
fn tour_cone_place(path: &NurbsCurve3<f64>) -> Affine3<f64> {
    let (lo, _) = path.domain();
    let d = path.deriv(lo);
    let n = d / d.norm();
    let helper = if n.z.abs() < 0.9 {
        Vec3::unit_z()
    } else {
        Vec3::unit_x()
    };
    let u = helper.cross(n);
    let u = u / u.norm();
    Affine3::from_frame(path.eval(lo), u, n.cross(u))
}

fn bits12(a: &Affine3<f64>) -> [u64; 12] {
    let (m, t) = (a.linear, a.translation);
    [
        m.c0.x, m.c0.y, m.c0.z, m.c1.x, m.c1.y, m.c1.z, m.c2.x, m.c2.y, m.c2.z, t.x, t.y, t.z,
    ]
    .map(f64::to_bits)
}

const S_RADIUS: f64 = 2.0;
fn inflecting_path() -> NurbsCurve3<f64> {
    let pts: Vec<Point3<f64>> = (0..=8)
        .map(|k| {
            let th = FRAC_PI_2 * f64::from(k) / 8.0;
            Point3::new(0.0, S_RADIUS * (1.0 - th.cos()), S_RADIUS * th.sin())
        })
        .chain((1..=8).map(|k| {
            let ph = FRAC_PI_2 * f64::from(k) / 8.0;
            Point3::new(
                0.0,
                S_RADIUS + S_RADIUS * ph.sin(),
                2.0 * S_RADIUS - S_RADIUS * ph.cos(),
            )
        }))
        .collect();
    NurbsCurve3::<f64>::interpolate(&pts, 3).unwrap()
}

/// One quarter arc only — a path that turns one way throughout.
fn one_way_path() -> NurbsCurve3<f64> {
    let pts: Vec<Point3<f64>> = (0..=8)
        .map(|k| {
            let th = FRAC_PI_2 * f64::from(k) / 8.0;
            Point3::new(0.0, S_RADIUS * (1.0 - th.cos()), S_RADIUS * th.sin())
        })
        .collect();
    NurbsCurve3::<f64>::interpolate(&pts, 3).unwrap()
}

fn torsion_path() -> NurbsCurve3<f64> {
    let (a, b, c) = (2.2, 1.3, 1.5);
    let pts: Vec<Point3<f64>> = (0..=32)
        .map(|k| {
            let t = 2.0_f64.mul_add(f64::from(k) / 32.0, -1.0);
            Point3::new(a * t, b * t * t, c * t * t * t)
        })
        .collect();
    NurbsCurve3::<f64>::interpolate(&pts, 3).unwrap()
}

fn helix_path(turns: f64) -> NurbsCurve3<f64> {
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let n = ((32.0 * turns).ceil() as usize).max(32);
    let pts: Vec<Point3<f64>> = (0..=n)
        .map(|k| {
            let s = k as f64 / n as f64;
            let a = std::f64::consts::TAU * turns * s;
            Point3::new(a.cos(), a.sin(), 0.4 * turns * s)
        })
        .collect();
    NurbsCurve3::<f64>::interpolate(&pts, 3).unwrap()
}

fn lily_spine() -> NurbsCurve3<f64> {
    let (len, curl) = (1.25f64, -0.40f64);
    let r = len / curl;
    let pts: Vec<Point3<f64>> = (0..9)
        .map(|k| {
            let a = curl * (k as f64) / 8.0;
            Point3::new(r * a.sin(), r * (1.0 - a.cos()), 0.0)
        })
        .collect();
    NurbsCurve3::<f64>::interpolate(&pts, 3).unwrap()
}

fn tour_arc_path() -> NurbsCurve3<f64> {
    segment_curve(
        0,
        SketchSegment::Arc {
            a: Point2::new(0.0, 0.0),
            b: Point2::new(3.0, 3.0),
            bulge: 0.4,
        },
        Affine3::identity(),
    )
    .unwrap()
}

fn leaning_path() -> NurbsCurve3<f64> {
    let cos = 0.95f64;
    let off = ((1.0 - cos * cos) / 2.0).sqrt();
    let dir = Vec3::new(off, off, cos);
    let base = Point3::new(1.5, -2.25, 3.125);
    NurbsCurve3::<f64>::interpolate(
        &(0..=4).map(|k| base + dir * (k as f64)).collect::<Vec<_>>(),
        3,
    )
    .unwrap()
}

fn fixtures() -> Vec<(&'static str, NurbsCurve3<f64>)> {
    vec![
        ("inflecting_path (S duct)", inflecting_path()),
        ("torsion_path (twisted duct)", torsion_path()),
        ("helix 0.5", helix_path(0.5)),
        ("helix 1.0", helix_path(1.0)),
        ("helix 2.0", helix_path(2.0)),
        ("lily spine", lily_spine()),
        ("tour arc path", tour_arc_path()),
        ("leaning 0.95 (new pin)", leaning_path()),
    ]
}

/// Claim 1: the door IS the copy, bit for bit, wherever `|n.z| < 0.9`.
#[test]
fn r2_p1_door_vs_cone_bits_at_every_fixture() {
    let mut inside = 0;
    for (name, path) in fixtures() {
        let (lo, _) = path.domain();
        let d = path.deriv(lo);
        let n = d / d.norm();
        let cone = cone_place(&path);
        let tour = tour_cone_place(&path);
        let door = normal_start_place(&path);
        let door_direct = path_start_frame(path.eval(lo), d, Tol::witness()).unwrap();
        assert_eq!(bits12(&door), bits12(&door_direct), "{name}: the helper is the door");
        let same = bits12(&cone) == bits12(&door);
        let tour_same = bits12(&tour) == bits12(&door);
        let tour_off = (0..12)
            .map(|i| {
                let (a, b) = (bits12(&tour)[i], bits12(&door)[i]);
                (f64::from_bits(a) - f64::from_bits(b)).abs()
            })
            .fold(0.0f64, f64::max);
        println!(
            "{name:28} |n.z| = {:.9}  cone helper {}  sweep-copy==door: {same}  tour-copy==door: {tour_same} (max |Δ| {tour_off:.2e})",
            n.z.abs(),
            if n.z.abs() < 0.9 { "+Z" } else { "+X" }
        );
        if n.z.abs() < 0.9 {
            assert!(same, "{name}: outside the cone the sweep copy must be the door bitwise");
        } else {
            inside += 1;
            assert!(!same, "{name}: inside the cone the frames must differ");
        }
    }
    assert_eq!(inside, 2, "the inflecting duct and the new leaning pin are the cone's only members");
}

fn swept_square(path: &NurbsCurve3<f64>, place: Affine3<f64>, stations: usize) -> Lofted<f64> {
    let h = 0.25;
    let profile = quad([(-h, -h), (h, -h), (h, h), (-h, h)]);
    sweep_body::<f64>(&profile, place, path, stations, 3, Tol::witness()).unwrap()
}

/// Claims 3 and 8: the duct under both frames.
#[test]
fn r2_p2_inflecting_duct_under_both_frames() {
    let path = inflecting_path();
    let cone = cone_place(&path);
    let door = normal_start_place(&path);
    println!("cone frame: {cone:?}");
    println!("door frame: {door:?}");
    // Is `cone = door · R_z(±90°)` exact in the stored bits?
    let rz = |s: f64| Mat3::from_cols(Vec3::new(0.0, s, 0.0), Vec3::new(-s, 0.0, 0.0), Vec3::unit_z());
    for (label, s) in [("+90", 1.0), ("-90", -1.0)] {
        let composed = Affine3::from_parts(door.linear * rz(s), door.translation);
        let off = (0..12)
            .map(|i| (f64::from_bits(bits12(&composed)[i]) - f64::from_bits(bits12(&cone)[i])).abs())
            .fold(0.0f64, f64::max);
        println!("cone == door·R_z({label}°)? bitwise {}  max |Δ| {off:.3e}", bits12(&composed) == bits12(&cone));
    }
    // Same station rings as point sets?
    let stations = 13;
    let pc = sweep_places(cone, &path, stations).unwrap();
    let pd = sweep_places(door, &path, stations).unwrap();
    let h = 0.25;
    let corners = [(-h, -h), (h, -h), (h, h), (-h, h)];
    let mut worst = 0.0f64;
    for (a, b) in pc.iter().zip(&pd) {
        let ring = |p: &Affine3<f64>| {
            corners.map(|(x, y)| p.transform_point(Point3::new(x, y, 0.0)))
        };
        let (ra, rb) = (ring(a), ring(b));
        for q in ra {
            let d = rb.iter().map(|r| (*r - q).norm()).fold(f64::INFINITY, f64::min);
            worst = worst.max(d);
        }
    }
    println!("station rings as point sets: worst nearest-corner distance {worst:.3e}");

    let sc = swept_square(&path, cone, stations);
    let sd = swept_square(&path, door, stations);
    let vc = topo::mass_properties(&sc.body, Tol::witness()).unwrap();
    let vd = topo::mass_properties(&sd.body, Tol::witness()).unwrap();
    let al = (2.0 * h) * (2.0 * h) * (2.0 * S_RADIUS * FRAC_PI_2);
    println!("volume cone {:.15} ± {:.1e}", vc.volume, vc.volume_pad);
    println!("volume door {:.15} ± {:.1e}", vd.volume, vd.volume_pad);
    println!("A·L         {al:.15}");
    println!("centroid v=0.5 cone {:?}", ring_centroid(&sc, 0.5));
    println!("centroid v=0.5 door {:?}", ring_centroid(&sd, 0.5));
    println!("section_params cone {:?}", sc.section_params);
    println!("section_params door {:?}", sd.section_params);
}

fn signed_angle_about(axis: Vec3<f64>, a: Vec3<f64>, b: Vec3<f64>) -> f64 {
    let n = axis / axis.norm();
    let a = a - n * a.dot(n);
    let b = b - n * b.dot(n);
    a.cross(b).dot(n).atan2(a.dot(b))
}

fn spine_chords(lofted: &Lofted<f64>, steps: usize) -> Vec<Vec3<f64>> {
    let centres: Vec<Point3<f64>> = (0..=steps)
        .map(|i| ring_centroid(lofted, i as f64 / steps as f64))
        .collect();
    centres.windows(2).map(|w| w[1] - w[0]).collect()
}

fn old_row(spine: &[Vec3<f64>]) -> (f64, f64) {
    let plane_normal = Vec3::new(1.0, 0.0, 0.0);
    let signed_turn = |half: &[Vec3<f64>]| -> f64 {
        half.windows(2)
            .map(|w| signed_angle_about(plane_normal, w[0], w[1]))
            .sum()
    };
    let mid = spine.len() / 2;
    (signed_turn(&spine[..=mid]), signed_turn(&spine[mid..]))
}

fn new_row(spine: &[Vec3<f64>]) -> (f64, f64) {
    let plane_normal = Vec3::new(1.0, 0.0, 0.0);
    spine.windows(2).fold((0.0, 0.0), |(neg, pos), w| {
        let d = signed_angle_about(plane_normal, w[0], w[1]);
        if d < 0.0 { (neg + d, pos) } else { (neg, pos + d) }
    })
}

/// Claim 3: the old split reads short on the new body; the new sums
/// go red on a one-way path (anti-vacuity is real).
#[test]
fn r2_p3_row_mutations() {
    let bar = 0.9 * FRAC_PI_2;
    let path = inflecting_path();
    let new_body = swept_square(&path, normal_start_place(&path), 13);
    let old_body = swept_square(&path, cone_place(&path), 13);
    let (ns, os) = (spine_chords(&new_body, 64), spine_chords(&old_body, 64));
    println!("bar {bar}");
    println!("old row on old body: {:?}", old_row(&os));
    println!("old row on new body: {:?}", old_row(&ns));
    println!("new row on old body: {:?}", new_row(&os));
    println!("new row on new body: {:?}", new_row(&ns));
    let (f, s) = old_row(&ns);
    assert!(!(f <= -bar && s >= bar), "the old split must fail on the new body, or the PR's story is wrong");
    let one = one_way_path();
    let elbow = swept_square(&one, normal_start_place(&one), 7);
    let (neg, pos) = new_row(&spine_chords(&elbow, 64));
    println!("new row on a one-way quarter arc: ({neg}, {pos})");
    assert!(!(neg <= -bar && pos >= bar), "the new row must go red on a path that turns one way");
}

/// The tour's `s_duct` scene: `Affine3::identity()` against the door.
#[test]
fn r2_p4_s_duct_identity_vs_door() {
    let path = inflecting_path();
    let door = normal_start_place(&path);
    let (lo, _) = path.domain();
    let n = path.deriv(lo).normalize();
    let tilt = n.dot(Vec3::unit_z()).clamp(-1.0, 1.0).acos();
    println!("S path start tangent {n:?}; identity's +Z is {tilt:.3e} rad off it");
    println!("door frame {door:?}");
    let id = swept_square(&path, Affine3::identity(), 13);
    let dr = swept_square(&path, door, 13);
    let vi = topo::mass_properties(&id.body, Tol::witness()).unwrap().volume;
    let vd = topo::mass_properties(&dr.body, Tol::witness()).unwrap().volume;
    println!("s_duct volume: identity placement {vi:.15}, door placement {vd:.15}");
}

/// The composed roll's origin: `rotation_about_axis(p, d, a) * frame`
/// re-derives the translation as `R·q + (I−R)·q`. Is it still `q`?
#[test]
fn r2_p5_composed_roll_moves_the_origin_by_ulps() {
    let path = torsion_path();
    let (lo, hi) = path.domain();
    let mut worst = 0.0f64;
    for i in 0..6 {
        let t = (hi - lo).mul_add(i as f64 / 5.0, lo);
        let (p, d) = (path.eval(t), path.deriv(t));
        let plane = path_start_frame(p, d, Tol::witness()).unwrap();
        let rolled = Affine3::rotation_about_axis(p, d, 1.0 * i as f64 / 5.0) * plane;
        let off = (rolled.translation - plane.translation).norm();
        let ident = Affine3::rotation_about_axis(p, d, 0.0) * plane;
        println!(
            "station {i}: origin moved by {off:.3e}; R(0)·plane bitwise plane? {}",
            bits12(&ident) == bits12(&plane)
        );
        worst = worst.max(off);
    }
    println!("worst origin drift {worst:.3e}");
}

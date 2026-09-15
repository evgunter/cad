//! R1 review probes for S393 — not for merge as-is.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use std::f64::consts::FRAC_PI_2;

use crate::common::{normal_start_place, quad};
use geom::NurbsCurve3;
use geom_core::{Affine3, Mat3, Point3, Tol, Vec3};
use sweep::sweep_body;

/// The retired hand cone, transcribed verbatim from the merge base's
/// `common/mod.rs::normal_start_place`.
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

fn bits(a: &Affine3<f64>) -> [u64; 12] {
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

const R: f64 = 1.0;
const PITCH: f64 = 0.2;

fn helix_path(turns: f64) -> NurbsCurve3<f64> {
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let n = ((32.0 * turns).ceil() as usize).max(32);
    let pts: Vec<Point3<f64>> = (0..=n)
        .map(|k| {
            let s = f64::from(u32::try_from(k).unwrap()) / f64::from(u32::try_from(n).unwrap());
            let a = std::f64::consts::TAU * turns * s;
            Point3::new(R * a.cos(), R * a.sin(), PITCH * turns * s)
        })
        .collect();
    NurbsCurve3::<f64>::interpolate(&pts, 3).unwrap()
}

/// The `cert5` lily-crescent spine, transcribed from that suite.
fn lily_spine() -> NurbsCurve3<f64> {
    let (len, curl) = (1.25f64, -0.40f64);
    let r = len / curl;
    let pts: Vec<Point3<f64>> = (0..=12)
        .map(|k| {
            let s = f64::from(k) / 12.0;
            let a = curl * s;
            Point3::new(r * a.sin(), r * (1.0 - a.cos()), 0.0)
        })
        .collect();
    NurbsCurve3::<f64>::interpolate(&pts, 3).unwrap()
}

/// CLAIM 1/2: outside the cone the door and the retired copy agree bit
/// for bit; the inflecting path is the only corpus fixture inside it.
#[test]
fn r1_cone_versus_door_over_the_corpus() {
    let cases: Vec<(&str, NurbsCurve3<f64>)> = vec![
        ("inflecting_path", inflecting_path()),
        ("torsion_path", torsion_path()),
        ("helix_path(0.5)", helix_path(0.5)),
        ("helix_path(1.0)", helix_path(1.0)),
        ("helix_path(2.0)", helix_path(2.0)),
        ("lily_spine", lily_spine()),
    ];
    let mut inside = Vec::new();
    for (name, path) in &cases {
        let (lo, _) = path.domain();
        let d = path.deriv(lo);
        let n = d / d.norm();
        let cross = Vec3::new(0.0, 0.0, 1.0).cross(n).norm();
        let same = bits(&cone_place(path)) == bits(&normal_start_place(path));
        println!(
            "{name}: d=({:.6},{:.6},{:.6}) |n.z|={:.9} |Z x n|={:.6e} bitwise_equal={same}",
            d.x,
            d.y,
            d.z,
            n.z.abs(),
            cross
        );
        if n.z.abs() >= 0.9 {
            inside.push(*name);
            assert!(!same, "{name}: inside the cone the two must differ");
        } else {
            assert!(
                same,
                "{name}: outside the cone the two must agree bit for bit"
            );
        }
    }
    println!("inside the cone: {inside:?}");
    assert_eq!(inside, vec!["inflecting_path"]);
}

/// CLAIM 3: the duct's change is R_z(90 deg) exactly in the stored bits.
#[test]
fn r1_the_inflecting_ducts_frames_differ_by_a_quarter_turn_about_the_tangent() {
    let path = inflecting_path();
    let door = normal_start_place(&path);
    let cone = cone_place(&path);
    for (tag, rz) in [
        (
            "+90",
            Mat3::from_cols(
                Vec3::new(0.0, 1.0, 0.0),
                Vec3::new(-1.0, 0.0, 0.0),
                Vec3::new(0.0, 0.0, 1.0),
            ),
        ),
        (
            "-90",
            Mat3::from_cols(
                Vec3::new(0.0, -1.0, 0.0),
                Vec3::new(1.0, 0.0, 0.0),
                Vec3::new(0.0, 0.0, 1.0),
            ),
        ),
    ] {
        let composed = Affine3::from_parts(door.linear * rz, door.translation);
        let worst = [
            (cone.linear.c0 - composed.linear.c0).norm(),
            (cone.linear.c1 - composed.linear.c1).norm(),
            (cone.linear.c2 - composed.linear.c2).norm(),
        ]
        .into_iter()
        .fold(0.0f64, f64::max);
        println!(
            "door*Rz({tag}) vs cone: bitwise_equal={} worst_column_offset={worst:e}",
            bits(&cone) == bits(&composed)
        );
    }
    println!("cone c0 = {:?}", cone.linear.c0);
    println!("door c0 = {:?}", door.linear.c0);
    println!("door c1 = {:?}", door.linear.c1);
}

/// CLAIM 3/8: the bodies. Same station rings, different solid.
#[test]
fn r1_the_inflecting_duct_bodies_measured_both_ways() {
    let path = inflecting_path();
    let h = 0.25;
    let profile = quad([(-h, -h), (h, -h), (h, h), (-h, h)]);
    let build =
        |place| sweep_body::<f64>(&profile, place, &path, 13, 3, Tol::witness()).expect("sweeps");
    let old = build(cone_place(&path));
    let new = build(normal_start_place(&path));
    for (tag, b) in [("cone", &old), ("door", &new)] {
        let m = topo::props::mass_properties(&b.body, Tol::witness()).expect("props");
        let c = common::orient::ring_centroid(b, 0.5);
        println!("{tag}: volume = {:.15} pad = {:e}", m.volume, m.volume_pad);
        println!(
            "{tag}: ring centroid at v=0.5 = ({:.12}, {:.12}, {:.12})",
            c.x, c.y, c.z
        );
    }
    let a_l = (2.0 * h) * (2.0 * h) * (2.0 * FRAC_PI_2 * S_RADIUS);
    println!("continuum A.L = {a_l:.15}");
    let (c_old, c_new) = (
        common::orient::ring_centroid(&old, 0.5),
        common::orient::ring_centroid(&new, 0.5),
    );
    println!("centroid separation = {:.6}", (c_new - c_old).norm());

    // CLAIM: every station's ring is the same SET of world points.
    let verts = |b: &sweep::Lofted<f64>| -> Vec<(f64, f64, f64)> {
        let mut v: Vec<(f64, f64, f64)> = b
            .body
            .points()
            .map(|(_, x)| (x.x, x.y, x.z))
            .collect();
        v.sort_by(|a, b| a.partial_cmp(b).unwrap());
        v
    };
    let (va, vb) = (verts(&old), verts(&new));
    println!("vertex counts: cone {} door {}", va.len(), vb.len());
    let worst = va
        .iter()
        .zip(&vb)
        .map(|(p, q)| {
            ((p.0 - q.0).powi(2) + (p.1 - q.1).powi(2) + (p.2 - q.2).powi(2)).sqrt()
        })
        .fold(0.0f64, f64::max);
    println!("worst vertex-set mismatch (sorted) = {worst:e}");
}

/// CLAIM 3 (anti-vacuity): the re-shaped reversal condition. Sum the
/// negative and positive turn increments apart, on the inflecting path
/// and on a path that turns ONE way throughout, and print both against
/// the bar.
#[test]
fn r1_the_reshaped_reversal_condition_can_go_red() {
    let one_way: NurbsCurve3<f64> = {
        // A single quarter arc in the same plane: the elbow shape.
        let pts: Vec<Point3<f64>> = (0..=8)
            .map(|k| {
                let th = FRAC_PI_2 * f64::from(k) / 8.0;
                Point3::new(0.0, S_RADIUS * (1.0 - th.cos()), S_RADIUS * th.sin())
            })
            .collect();
        NurbsCurve3::<f64>::interpolate(&pts, 3).unwrap()
    };
    let h = 0.25;
    let profile = quad([(-h, -h), (h, -h), (h, h), (-h, h)]);
    let plane_normal = Vec3::new(1.0, 0.0, 0.0);
    let signed_angle_about = |axis: Vec3<f64>, a: Vec3<f64>, b: Vec3<f64>| -> f64 {
        let n = axis / axis.norm();
        let a = a - n * a.dot(n);
        let b = b - n * b.dot(n);
        a.cross(b).dot(n).atan2(a.dot(b))
    };
    for (tag, path) in [
        ("inflecting (shipped)", inflecting_path()),
        ("one-way quarter arc", one_way),
    ] {
        let swept = sweep_body::<f64>(
            &profile,
            normal_start_place(&path),
            &path,
            13,
            3,
            Tol::witness(),
        )
        .expect("sweeps");
        let steps = 64usize;
        #[allow(clippy::cast_precision_loss)]
        let centres: Vec<Point3<f64>> = (0..=steps)
            .map(|i| common::orient::ring_centroid(&swept, i as f64 / steps as f64))
            .collect();
        let chords: Vec<Vec3<f64>> = centres.windows(2).map(|w| w[1] - w[0]).collect();
        let (neg, pos) = chords.windows(2).fold((0.0, 0.0), |(n, p), w| {
            let d = signed_angle_about(plane_normal, w[0], w[1]);
            if d < 0.0 { (n + d, p) } else { (n, p + d) }
        });
        // The row's OLD form, for comparison.
        let mid = chords.len() / 2;
        let half = |c: &[Vec3<f64>]| -> f64 {
            c.windows(2)
                .map(|w| signed_angle_about(plane_normal, w[0], w[1]))
                .sum()
        };
        let bar = 0.9 * FRAC_PI_2;
        println!(
            "{tag}: new form neg={neg:.6} pos={pos:.6} | old form first={:.6} second={:.6} | bar={bar:.6} -> new passes: {}",
            half(&chords[..=mid]),
            half(&chords[mid..]),
            neg <= -bar && pos >= bar
        );
    }
}

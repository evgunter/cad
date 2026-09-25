//! **The path sweep's start frame is the kernel's door**, and what
//! that changed.
//!
//! The corpus used to build its own start frame: a hard cone
//! (`+Z` if `|n̂.z| < 0.9`, else `+X`) picking the helper axis the
//! in-plane axes are rolled off. `path_start_frame` takes `+Z`
//! whenever `|Ẑ × n̂|` decides definitely positive under the linear
//! band instead, so the two agree outside the cone and disagree inside
//! it. Three rows:
//!
//! 1. the differential itself, over every path this corpus and the
//!    demo tour sweep — bitwise equal outside the cone, and named
//!    fixtures inside it;
//! 2. what the disagreement costs on the one shipped fixture inside
//!    the cone: the same station rings, a different solid;
//! 3. why an identity placement is not an alternative on such a path.
//!
//! The retired recipe is transcribed here, and only here, because a
//! differential needs both sides.

#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::cast_precision_loss
)]

use core::f64::consts::FRAC_PI_2;

use geom::NurbsCurve3;
use geom_core::linalg::frame::path_start_frame;
use geom_core::{Affine3, Mat3, Point2, Point3, Tol, Vec3};
use sweep::skin::{segment_curve, sweep_places};
use sweep::test_support::bulge_arc;
use sweep::{Lofted, sweep_body};

use crate::common;
use common::orient::ring_centroid;
use common::{normal_start_place, quad};

/// The retired hand recipe, transcribed: the `0.9` cone picking the
/// helper axis, then Gram–Schmidt. The only copy of it left in the
/// tree, kept because the rows below are differentials against it.
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

/// The twelve stored numbers of a placement, as bits.
fn bits12(a: &Affine3<f64>) -> [u64; 12] {
    let (m, t) = (a.linear, a.translation);
    [
        m.c0.x, m.c0.y, m.c0.z, m.c1.x, m.c1.y, m.c1.z, m.c2.x, m.c2.y, m.c2.z, t.x, t.y, t.z,
    ]
    .map(f64::to_bits)
}

const S_RADIUS: f64 = 2.0;
const H: f64 = 0.25;

/// The S: two opposed quarter arcs in `x = 0`, 17 exact points at
/// degree 3. `turning_orientation`'s `inflecting_path` and the tour's
/// `s_duct` spine are this curve.
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

fn helix_path(turns: f64) -> NurbsCurve3<f64> {
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let n = ((32.0 * turns).ceil() as usize).max(32);
    let pts: Vec<Point3<f64>> = (0..=n)
        .map(|k| {
            let s = k as f64 / n as f64;
            let a = core::f64::consts::TAU * turns * s;
            Point3::new(a.cos(), a.sin(), 0.4 * turns * s)
        })
        .collect();
    NurbsCurve3::<f64>::interpolate(&pts, 3).unwrap()
}

/// The `cert5` lily-crescent spine: planar in `z = 0`.
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

/// The tour's sweep-cell path: one arc converted through the sketch
/// door, which is how a user gets a path from a sketch segment.
fn tour_arc_path() -> NurbsCurve3<f64> {
    segment_curve(
        0,
        bulge_arc(Point2::new(0.0, 0.0), Point2::new(3.0, 3.0), 0.4),
        Affine3::identity(),
    )
    .unwrap()
}

/// A straight path leaning on world +Z at `|n̂.z| = 0.95` — deep inside
/// the retired cone, with both off-axis components nonzero.
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
        ("inflecting_path (S duct, s_duct)", inflecting_path()),
        ("torsion_path (twisted duct)", torsion_path()),
        ("helix 0.5", helix_path(0.5)),
        ("helix 1.0", helix_path(1.0)),
        ("helix 2.0", helix_path(2.0)),
        ("lily spine", lily_spine()),
        ("tour arc path", tour_arc_path()),
        ("leaning 0.95", leaning_path()),
    ]
}

fn swept_square(path: &NurbsCurve3<f64>, place: Affine3<f64>, stations: usize) -> Lofted<f64> {
    let profile = quad([(-H, -H), (H, -H), (H, H), (-H, H)]);
    sweep_body::<f64>(&profile, place, path, stations, 3, Tol::witness()).unwrap()
}

fn volume(b: &Lofted<f64>) -> f64 {
    topo::mass_properties(&b.body, Tol::witness())
        .unwrap()
        .volume
}

/// **The door IS the retired recipe outside the cone, bit for bit, and
/// is NOT it inside** — over every path this corpus and the demo tour
/// sweep.
///
/// The cone is not a policy any more, so what this row protects is the
/// claim the change rests on: folding the copies moved exactly the
/// fixtures inside `|n̂.z| ≥ 0.9` and nothing else. A door whose helper
/// choice drifted would redden it at the first fixture that changed
/// side.
///
/// ANTI-VACUITY: the set of fixtures inside the cone is named, so a
/// corpus that drifted out of the interesting case cannot leave the
/// row trivially green.
#[test]
fn the_door_is_the_retired_cone_recipe_outside_the_cone_and_not_inside() {
    let tol = Tol::witness();
    let mut inside = Vec::new();
    for (name, path) in fixtures() {
        let (lo, _) = path.domain();
        let d = path.deriv(lo);
        let n = d / d.norm();
        let door = normal_start_place(&path);
        assert_eq!(
            bits12(&door),
            bits12(&path_start_frame(path.eval(lo), d, tol).unwrap()),
            "{name}: the corpus helper must BE the door, not a second spelling of it"
        );
        let same = bits12(&cone_place(&path)) == bits12(&door);
        println!(
            "{name:34} |n.z| = {:.9}  |Z x n| = {:.3e}  door == retired cone: {same}",
            n.z.abs(),
            Vec3::new(0.0, 0.0, 1.0).cross(n).norm()
        );
        if n.z.abs() < 0.9 {
            assert!(
                same,
                "{name}: outside the cone the two must agree bit for bit"
            );
        } else {
            inside.push(name);
            assert!(!same, "{name}: inside the cone the two must differ");
        }
    }
    assert_eq!(
        inside,
        vec!["inflecting_path (S duct, s_duct)", "leaning 0.95"],
        "the fixtures inside the retired cone are the S duct and the leaning pin"
    );
}

/// **The two frames place the same rings and build a different
/// solid** — the one shipped fixture inside the cone, measured.
///
/// The retired frame is the door's turned by `R_z(−90°)` about the
/// start tangent, exactly in the stored bits, and a centred square is
/// invariant under that turn: every station's ring is the same set of
/// world points. The body still moves, because the loft's
/// v-parameterization is the FIRST STRIP's and the turn changes which
/// profile edge that is — and the kernel's frame lands closer to the
/// continuum `A·L`.
///
/// ANTI-VACUITY: the ring identity and the volume difference are
/// asserted together. Either alone is satisfiable by a change that did
/// nothing (identical frames) or by one that moved the material.
#[test]
fn the_ducts_two_frames_place_the_same_rings_and_build_a_different_body() {
    let path = inflecting_path();
    let door = normal_start_place(&path);
    let cone = cone_place(&path);

    // R_z(−90°) under the module's column convention: x̂ ↦ −ŷ, ŷ ↦ x̂.
    let rz_minus_90 = Mat3::from_cols(
        Vec3::new(0.0, -1.0, 0.0),
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::unit_z(),
    );
    assert_eq!(
        bits12(&cone),
        bits12(&Affine3::from_parts(
            door.linear * rz_minus_90,
            door.translation
        )),
        "the retired frame is the door's quarter-turned about the tangent, exactly"
    );

    let stations = 13;
    let corners = [(-H, -H), (H, -H), (H, H), (-H, H)];
    let ring = |p: &Affine3<f64>| corners.map(|(x, y)| p.transform_point(Point3::new(x, y, 0.0)));
    let (pc, pd) = (
        sweep_places(cone, &path, stations).unwrap(),
        sweep_places(door, &path, stations).unwrap(),
    );
    let mut worst = 0.0f64;
    for (a, b) in pc.iter().zip(&pd) {
        for q in ring(a) {
            let d = ring(b)
                .iter()
                .map(|r| (*r - q).norm())
                .fold(f64::INFINITY, f64::min);
            worst = worst.max(d);
        }
    }
    println!("station rings: worst nearest-corner distance {worst:.3e}");
    assert!(
        worst < 1e-15,
        "every station's ring must be the SAME set of world points under both \
         frames; worst corner is {worst:e} away from its partner"
    );

    let (vc, vd) = (
        volume(&swept_square(&path, cone, stations)),
        volume(&swept_square(&path, door, stations)),
    );
    let a_l = (2.0 * H) * (2.0 * H) * (2.0 * S_RADIUS * FRAC_PI_2);
    println!("volume: retired {vc:.15}  door {vd:.15}  A.L {a_l:.15}");
    assert!(
        (vd - vc).abs() > 1e-6,
        "the same rings must still build a different solid (the loft's v comes from \
         the first strip): the two volumes differ by only {:e}",
        (vd - vc).abs()
    );
    assert!(
        (a_l - vd).abs() < (a_l - vc).abs(),
        "the kernel's frame must land closer to the continuum A.L = {a_l}: door {vd}, \
         retired {vc}"
    );
    let (cc, cd) = (
        ring_centroid(&swept_square(&path, cone, stations), 0.5),
        ring_centroid(&swept_square(&path, door, stations), 0.5),
    );
    println!("ring centroid at v = 0.5: retired {cc:?}  door {cd:?}");
    assert!(
        (cd - cc).norm() > 1e-3,
        "v = 0.5 must land somewhere else on the spine under the re-parameterization"
    );
}

/// **An identity placement is not the normal plane on this path**, and
/// a scene that used one was drawing its profile in a tilted plane.
///
/// The S's start tangent is near world +Z and not on it: the
/// interpolant departs at `|n̂.z| = 0.99999911`, so identity's +Z is
/// about `1.3e-3` rad off the tangent. That is small and it is not
/// nothing — the swept body differs.
///
/// ANTI-VACUITY: the tilt is asserted to be in a band, so a fixture
/// that drifted onto the axis (making identity correct by accident)
/// reddens the row instead of silently making it free.
#[test]
fn an_identity_start_placement_is_not_the_normal_plane_of_the_s_path() {
    let path = inflecting_path();
    let (lo, _) = path.domain();
    let tilt = path
        .deriv(lo)
        .normalize()
        .dot(Vec3::unit_z())
        .clamp(-1.0, 1.0)
        .acos();
    println!("identity's +Z is {tilt:.6e} rad off the S path's start tangent");
    assert!(
        (1e-3..2e-3).contains(&tilt),
        "the S path must depart NEAR world +Z and not ON it; measured {tilt} rad"
    );

    let (vi, vd) = (
        volume(&swept_square(&path, Affine3::identity(), 13)),
        volume(&swept_square(&path, normal_start_place(&path), 13)),
    );
    println!("s_duct volume: identity {vi:.15}  door {vd:.15}");
    assert!(
        (vd - vi).abs() > 1e-9,
        "the tilted profile plane must build a different body: the two volumes differ \
         by only {:e}",
        (vd - vi).abs()
    );
}

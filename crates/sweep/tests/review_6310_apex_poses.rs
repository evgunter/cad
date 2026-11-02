//! Review probes for PR 6310: planes through a cone's apex across
//! half-angles 15°–75° and tilts, every op both ways, held to the
//! closed form, to the kernel's membership lattice, and to an
//! independent mesh oracle (ray parity on the tessellation, and the
//! mesh's divergence volume). Review-branch only.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common::membership::{self, Region};
use crate::common::wrap_tip::{NORTH, one_wrap_edge};
use core::f64::consts::PI;
use geom_core::{Affine3, Point2, Point3, Tol, Vec3};
use sweep::Revolution;
use sweep::test_support::{finished, revolved_about_y};
use topo::{AtRestBody, BooleanResult, intersect, subtract, union};

fn tol() -> Tol {
    Tol::witness()
}

fn cone(alpha: f64) -> topo::Body<f64> {
    let r = alpha.to_radians().tan();
    revolved_about_y(
        vec![
            (Point2::new(0.0, 0.0), 0.0),
            (Point2::new(r, 0.0), 0.0),
            (Point2::new(0.0, 1.0), 0.0),
        ],
        Revolution::Full,
        tol(),
    )
}

fn tipped(alpha: f64) -> AtRestBody<f64> {
    finished("the tipped cone", one_wrap_edge(cone(alpha), true).0, tol())
}

fn revolved(alpha: f64) -> AtRestBody<f64> {
    crate::revolve_common::needle(alpha, None, tol())
}

fn the_cone(r: f64) -> Region {
    Box::new(move |p: Point3<f64>| (r * (1.0 - p.y) - p.x.hypot(p.z)).min(p.y).min(1.0 - p.y))
}

fn half_space(n: Vec3<f64>) -> AtRestBody<f64> {
    let n = n.normalize();
    let b = topo::test_support::brick((0.0, 8.0), (-4.0, 4.0), (-4.0, 4.0), tol());
    let x = Vec3::new(1.0, 0.0, 0.0);
    let turn = if x.cross(n).norm() < 1e-15 {
        Affine3::rotation_about_axis(
            Point3::origin(),
            Vec3::new(0.0, 0.0, 1.0),
            PI * (1.0 - n.x) / 2.0,
        )
    } else {
        Affine3::rotation_about_axis(
            Point3::origin(),
            x.cross(n).normalize(),
            x.dot(n).clamp(-1.0, 1.0).acos(),
        )
    };
    let m = Affine3::translation(NORTH - Point3::origin()) * turn;
    let moved =
        topo::transform_rigid(&b, &geom_core::test_support::rigid(&m, tol()), tol()).unwrap();
    finished("the half-space", moved, tol())
}

fn the_half_space(n: Vec3<f64>) -> Region {
    // The cube exactly: `p` taken back through the rotation `half_space`
    // applies (x onto n), then the brick's own bounds.
    let n = n.normalize();
    let x = Vec3::new(1.0, 0.0, 0.0);
    let k = x.cross(n);
    assert!(k.norm() > 1e-9, "probe tilts never hold the x axis");
    let k = k.normalize();
    let th = -x.dot(n).clamp(-1.0, 1.0).acos();
    Box::new(move |p: Point3<f64>| {
        let v = p - NORTH;
        let l = v * th.cos() + k.cross(v) * th.sin() + k * k.dot(v) * (1.0 - th.cos());
        l.x.min(8.0 - l.x).min(4.0 - l.y.abs()).min(4.0 - l.z.abs())
    })
}

fn cone_beyond(r: f64, n: Vec3<f64>) -> f64 {
    let m = n.x.hypot(n.z);
    let disc = PI * r * r;
    if m < 1e-15 {
        return if n.y < 0.0 { disc / 3.0 } else { 0.0 };
    }
    let d = n.y / m;
    let segment = if d >= r {
        0.0
    } else if d <= -r {
        disc
    } else {
        r * r * (d / r).acos() - d * (r * r - d * d).sqrt()
    };
    segment / 3.0
}

/// Ray parity against the mesh's triangles (Möller–Trumbore), along a
/// direction off every axis.
fn inside_mesh(m: &mesh::types::Mesh, p: Point3<f64>) -> bool {
    let d = Vec3::new(0.5773, 0.6181, 0.5331).normalize();
    let mut hits = 0;
    for patch in &m.patches {
        for t in &patch.triangles {
            let (a, b, c) = (
                m.positions[t[0] as usize],
                m.positions[t[1] as usize],
                m.positions[t[2] as usize],
            );
            let (e1, e2) = (b - a, c - a);
            let h = d.cross(e2);
            let det = e1.dot(h);
            if det.abs() < 1e-14 {
                continue;
            }
            let s = p - a;
            let u = s.dot(h) / det;
            if !(0.0..=1.0).contains(&u) {
                continue;
            }
            let q = s.cross(e1);
            let v = d.dot(q) / det;
            if v < 0.0 || u + v > 1.0 {
                continue;
            }
            if e2.dot(q) / det > 0.0 {
                hits += 1;
            }
        }
    }
    hits % 2 == 1
}

fn mesh_volume(m: &mesh::types::Mesh) -> f64 {
    let mut v = 0.0;
    for patch in &m.patches {
        for t in &patch.triangles {
            let (a, b, c) = (
                m.positions[t[0] as usize] - Point3::origin(),
                m.positions[t[1] as usize] - Point3::origin(),
                m.positions[t[2] as usize] - Point3::origin(),
            );
            v += a.dot(b.cross(c)) / 6.0;
        }
    }
    v
}

const DELTA: f64 = 2e-3;

fn independent(what: &str, b: &AtRestBody<f64>, want: f64, region: &Region, r: f64) {
    let m = mesh::tessellate(b, DELTA, tol()).unwrap_or_else(|e| panic!("{what}: mesh {e:?}"));
    mesh::validate::check_mesh(&m).unwrap_or_else(|e| panic!("{what}: check_mesh {e:?}"));
    let v = mesh_volume(&m);
    assert!(
        (v - want).abs() <= 0.01 * want.max(0.05) + 1e-3,
        "{what}: mesh volume {v}, closed form {want}"
    );
    let (lo, hi) = (
        Point3::new(-r - 0.25, -0.25, -r - 0.25),
        Point3::new(r + 0.25, 1.25, r + 0.25),
    );
    let n = 9;
    let mut asked = 0;
    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                let f = |t: usize, o: f64| (t as f64 + o) / (n as f64 - 1.0 + 0.41);
                let p = Point3::new(
                    lo.x + (hi.x - lo.x) * f(i, 0.13),
                    lo.y + (hi.y - lo.y) * f(j, 0.29),
                    lo.z + (hi.z - lo.z) * f(k, 0.05),
                );
                let s = region(p);
                if s.abs() < 20.0 * DELTA {
                    continue;
                }
                asked += 1;
                assert_eq!(
                    inside_mesh(&m, p),
                    s > 0.0,
                    "{what}: mesh parity at {p:?}, region {s}"
                );
            }
        }
    }
    assert!(asked > 50, "{what}: only {asked} lattice points asked");
    eprintln!("ORACLE {what}: mesh vol {v} want {want}, {asked} parity points");
}

fn six(what: &str, (c, r): (&AtRestBody<f64>, f64), n: Vec3<f64>) {
    let h = half_space(n);
    let (vc, vh) = (PI * r * r / 3.0, 512.0);
    let share = cone_beyond(r, n);
    let lattice = (
        Point3::new(-r - 0.25, -0.25, -r - 0.25),
        Point3::new(r + 0.25, 1.25, r + 0.25),
    );
    let mk = |k: u8| -> Region {
        let (c, h) = (the_cone(r), the_half_space(n));
        match k {
            0 => Box::new(move |p| c(p).min(h(p))),
            1 => Box::new(move |p| c(p).max(h(p))),
            2 => Box::new(move |p| c(p).min(-h(p))),
            _ => Box::new(move |p| h(p).min(-c(p))),
        }
    };
    let mut fails = Vec::new();
    for (op, result, want, k) in [
        ("c ∩ h", intersect(c, &h, tol()), share, 0),
        ("h ∩ c", intersect(&h, c, tol()), share, 0),
        ("c ∪ h", union(c, &h, tol()), vc + vh - share, 1),
        ("h ∪ c", union(&h, c, tol()), vc + vh - share, 1),
        ("c ∖ h", subtract(c, &h, tol()), vc - share, 2),
        ("h ∖ c", subtract(&h, c, tol()), vh - share, 3),
    ] {
        let what = format!("{what}, {op}");
        match result {
            Ok(BooleanResult::Body(b)) => {
                let region = mk(k);
                let run = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    membership::holds_short_of_tier_3_prime(&what, &b.body, want, &region, lattice, 7);
                    assert_eq!(
                        topo::validate_pseudomanifold(&b.body, &b.contacts, tol()),
                        Ok(()),
                        "{what}: tier 3′"
                    );
                    // The independent oracle on the bounded results (the
                    // half-space's cube is meshed whole on ∪ and h ∖ c).
                    independent(&what, &b.body, want, &region, r);
                }));
                if let Err(e) = run {
                    let msg = e
                        .downcast_ref::<String>()
                        .cloned()
                        .or_else(|| e.downcast_ref::<&str>().map(|s| s.to_string()))
                        .unwrap_or_default();
                    fails.push(format!("{what}: {msg}"));
                }
            }
            Ok(BooleanResult::Empty) => {
                if want.abs() >= 1e-12 {
                    fails.push(format!("{what}: empty, not {want}"));
                }
            }
            Err(e) => fails.push(format!("{what}: refused {e:?}")),
        }
    }
    for f in &fails {
        eprintln!("FAIL {f}");
    }
    assert!(fails.is_empty(), "{what}: {} of 6 ops failed", fails.len());
}

/// Tilts: the plane's base line at signed distance `f·r` from the base
/// centre, at a few azimuths, so every plane passes through the apex
/// and cuts the base disc (the line pair).
fn tilts() -> Vec<(f64, f64)> {
    vec![
        (-0.85, 0.3),
        (-0.4, 1.9),
        (0.0, 2.6),
        (0.3, 4.0),
        (0.8, 5.5),
        (0.95, 0.9),
    ]
}

fn sweep_alpha(alpha: f64, revolved_form: bool) {
    let r = alpha.to_radians().tan();
    let c = if revolved_form { revolved(alpha) } else { tipped(alpha) };
    let mut bad = Vec::new();
    for (f, az) in tilts() {
        // n = (cos az, d, sin az) with d = n_y / |n_xz| = f·r.
        let n = Vec3::new(az.cos(), f * r, az.sin());
        let what = format!(
            "{} {alpha}°, f {f}, az {az}, n {n:?}",
            if revolved_form { "revolved" } else { "tipped" }
        );
        if std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| six(&what, (&c, r), n))).is_err()
        {
            bad.push(what);
        }
    }
    assert!(bad.is_empty(), "failing poses: {bad:#?}");
}

#[test]
fn review_6310_tipped_15() {
    sweep_alpha(15.0, false);
}
#[test]
fn review_6310_tipped_25() {
    sweep_alpha(25.0, false);
}
#[test]
fn review_6310_tipped_40() {
    sweep_alpha(40.0, false);
}
#[test]
fn review_6310_tipped_55() {
    sweep_alpha(55.0, false);
}
#[test]
fn review_6310_tipped_75() {
    sweep_alpha(75.0, false);
}
#[test]
fn review_6310_revolved_15() {
    sweep_alpha(15.0, true);
}
#[test]
fn review_6310_revolved_25() {
    sweep_alpha(25.0, true);
}
#[test]
fn review_6310_revolved_40() {
    sweep_alpha(40.0, true);
}
#[test]
fn review_6310_revolved_55() {
    sweep_alpha(55.0, true);
}
#[test]
fn review_6310_revolved_75() {
    sweep_alpha(75.0, true);
}

/// Claim 5 hunt: a plane through the apex holding the revolved cone's
/// seam ruling to within `η` (the seam's side read at its reach `L`,
/// `η·L` against ε), tilted `γ` off the cone's tangent plane along the
/// seam. The seam bound reads On, and the apex germ runs along the
/// seam, off the true section ruling by about `η / sin γ`. Prints each
/// op's outcome; asserts no `JoinDesync`.
#[test]
fn review_6310_a_plane_through_the_apex_nearly_holding_the_seam() {
    let eps = tol().eps();
    let mut desyncs = Vec::new();
    for alpha in [30.0f64, 60.0] {
        let r = alpha.to_radians().tan();
        let c = revolved(alpha);
        let l = (1.0 + r * r).sqrt();
        let d = Vec3::new(r, -1.0, 0.0) / l;
        let t = Vec3::new(1.0, r, 0.0).normalize();
        let b = d.cross(t);
        for gamma in [0.5f64, 0.1, 0.02, 0.005] {
            for sign in [1.0, -1.0] {
                for k in [0.0, 0.1, 0.5, 0.9, 2.0, 10.0] {
                    let eta = k * eps / l;
                    let n0 = t * gamma.cos() + b * (sign * gamma.sin());
                    let n = (n0 - d * eta).normalize();
                    let h = half_space(n);
                    let share = cone_beyond(r, n);
                    let vc = PI * r * r / 3.0;
                    for (op, res, want) in [
                        ("c ∩ h", intersect(&c, &h, tol()), share),
                        ("c ∪ h", union(&c, &h, tol()), vc + 512.0 - share),
                        ("c ∖ h", subtract(&c, &h, tol()), vc - share),
                        ("h ∖ c", subtract(&h, &c, tol()), 512.0 - share),
                    ] {
                        let what = format!("α {alpha} γ {gamma} sign {sign} η·L {k}ε {op}");
                        let line = match &res {
                            Ok(BooleanResult::Body(bb)) => {
                                let mp = topo::test_support::pinned_volume(&what, &bb.body, tol());
                                format!(
                                    "OK vol {} want {want} err {:e} pad {:e}",
                                    mp.volume,
                                    (mp.volume - want).abs(),
                                    mp.volume_pad
                                )
                            }
                            Ok(BooleanResult::Empty) => format!("EMPTY want {want}"),
                            Err(e) => {
                                let s = format!("{e:?}");
                                if s.starts_with("JoinDesync") {
                                    desyncs.push(format!("{what}: {s}"));
                                }
                                format!("ERR {}", s.chars().take(160).collect::<String>())
                            }
                        };
                        eprintln!("{what}: {line}");
                    }
                }
            }
        }
    }
    assert!(desyncs.is_empty(), "JoinDesync on legal input: {desyncs:#?}");
}

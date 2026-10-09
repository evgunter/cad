//! **Review r1's probes of PR 4399** (`GermLane::EdgePlane`): poses past
//! the PR's own, each op in both orders printed as one
//! [`outcome`] line plus whether the body meshes (`check_mesh`). Run
//! `cargo test -p sweep --release --test all tube_on_ball_r1 --
//! --ignored --nocapture`.
//!
//! The closed forms are re-derived here, not shared with the suite: a
//! ball of radius `R` about the origin, a solid of revolution whose end
//! rim lies on it at height `y0` and which holds the ball's part past
//! the rim's plane, shares with it the cap `πh²(3R − h)/3`, `h = R − y0`.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::{FRAC_PI_2, PI, SQRT_2};

use geom_core::{Affine3, Point2, Point3, Tol, Vec3};
use sweep::test_support::{ball_poled_y, ball_poled_z, brick, finished, revolved_about_y};
use sweep::{ExtrudeSide, Extrusion, Revolution, extrude};
use topo::{AtRestBody, BooleanError, BooleanResult};

use crate::common::differential::outcome;

fn tol() -> Tol {
    Tol::witness()
}

fn cap(r: f64, h: f64) -> f64 {
    PI * h * h * (3.0 * r - h) / 3.0
}

fn ball_v(r: f64) -> f64 {
    4.0 / 3.0 * PI * r * r * r
}

fn mesh_of(r: &Result<BooleanResult<f64>, BooleanError>) -> String {
    match r.as_ref().map(BooleanResult::body) {
        Ok(Some(bb)) => match mesh::tessellate(&bb.body, 5e-3, tol()) {
            Ok(m) => match mesh::validate::check_mesh(&m) {
                Ok(()) => "mesh=ok".into(),
                Err(e) => format!("mesh=BAD {e:?}").chars().take(80).collect(),
            },
            Err(e) => format!("mesh=refused {e:?}").chars().take(80).collect(),
        },
        _ => String::new(),
    }
}

/// Every op of `a` (volume `va`) and `b` (`vb`) sharing `vab`, both
/// orders, one line each.
fn six(name: &str, a: &AtRestBody<f64>, b: &AtRestBody<f64>, (va, vb, vab): (f64, f64, f64)) {
    let t = tol();
    let runs = [
        ("A∪B", topo::union(a, b, t), va + vb - vab),
        ("B∪A", topo::union(b, a, t), va + vb - vab),
        ("A∩B", topo::intersect(a, b, t), vab),
        ("B∩A", topo::intersect(b, a, t), vab),
        ("A∖B", topo::subtract(a, b, t), va - vab),
        ("B∖A", topo::subtract(b, a, t), vb - vab),
    ];
    for (op, r, want) in runs {
        let m = mesh_of(&r);
        println!("R1 {name} {op} => {} {m}", outcome(r, want, t));
    }
}

/// The `z`-axis tube of radius `a` over `z ∈ [z0, z0 + len]`, tilted
/// about `x` then turned about `z` about the origin.
fn tube(a: f64, z0: f64, len: f64, tilt: f64, turn: f64) -> AtRestBody<f64> {
    let t = tol();
    let circle = profile::circle(Point2::new(0.0, 0.0), a, t).unwrap();
    let plane = profile::SketchPlane::new(Affine3::translation(Vec3::new(0.0, 0.0, z0)));
    let p = profile::Profile::new(plane, vec![circle.into()])
        .validate(t)
        .unwrap();
    let side = ExtrudeSide::Along;
    let body = extrude(&p, Extrusion::Distance { depth: len, side }, t)
        .unwrap()
        .body;
    let o = Point3::origin();
    let place = Affine3::rotation_about_axis(o, Vec3::unit_z(), turn)
        * Affine3::rotation_about_axis(o, Vec3::unit_x(), tilt);
    finished("tube", topo::transform_rigid(&body, &place, t).unwrap(), t)
}

fn ball_z(r: f64) -> AtRestBody<f64> {
    finished("ball", ball_poled_z(r, Vec3::zero(), tol()), tol())
}

/// Family 1: tubes on the ball of radius √2 past the PR's poses — the
/// rim near the equator, the tube downward (tilt π) and askew below,
/// short tubes whose top cap also cuts the ball, and the rim moved off
/// the sphere by `d` along the tube's axis.
#[test]
#[ignore = "review probe; --ignored --nocapture"]
fn tube_on_ball_r1_family_tubes() {
    let r = SQRT_2;
    let b = ball_z(r);
    let vb = ball_v(r);
    // (a, len, tilt, turn, d)
    let poses: &[(f64, f64, f64, f64, f64)] = &[
        (1.0, 2.0, 0.0, 0.0, 0.0),
        (1.3, 2.0, 0.0, 0.0, 0.0),
        (1.4, 2.0, 0.0, 0.0, 0.0),
        (1.41, 2.0, 0.0, 0.0, 0.0),
        (1.414, 2.0, 0.0, 0.0, 0.0),
        (1.0, 2.0, PI, 0.0, 0.0),
        (1.3, 2.0, PI, 0.0, 0.0),
        (0.7, 2.0, PI - 0.5, 0.3, 0.0),
        (1.0, 2.0, PI, 0.4, 0.0),
        (1.0, 0.2, 0.0, 0.0, 0.0),
        (1.3, 0.1, 0.0, 0.0, 0.0),
        (1.0, 0.2, PI, 0.0, 0.0),
        (1.0, 2.0, 0.0, 0.0, 1e-12),
        (1.0, 2.0, 0.0, 0.0, -1e-12),
        (1.0, 2.0, 0.0, 0.0, 1e-10),
        (1.0, 2.0, 0.0, 0.0, -1e-10),
        (1.0, 2.0, 0.0, 0.0, 1e-9),
        (1.0, 2.0, 0.0, 0.0, -1e-9),
        (1.0, 2.0, 0.0, 0.0, 1e-8),
        (1.0, 2.0, 0.0, 0.0, -1e-8),
        (1.0, 2.0, 0.0, 0.0, 1e-6),
        (1.0, 2.0, 0.0, 0.0, -1e-6),
    ];
    for &(a, len, tilt, turn, d) in poses {
        let z0 = (2.0 - a * a).sqrt();
        let t = tube(a, z0 + d, len, tilt, turn);
        let vt = PI * a * a * len;
        let lo = z0 + d;
        let hi = (lo + len).min(r);
        // The ball's part inside the tube's slab: for d < 0 the slab
        // between the rim and the sphere is the tube's full disc.
        let vab = if d < 0.0 {
            PI * a * a * (-d) + cap(r, r - z0) - cap(r, (r - (z0 + len + d)).max(0.0))
        } else {
            cap(r, r - lo) - cap(r, r - hi)
        };
        six(
            &format!("tube a={a} len={len} tilt={tilt:.3} turn={turn} d={d:e}"),
            &t,
            &b,
            (vt, vb, vab),
        );
    }
}

/// A revolved solid about the `y` axis from the sketch polygon `verts`.
fn rev_y(verts: &[(f64, f64)]) -> AtRestBody<f64> {
    let v = verts
        .iter()
        .map(|&(x, y)| (Point2::new(x, y), 0.0))
        .collect();
    finished("rev", revolved_about_y(v, Revolution::Full, tol()), tol())
}

/// Family 2: a cone or a frustum whose end rim lies on the ball (poled
/// `y`, the revolve's axis).
#[test]
#[ignore = "review probe; --ignored --nocapture"]
fn tube_on_ball_r1_family_cones() {
    let r = SQRT_2;
    let b = finished("ball", ball_poled_y(r, Vec3::zero(), tol()), tol());
    let vb = ball_v(r);
    let frustum = |r0: f64, r1: f64, h: f64| PI * h * (r0 * r0 + r0 * r1 + r1 * r1) / 3.0;
    // Upward, narrowing frustum from the rim (1, 1) to (0.5, 3).
    let up = |r0: f64, r1: f64, y1: f64| {
        let y0 = (2.0 - r0 * r0).sqrt();
        if r1 == 0.0 {
            (
                rev_y(&[(0.0, y0), (r0, y0), (0.0, y1)]),
                frustum(r0, 0.0, y1 - y0),
                y0,
            )
        } else {
            (
                rev_y(&[(0.0, y0), (r0, y0), (r1, y1), (0.0, y1)]),
                frustum(r0, r1, y1 - y0),
                y0,
            )
        }
    };
    for (name, r0, r1, y1) in [
        ("frustum narrowing", 1.0, 0.5, 3.0),
        ("frustum widening", 1.0, 2.0, 3.0),
        ("cone to an apex", 1.0, 0.0, 3.0),
        ("rim near the equator, widening", 1.4, 1.6, 2.0),
        ("cone steeper than the sphere (inside it)", 1.0, 0.0, 1.3),
    ] {
        let (c, vc, y0) = up(r0, r1, y1);
        // The steep cone lies inside the ball but for its rim.
        let vab = if name.contains("steeper") {
            vc
        } else {
            cap(r, r - y0)
        };
        six(name, &c, &b, (vc, vb, vab));
    }
    // Downward frustum: rim (1, −1) to (0.5, −3).
    let c = rev_y(&[(0.0, -3.0), (0.5, -3.0), (1.0, -1.0), (0.0, -1.0)]);
    six(
        "frustum downward",
        &c,
        &b,
        (frustum(1.0, 0.5, 2.0), vb, cap(r, r - 1.0)),
    );
}

/// Family 3: two tubes ending on one ball, the second against the
/// union of the first, and both from the ball.
#[test]
#[ignore = "review probe; --ignored --nocapture"]
fn tube_on_ball_r1_family_two_tubes() {
    let t = tol();
    let r = SQRT_2;
    let b = ball_z(r);
    let (vb, c) = (ball_v(r), cap(r, r - 1.0));
    let vt = 2.0 * PI;
    let up = tube(1.0, 1.0, 2.0, 0.0, 0.0);
    for (name, other) in [
        ("down", tube(1.0, 1.0, 2.0, PI, 0.0)),
        ("down turned", tube(1.0, 1.0, 2.0, PI, 0.4)),
        ("askew", tube(0.5, (2.0 - 0.25_f64).sqrt(), 2.0, 2.0, 0.0)),
    ] {
        let (a0, z0) = if name == "askew" {
            (0.5, (2.0 - 0.25_f64).sqrt())
        } else {
            (1.0, 1.0)
        };
        let (vo, co) = (PI * a0 * a0 * 2.0, cap(r, r - z0));
        println!(
            "R1 two/{name} first union => {}",
            outcome(topo::union(&b, &up, t), vb + vt - c, t)
        );
        let Ok(BooleanResult::Body(u)) = topo::union(&b, &up, t) else {
            continue;
        };
        let u = u.body;
        six(
            &format!("two/{name} (ball∪up) vs other"),
            &u,
            &other,
            (vb + vt - c, vo, co),
        );
        let s = topo::subtract(&b, &up, t);
        if let Ok(BooleanResult::Body(s)) = s {
            let s = s.body;
            six(
                &format!("two/{name} (ball∖up) vs other"),
                &s,
                &other,
                (vb - c, vo, co),
            );
        }
    }
}

/// Family 4: a tube on the ball and a plane through both — the ball
/// halved by a brick first (`x ≥ c` kept), so the rim crosses the
/// half-ball's planar face; the shared volume is the cap's part past
/// the plane, integrated in `z` (closed-form segment areas).
#[test]
#[ignore = "review probe; --ignored --nocapture"]
fn tube_on_ball_r1_family_plane() {
    let t = tol();
    let r = SQRT_2;
    let b = ball_z(r);
    // The area of the disc of radius `rho` past the chord `x = c`.
    let seg = |rho: f64, c: f64| {
        if rho <= c {
            0.0
        } else if rho <= -c {
            PI * rho * rho
        } else {
            rho * rho * (c / rho).acos() - c * (rho * rho - c * c).sqrt()
        }
    };
    // `∫ seg(√(R² − z²), c) dz` over `[z0, R]`, Simpson.
    let slab = |z0: f64, c: f64| {
        let n = 2_000_000;
        let h = (r - z0) / n as f64;
        let f = |z: f64| seg((2.0 - z * z).max(0.0).sqrt(), c);
        let mut s = f(z0) + f(r);
        for i in 1..n {
            s += f(z0 + i as f64 * h) * if i % 2 == 1 { 4.0 } else { 2.0 };
        }
        s * h / 3.0
    };
    for (name, c, turn) in [
        ("x≥0", 0.0, 0.0),
        ("x≥0.3", 0.3, 0.0),
        ("x≥-0.4", -0.4, 0.0),
        ("x≥0, tube turned 0.4", 0.0, 0.4),
    ] {
        let cut = finished("brick", brick((-5.0, c), (-5.0, 5.0), (-5.0, 5.0), t), t);
        let half = match topo::subtract(&b, &cut, t) {
            Ok(BooleanResult::Body(h)) => h.body,
            other => {
                println!("R1 plane/{name} half ball => {}", outcome(other, 0.0, t));
                continue;
            }
        };
        let vh = slab(-r, c);
        let tb = tube(1.0, 1.0, 2.0, 0.0, turn);
        six(
            &format!("plane/{name}"),
            &tb,
            &half,
            (2.0 * PI, vh, slab(1.0, c)),
        );
    }
}

/// Family 5: other holders of the edge's face — a cone's rim on a
/// cylinder, a tube's rim on a cone, two coaxial tubes of one radius
/// overlapping (the coaxial cylinder pair's `None`), and a tube on a
/// torus.
#[test]
#[ignore = "review probe; --ignored --nocapture"]
fn tube_on_ball_r1_family_holders() {
    let t = tol();
    // A tube of radius 1 along `y`, over y ∈ [0, 3].
    let tube_y = |a: f64, y0: f64, y1: f64| rev_y(&[(0.0, y0), (a, y0), (a, y1), (0.0, y1)]);
    let cyl = tube_y(1.0, 0.0, 2.5);
    // A cone widening from the rim (1, 1) to (2, 3): the cylinder above
    // y = 1 lies inside it.
    let cone_w = rev_y(&[(0.0, 1.0), (1.0, 1.0), (2.0, 3.0), (0.0, 3.0)]);
    let vcw = PI * 2.0 * (1.0 + 2.0 + 4.0) / 3.0;
    six(
        "holder/cone rim on cylinder",
        &cone_w,
        &cyl,
        (vcw, 2.5 * PI, 1.5 * PI),
    );
    // The cone r = y over y ∈ [0, 2] (apex at the origin); a tube of
    // radius 1 over y ∈ [1, 3] has its rim on it, inside it to y = 2.
    let cone = rev_y(&[(0.0, 0.0), (2.0, 2.0), (0.0, 2.0)]);
    let vcone = PI * 4.0 * 2.0 / 3.0;
    six(
        "holder/tube rim on cone",
        &tube_y(1.0, 1.0, 3.0),
        &cone,
        (2.0 * PI, vcone, PI),
    );
    // A frustum on a cone: the frustum r = 1 + 0.5 (y − 1) up to y = 3
    // starts on the cone r = y at y = 1; for y > 1 it is inside it.
    let fr = rev_y(&[(0.0, 1.0), (1.0, 1.0), (1.75, 2.5), (0.0, 2.5)]);
    let vfr = PI * 1.5 * (1.0 + 1.75 + 1.75 * 1.75) / 3.0;
    let cone3 = rev_y(&[(0.0, 0.0), (3.0, 3.0), (0.0, 3.0)]);
    six(
        "holder/frustum rim on cone",
        &fr,
        &cone3,
        (vfr, PI * 9.0, vfr),
    );
    // Two coaxial tubes of one radius, overlapping on y ∈ [1, 2].
    six(
        "holder/coaxial tubes overlapping",
        &tube_y(1.0, 0.0, 2.0),
        &tube_y(1.0, 1.0, 3.0),
        (2.0 * PI, 2.0 * PI, PI),
    );
    // The tube on a torus (R 2, r 1, axis y): radius 2 + 1/√2 standing
    // on the 45° latitude y = 1/√2.
    let torus = {
        let mut b = revolved_about_y(
            vec![(Point2::new(1.0, 0.0), 1.0), (Point2::new(3.0, 0.0), 1.0)],
            Revolution::Full,
            t,
        );
        b.merge_coplanar_faces(t).unwrap();
        finished("torus", b, t)
    };
    let s = core::f64::consts::FRAC_1_SQRT_2;
    let tor_v = 2.0 * PI * PI * 2.0;
    six(
        "holder/tube on a torus (outer 45°)",
        &tube_y(2.0 + s, s, s + 2.0),
        &torus,
        (
            PI * (2.0 + s).powi(2) * 2.0,
            tor_v,
            2.0 * PI * 2.0 * (PI / 4.0 - 0.5),
        ),
    );
    let _ = FRAC_PI_2;
}

/// Family 6: a LINE edge of one solid lying in a cylinder face — the
/// one-sided frame now reads the edge's line where it read the face
/// pair's section. A cylinder over z ∈ [−1, 2] about `(cx, cy)`
/// against the box `[0, 2]² × [0, 1]`.
#[test]
#[ignore = "review probe; --ignored --nocapture"]
fn tube_on_ball_r1_family_line_edges() {
    let t = tol();
    let cyl = |cx: f64, cy: f64, a: f64| {
        let circle = profile::circle(Point2::new(cx, cy), a, t).unwrap();
        let plane = profile::SketchPlane::new(Affine3::translation(Vec3::new(0.0, 0.0, -1.0)));
        let p = profile::Profile::new(plane, vec![circle.into()])
            .validate(t)
            .unwrap();
        let side = ExtrudeSide::Along;
        let body = extrude(&p, Extrusion::Distance { depth: 3.0, side }, t)
            .unwrap()
            .body;
        finished("cyl", body, t)
    };
    let bx = finished("box", brick((0.0, 2.0), (0.0, 2.0), (0.0, 1.0), t), t);
    // The box's edge x = y = 0 on the wall, the face y = 0 tangent to it
    // along that edge: they touch in the line alone.
    six(
        "line/tangent face",
        &cyl(0.0, -1.0, 1.0),
        &bx,
        (3.0 * PI, 4.0, 0.0),
    );
    // The wall through the box's corner edge, both faces crossing it.
    let h = 0.5_f64.sqrt();
    six(
        "line/crossing faces",
        &cyl(0.5, 0.5, h),
        &bx,
        (3.0 * PI * 0.5, 4.0, PI / 4.0 + 0.5),
    );
}

/// Probe: the aux planes `AuxDatum::EdgePlane` mints for the witness's
/// rim pieces — one per split edge (its doc: "the pieces of one split
/// edge share it"), or one per piece. Prints every plane surface of each
/// result at the rim's height and every edge described against one.
#[test]
#[ignore = "review probe; --ignored --nocapture"]
fn tube_on_ball_r1_aux_planes() {
    let t = tol();
    let b = ball_z(SQRT_2);
    for (turn, name) in [(0.0, "turn 0"), (0.4, "turn 0.4")] {
        let tb = tube(1.0, 1.0, 2.0, 0.0, turn);
        for (op, r) in [
            ("A∪B", topo::union(&tb, &b, t)),
            ("A∩B", topo::intersect(&tb, &b, t)),
            ("B∖A", topo::subtract(&b, &tb, t)),
            ("A∖B", topo::subtract(&tb, &b, t)),
        ] {
            let Ok(BooleanResult::Body(bb)) = r else {
                println!("R1AUX {name} {op}: no body");
                continue;
            };
            let body = &bb.body;
            let planes: Vec<_> = body
                .surfaces()
                .filter_map(|(k, s)| match *s {
                    geom::Surface::Plane { origin, normal, .. }
                        if (origin.z - 1.0).abs() < 1e-9 && normal.z.abs() > 0.999 =>
                    {
                        Some(k)
                    }
                    _ => None,
                })
                .collect();
            let mut described = Vec::new();
            for (e, ed) in body.edges() {
                let Some(c) = body
                    .get_curve_geom(ed.curve)
                    .and_then(topo::CurveGeom::certified)
                else {
                    continue;
                };
                if let geom_brep::EdgeDescription::Intersection { s1, s2, .. } = c.description() {
                    for s in [s1, s2] {
                        if planes.contains(s) {
                            described.push((e, *s));
                        }
                    }
                }
            }
            let used: std::collections::BTreeSet<_> = described.iter().map(|d| d.1).collect();
            println!(
                "R1AUX {name} {op}: z=1 planes {} ({planes:?}); edges described against them {}; distinct planes used {}",
                planes.len(),
                described.len(),
                used.len()
            );
        }
    }
}

/// Family 7: a one-sided along-edge germ on a pair that HAS a conic
/// frame — the ball's seam meridians lie in `y = 0`, so a brick face in
/// that plane holds them (plane × sphere, a circle frame), and the
/// frame is now read off the meridian edge instead of the pair. The
/// brick keeps `y ≥ d`; at `d = 0` that is half the ball, and in band
/// it is too.
#[test]
#[ignore = "review probe; --ignored --nocapture"]
fn tube_on_ball_r1_family_seam_plane() {
    let t = tol();
    let r = SQRT_2;
    let b = ball_z(r);
    for d in [0.0, 1e-12, -1e-12, 1e-6] {
        let k = finished("brick", brick((-5.0, 5.0), (d, 5.0), (-5.0, 5.0), t), t);
        // The ball's part at y ≥ d: a cap of height r − d.
        let vab = cap(r, r - d);
        six(
            &format!("seam-plane d={d:e}"),
            &b,
            &k,
            (ball_v(r), 10.0 * 10.0 * (5.0 - d), vab),
        );
    }
}

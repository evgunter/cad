//! **Review r2 probes for PR 4345** (a plane across a one-face wall):
//! families past the PR's six witnesses, each pose run as ∪, ∩, A∖B,
//! B∖A and ∪, ∩ with the operands swapped, read through
//! `common::differential::outcome` (tiers 2 and 3′, the certificate,
//! the legal-operand check, the closed-form volume) and, where it
//! builds, tessellated and `check_mesh`ed. One line per run, to diff
//! base against head. `#[ignore]`d: run with `--ignored --nocapture`.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::{PI, TAU};

use geom_core::{Affine3, Arc2, Point2, Point3, Tol, Vec2, Vec3};
use profile::{Profile, ProfileLoop, RawLoop, Segment, SketchPlane};
use sweep::test_support::{ball_poled_y, brick, revolved_about_y};
use sweep::{ExtrudeSide, Extrusion, Revolution, extrude};
use topo::{AtRestBody, Body};

use crate::common::differential::outcome;

fn tol() -> Tol {
    Tol::witness()
}

fn circle(c: (f64, f64), r: f64, az: f64, sweep: f64) -> ProfileLoop<f64> {
    RawLoop::new([(
        Point2::new(c.0 + r * az.cos(), c.1 + r * az.sin()),
        Segment::Arc(Arc2 {
            centre: Point2::new(c.0, c.1),
            radius: r,
            sweep,
        }),
    )])
}

fn extruded(loops: Vec<ProfileLoop<f64>>, z0: f64, h: f64) -> Body<f64> {
    let profile = Profile::new(SketchPlane::<f64>::xy(), loops)
        .validate(tol())
        .unwrap();
    let depth = Extrusion::Distance {
        depth: h,
        side: ExtrudeSide::Along,
    };
    let b = extrude(&profile, depth, tol()).unwrap().body;
    topo::transform_rigid(&b, &Affine3::translation(Vec3::new(0.0, 0.0, z0)), tol()).unwrap()
}

/// A one-segment circle's cylinder: one wall, its wrap edge at azimuth `az`.
fn seam_cyl(r: f64, c: (f64, f64), z0: f64, h: f64, az: f64) -> Body<f64> {
    extruded(vec![circle(c, r, az, TAU)], z0, h)
}

/// The half-space below the plane through `at` with normal `R·ẑ`.
fn below(at: Point3<f64>, turn: Affine3<f64>) -> Body<f64> {
    let raw = brick((-4.0, 4.0), (-4.0, 4.0), (-8.0, 0.0), tol());
    let to = Affine3::translation(at - Point3::origin());
    topo::transform_rigid(&raw, &(to * turn), tol()).unwrap()
}

/// Volume of the disc of radius `r` times `[0, f]`, `f` linear with value
/// `h0` at the centre and gradient magnitude `g` (`f ≤ H` assumed).
fn under_plane(r: f64, h0: f64, g: f64) -> f64 {
    let d = if g == 0.0 { -r } else { (-h0 / g).clamp(-r, r) };
    let s = (r * r - d * d).max(0.0).sqrt();
    h0 * (r * r * (d / r).acos() - d * s) + g * (2.0 / 3.0) * s * s * s
}

fn fin(what: &str, b: Body<f64>) -> AtRestBody<f64> {
    topo::test_support::finished(what, b, tol())
}

fn line(row: &str, r: Result<topo::BooleanResult<f64>, topo::BooleanError>, want: f64) {
    let mesh = match &r {
        Ok(res) => match res.body() {
            Some(bb) => match mesh::tessellate(&bb.body, 5e-3, tol()) {
                Ok(m) => match mesh::validate::check_mesh(&m) {
                    Ok(()) => "mesh=ok".to_string(),
                    Err(e) => format!("mesh=BAD {e:?}"),
                },
                Err(e) => format!("mesh=ERR {e:?}"),
            },
            None => "mesh=-".into(),
        },
        Err(_) => "mesh=-".into(),
    };
    let o = outcome(r, want, tol());
    println!("R2 {row} | {o} | {mesh}");
}

/// Every op in both orders.
fn every_op(what: &str, a: Body<f64>, b: Body<f64>, v: (f64, f64, f64)) {
    every_op_r(what, fin("A", a), fin("B", b), v);
}

fn every_op_r(what: &str, a: AtRestBody<f64>, b: AtRestBody<f64>, (va, vb, vab): (f64, f64, f64)) {
    let rows: [(&str, &AtRestBody<f64>, &AtRestBody<f64>, f64); 6] = [
        ("AuB", &a, &b, va + vb - vab),
        ("BuA", &b, &a, va + vb - vab),
        ("AnB", &a, &b, vab),
        ("BnA", &b, &a, vab),
        ("A-B", &a, &b, va - vab),
        ("B-A", &b, &a, vb - vab),
    ];
    for (op, x, y, want) in rows {
        let r = match &op[1..2] {
            "u" => topo::union(x, y, tol()),
            "n" => topo::intersect(x, y, tol()),
            _ => topo::subtract(x, y, tol()),
        };
        line(&format!("{what} {op}"), r, want);
    }
}

/// F1: the one-segment cylinder under a plane through `p` with normal
/// `R(axis, th)·ẑ`, centred off the origin, its wrap edge at `az`.
fn plane_pose(what: &str, r: f64, c: (f64, f64), h: f64, az: f64, p: Point3<f64>, axis: Vec3<f64>, th: f64) {
    let turn = Affine3::rotation_about_axis(Point3::origin(), axis, th);
    let n = turn.transform_vec(Vec3::new(0.0, 0.0, 1.0));
    let fz = |x: f64, y: f64| p.z - (n.x * (x - p.x) + n.y * (y - p.y)) / n.z;
    let h0 = fz(c.0, c.1);
    let g = (n.x * n.x + n.y * n.y).sqrt() / n.z;
    assert!(h0 + g * r < h - 1e-3, "{what}: the plane leaves the wall at the top");
    let vab = under_plane(r, h0, g);
    every_op(what, seam_cyl(r, c, 0.0, h, az), below(p, turn), (PI * r * r * h, 512.0, vab));
}

#[test]
#[ignore = "review probe; run with --ignored --nocapture"]
fn r2_f1_planes_across_a_one_segment_cylinder() {
    let x = Vec3::new(1.0, 0.0, 0.0);
    let y = Vec3::new(0.0, 1.0, 0.0);
    // Horizontal, off-centre, wrap edge at several azimuths.
    for (c, az, z) in [((0.0, 0.0), 0.0, 0.3), ((0.7, -0.4), 2.0, 1.7), ((-1.1, 0.9), 4.0, 1.0)] {
        plane_pose(&format!("F1 flat c={c:?} az={az} z={z}"), 1.0, c, 2.0, az, Point3::new(0.0, 0.0, z), x, 0.0);
    }
    // Tilted about x through the axis (the seam on the level line).
    for deg in [0.05f64, 1.0, 10.0, 30.0, 44.0] {
        plane_pose(&format!("F1 tilt-x {deg}deg"), 1.0, (0.0, 0.0), 2.0, 0.0, Point3::new(0.0, 0.0, 1.0), x, deg.to_radians());
    }
    // Tilted about y: steep planes meet the wrap edge at a shallow angle.
    for deg in [0.5f64, 30.0, 60.0, 80.0, 85.0, 88.0] {
        let t = deg.to_radians().tan();
        let h = 2.0 * t + 2.0;
        plane_pose(&format!("F1 tilt-y {deg}deg"), 1.0, (0.0, 0.0), h, 0.0, Point3::new(0.0, 0.0, h / 2.0), y, deg.to_radians());
    }
    // Tilt axes at several azimuths, off-centre cylinder, odd radius.
    for (k, adeg) in [0.0f64, 45.0, 90.0, 135.0, 200.0].into_iter().enumerate() {
        let a = adeg.to_radians();
        for deg in [15.0f64, 60.0] {
            let t = deg.to_radians().tan();
            let r = 0.37 + 0.2 * k as f64;
            let h = 2.0 * r * t + 2.0;
            let c = (0.3, -0.2);
            plane_pose(
                &format!("F1 axis{adeg} tilt{deg} r={r:.2}"),
                r,
                c,
                h,
                0.7,
                Point3::new(c.0, c.1, h / 2.0),
                Vec3::new(a.cos(), a.sin(), 0.0),
                deg.to_radians(),
            );
        }
    }
    // Close by the wrap edge's bottom end vertex (1,0,0), plane
    // z = c + m(1 − x).
    for c in [1e-2, 1e-4, 1e-6] {
        let m: f64 = 0.5;
        plane_pose(&format!("F1 near-end c={c}"), 1.0, (0.0, 0.0), 2.0, 0.0, Point3::new(1.0, 0.0, c), y, m.atan());
    }
}

#[test]
#[ignore = "review probe; run with --ignored --nocapture"]
fn r2_f1v_planes_through_the_wrap_edges_end_vertex() {
    // z = m(1 − x): tangent to the bottom cap at its vertex.
    let m: f64 = 0.5;
    let turn = Affine3::rotation_about_axis(Point3::origin(), Vec3::new(0.0, 1.0, 0.0), m.atan());
    every_op("F1v tangent-at-vertex", seam_cyl(1.0, (0.0, 0.0), 0.0, 2.0, 0.0), below(Point3::new(1.0, 0.0, 0.0), turn), (TAU, 512.0, PI * m));
    // z = m(1 − x) + k·y: transverse through the vertex, cutting the cap.
    for k in [0.3f64, 0.8] {
        let n = Vec3::new(m, -k, 1.0).normalize();
        let axis = Vec3::new(0.0, 0.0, 1.0).cross(n);
        let th = n.z.acos();
        let turn = Affine3::rotation_about_axis(Point3::origin(), axis.normalize(), th);
        let g = (m * m + k * k).sqrt();
        let vab = under_plane(1.0, m, g);
        every_op(&format!("F1v through-vertex k={k}"), seam_cyl(1.0, (0.0, 0.0), 0.0, 2.0, 0.0), below(Point3::new(1.0, 0.0, 0.0), turn), (TAU, 512.0, vab));
    }
}

fn tube(ri: f64, ro: f64) -> Body<f64> {
    let p = |x, y| (Point2::new(x, y), 0.0);
    revolved_about_y(vec![p(ri, -0.5), p(ro, -0.5), p(ro, 1.5), p(ri, 1.5)], Revolution::Full, tol())
}

#[test]
#[ignore = "review probe; run with --ignored --nocapture"]
fn r2_f2_tubes_under_boxes() {
    // Both radii, thin and thick; the slab y ∈ [y0, y1].
    for (ri, ro) in [(0.3, 0.5), (0.05, 0.1), (0.9, 1.2), (0.45, 0.5)] {
        for (y0, y1) in [(0.0, 1.0), (-0.2, 0.3)] {
            let ann = PI * (ro * ro - ri * ri);
            let block = brick((-1.5, 1.5), (y0, y1), (-1.5, 1.5), tol());
            every_op(&format!("F2 tube r=[{ri},{ro}] y=[{y0},{y1}]"), tube(ri, ro), block, (2.0 * ann, 9.0 * (y1 - y0), ann * (y1 - y0)));
        }
    }
    // Off-centre tube.
    let off = topo::transform_rigid(&tube(0.3, 0.5), &Affine3::translation(Vec3::new(0.6, 0.0, -0.3)), tol()).unwrap();
    let block = brick((-1.5, 1.5), (0.0, 1.0), (-1.5, 1.5), tol());
    every_op("F2 tube off-centre", off, block, (0.32 * PI, 9.0, 0.16 * PI));
    // The slab turned about z and about x through (0, 0.5, 0).
    for (name, axis) in [("z", Vec3::new(0.0, 0.0, 1.0)), ("x", Vec3::new(1.0, 0.0, 0.0))] {
        for deg in [3.0f64, 15.0] {
            let th = deg.to_radians();
            let raw = brick((-1.5, 1.5), (0.0, 1.0), (-1.5, 1.5), tol());
            let turned = topo::transform_rigid(&raw, &Affine3::rotation_about_axis(Point3::new(0.0, 0.5, 0.0), axis, th), tol()).unwrap();
            every_op(&format!("F2 tube slab-turned-{name} {deg}deg"), tube(0.3, 0.5), turned, (0.32 * PI, 9.0, 0.16 * PI / th.cos()));
        }
    }
}

#[test]
#[ignore = "review probe; run with --ignored --nocapture"]
fn r2_f3_rings_in_the_pierced_face() {
    let plate = || brick((-3.0, 3.0), (-3.0, 3.0), (0.0, 1.0), tol());
    let pin = |c: (f64, f64), r: f64| seam_cyl(r, c, -1.0, 3.0, 1.0);
    let raised = || seam_cyl(1.0, (0.0, 0.0), 0.5, 2.0, 0.0);
    let (r1, r2) = (0.15, 0.3);
    // Holes through the plate: one inside the conic, one outside.
    let a = fin("plate", plate());
    let h1 = topo::subtract(&a, &fin("pin", pin((0.3, 0.2), r1)), tol()).unwrap().body().unwrap().body.clone();
    let h2 = topo::subtract(&h1, &fin("pin", pin((2.0, 2.0), r2)), tol()).unwrap().body().unwrap().body.clone();
    let holed = 36.0 - PI * (r1 * r1 + r2 * r2);
    every_op_r("F3 holes in+out", h2, fin("B", raised()), (holed, TAU, 0.5 * PI * (1.0 - r1 * r1)));
    // A hole at the conic's centre.
    let hc = topo::subtract(&fin("plate", plate()), &fin("pin", pin((0.0, 0.0), 0.4)), tol()).unwrap().body().unwrap().body.clone();
    every_op_r("F3 hole centred", hc, fin("B", raised()), (36.0 - PI * 0.16, TAU, 0.5 * PI * (1.0 - 0.16)));
    // A boss through the top face inside the conic: z ∈ [0.5, 1.5].
    let boss = seam_cyl(r1, (0.3, 0.2), 0.5, 1.0, 1.0);
    let bossed = topo::union(&fin("plate", plate()), &fin("boss", boss), tol()).unwrap().body().unwrap().body.clone();
    let bv = PI * r1 * r1 * 0.5;
    every_op_r("F3 boss inside", bossed, fin("B", raised()), (36.0 + bv, TAU, 0.5 * PI + bv));
    // Two one-site loops in one face: an annular seam tube r ∈ [0.5, 1],
    // its two wrap edges at two azimuths.
    for (ao, ai) in [(0.0, 0.0), (0.0, PI), (1.0, 4.0)] {
        for sweep_in in [TAU, -TAU] {
            let loops = vec![circle((0.0, 0.0), 1.0, ao, TAU), circle((0.0, 0.0), 0.5, ai, sweep_in)];
            let Ok(profile) = Profile::new(SketchPlane::<f64>::xy(), loops).validate(tol()) else {
                println!("R2 F3 annulus ao={ao} ai={ai} sweep_in={sweep_in} | profile refused");
                continue;
            };
            let depth = Extrusion::Distance { depth: 2.0, side: ExtrudeSide::Along };
            let t = extrude(&profile, depth, tol()).unwrap().body;
            let t = topo::transform_rigid(&t, &Affine3::translation(Vec3::new(0.0, 0.0, 0.5)), tol()).unwrap();
            let ann = PI * 0.75;
            every_op(&format!("F3 annulus ao={ao} ai={ai}"), plate(), t, (36.0, 2.0 * ann, 0.5 * ann));
            break;
        }
    }
}

/// A one-face torus about y: the one-segment circle of radius `a` at
/// `(big, 0)`, its vertex at profile azimuth `az`, revolved a full turn.
fn torus(big: f64, a: f64, az: f64) -> Body<f64> {
    let profile = Profile::new(SketchPlane::<f64>::xy(), vec![circle((big, 0.0), a, az, TAU)])
        .validate(tol())
        .unwrap();
    let axis = sweep::RevolveAxis { origin: Point2::new(0.0, 0.0), dir: Vec2::new(0.0, 1.0) };
    sweep::revolve(&profile, axis, Revolution::Full, tol()).unwrap().body
}

#[test]
#[ignore = "review probe; run with --ignored --nocapture"]
fn r2_f4_gate_probes() {
    // (a) Two one-site loops on one locus pair: a plane y = c across a
    // one-face torus cuts two circles, each through the profile seam once.
    let (big, a) = (2.0f64, 0.5f64);
    let tv = 2.0 * PI * PI * big * a * a;
    for (c, az) in [(0.2f64, 0.0f64), (-0.1, 0.0), (0.2, PI / 2.0)] {
        let seg = a * a * (c / a).acos() - c * (a * a - c * c).sqrt();
        let block = brick((-3.0, 3.0), (c, 2.0), (-3.0, 3.0), tol());
        every_op(&format!("F4a torus y>={c} az={az:.2}"), torus(big, a, az), block, (tv, 36.0 * (2.0 - c), TAU * big * seg));
    }
    // (b) A conic lying in the partner's face: a tube whose outer wall
    // breaks from cylinder to cone on the circle y = 0, in the box face
    // y = 0; its inner wall crosses that face across its seam.
    let p = |x, y| (Point2::new(x, y), 0.0);
    let kinked = revolved_about_y(vec![p(0.3, -0.5), p(0.5, -0.5), p(0.5, 0.0), p(0.7, 1.0), p(0.3, 1.0)], Revolution::Full, tol());
    let kv = PI * ((0.25 - 0.09) * 0.5 + (0.25 + 0.1 + 0.04 / 3.0 - 0.09));
    let block = brick((-1.5, 1.5), (0.0, 2.0), (-1.5, 1.5), tol());
    every_op("F4b in-face conic", kinked, block, (kv, 18.0, PI * (0.25 + 0.1 + 0.04 / 3.0 - 0.09)));
    // (c) The cap of a one-segment cylinder lying in a box face.
    let block = brick((-3.0, 3.0), (-3.0, 3.0), (-1.0, 0.0), tol());
    every_op("F4c cap in face (touch)", seam_cyl(1.0, (0.0, 0.0), 0.0, 2.0, 0.0), block, (TAU, 36.0, 0.0));
    let block = brick((-3.0, 3.0), (-3.0, 3.0), (1.0, 2.0), tol());
    every_op("F4c cap in face (flush top)", seam_cyl(1.0, (0.0, 0.0), 0.0, 2.0, 0.0), block, (TAU, 36.0, PI));
}

/// The one-segment cylinder of radius 1 turned onto `+y`, `y ∈ [y0, y0 + h]`,
/// its wrap edge at azimuth `az` from `+x` in the `xz` plane (0 is the
/// revolve seam's half-plane `z = 0, x > 0`).
fn seam_cyl_y(y0: f64, h: f64, az: f64) -> Body<f64> {
    let c = seam_cyl(1.0, (0.0, 0.0), 0.0, h, az);
    let turn = Affine3::rotation_about_axis(Point3::origin(), Vec3::new(1.0, 0.0, 0.0), -PI / 2.0);
    let q = turn.transform_point(Point3::new(1.0, 0.0, 1.0));
    assert!((q.x - 1.0).abs() < 1e-12 && (q.y - 1.0).abs() < 1e-12 && q.z.abs() < 1e-12, "{q:?}");
    let to = Affine3::translation(Vec3::new(0.0, y0, 0.0));
    topo::transform_rigid(&c, &(to * turn), tol()).unwrap()
}

#[test]
#[ignore = "review probe; run with --ignored --nocapture"]
fn r2_f5_curved_partners() {
    // A coaxial cone-tube r ∈ [0.2, 2 − y], y ∈ [0, 1.5], against the
    // cylinder r = 1, y ∈ [0.5, 2]: the walls meet in the circle y = 1;
    // the cylinder's cap y = 0.5 is pierced by the tube's inner wall.
    let p = |x, y| (Point2::new(x, y), 0.0);
    let cone = || revolved_about_y(vec![p(0.2, 0.0), p(2.0, 0.0), p(0.5, 1.5), p(0.2, 1.5)], Revolution::Full, tol());
    let cv = PI * ((8.0 - 0.125) / 3.0 - 0.06);
    let ov = PI * (0.48 + (1.0 - 0.125) / 3.0 - 0.02);
    for az in [0.0, 1.3, -0.000_001] {
        every_op(&format!("F5 cone-tube cyl-az={az}"), cone(), seam_cyl_y(0.5, 1.5, az), (cv, 1.5 * PI, ov));
    }
    // A ball r = 1.5 about the origin against the cylinder y ∈ [0, 3].
    let hh = 1.25f64.sqrt();
    let capv = PI * (2.25 * (1.5 - hh) - (3.375 - hh * hh * hh) / 3.0);
    for az in [0.0, 1.3] {
        let ball = ball_poled_y(1.5, Vec3::new(0.0, 0.0, 0.0), tol());
        every_op(&format!("F5 ball cyl-az={az}"), ball, seam_cyl_y(0.0, 3.0, az), (4.5 * PI, 3.0 * PI, PI * hh + capv));
    }
}

fn detail(row: &str, r: Result<topo::BooleanResult<f64>, topo::BooleanError>) {
    let Ok(res) = r else {
        println!("D2 {row} | refused {:?}", r.err());
        return;
    };
    let Some(bb) = res.body() else {
        println!("D2 {row} | empty");
        return;
    };
    let t3 = topo::validate_pseudomanifold(&bb.body, &bb.contacts, tol());
    let far = fin("far", brick((50.0, 51.0), (50.0, 51.0), (50.0, 51.0), tol()));
    let u = topo::union(&bb.body, &far, tol()).err();
    let s = format!("{u:?}");
    println!(
        "D2 {row} | t3p={:?} | far-union-err={}",
        t3.err(),
        s.chars().take(400).collect::<String>()
    );
}

fn two_arc_cyl(r: f64, z0: f64, h: f64) -> Body<f64> {
    sweep::test_support::cylinder_of_arcs_at(2, r, Point2::new(0.0, 0.0), z0, h, tol())
}

#[test]
#[ignore = "review probe; run with --ignored --nocapture"]
fn r2_d_details() {
    let x = Vec3::new(1.0, 0.0, 0.0);
    for (name, cyl) in [("one-seg", seam_cyl(1.0, (0.0, 0.0), 0.0, 2.0, 0.0)), ("two-arc", two_arc_cyl(1.0, 0.0, 2.0))] {
        for deg in [0.0f64, 10.0] {
            let turn = Affine3::rotation_about_axis(Point3::origin(), x, deg.to_radians());
            let a = fin("A", cyl.clone());
            let b = fin("B", below(Point3::new(0.0, 0.0, 1.0), turn));
            detail(&format!("{name} tilt{deg} AnB"), topo::intersect(&a, &b, tol()));
            detail(&format!("{name} tilt{deg} AuB"), topo::union(&a, &b, tol()));
        }
        // An axis-aligned slab, as the PR's witness.
        let a = fin("A", cyl.clone());
        let b = fin("B", brick((-3.0, 3.0), (-3.0, 3.0), (0.5, 1.0), tol()));
        detail(&format!("{name} slab AnB"), topo::intersect(&a, &b, tol()));
    }
    // Box minus tube: the one-face revolve tube and a two-arc extruded
    // annulus turned onto y.
    let block = fin("box", brick((-1.5, 1.5), (0.0, 1.0), (-1.5, 1.5), tol()));
    detail("tube B-A", topo::subtract(&block, &fin("t", tube(0.3, 0.5)), tol()));
    let outer = fin("o", two_arc_cyl(0.5, -1.5, 2.0));
    let inner = fin("i", two_arc_cyl(0.3, -2.0, 3.0));
    let ann = topo::subtract(&outer, &inner, tol()).unwrap().body().unwrap().body.clone();
    let turn = Affine3::rotation_about_axis(Point3::origin(), x, -PI / 2.0);
    let ann = fin("ann", topo::transform_rigid(&ann, &turn, tol()).unwrap());
    detail("two-arc annulus B-A", topo::subtract(&block, &ann, tol()));
    let y = Vec3::new(0.0, 1.0, 0.0);
    for deg in [60.0f64, 80.0] {
        let t = deg.to_radians().tan();
        let h = 2.0 * t + 2.0;
        for (name, cyl) in [("one-seg", seam_cyl(1.0, (0.0, 0.0), 0.0, h, 0.0)), ("two-arc", two_arc_cyl(1.0, 0.0, h))] {
            let turn = Affine3::rotation_about_axis(Point3::origin(), y, deg.to_radians());
            let a = fin("A", cyl);
            let b = fin("B", below(Point3::new(0.0, 0.0, h / 2.0), turn));
            detail(&format!("{name} tilt-y{deg} AuB"), topo::union(&a, &b, tol()));
            detail(&format!("{name} tilt-y{deg} AnB"), topo::intersect(&a, &b, tol()));
        }
    }
    // The whole-cylinder core plus a ring: a box minus a one-face tube
    // where the core is solid: is it one lump or two?
}

#[test]
#[ignore = "review probe; run with --ignored --nocapture"]
fn r2_d_steep() {
    let y = Vec3::new(0.0, 1.0, 0.0);
    for deg in [62.0f64, 66.0, 70.0, 74.0, 78.0] {
        let t = deg.to_radians().tan();
        let h = 2.0 * t + 2.0;
        let turn = Affine3::rotation_about_axis(Point3::origin(), y, deg.to_radians());
        let a = fin("A", seam_cyl(1.0, (0.0, 0.0), 0.0, h, 0.0));
        let b = fin("B", below(Point3::new(0.0, 0.0, h / 2.0), turn));
        detail(&format!("steep one-seg tilt-y{deg} AuB"), topo::union(&a, &b, tol()));
        // Same tilt, seam on the level line instead of at the low point.
        let a = fin("A", seam_cyl(1.0, (0.0, 0.0), 0.0, h, PI / 2.0));
        detail(&format!("steep one-seg seam-on-level tilt-y{deg} AuB"), topo::union(&a, &b, tol()));
        // Same tilt, a shorter wall: the plane's highest point nearer the top.
        let a = fin("A", seam_cyl(1.0, (0.0, 0.0), 0.0, 2.0 * t + 1.2, 0.0));
        let b = fin("B", below(Point3::new(0.0, 0.0, t + 0.6), turn));
        detail(&format!("steep one-seg short tilt-y{deg} AuB"), topo::union(&a, &b, tol()));
    }
}

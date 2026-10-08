//! Review r1 of PR 4345: probe families for the one-site wrap-edge
//! section loop. Every build is read through
//! [`crate::common::differential::outcome`] and tessellated with
//! `check_mesh`. `cargo test -p sweep --release --test all
//! review_r1_wrap_probes -- --ignored --nocapture`, on two trees, and
//! diff the `R1` lines.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::{PI, TAU};

use geom_core::{Affine3, Arc2, Point2, Point3, Tol, Vec3};
use profile::{Profile, ProfileLoop, RawLoop, Segment, SketchPlane};
use sweep::test_support::{ball_poled, ball_poled_z, brick, revolved_about_y};
use sweep::{ExtrudeSide, Extrusion, Revolution, extrude};
use topo::{AtRestBody, Body, BooleanError, BooleanResult};

use crate::common::differential::{clip_convex, disc_clip_area, moments, outcome};

fn tol() -> Tol {
    Tol::witness()
}

fn fin(body: Body<f64>) -> AtRestBody<f64> {
    topo::test_support::finished("probe operand", body, tol())
}

/// A circle of radius `r` about `(cx, cy)` in `segs` arcs, extruded
/// `z ∈ [z0, z0 + h]`; `segs == 1` is the one-face wall with a wrap edge.
fn cyl(cx: f64, cy: f64, r: f64, z0: f64, h: f64, segs: usize) -> Body<f64> {
    let sweep = TAU / segs as f64;
    let raw: Vec<(Point2<f64>, Segment<f64>)> = (0..segs)
        .map(|k| {
            let t = k as f64 * sweep;
            (
                Point2::new(cx + r * t.cos(), cy + r * t.sin()),
                Segment::Arc(Arc2 {
                    centre: Point2::new(cx, cy),
                    radius: r,
                    sweep,
                }),
            )
        })
        .collect();
    let lp: ProfileLoop<f64> = RawLoop::new(raw);
    let profile = Profile::new(SketchPlane::<f64>::xy(), vec![lp])
        .validate(tol())
        .unwrap();
    let depth = Extrusion::Distance {
        depth: h,
        side: ExtrudeSide::Along,
    };
    let body = extrude(&profile, depth, tol()).unwrap().body;
    let up = Affine3::translation(Vec3::new(0.0, 0.0, z0));
    topo::transform_rigid(&body, &up, tol()).unwrap()
}

fn moved(b: &Body<f64>, m: Affine3<f64>) -> Body<f64> {
    topo::transform_rigid(b, &m, tol()).unwrap()
}

fn turn(axis: Vec3<f64>, deg: f64) -> Affine3<f64> {
    Affine3::rotation_about_axis(Point3::origin(), axis, deg.to_radians())
}

/// The half-space below the plane through `at` with upward normal
/// `R·ẑ`, as a box 8 × 8 × 8 under it.
fn below(at: Point3<f64>, r: Affine3<f64>) -> Body<f64> {
    let raw = brick((-4.0, 4.0), (-4.0, 4.0), (-8.0, 0.0), tol());
    moved(&raw, Affine3::translation(at - Point3::origin()) * r)
}

fn mesh_note(r: &Result<BooleanResult<f64>, BooleanError>) -> String {
    match r {
        Ok(res) => match res.body() {
            Some(bb) => match mesh::tessellate(&bb.body, 1e-2, tol()) {
                Ok(m) => match mesh::validate::check_mesh(&m) {
                    Ok(()) => format!("mesh=ok mv={:.4}", mesh::validate::signed_volume(&m)),
                    Err(e) => format!("mesh=BAD {e:?}"),
                },
                Err(e) => format!("mesh=ERR {e:?}"),
            },
            None => "mesh=-".into(),
        },
        Err(_) => "mesh=-".into(),
    }
}

/// Every op in both orders through `outcome` and `check_mesh`.
fn every_op(what: &str, a: Body<f64>, b: Body<f64>, v: (f64, f64, f64)) {
    every_op_at(what, fin(a), fin(b), v);
}

/// [`every_op`] of a built result `a` (already at rest).
fn every_op_at(
    what: &str,
    a: AtRestBody<f64>,
    b: AtRestBody<f64>,
    (va, vb, vab): (f64, f64, f64),
) {
    let rows: [(&str, &AtRestBody<f64>, &AtRestBody<f64>, f64); 6] = [
        ("A∪B", &a, &b, va + vb - vab),
        ("B∪A", &b, &a, va + vb - vab),
        ("A∩B", &a, &b, vab),
        ("B∩A", &b, &a, vab),
        ("A∖B", &a, &b, va - vab),
        ("B∖A", &b, &a, vb - vab),
    ];
    for (op, x, y, want) in rows {
        let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            match &op[1..op.len() - 1] {
                "∪" => topo::union(x, y, tol()),
                "∩" => topo::intersect(x, y, tol()),
                _ => topo::subtract(x, y, tol()),
            }
        }));
        match r {
            Ok(r) => {
                let m = mesh_note(&r);
                println!("R1 {what}: {op} => {} {m}", outcome(r, want, tol()));
            }
            Err(_) => println!("R1 {what}: {op} => PANIC"),
        }
    }
}

fn tube(ri: f64, ro: f64) -> Body<f64> {
    let p = |x, y| (Point2::new(x, y), 0.0);
    revolved_about_y(
        vec![p(ri, -0.5), p(ro, -0.5), p(ro, 1.5), p(ri, 1.5)],
        Revolution::Full,
        tol(),
    )
}

fn square(h: f64) -> Vec<(f64, f64)> {
    vec![(-h, -h), (h, -h), (h, h), (-h, h)]
}

/// Pappus volume of a full revolve about `y` of the polygon `p` (x > 0),
/// clipped to `y ≤ c`.
fn pappus_below(p: &[(f64, f64)], c: f64) -> f64 {
    let clip = [(0.0, -50.0), (50.0, -50.0), (50.0, c), (0.0, c)];
    let q = clip_convex(p, &clip);
    TAU * moments(&q).0.abs()
}

/// A circle sampled finely, as an arc of `sweep` from angle `a0`, then
/// closed by its chord.
fn arc_poly(cx: f64, cy: f64, r: f64, a0: f64, sweep: f64, n: usize) -> Vec<(f64, f64)> {
    (0..=n)
        .map(|k| {
            let t = a0 + sweep * k as f64 / n as f64;
            (cx + r * t.cos(), cy + r * t.sin())
        })
        .collect()
}

#[test]
#[ignore = "review probe battery"]
fn review_r1_wrap_probes() {
    let seam = cyl(0.0, 0.0, 1.0, 0.0, 2.0, 1);
    // ---- the PR's six witnesses through `outcome` ----
    every_op(
        "W1 cleave tube",
        tube(0.3, 0.5),
        brick((-1.5, 1.5), (0.0, 1.0), (-1.5, 1.5), tol()),
        (0.32 * PI, 9.0, 0.16 * PI),
    );
    every_op(
        "W2 slab",
        seam.clone(),
        brick((-3.0, 3.0), (-3.0, 3.0), (0.5, 1.0), tol()),
        (TAU, 18.0, 0.5 * PI),
    );
    every_op(
        "W3 pocket",
        brick((-3.0, 3.0), (-3.0, 3.0), (0.0, 1.0), tol()),
        cyl(0.0, 0.0, 1.0, 0.5, 2.0, 1),
        (36.0, TAU, 0.5 * PI),
    );
    let x = Vec3::new(1.0, 0.0, 0.0);
    let y = Vec3::new(0.0, 1.0, 0.0);
    let z = Vec3::new(0.0, 0.0, 1.0);
    every_op(
        "W4 tilted 20",
        seam.clone(),
        below(Point3::new(0.0, 0.0, 1.0), turn(x, 20.0)),
        (TAU, 512.0, PI),
    );
    // `deg` is the plane's tilt; the slope it sets is `tan(deg)`, and the
    // tiny tilts are planes nearly flat at the wrap edge's end vertex.
    for (c, deg) in [
        (0.02, 0.5f64.atan().to_degrees()),
        (1e-3, 0.5f64.atan().to_degrees()),
        (0.03, 0.95f64.atan().to_degrees()),
        (1e-4, 0.5f64.atan().to_degrees()),
        (0.3, 0.999f64.atan().to_degrees()),
        (0.02, 0.4636),
        (1e-3, 0.4636),
        (1e-4, 0.4636),
        (0.03, 0.02),
    ] {
        let m = deg.to_radians().tan();
        every_op(
            &format!("W5 near end c={c} tilt={deg:.4}"),
            seam.clone(),
            below(Point3::new(1.0, 0.0, c), turn(y, deg)),
            (TAU, 512.0, PI * (c + m)),
        );
    }
    let segment = 0.25 * 0.8f64.acos() - 0.4 * 0.3;
    every_op(
        "W6 beside edge",
        tube(0.3, 0.5),
        brick((-1.5, 0.4), (0.0, 1.0), (-1.5, 1.5), tol()),
        (0.32 * PI, 5.7, 0.16 * PI - segment),
    );
    // ---- off-centre, turned and tilted planes across the seam cylinder ----
    let (cx, cy) = (0.37, -0.61);
    let off = cyl(cx, cy, 1.0, 0.0, 2.0, 1);
    let diag = Vec3::new(1.0, 1.0, 0.0) * (0.5f64).sqrt();
    for (name, axis, deg) in [
        ("x5", x, 5.0),
        ("x33", x, 33.0),
        ("x44", x, 44.0),
        ("y10", y, 10.0),
        ("y-40", y, -40.0),
        ("d30", diag, 30.0),
        ("d-44", diag, -44.0),
        ("flat", z, 0.0),
    ] {
        every_op(
            &format!("P2 off-centre {name}"),
            off.clone(),
            below(Point3::new(cx, cy, 1.0), turn(axis, deg)),
            (TAU, 512.0, PI),
        );
    }
    // the cylinder turned about its own axis, so its seam sits elsewhere
    for deg in [90.0, 200.0, 333.0] {
        every_op(
            &format!("P2 seam turned {deg}"),
            moved(&seam, turn(z, deg)),
            below(Point3::new(0.0, 0.0, 0.7), turn(diag, 25.0)),
            (TAU, 512.0, 0.7 * PI),
        );
    }
    // a small and a large radius
    for r in [0.05, 7.0] {
        let big = cyl(0.0, 0.0, r, 0.0, 2.0, 1);
        let w = if r > 4.0 { 12.0 } else { 4.0 };
        let raw = brick((-w, w), (-w, w), (-2.0 * w, 0.0), tol());
        let cut = moved(&raw, Affine3::translation(Vec3::new(0.0, 0.0, 1.0)) * turn(x, 5.0));
        every_op(
            &format!("P2 radius {r}"),
            big,
            cut,
            (2.0 * PI * r * r, 8.0 * w * w * w, PI * r * r),
        );
    }
    // a vertical plane near the wall: rulings, not one-site (control)
    for xc in [0.999, 0.5] {
        let b = brick((xc, 3.0), (-3.0, 3.0), (-1.0, 3.0), tol());
        let a_seg = (xc).acos() - xc * (1.0 - xc * xc).sqrt();
        every_op(
            &format!("P2 vertical x={xc}"),
            seam.clone(),
            b,
            (TAU, (3.0 - xc) * 24.0, 2.0 * a_seg),
        );
    }
    // ---- both radii of a tube ----
    every_op(
        "P3 tube thin inner",
        tube(0.05, 0.9),
        brick((-1.5, 1.5), (0.0, 1.0), (-1.5, 1.5), tol()),
        (2.0 * PI * (0.81 - 0.0025), 9.0, PI * (0.81 - 0.0025)),
    );
    every_op(
        "P3 box over inner, part of outer",
        tube(0.3, 0.5),
        brick((-0.4, 0.4), (0.0, 1.0), (-0.4, 0.4), tol()),
        (0.32 * PI, 0.64, disc_clip_area(0.5, &square(0.4)) - 0.09 * PI),
    );
    every_op(
        "P3 box between the walls",
        tube(0.3, 0.5),
        brick((-0.35, 0.35), (0.0, 1.0), (-0.35, 0.35), tol()),
        (0.32 * PI, 0.49, 0.49 - 0.09 * PI),
    );
    let tilt = Affine3::translation(Vec3::new(0.0, 0.5, 0.0))
        * turn(x, 20.0)
        * Affine3::translation(Vec3::new(0.0, -0.5, 0.0));
    every_op(
        "P3 tilted tube",
        moved(&tube(0.3, 0.5), tilt),
        brick((-1.5, 1.5), (0.0, 1.0), (-1.5, 1.5), tol()),
        (0.32 * PI, 9.0, 0.16 * PI / 20f64.to_radians().cos()),
    );
    // ---- rings already in the planar face ----
    let plate = || fin(brick((-3.0, 3.0), (-3.0, 3.0), (0.0, 1.0), tol()));
    for (name, hx, segs) in [
        ("enclosed 2-arc", 0.0, 2),
        ("enclosed 1-arc", 0.0, 1),
        ("outside 2-arc", 2.0, 2),
        ("outside 1-arc", 2.0, 1),
    ] {
        let hole = fin(cyl(hx, 0.0, 0.2, -1.0, 3.0, segs));
        let holed = match topo::subtract(&plate(), &hole, tol()) {
            Ok(r) => r.body().unwrap().body.clone(),
            Err(e) => {
                println!("R1 P4 {name}: holed plate => ERR {e:?}");
                continue;
            }
        };
        let in_cyl = if hx == 0.0 { 0.04 * PI * 0.5 } else { 0.0 };
        every_op_at(
            &format!("P4 holed plate {name}"),
            holed,
            fin(cyl(0.0, 0.0, 1.0, 0.5, 2.0, 1)),
            (36.0 - 0.04 * PI, TAU, 0.5 * PI - in_cyl),
        );
    }
    // a blind pocket inside a later, wider pocket's conic, and one outside
    for (name, px, want_extra) in [("inside", 0.0, 0.04 * PI * 0.3), ("outside", 2.0, 0.0)] {
        let first = fin(cyl(px, 0.0, 0.2, 0.5, 2.0, 1));
        let pocketed = match topo::subtract(&plate(), &first, tol()) {
            Ok(r) => r.body().unwrap().body.clone(),
            Err(e) => {
                println!("R1 P4 pocket {name}: first => ERR {e:?}");
                continue;
            }
        };
        every_op_at(
            &format!("P4 second pocket, first {name}"),
            pocketed,
            fin(cyl(0.0, 0.0, 1.0, 0.7, 2.0, 1)),
            (36.0 - 0.02 * PI, TAU, 0.3 * PI - want_extra),
        );
    }
    // two one-site loops in one planar face: two seam cylinders as one operand
    let two = topo::union(
        &fin(cyl(-1.5, 0.0, 1.0, 0.0, 2.0, 1)),
        &fin(cyl(1.5, 0.0, 1.0, 0.0, 2.0, 1)),
        tol(),
    );
    match two {
        Ok(r) => every_op_at(
            "P4 two walls one plane",
            r.body().unwrap().body.clone(),
            fin(brick((-3.0, 3.0), (-3.0, 3.0), (0.5, 1.0), tol())),
            (2.0 * TAU, 18.0, PI),
        ),
        Err(e) => println!("R1 P4 two walls: union => ERR {e:?}"),
    }
    // nested: a seam cylinder inside a seam tube made by subtraction
    let ring = topo::subtract(
        &fin(cyl(0.0, 0.0, 1.0, 0.0, 2.0, 1)),
        &fin(cyl(0.0, 0.0, 0.5, -1.0, 4.0, 1)),
        tol(),
    );
    match ring {
        Ok(r) => every_op_at(
            "P4 nested walls one plane",
            r.body().unwrap().body.clone(),
            fin(brick((-3.0, 3.0), (-3.0, 3.0), (0.5, 1.0), tol())),
            (2.0 * PI * 0.75, 18.0, 0.5 * PI * 0.75),
        ),
        Err(e) => println!("R1 P4 nested: subtract => ERR {e:?}"),
    }
    // ---- tori: two semicircles, and a >180° arc giving two loops on one face pair ----
    let p = |x, y, b| (Point2::new(x, y), b);
    let torus = revolved_about_y(
        vec![p(1.5, 0.0, 1.0), p(0.5, 0.0, 1.0)],
        Revolution::Full,
        tol(),
    );
    let tp = arc_poly(1.0, 0.0, 0.5, 0.0, TAU, 200_000);
    for c in [0.2, -0.3] {
        every_op(
            &format!("P5 torus below y={c}"),
            torus.clone(),
            moved(&below(Point3::new(0.0, 0.0, c), Affine3::identity()), turn(x, -90.0)),
            (pappus_below(&tp, 9.0), 512.0, pappus_below(&tp, c)),
        );
    }
    let cx2 = 1.0 + 0.75f64.sqrt();
    let big = revolved_about_y(
        vec![p(1.0, -0.5, (75f64).to_radians().tan()), p(1.0, 0.5, 0.0)],
        Revolution::Full,
        tol(),
    );
    let bp = arc_poly(cx2, 0.0, 1.0, 210f64.to_radians(), 300f64.to_radians(), 200_000);
    for c in [0.7, 0.2] {
        every_op(
            &format!("P5 c-torus below y={c}"),
            big.clone(),
            moved(&below(Point3::new(0.0, 0.0, c), Affine3::identity()), turn(x, -90.0)),
            (pappus_below(&bp, 9.0), 512.0, pappus_below(&bp, c)),
        );
    }
    // ---- a curved partner: a ball the seam cylinder passes through ----
    let ball_v = 32.0 * PI / 3.0;
    let lens = TAU * (2.0 / 3.0) * (8.0 - 3.0f64.powf(1.5));
    for deg in [0.0, 90.0, 180.0, 270.0, 57.0] {
        every_op(
            &format!("P6 ball through cylinder turned {deg}"),
            moved(&cyl(0.0, 0.0, 1.0, -3.0, 6.0, 1), turn(z, deg)),
            ball_poled_z(2.0, Vec3::new(0.0, 0.0, 0.0), tol()),
            (6.0 * PI, ball_v, lens),
        );
    }
    // ---- a cone wall: a box across it, and a coaxial tube through it ----
    let tri = vec![(0.0, -0.5), (1.5, -0.5), (0.0, 1.5)];
    let cone = revolved_about_y(
        vec![p(0.0, -0.5, 0.0), p(1.5, -0.5, 0.0), p(0.0, 1.5, 0.0)],
        Revolution::Full,
        tol(),
    );
    let pv = |q: &[(f64, f64)]| TAU * moments(q).0.abs();
    let slabp = [(0.0, 0.0), (5.0, 0.0), (5.0, 0.5), (0.0, 0.5)];
    // the box y ∈ [0, 0.5], x, z ∈ ±3
    every_op(
        "P8 cone under a box",
        cone.clone(),
        brick((-3.0, 3.0), (0.0, 0.5), (-3.0, 3.0), tol()),
        (pv(&tri), 18.0, pv(&clip_convex(&tri, &slabp))),
    );
    let ann = [(0.2, -1.0), (1.0, -1.0), (1.0, 2.0), (0.2, 2.0)];
    let tubec = revolved_about_y(
        ann.iter().map(|&(a, b)| p(a, b, 0.0)).collect(),
        Revolution::Full,
        tol(),
    );
    for deg in [0.0, 40.0] {
        every_op(
            &format!("P8 cone through a tube turned {deg}"),
            moved(&cone, turn(y, deg)),
            tubec.clone(),
            (pv(&tri), pv(&ann), pv(&clip_convex(&tri, &ann))),
        );
    }
    // ---- two balls meeting in a circle: seams aligned and turned apart ----
    let lens2 = PI * (4.0 + 1.2) * 0.8 * 0.8 / 12.0;
    for deg in [0.0, 90.0, 33.0] {
        every_op(
            &format!("P9 two balls, one turned {deg}"),
            ball_poled_z(1.0, Vec3::new(0.0, 0.0, 0.0), tol()),
            moved(&ball_poled_z(1.0, Vec3::new(0.0, 0.0, 1.2), tol()), turn(z, deg)),
            (4.0 * PI / 3.0, 4.0 * PI / 3.0, lens2),
        );
    }
    // a ball under a plane (builds on main, the PR says)
    for h in [0.3, -0.55] {
        let cap = PI * (1.0 - h) * (1.0 - h) * (2.0 + h) / 3.0;
        every_op(
            &format!("P9 ball above z={h}"),
            ball_poled_z(1.0, Vec3::new(0.0, 0.0, 0.0), tol()),
            brick((-3.0, 3.0), (-3.0, 3.0), (h, 3.0), tol()),
            (4.0 * PI / 3.0, 36.0 * (3.0 - h), cap),
        );
    }
    // ---- a sphere pair's radical circle: one ball poled across the other's axis ----
    for (pole, pn) in [
        (Vec3::new(1.0, 0.0, 0.0), "x"),
        (Vec3::new(0.0, 1.0, 0.0), "y"),
        (Vec3::new(-1.0, 0.0, 0.0), "-x"),
        (Vec3::new(0.6, 0.8, 0.0), "xy"),
    ] {
        for deg in [0.0, 90.0, 180.0, 270.0, 45.0] {
            every_op(
                &format!("P11 ball poled {pn} over ball turned {deg}"),
                moved(&ball_poled_z(1.0, Vec3::new(0.0, 0.0, 0.0), tol()), turn(z, deg)),
                ball_poled(1.0, Vec3::new(0.0, 0.0, 1.2), pole, tol()),
                (4.0 * PI / 3.0, 4.0 * PI / 3.0, lens2),
            );
        }
    }
    // ---- tangencies: a plane touching a ball, and a seam cylinder, at one point ----
    for (name, at, deg) in [
        ("ball +x", Point3::new(1.0, 0.0, 0.0), 90.0),
        ("ball -x", Point3::new(-1.0, 0.0, 0.0), -90.0),
    ] {
        every_op(
            &format!("P10 plane tangent to {name}"),
            ball_poled_z(1.0, Vec3::new(0.0, 0.0, 0.0), tol()),
            below(at, turn(y, deg)),
            (4.0 * PI / 3.0, 512.0, 4.0 * PI / 3.0),
        );
    }
    for deg in [0.0, 90.0, 180.0, 270.0] {
        let ball = moved(&ball_poled_z(1.0, Vec3::new(0.0, 0.0, 0.0), tol()), turn(z, deg));
        every_op(
            &format!("P10 box touching ball turned {deg} at +x"),
            ball,
            brick((1.0, 3.0), (-3.0, 3.0), (-3.0, 3.0), tol()),
            (4.0 * PI / 3.0, 72.0, 0.0),
        );
    }
    // ---- coincidences: must refuse (or build right), never wrong ----
    every_op(
        "P7 flush cap",
        seam.clone(),
        brick((-3.0, 3.0), (-3.0, 3.0), (-1.0, 0.0), tol()),
        (TAU, 36.0, 0.0),
    );
    every_op(
        "P7 plane through the end vertex",
        seam.clone(),
        below(Point3::new(1.0, 0.0, 0.0), turn(y, 0.5f64.atan().to_degrees())),
        (TAU, 512.0, PI * 0.5),
    );
    every_op(
        "P7 plane through the far rim point",
        seam.clone(),
        below(Point3::new(-1.0, 0.0, 0.0), turn(y, -0.5f64.atan().to_degrees())),
        (TAU, 512.0, PI * 0.5),
    );
    every_op(
        "P7 tube flush end",
        tube(0.3, 0.5),
        brick((-1.5, 1.5), (-0.5, 1.0), (-1.5, 1.5), tol()),
        (0.32 * PI, 13.5, 0.24 * PI),
    );
}

fn diag(what: &str, r: Result<BooleanResult<f64>, BooleanError>) {
    let Ok(r) = r else {
        println!("D1 {what}: ERR {:?}", r.err());
        return;
    };
    let Some(bb) = r.body() else {
        println!("D1 {what}: empty");
        return;
    };
    let far = fin(brick((50.0, 51.0), (50.0, 51.0), (50.0, 51.0), tol()));
    let t3 = topo::validate_pseudomanifold(&bb.body, &bb.contacts, tol());
    let u = topo::union(&bb.body, &far, tol()).map(|_| ());
    let s = format!("{t3:?}");
    let su = format!("{u:?}");
    println!(
        "D1 {what}: shells={} t3p={} | operand={}",
        bb.body.shells().count(),
        &s[..s.len().min(400)],
        &su[..su.len().min(400)]
    );
}

#[test]
#[ignore = "review diagnostic"]
fn review_r1_diag() {
    let x = Vec3::new(1.0, 0.0, 0.0);
    let cutter = fin(below(Point3::new(0.0, 0.0, 1.0), turn(x, 20.0)));
    let far = fin(brick((50.0, 51.0), (50.0, 51.0), (50.0, 51.0), tol()));
    println!("D1 tilted cutter alone ∪ far: {:?}", topo::union(&cutter, &far, tol()).map(|_| ()));
    let seam = fin(cyl(0.0, 0.0, 1.0, 0.0, 2.0, 1));
    println!("D1 seam cyl alone ∪ far: {:?}", topo::union(&seam, &far, tol()).map(|_| ()));
    let two = fin(cyl(0.0, 0.0, 1.0, 0.0, 2.0, 4));
    diag("4-arc cyl ∩ tilted", topo::intersect(&two, &cutter, tol()));
    diag("seam cyl ∩ tilted", topo::intersect(&seam, &cutter, tol()));
    let boxed = fin(brick((-1.0, 1.0), (-1.0, 1.0), (0.0, 2.0), tol()));
    diag("box ∩ tilted", topo::intersect(&boxed, &cutter, tol()));
    let tube = fin(tube(0.3, 0.5));
    let block = fin(brick((-1.5, 1.5), (0.0, 1.0), (-1.5, 1.5), tol()));
    diag("cleave B∖A", topo::subtract(&block, &tube, tol()));
    // the same shape with a 4-arc tube built by subtraction of 4-arc cylinders
    let outer = fin(cyl(0.0, 0.0, 0.5, -0.5, 2.0, 4));
    let inner = fin(cyl(0.0, 0.0, 0.3, -1.0, 3.0, 4));
    let t4 = topo::subtract(&outer, &inner, tol()).unwrap().body().unwrap().body.clone();
    let slab = fin(brick((-1.5, 1.5), (-1.5, 1.5), (0.0, 1.0), tol()));
    diag("slab ∖ 4-arc tube", topo::subtract(&slab, &t4, tol()));
}

//! JOIN-1 review lane r1: a differential battery over z-prisms against
//! boxes whose faces hold the prisms' side edges (edge-in-face), whose
//! edges coincide with them (edge-edge), and whose end walls sit inside,
//! across and past them, under ∪, ∖ and ∩ in both operand orders. Each
//! building pose is checked at tiers 2, 3′ and the at-rest certificate,
//! and against the closed-form volume (polygon clipping; the rod by a
//! 2^18-gon). `#[ignore]`d: run with `--ignored`, and it prints one
//! line per pose so that two trees can be diffed.
//!
//! The batteries: `join1_r1_battery` (z-prisms: diamond, hexagon,
//! triangle, rod, against boxes), `join1_r1_seam_battery` (a revolved
//! ball and a revolved solid cylinder — the latter refused at the
//! operand gate, `NonMaximalFaces`), `join1_r1_declared_battery`
//! (flush-declared z-prisms against boxes, coplanar caps and sides),
//! `join1_r1_reflex_battery` (the reflex-corner issue's probe rebuilt
//! from its text), `join1_r1_tube_battery` (a bored revolve whose ONE
//! outer wall face carries its seam meridian, against boxes and wedges
//! along that seam) and `join1_r1_bored_capsule_battery` (a crease
//! circle between two curved faces in a box face's plane).
//! `join1_r1_hex_detail` prints the hexagon ∪ box failure.
//!
//! Measured on the reviewed head against its merge base, release
//! build: see the review report.

#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::type_complexity
)]

use geom_core::{Point2, Tol};
use profile::{Profile, SketchPlane, test_support::bulge_loop};
use sweep::ExtrudeSide;
use sweep::test_support::{brick, finished};
use sweep::{Extrusion, extrude};
use topo::AtRestBody;

use crate::common::differential::{
    REFLEX_OPS, REFLEX_PROFILES, area, clip_convex, clip_rect, outcome, reflex_pose, reflex_run,
};

fn tol() -> Tol {
    Tol::witness()
}

fn prism(pts: &[(f64, f64)], h: f64) -> AtRestBody<f64> {
    let lp = bulge_loop(pts.iter().map(|&(x, y)| (Point2::new(x, y), 0.0)).collect());
    let profile = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(tol())
        .unwrap();
    let prism = extrude(
        &profile,
        Extrusion::Distance {
            depth: h,
            side: ExtrudeSide::Along,
        },
        tol(),
    )
    .unwrap()
    .body;
    finished("the prism", prism, tol())
}

fn rod(h: f64) -> AtRestBody<f64> {
    let disc = profile::circle(Point2::new(0.0, 0.0), 0.5, tol()).unwrap();
    let profile = Profile::new(SketchPlane::xy(), vec![disc.into()])
        .validate(tol())
        .unwrap();
    let rod = extrude(
        &profile,
        Extrusion::Distance {
            depth: h,
            side: ExtrudeSide::Along,
        },
        tol(),
    )
    .unwrap()
    .body;
    finished("the rod", rod, tol())
}

fn overlap(a: (f64, f64), b: (f64, f64)) -> f64 {
    (a.1.min(b.1) - a.0.max(b.0)).max(0.0)
}

#[test]
#[ignore = "differential battery; run with --ignored"]
fn join1_r1_battery() {
    let h = (0.0, 2.0);
    let n = 1 << 18;
    let circle: Vec<(f64, f64)> = (0..n)
        .map(|i| {
            let t = (i as f64) * std::f64::consts::TAU / (n as f64);
            (0.5 * t.cos(), 0.5 * t.sin())
        })
        .collect();
    let shapes: Vec<(&str, Vec<(f64, f64)>, bool)> = vec![
        (
            "diamond",
            vec![(0.5, 0.0), (0.0, 0.5), (-0.5, 0.0), (0.0, -0.5)],
            false,
        ),
        (
            "hex",
            vec![
                (0.5, 0.0),
                (0.25, 0.5),
                (-0.25, 0.5),
                (-0.5, 0.0),
                (-0.25, -0.5),
                (0.25, -0.5),
            ],
            false,
        ),
        ("tri", vec![(-0.5, -0.5), (0.5, 0.0), (-0.5, 0.5)], false),
        ("rod", circle, true),
    ];
    let coords = [-1.0, -0.5, -0.25, 0.0, 0.25, 0.5, 1.0];
    let mut ranges = Vec::new();
    for i in 0..coords.len() {
        for j in i + 1..coords.len() {
            ranges.push((coords[i], coords[j]));
        }
    }
    let zs = [(-1.0, 3.0), (1.0, 3.0), (-1.0, 1.0), (0.5, 1.5)];
    let only = std::env::var("J1_SHAPE").ok();
    for (name, pts, is_rod) in &shapes {
        if only.as_deref().is_some_and(|o| o != *name) {
            continue;
        }
        let a = if *is_rod { rod(h.1) } else { prism(pts, h.1) };
        let va = area(pts) * (h.1 - h.0);
        for &x in &ranges {
            for &y in &ranges {
                for &z in &zs {
                    let b = finished("the brick", brick(x, y, z, tol()), tol());
                    let vb = (x.1 - x.0) * (y.1 - y.0) * (z.1 - z.0);
                    let vi = area(&clip_rect(pts, x, y)) * overlap(h, z);
                    for (op, want_ab, want_ba) in [
                        ("U", va + vb - vi, va + vb - vi),
                        ("S", va - vi, vb - vi),
                        ("I", vi, vi),
                    ] {
                        for (order, want) in [("AB", want_ab), ("BA", want_ba)] {
                            let (l, r) = if order == "AB" { (&a, &b) } else { (&b, &a) };
                            let res = match op {
                                "U" => topo::union(l, r, tol()),
                                "S" => topo::subtract(l, r, tol()),
                                _ => topo::intersect(l, r, tol()),
                            };
                            println!(
                                "POSE {name} x={x:?} y={y:?} z={z:?} {op} {order} => {}",
                                outcome(res, want, tol())
                            );
                        }
                    }
                }
            }
        }
    }
}

fn disc_poly(r: f64, n: usize) -> Vec<(f64, f64)> {
    (0..n)
        .map(|i| {
            let t = (i as f64) * std::f64::consts::TAU / (n as f64);
            (r * t.cos(), r * t.sin())
        })
        .collect()
}

/// A full revolve about `y` of the rectangle `x ∈ [0, r], y ∈ [0, h]`:
/// ONE wall face whose seam meridian is an edge with both halves in it.
fn revolved_cylinder(r: f64, h: f64) -> AtRestBody<f64> {
    let cylinder = sweep::test_support::revolved_about_y(
        vec![
            (Point2::new(0.0, 0.0), 0.0),
            (Point2::new(r, 0.0), 0.0),
            (Point2::new(r, h), 0.0),
            (Point2::new(0.0, h), 0.0),
        ],
        sweep::Revolution::Full,
        tol(),
    );
    finished("the revolved cylinder", cylinder, tol())
}

/// The volume of the ball of radius `r` at the origin inside the box,
/// slice by slice along `y` (Gauss–Legendre on 400 panels; each slice
/// is a 2^13-gon clipped to the box's `x`, `z` rectangle).
fn ball_box(r: f64, x: (f64, f64), y: (f64, f64), z: (f64, f64)) -> f64 {
    let (y0, y1) = (y.0.max(-r), y.1.min(r));
    if y1 <= y0 {
        return 0.0;
    }
    let base = disc_poly(1.0, 1 << 13);
    let slice = |yy: f64| {
        let rho = (r * r - yy * yy).max(0.0).sqrt();
        let p: Vec<(f64, f64)> = base.iter().map(|&(a, b)| (a * rho, b * rho)).collect();
        area(&clip_rect(&p, x, z))
    };
    let g = [
        (-0.774_596_669_241_483_4, 5.0 / 9.0),
        (0.0, 8.0 / 9.0),
        (0.774_596_669_241_483_4, 5.0 / 9.0),
    ];
    let panels = 400;
    let w = (y1 - y0) / panels as f64;
    let mut s = 0.0;
    for k in 0..panels {
        let c = y0 + (k as f64 + 0.5) * w;
        for (t, wt) in g {
            s += wt * slice(c + t * w / 2.0) * w / 2.0;
        }
    }
    s
}

#[test]
#[ignore = "differential battery; run with --ignored"]
fn join1_r1_seam_battery() {
    let r = 0.5;
    let n = 1 << 16;
    let cyl = revolved_cylinder(r, 2.0);
    let ball = sweep::test_support::ball_poled_y(r, geom_core::Vec3::new(0.0, 0.0, 0.0), tol());
    let ball = finished("the ball", ball, tol());
    let coords = [-1.0, -0.25, 0.0, 0.25, 0.5, 1.0];
    let mut ranges = Vec::new();
    for i in 0..coords.len() {
        for j in i + 1..coords.len() {
            ranges.push((coords[i], coords[j]));
        }
    }
    let disc = disc_poly(r, n);
    let disc_area = area(&disc);
    for (name, body, ys, vol) in [
        (
            "rcyl",
            &cyl,
            vec![(-1.0, 3.0), (1.0, 3.0), (-1.0, 1.0), (0.5, 1.5)],
            disc_area * 2.0,
        ),
        (
            "ball",
            &ball,
            vec![
                (-1.0, 1.0),
                (0.0, 1.0),
                (-1.0, 0.0),
                (0.25, 1.0),
                (-0.25, 0.25),
            ],
            4.0 / 3.0 * std::f64::consts::PI * r * r * r,
        ),
    ] {
        for &x in &ranges {
            for &z in &ranges {
                for &y in &ys {
                    let b = finished("the brick", brick(x, y, z, tol()), tol());
                    let vb = (x.1 - x.0) * (y.1 - y.0) * (z.1 - z.0);
                    let vi = if name == "rcyl" {
                        area(&clip_rect(&disc, x, z)) * overlap((0.0, 2.0), y)
                    } else {
                        ball_box(r, x, y, z)
                    };
                    for (op, want_ab, want_ba) in [
                        ("U", vol + vb - vi, vol + vb - vi),
                        ("S", vol - vi, vb - vi),
                        ("I", vi, vi),
                    ] {
                        for (order, want) in [("AB", want_ab), ("BA", want_ba)] {
                            let (l, rr) = if order == "AB" {
                                (body, &b)
                            } else {
                                (&b, body)
                            };
                            let res = match op {
                                "U" => topo::union(l, rr, tol()),
                                "S" => topo::subtract(l, rr, tol()),
                                _ => topo::intersect(l, rr, tol()),
                            };
                            println!(
                                "SEAM {name} x={x:?} y={y:?} z={z:?} {op} {order} => {}",
                                outcome(res, want, tol())
                            );
                        }
                    }
                }
            }
        }
    }
}

/// A prism along `+y` over `y ∈ [y0, y1]` of a polygon given in world
/// `(x, z)`: sketched in the zx plane (sketch `(u, v)` is world
/// `(v, 0, u)`), so the sketch takes `(z, x)`.
fn y_prism(xz: &[(f64, f64)], y: (f64, f64)) -> AtRestBody<f64> {
    let lp = bulge_loop(xz.iter().map(|&(x, z)| (Point2::new(z, x), 0.0)).collect());
    let profile = Profile::new(SketchPlane::zx(), vec![lp])
        .validate(tol())
        .unwrap();
    let body = extrude(
        &profile,
        Extrusion::Distance {
            depth: y.1 - y.0,
            side: ExtrudeSide::Along,
        },
        tol(),
    )
    .unwrap()
    .body;
    let placed = topo::transform_rigid(
        &body,
        &geom_core::Affine3::translation(geom_core::Vec3::new(0.0, y.0, 0.0)),
        tol(),
    )
    .unwrap();
    finished("the y prism", placed, tol())
}

/// Flush-declared poses: z-prisms against boxes whose caps and side
/// faces may be coplanar with the prism's, every flush pair declared.
#[test]
#[ignore = "differential battery; run with --ignored"]
fn join1_r1_declared_battery() {
    use topo::flush::{declare_all, find_flush_candidates};
    let h = (0.0, 2.0);
    let shapes: Vec<(&str, Vec<(f64, f64)>)> = vec![
        (
            "square",
            vec![(-0.5, -0.5), (0.5, -0.5), (0.5, 0.5), (-0.5, 0.5)],
        ),
        (
            "diamond",
            vec![(0.5, 0.0), (0.0, 0.5), (-0.5, 0.0), (0.0, -0.5)],
        ),
        ("tri", vec![(-0.5, -0.5), (0.5, 0.0), (-0.5, 0.5)]),
        (
            "hex",
            vec![
                (0.5, 0.0),
                (0.25, 0.5),
                (-0.25, 0.5),
                (-0.5, 0.0),
                (-0.25, -0.5),
                (0.25, -0.5),
            ],
        ),
    ];
    let coords = [-1.0, -0.5, 0.0, 0.25, 0.5, 1.0];
    let mut ranges = Vec::new();
    for i in 0..coords.len() {
        for j in i + 1..coords.len() {
            ranges.push((coords[i], coords[j]));
        }
    }
    let zs = [(0.0, 2.0), (0.0, 1.0), (1.0, 2.0), (-1.0, 2.0), (1.0, 3.0)];
    for (name, pts) in &shapes {
        let a = prism(pts, h.1);
        let va = area(pts) * (h.1 - h.0);
        for &x in &ranges {
            for &y in &ranges {
                for &z in &zs {
                    let b = finished("the brick", brick(x, y, z, tol()), tol());
                    let vb = (x.1 - x.0) * (y.1 - y.0) * (z.1 - z.0);
                    let vi = area(&clip_rect(pts, x, y)) * overlap(h, z);
                    for (op, want_ab, want_ba) in [
                        ("U", va + vb - vi, va + vb - vi),
                        ("S", va - vi, vb - vi),
                        ("I", vi, vi),
                    ] {
                        for (order, want) in [("AB", want_ab), ("BA", want_ba)] {
                            let (l, r) = if order == "AB" { (&a, &b) } else { (&b, &a) };
                            let line = match find_flush_candidates(l, r, tol()) {
                                Err(e) => format!("FLUSHERR {e:?}"),
                                Ok(found) => {
                                    let d = declare_all(&found);
                                    let res = match op {
                                        "U" => topo::union_with(l, r, &d, tol()),
                                        "S" => topo::subtract_with(l, r, &d, tol()),
                                        _ => topo::intersect_with(l, r, &d, tol()),
                                    };
                                    outcome(res, want, tol())
                                }
                            };
                            println!("DECL {name} x={x:?} y={y:?} z={z:?} {op} {order} => {line}");
                        }
                    }
                }
            }
        }
    }
}

#[test]
#[ignore = "detail probe"]
fn join1_r1_hex_detail() {
    let hex = prism(
        &[
            (0.5, 0.0),
            (0.25, 0.5),
            (-0.25, 0.5),
            (-0.5, 0.0),
            (-0.25, -0.5),
            (0.25, -0.5),
        ],
        2.0,
    );
    let b = finished(
        "the box",
        brick((-0.5, -0.25), (-0.5, -0.25), (-1.0, 3.0), tol()),
        tol(),
    );
    let r = topo::union(&hex, &b, tol()).expect("builds");
    let bb = r.body().expect("a body");
    println!("t2 {:?}", topo::validate_closed(&bb.body));
    println!(
        "t3p {:?}",
        topo::validate_pseudomanifold(&bb.body, &bb.contacts, tol())
    );
    println!(
        "cert {:?}",
        topo::validate_geometric_certificate(&bb.body, tol())
    );
    println!("contacts {:?}", bb.contacts);
    println!("faces {}", bb.body.faces().count());
}

/// The `reflex-corner-vertex-vertex-sites-refuse-under-a-tilted-cap`
/// probe, rebuilt from its issue text: `a` the 315° reflex prism, `b` a
/// prism over z ∈ (1, 3) sheared `z' = z + sx·x + sy·y`, its bottom cap
/// through `a`'s reflex corner `(0, 0, 1)`; flush-declared.
#[test]
#[ignore = "differential battery; run with --ignored"]
fn join1_r1_reflex_battery() {
    let shears = [-0.5, -0.25, 0.0, 0.25, 0.5];
    for (name, _) in REFLEX_PROFILES {
        for &sx in &shears {
            for &sy in &shears {
                if sx == 0.0 && sy == 0.0 {
                    continue;
                }
                let p = reflex_pose(name, 0.0, sx, sy, tol());
                for (op, want) in REFLEX_OPS.iter().zip(p.want) {
                    println!(
                        "REFLEX {name} sx={sx} sy={sy} {op} => {}",
                        outcome(reflex_run(&p, op, tol()), want, tol())
                    );
                }
            }
        }
    }
}

/// A full revolve about `y` of an ANNULAR profile (bored at `bore`, so
/// the revolve gate takes it): one outer wall face whose seam meridian
/// (world `z = 0`, `x > 0`) is an edge with both halves in that face.
fn revolved_tube(bore: f64, r: f64, h: f64) -> AtRestBody<f64> {
    let tube = sweep::test_support::revolved_about_y(
        vec![
            (Point2::new(bore, 0.0), 0.0),
            (Point2::new(r, 0.0), 0.0),
            (Point2::new(r, h), 0.0),
            (Point2::new(bore, h), 0.0),
        ],
        sweep::Revolution::Full,
        tol(),
    );
    finished("the revolved tube", tube, tol())
}

/// A bored capsule: the tube `bore ≤ ρ ≤ r` over `y ∈ [0, c]` capped by
/// a sphere zone of radius `big` centred on the axis below `c`, meeting
/// the wall at a crease along the circle `ρ = r, y = c` (an edge
/// between two CURVED faces, so a box face in `y = c` holds it with no
/// coplanar face). Returns the body and its profile's top `y`.
fn bored_capsule(bore: f64, r: f64, c: f64, big: f64) -> (AtRestBody<f64>, f64) {
    let d = (big * big - r * r).sqrt();
    let yc = c - d;
    let top = yc + (big * big - bore * bore).sqrt();
    let t1 = d.atan2(r);
    let t2 = (top - yc).atan2(bore);
    let bulge = ((t2 - t1) / 4.0).tan();
    let body = sweep::test_support::revolved_about_y(
        vec![
            (Point2::new(bore, 0.0), 0.0),
            (Point2::new(r, 0.0), 0.0),
            (Point2::new(r, c), bulge),
            (Point2::new(bore, top), 0.0),
        ],
        sweep::Revolution::Full,
        tol(),
    );
    (finished("the bored capsule", body, tol()), top)
}

/// `∫ area((disc ρ_out(y) ∖ disc bore) ∩ rect_xz) dy` over `y ∈ ys`,
/// Gauss–Legendre on 400 panels, 2^13-gon discs.
fn solid_of_rev_box(
    rho_out: &dyn Fn(f64) -> f64,
    bore: f64,
    ys: (f64, f64),
    x: (f64, f64),
    z: (f64, f64),
) -> f64 {
    if ys.1 <= ys.0 {
        return 0.0;
    }
    let base = disc_poly(1.0, 1 << 13);
    let disc_rect = |rho: f64| {
        let p: Vec<(f64, f64)> = base.iter().map(|&(a, b)| (a * rho, b * rho)).collect();
        area(&clip_rect(&p, x, z))
    };
    let inner = disc_rect(bore);
    let g = [
        (-0.774_596_669_241_483_4, 5.0 / 9.0),
        (0.0, 8.0 / 9.0),
        (0.774_596_669_241_483_4, 5.0 / 9.0),
    ];
    let panels = 400;
    let w = (ys.1 - ys.0) / panels as f64;
    let mut s = 0.0;
    for k in 0..panels {
        let c = ys.0 + (k as f64 + 0.5) * w;
        for (t, wt) in g {
            let yy = c + t * w / 2.0;
            s += wt * (disc_rect(rho_out(yy)) - inner) * w / 2.0;
        }
    }
    s
}

#[test]
#[ignore = "differential battery; run with --ignored"]
fn join1_r1_tube_battery() {
    let (bore, r, h) = (0.2, 0.5, 2.0);
    let tube = revolved_tube(bore, r, h);
    let ann = |rect_x: (f64, f64), rect_z: (f64, f64)| {
        area(&clip_rect(&disc_poly(r, 1 << 16), rect_x, rect_z))
            - area(&clip_rect(&disc_poly(bore, 1 << 16), rect_x, rect_z))
    };
    let vt = ann((-1.0, 1.0), (-1.0, 1.0)) * h;
    // Boxes whose faces hold the seam ruling (x = 0.5, z = 0): the face
    // z = 0 through the axis, with every x range; plus wedges (y-prisms)
    // whose apex edge is the seam ruling.
    let coords = [-1.0, -0.25, 0.0, 0.35, 0.5, 1.0];
    let mut ranges = Vec::new();
    for i in 0..coords.len() {
        for j in i + 1..coords.len() {
            ranges.push((coords[i], coords[j]));
        }
    }
    let ys = [(-1.0, 3.0), (1.0, 3.0), (-1.0, 1.0), (0.5, 1.5), (0.0, 2.0)];
    let run = |tag: &str, w: &AtRestBody<f64>, vw: f64, vi: f64| {
        for (op, want_ab, want_ba) in [
            ("U", vt + vw - vi, vt + vw - vi),
            ("S", vt - vi, vw - vi),
            ("I", vi, vi),
        ] {
            for (order, want) in [("AB", want_ab), ("BA", want_ba)] {
                let (l, rr) = if order == "AB" {
                    (&tube, w)
                } else {
                    (w, &tube)
                };
                let res = match op {
                    "U" => topo::union(l, rr, tol()),
                    "S" => topo::subtract(l, rr, tol()),
                    _ => topo::intersect(l, rr, tol()),
                };
                println!("TUBE {tag} {op} {order} => {}", outcome(res, want, tol()));
            }
        }
    };
    for &x in &ranges {
        for &z in &ranges {
            for &y in &ys {
                let b = finished("the brick", brick(x, y, z, tol()), tol());
                let vb = (x.1 - x.0) * (y.1 - y.0) * (z.1 - z.0);
                let vi = ann(x, z) * overlap((0.0, h), y);
                run(&format!("box x={x:?} y={y:?} z={z:?}"), &b, vb, vi);
            }
        }
    }
    let wedges: Vec<(&str, Vec<(f64, f64)>)> = vec![
        ("in45", vec![(0.5, 0.0), (-0.5, 1.0), (-0.5, -1.0)]),
        ("in20", vec![(0.5, 0.0), (-0.5, 0.36), (-0.5, -0.36)]),
        (
            "flatlo",
            vec![(0.5, 0.0), (-0.5, 0.0), (-0.5, -1.0), (0.5, -1.0)],
        ),
        ("skew", vec![(0.5, 0.0), (-0.2, 0.9), (-0.9, -0.3)]),
        (
            "half",
            vec![(0.5, 0.0), (1.0, 0.0), (1.0, 1.0), (-1.0, 1.0), (-1.0, 0.0)],
        ),
    ];
    for (name, xz) in &wedges {
        let mut p = xz.clone();
        if area(&p) < 0.0 {
            p.reverse();
        }
        let a_i = area(&clip_convex(&disc_poly(r, 1 << 16), &p))
            - area(&clip_convex(&disc_poly(bore, 1 << 16), &p));
        for &y in &ys {
            let w = y_prism(&p, y);
            run(
                &format!("wedge {name} y={y:?}"),
                &w,
                area(&p) * (y.1 - y.0),
                a_i * overlap((0.0, h), y),
            );
        }
    }
}

#[test]
#[ignore = "differential battery; run with --ignored"]
fn join1_r1_bored_capsule_battery() {
    let (bore, r, c, big) = (0.2, 0.5, 1.0, 0.7);
    let (body, top) = bored_capsule(bore, r, c, big);
    let yc = c - (big * big - r * r).sqrt();
    let rho = move |y: f64| {
        if y <= c {
            r
        } else {
            (big * big - (y - yc).powi(2)).max(0.0).sqrt()
        }
    };
    let vol = solid_of_rev_box(&rho, bore, (0.0, c), (-1.0, 1.0), (-1.0, 1.0))
        + solid_of_rev_box(&rho, bore, (c, top), (-1.0, 1.0), (-1.0, 1.0));
    let coords = [-1.0, -0.25, 0.0, 0.3, 1.0];
    let mut ranges = Vec::new();
    for i in 0..coords.len() {
        for j in i + 1..coords.len() {
            ranges.push((coords[i], coords[j]));
        }
    }
    for &x in &ranges {
        for &z in &ranges {
            for y in [(1.0, 3.0), (0.5, 3.0), (-1.0, 1.0), (1.0, 1.1)] {
                let b = finished("the brick", brick(x, y, z, tol()), tol());
                let vb = (x.1 - x.0) * (y.1 - y.0) * (z.1 - z.0);
                let lo = (y.0.max(0.0), y.1.min(c));
                let hi = (y.0.max(c), y.1.min(top));
                let vi =
                    solid_of_rev_box(&rho, bore, lo, x, z) + solid_of_rev_box(&rho, bore, hi, x, z);
                for (op, want_ab, want_ba) in [
                    ("U", vol + vb - vi, vol + vb - vi),
                    ("S", vol - vi, vb - vi),
                    ("I", vi, vi),
                ] {
                    for (order, want) in [("AB", want_ab), ("BA", want_ba)] {
                        let (l, rr) = if order == "AB" {
                            (&body, &b)
                        } else {
                            (&b, &body)
                        };
                        let res = match op {
                            "U" => topo::union(l, rr, tol()),
                            "S" => topo::subtract(l, rr, tol()),
                            _ => topo::intersect(l, rr, tol()),
                        };
                        println!(
                            "BCAP x={x:?} y={y:?} z={z:?} {op} {order} => {}",
                            outcome(res, want, tol())
                        );
                    }
                }
            }
        }
    }
}

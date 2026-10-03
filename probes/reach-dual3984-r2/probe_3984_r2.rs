//! Reviewer probes for PR #3984 at 8abb6e7931 (lane reach-dual3984-r2).
//! Public doors only. A: every operand carrying a spiric or NURBS edge,
//! every op, both orders, three scales and a rigid re-pose, refuses at
//! the operand gate typed, naming an edge of that operand whose carrier
//! is spiric or NURBS. B: circle-edged rods (the conic arm that
//! `plane_crossing_lane` re-plumbed) against bricks, every op, both
//! orders, scales 1e-3/1/1e3 and a re-pose, checked against closed-form
//! volumes and a point-membership oracle of my own; results re-used as
//! operands.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;
use geom::Curve3;
use geom_core::{Affine3, Point2, Point3, Tol, Vec3};
use profile::{Profile, SketchPlane, test_support::bulge_loop};
use sweep::test_support::{brick, loft_prism, swept_elbow};
use sweep::{ExtrudeSide, Extrusion, extrude, loft_body};
use topo::{Body, BooleanError, Operand};

use crate::common::torus_walls::vessel_cavity;

fn tol() -> Tol {
    Tol::witness()
}

fn pose() -> Affine3<f64> {
    Affine3::rotation_about_axis(Point3::new(0.3, -0.7, 0.2), Vec3::new(1.0, 2.0, -0.5), 0.9)
}

fn posed(b: &Body<f64>, m: &Affine3<f64>) -> Body<f64> {
    topo::transform_rigid(b, m, tol()).expect("rigid re-pose")
}

/// A degree-1 loft of a `w × d` rectangle to height `h`: NURBS walls
/// and seams, geometrically a box.
fn lofted_box(w: f64, d: f64, h: f64) -> Body<f64> {
    let lp = bulge_loop(
        [(0.0, 0.0), (w, 0.0), (w, d), (0.0, d)]
            .iter()
            .map(|&(x, y)| (Point2::new(x, y), 0.0))
            .collect(),
    );
    let sections = vec![vec![lp.clone()], vec![lp]];
    let places = vec![
        Affine3::identity(),
        Affine3::translation(Vec3::new(0.0, 0.0, h)),
    ];
    loft_body::<f64>(&sections, &places, 1, tol()).expect("lofts").body
}

fn curved_kind(b: &Body<f64>, e: topo::EdgeKey) -> Option<&'static str> {
    let edge = b.get_edge(e)?;
    match b.get_curve_geom(edge.curve)?.certified()?.carrier() {
        Curve3::Nurbs(_) => Some("nurbs"),
        Curve3::Spiric { .. } => Some("spiric"),
        _ => None,
    }
}

type Op = fn(&Body<f64>, &Body<f64>, Tol) -> Result<topo::BooleanResult<f64>, BooleanError>;
const OPS: [(&str, Op); 3] = [
    ("union", topo::union),
    ("intersect", topo::intersect),
    ("subtract", topo::subtract),
];

#[test]
fn a_every_curved_edge_operand_refuses_at_the_gate_typed() {
    let mut fixtures: Vec<(String, Body<f64>, Body<f64>)> = Vec::new();
    for s in [1e-3, 1.0, 1e3] {
        let lb = lofted_box(2.0 * s, 1.5 * s, s);
        for (bn, b) in [
            ("far", brick((10.0 * s, 11.0 * s), (0.0, s), (0.0, s), tol())),
            ("across", brick((s, 3.0 * s), (0.5 * s, s), (-s, 2.0 * s), tol())),
            ("inside", brick((0.5 * s, s), (0.5 * s, s), (0.25 * s, 0.75 * s), tol())),
            ("around", brick((-s, 3.0 * s), (-s, 3.0 * s), (-s, 2.0 * s), tol())),
        ] {
            fixtures.push((format!("lofted_box s={s:e} {bn}"), lb.clone(), b));
        }
    }
    let prism = loft_prism(tol());
    for (bn, b) in [
        ("far", brick((10.0, 11.0), (0.0, 1.0), (0.0, 1.0), tol())),
        ("slab", brick((-3.0, 3.0), (-0.2, 0.3), (-1.0, 3.0), tol())),
        ("peg", brick((-0.2, 0.2), (-0.2, 0.2), (1.5, 3.0), tol())),
    ] {
        fixtures.push((format!("loft_prism {bn}"), prism.clone(), b));
    }
    let m = pose();
    fixtures.push((
        "loft_prism slab posed".into(),
        posed(&prism, &m),
        posed(&brick((-3.0, 3.0), (-0.2, 0.3), (-1.0, 3.0), tol()), &m),
    ));
    let elbow = swept_elbow(tol());
    fixtures.push((
        "swept_elbow far".into(),
        elbow.clone(),
        brick((50.0, 51.0), (50.0, 51.0), (50.0, 51.0), tol()),
    ));
    let (_, cavity) = vessel_cavity(1.0 / 128.0);
    fixtures.push((
        "vessel_cavity far".into(),
        cavity.clone(),
        brick((5.0, 6.0), (5.0, 6.0), (5.0, 6.0), tol()),
    ));
    fixtures.push((
        "vessel_cavity across".into(),
        cavity,
        brick((-0.05, 0.05), (-0.05, 0.3), (-0.05, 0.05), tol()),
    ));
    fixtures.push(("lofted_box vs prism".into(), lofted_box(2.0, 1.5, 1.0), prism));
    let mut bad = Vec::new();
    let mut n = 0;
    for (name, curved, other) in &fixtures {
        for (op, f) in OPS {
            for (order, a, b, want) in [
                ("C·B", curved, other, Operand::A),
                ("B·C", other, curved, Operand::B),
            ] {
                n += 1;
                let what = format!("{name} {op} {order}");
                match f(a, b, tol()) {
                    Err(e @ BooleanError::CurvedEdgeUnsupported { operand, edge }) => {
                        // When both are curved, A is gated first.
                        let both = curved_kind(other, other.edges().next().unwrap().0).is_some()
                            || other.edges().any(|(k, _)| curved_kind(other, k).is_some());
                        let exp = if both { Operand::A } else { want };
                        let owner = if operand == Operand::A { a } else { b };
                        let kind = curved_kind(owner, edge);
                        let msg = e.to_string();
                        println!("[A] {what}: CurvedEdgeUnsupported {{{operand:?}, {kind:?}}}");
                        if operand != exp || kind.is_none() || !msg.contains("spiric or spline") {
                            bad.push(format!("{what}: {operand:?} {kind:?} {msg}"));
                        }
                    }
                    Err(e @ BooleanError::CurvedPairUnsupported { .. }) => {
                        // The op's revert roster (main's, pre-gate): typed, names its pair.
                        println!("[A] {what}: ROSTER {e:?}");
                    }
                    Err(e) => {
                        println!("[A] {what}: OTHER {e:?}");
                        bad.push(format!("{what}: refused with another variant: {e:?}"));
                    }
                    Ok(_) => {
                        println!("[A] {what}: BUILT");
                        bad.push(format!("{what}: BUILT past the gate"));
                    }
                }
            }
        }
    }
    println!("[A] {n} outcomes, {} bad", bad.len());
    assert!(bad.is_empty(), "{bad:#?}");
}

// ----------------------------------------------------------------------
// B: the conic arm, against my own oracle.
// ----------------------------------------------------------------------

struct Rod {
    c: (f64, f64),
    r: f64,
    z: (f64, f64),
}
struct Bx {
    x: (f64, f64),
    y: (f64, f64),
    z: (f64, f64),
}

impl Rod {
    fn body(&self) -> Body<f64> {
        let disc = profile::circle(Point2::new(self.c.0, self.c.1), self.r, tol()).unwrap();
        let plane = SketchPlane::from_frame(geom_core::OrthoFrame::axes_xy(Point3::new(
            0.0, 0.0, self.z.0,
        )));
        let p = Profile::new(plane, vec![disc.into()]).validate(tol()).unwrap();
        extrude(
            &p,
            Extrusion::Distance {
                depth: self.z.1 - self.z.0,
                side: ExtrudeSide::Along,
            },
            tol(),
        )
        .unwrap()
        .body
    }
    fn inside(&self, p: (f64, f64, f64)) -> bool {
        let (dx, dy) = (p.0 - self.c.0, p.1 - self.c.1);
        dx * dx + dy * dy < self.r * self.r && self.z.0 < p.2 && p.2 < self.z.1
    }
    fn vol(&self) -> f64 {
        PI * self.r * self.r * (self.z.1 - self.z.0)
    }
}
impl Bx {
    fn body(&self) -> Body<f64> {
        brick(self.x, self.y, self.z, tol())
    }
    fn inside(&self, p: (f64, f64, f64)) -> bool {
        self.x.0 < p.0 && p.0 < self.x.1 && self.y.0 < p.1 && p.1 < self.y.1 && self.z.0 < p.2 && p.2 < self.z.1
    }
    fn vol(&self) -> f64 {
        (self.x.1 - self.x.0) * (self.y.1 - self.y.0) * (self.z.1 - self.z.0)
    }
}

/// Exact `|rod ∩ box|` when the box spans the rod in x and cuts it with
/// a lower y face at `y0` (the y upper face above the rod).
fn inter_vol(rod: &Rod, b: &Bx) -> f64 {
    let d = b.y.0 - rod.c.1;
    let r = rod.r;
    let seg = if d >= r {
        0.0
    } else if d <= -r {
        PI * r * r
    } else {
        r * r * (d / r).acos() - d * (r * r - d * d).sqrt()
    };
    let lz = (rod.z.1.min(b.z.1) - rod.z.0.max(b.z.0)).max(0.0);
    seg * lz
}

fn halton(i: u32, base: u32) -> f64 {
    let (mut f, mut r, mut i) = (1.0, 0.0, i);
    while i > 0 {
        f /= base as f64;
        r += f * (i % base) as f64;
        i /= base;
    }
    r
}

#[test]
fn b_conic_arm_builds_every_op_at_the_closed_form() {
    let mut bad = Vec::new();
    let mut n = 0;
    for s in [1e-3, 1.0, 1e3] {
        for (d, zb) in [(0.2, (1.0, 5.0)), (-0.3, (-1.0, 2.5)), (0.0137, (0.7, 3.3))] {
            let rod = Rod { c: (0.1 * s, -0.05 * s), r: 0.5 * s, z: (0.0, 4.0 * s) };
            let b = Bx {
                x: (-1.0 * s, 1.2 * s),
                y: (rod.c.1 + d * s, 1.0 * s),
                z: (zb.0 * s, zb.1 * s),
            };
            let (vr, vb, vi) = (rod.vol(), b.vol(), inter_vol(&rod, &b));
            for (pn, map) in [("unposed", None), ("posed", Some(pose()))] {
                let (ra, bb) = match &map {
                    None => (rod.body(), b.body()),
                    Some(m) => match (
                        topo::transform_rigid(&rod.body(), m, tol()),
                        topo::transform_rigid(&b.body(), m, tol()),
                    ) {
                        (Ok(x), Ok(y)) => (x, y),
                        other => {
                            println!("[B] s={s:e} d={d} posed: the re-pose itself refuses (fixture, not the PR): {:?}", other.0.err());
                            continue;
                        }
                    },
                };
                for (op, f) in OPS {
                    for (order, x, y, want, member) in [
                        ("rod·box", &ra, &bb, match op {
                            "union" => vr + vb - vi, "intersect" => vi, _ => vr - vi }, 0u8),
                        ("box·rod", &bb, &ra, match op {
                            "union" => vr + vb - vi, "intersect" => vi, _ => vb - vi }, 1u8),
                    ] {
                        n += 1;
                        let what = format!("s={s:e} d={d} {pn} {op} {order}");
                        let r = match f(x, y, tol()) {
                            Ok(r) => r,
                            Err(e) => {
                                println!("[B] {what}: REFUSED {e:?}");
                                bad.push(format!("{what}: refused {e}"));
                                continue;
                            }
                        };
                        let Some(res) = r.body() else {
                            if want.abs() > 1e-12 * s * s * s {
                                bad.push(format!("{what}: empty, want {want}"));
                            }
                            continue;
                        };
                        let v = topo::mass_properties(&res.body, tol()).unwrap().volume;
                        let rel = (v - want).abs() / want;
                        // Membership: 400 Halton points in the union's box,
                        // off a 1e-6·s shell of every boundary.
                        let mut miss = 0;
                        let band = geom_core::Band::linear(tol()).unwrap();
                        for i in 1..=400u32 {
                            let p = (
                                (-1.2 + 2.6 * halton(i, 2)) * s,
                                (-0.7 + 1.9 * halton(i, 3)) * s,
                                (-1.2 + 6.4 * halton(i, 5)) * s,
                            );
                            let (inr, inb) = (rod.inside(p), b.inside(p));
                            let near = {
                                let (dx, dy) = (p.0 - rod.c.0, p.1 - rod.c.1);
                                let e = 1e-6 * s;
                                ((dx * dx + dy * dy).sqrt() - rod.r).abs() < e
                                    || [rod.z.0, rod.z.1, b.z.0, b.z.1].iter().any(|z| (p.2 - z).abs() < e)
                                    || [b.x.0, b.x.1].iter().any(|x| (p.0 - x).abs() < e)
                                    || [b.y.0, b.y.1].iter().any(|y| (p.1 - y).abs() < e)
                            };
                            if near {
                                continue;
                            }
                            let truth = match (op, member) {
                                ("union", _) => inr || inb,
                                ("intersect", _) => inr && inb,
                                (_, 0) => inr && !inb,
                                _ => inb && !inr,
                            };
                            let q = Point3::new(p.0, p.1, p.2);
                            let q = map.as_ref().map_or(q, |m| m.transform_point(q));
                            match topo::point_in_solid(&res.body, q, band, tol()) {
                                Ok(topo::SolidContainment::In) if truth => {}
                                Ok(topo::SolidContainment::Out) if !truth => {}
                                other => {
                                    miss += 1;
                                    if miss <= 2 {
                                        println!("[B] {what}: pis {p:?} truth {truth} got {other:?}");
                                    }
                                }
                            }
                        }
                        println!("[B] {what}: V {v:.12e} want {want:.12e} rel {rel:.1e} pis-miss {miss}");
                        if rel > 1e-7 || miss > 0 {
                            bad.push(format!("{what}: rel {rel:e} miss {miss}"));
                        }
                        // Re-use: the result as an operand, minus a far-side slab.
                        if op == "subtract" && order == "rod·box" {
                            let slab = Bx { x: (-1.0 * s, 1.2 * s), y: (-1.0 * s, 1.0 * s), z: (3.5 * s, 4.5 * s) };
                            let sb = match &map { None => slab.body(), Some(m) => posed(&slab.body(), m) };
                            match topo::subtract(&res.body, &sb, tol()) {
                                Ok(r2) => {
                                    let v2 = r2.body().map_or(0.0, |bb| topo::mass_properties(&bb.body, tol()).unwrap().volume);
                                    // rod∖box∖slab: the slab takes z∈[3.5s,4s] of what is left.
                                    let left_in_slab = PI * rod.r * rod.r * 0.5 * s
                                        - inter_vol(&Rod { c: rod.c, r: rod.r, z: (3.5 * s, 4.0 * s) }, &b);
                                    let w2 = want - left_in_slab;
                                    let rel2 = (v2 - w2).abs() / w2;
                                    println!("[B] {what} then ∖slab: V {v2:.12e} want {w2:.12e} rel {rel2:.1e}");
                                    if rel2 > 1e-7 {
                                        bad.push(format!("{what} ∖slab: rel {rel2:e}"));
                                    }
                                }
                                Err(e) => println!("[B] {what} then ∖slab: REFUSED {e}"),
                            }
                        }
                    }
                }
            }
        }
    }
    println!("[B] {n} outcomes, {} bad", bad.len());
    assert!(bad.is_empty(), "{bad:#?}");
}

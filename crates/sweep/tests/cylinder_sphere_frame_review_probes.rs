//! Review probes for PR #4025 (the transverse cylinder × sphere frame):
//! poses the PR's own rows do not reach — a ball centre near the wall,
//! reaches either side of the one-to-two-loop transition, a ball radius
//! near the drum's, an off-centre ball, a tilted drum — each run in both
//! orders under every op, held to the lane door and to the matcher's
//! neighbour pairing read off the cylinder chart (as in
//! `cylinder_sphere_frame::every_segment_joins_neighbouring_sites_along_its_loop`).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use crate::common::germ_pair::cyl;
use crate::conic_edge_curved_face::ball;
use geom_core::{Affine3, Point3, Tol, Vec3};
use topo::{Body, BooleanError, BooleanOp};

const R: f64 = 0.5;

struct Pose {
    label: &'static str,
    big: f64,
    c: [f64; 3],
    /// A rigid tilt of both operands: angle about `(0.3, -0.8, 0.5)`
    /// through `(0.1, 0.2, -0.1)`.
    tilt: f64,
}

fn tilt_map(angle: f64) -> Affine3<f64> {
    Affine3::rotation_about_axis(
        Point3::new(0.1, 0.2, -0.1),
        Vec3::new(0.3, -0.8, 0.5).normalize(),
        angle,
    )
}

fn poses() -> Vec<Pose> {
    let p = |label, big, c, tilt| Pose {
        label,
        big,
        c,
        tilt,
    };
    vec![
        p("near wall inside 1e-6", 0.3, [0.5 - 1e-6, 0.0, 0.0], 0.0),
        p("near wall outside 1e-6", 0.3, [0.5 + 1e-6, 0.0, 0.0], 0.0),
        p("near wall, off x", 0.3, [0.3, 0.4 - 1e-5, 0.1], 0.0),
        p("one loop just short 1e-3", 0.7 - 1e-3, [0.2, 0.0, 0.0], 0.0),
        p("two loops just past 1e-3", 0.7 + 1e-3, [0.2, 0.0, 0.0], 0.0),
        p("one loop just short 1e-5", 0.7 - 1e-5, [0.2, 0.0, 0.0], 0.0),
        p("two loops just past 1e-5", 0.7 + 1e-5, [0.2, 0.0, 0.0], 0.0),
        p("one loop just short 1e-7", 0.7 - 1e-7, [0.2, 0.0, 0.0], 0.0),
        p("two loops just past 1e-7", 0.7 + 1e-7, [0.2, 0.0, 0.0], 0.0),
        p("R = r, d 0.05", 0.5, [0.05, 0.0, 0.0], 0.0),
        p("R = r, d 1e-3", 0.5, [0.0, 1e-3, 0.0], 0.0),
        p("R = r + 1e-3, d 1e-4", 0.5 + 1e-3, [1e-4, 0.0, 0.0], 0.0),
        p("off-centre ball", 0.45, [-0.2, 0.3, 0.4], 0.0),
        p("centre far outside", 0.3, [0.0, -0.75, -0.3], 0.0),
        p("tilted: one loop", 0.3, [0.35, 0.0, 0.1], 0.8),
        p("tilted: two loops", 0.75, [0.1, 0.15, 0.0], 1.3),
        p("tilted: just short", 0.7 - 1e-4, [0.0, 0.2, 0.0], 2.1),
        p("tilted: just past", 0.7 + 1e-4, [0.0, 0.2, 0.0], 2.1),
        p("tilted: near wall", 0.3, [-0.5 + 1e-6, 0.0, 0.2], 0.5),
    ]
}

impl Pose {
    fn operands(&self) -> (Body<f64>, Body<f64>) {
        let (a, b) = (cyl(R, 1.0), ball(self.big, self.c));
        if self.tilt == 0.0 {
            return (a, b);
        }
        let m = tilt_map(self.tilt);
        (
            topo::transform_rigid(&a, &m, Tol::witness()).unwrap(),
            topo::transform_rigid(&b, &m, Tol::witness()).unwrap(),
        )
    }

    /// A site's place on its loop (see the PR's `place`), read after
    /// undoing the tilt.
    fn place(&self, p: Point3<f64>) -> (i8, f64, f64) {
        let p = if self.tilt == 0.0 {
            p
        } else {
            tilt_map(-self.tilt).transform_point(p)
        };
        let d = self.c[0].hypot(self.c[1]);
        let theta = {
            let t = p.y.atan2(p.x) - self.c[1].atan2(self.c[0]);
            (t + PI).rem_euclid(2.0 * PI) - PI
        };
        let up = p.z >= self.c[2];
        if self.big > R + d {
            (if up { 1 } else { -1 }, theta.rem_euclid(2.0 * PI), 2.0 * PI)
        } else {
            let t0 = ((R * R + d * d - self.big * self.big) / (2.0 * R * d)).acos();
            let tau = if up { theta + t0 } else { 3.0 * t0 - theta };
            (0, tau.rem_euclid(4.0 * t0), 4.0 * t0)
        }
    }
}

const OPS: [(&str, BooleanOp, bool); 6] = [
    ("A ∪ B", BooleanOp::Union, false),
    ("B ∪ A", BooleanOp::Union, true),
    ("A ∩ B", BooleanOp::Intersect, false),
    ("B ∩ A", BooleanOp::Intersect, true),
    ("A ∖ B", BooleanOp::Subtract, false),
    ("B ∖ A", BooleanOp::Subtract, true),
];

fn run(op: BooleanOp, x: &Body<f64>, y: &Body<f64>) -> Result<topo::BooleanResult<f64>, BooleanError> {
    let tol = Tol::witness();
    match op {
        BooleanOp::Union => topo::union(x, y, tol),
        BooleanOp::Intersect => topo::intersect(x, y, tol),
        BooleanOp::Subtract => topo::subtract(x, y, tol),
    }
}

fn door(e: &BooleanError) -> String {
    let s = format!("{e:?}");
    let head: String = s.chars().take_while(|c| *c != ' ' && *c != '{' && *c != '(').collect();
    match e {
        BooleanError::CurvedBooleanUnsupported { kind, .. } => format!("{head}({kind:?})"),
        BooleanError::Escalated { diag, .. } => format!("{head}({:?})", diag.predicate),
        _ => s.chars().take(220).collect(),
    }
}

fn cyclic_gap(a: f64, b: f64, period: f64) -> f64 {
    let g = (a - b).rem_euclid(period);
    g.min(period - g)
}

/// The probe battery: one line per run (door) and per matcher check.
/// Asserts nothing so the whole table prints; read its lines.
#[test]
#[ignore = "review probe battery"]
fn review_probe_battery() {
    let tol = Tol::witness();
    for pose in poses() {
        let (a, b) = pose.operands();
        for (op_label, op, swapped) in OPS {
            let label = format!("{}, {op_label}", pose.label);
            let (x, y) = if swapped { (&b, &a) } else { (&a, &b) };
            let got = run(op, x, y);
            let d = match &got {
                Ok(_) => "BUILT".to_string(),
                Err(e) => door(e),
            };
            let seg = match topo::test_support::boolean_segment_sites(op, x, y, tol) {
                Err(e) => format!("matcher refused {}", door(&e)),
                Ok(None) => "no pair".to_string(),
                Ok(Some((records, segments))) => {
                    let mut sites: Vec<(i8, f64, f64)> = Vec::new();
                    for s in &segments {
                        for &p in s {
                            let at = pose.place(p);
                            if !sites.iter().any(|q| q.0 == at.0 && cyclic_gap(q.1, at.1, at.2) < 1e-7) {
                                sites.push(at);
                            }
                        }
                    }
                    let mut bad = Vec::new();
                    for s in &segments {
                        let (p, q) = (pose.place(s[0]), pose.place(s[1]));
                        if p.0 != q.0 {
                            bad.push("cross-loop".to_string());
                            continue;
                        }
                        let mut along: Vec<f64> = sites.iter().filter(|t| t.0 == p.0).map(|t| t.1).collect();
                        along.sort_by(f64::total_cmp);
                        let index = |x: f64| along.iter().position(|&t| cyclic_gap(t, x, p.2) < 1e-7).unwrap();
                        let (i, j, n) = (index(p.1), index(q.1), along.len());
                        if !((i + 1) % n == j || (j + 1) % n == i) {
                            bad.push(format!("skip {i}->{j}/{n}"));
                        }
                    }
                    format!(
                        "records {records} segments {} sites {} {}",
                        segments.len(),
                        sites.len(),
                        if bad.is_empty() && segments.len() == records && segments.len() == sites.len() {
                            "NEIGHBOURS-OK".to_string()
                        } else {
                            format!("MATCH-BAD {bad:?}")
                        }
                    )
                }
            };
            println!("PROBE {label}: run={d} | {seg}");
        }
    }
}

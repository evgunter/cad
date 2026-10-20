//! Review probe (PR 5431, review 1): an independent reading of the
//! material round an edge that tier 3′ refuses `FaceThroughEdgeWedge`.
//!
//! Point classification of the edge's own solid on a small circle about
//! the edge's midpoint, in the plane across the edge. D1: the material
//! sectors on one line are disjoint and each is one edge, so a single
//! edge there is valid only where the material round the line is ONE
//! sector, the edge's own. Two or more material arcs about the line mean
//! the one-edge form is invalid; exactly one means the edge could be a
//! valid touch, and a decided refusal of it would be a wrong verdict.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, dead_code)]

use geom_core::{Band, Point3, Tol, Vec3};
use topo::readback::{face_pose, vertex_point};
use topo::{Body, EdgeKey, FaceKey, SolidContainment, ValidationError};

/// The number of material arcs about `edge`'s midpoint, and whether the
/// face's two in-plane directions read outside the material just off
/// the face on either side.
pub fn arcs(body: &Body<f64>, edge: EdgeKey, face: FaceKey, tol: Tol) -> (usize, String) {
    let e = body.get_edge(edge).unwrap();
    let plus = body.get_half_edge(e.he_plus).unwrap();
    let a = vertex_point(body, plus.start).unwrap();
    let b = vertex_point(body, body.get_half_edge(plus.next).unwrap().start).unwrap();
    let d = (b - a) * (1.0 / (b - a).norm());
    let mid = Point3::new((a.x + b.x) / 2.0, (a.y + b.y) / 2.0, (a.z + b.z) / 2.0);
    let n = face_pose(body, face).unwrap().axis;
    let n = Vec3::new(n.x, n.y, n.z);
    let u = d.cross(n);
    let u = u * (1.0 / u.norm());
    let solid = body
        .get_shell(body.get_face(face).unwrap().shell)
        .unwrap()
        .solid;
    let band = Band::linear(tol).unwrap();
    let len = (b - a).norm();
    let mut out = String::new();
    let mut best = 0;
    for r in [1e-2 * len.min(1.0), 1e-3 * len.min(1.0)] {
        let m = 180;
        let mut read = Vec::with_capacity(m);
        for k in 0..m {
            let t = (k as f64 + 0.5) / m as f64 * std::f64::consts::TAU;
            let q = mid + (u * t.cos() + n * t.sin()) * r;
            read.push(topo::point_in_solid_of(body, solid, q, band, tol).ok());
        }
        let is_in = |x: &Option<SolidContainment>| *x == Some(SolidContainment::In);
        let rises = (0..m)
            .filter(|&k| is_in(&read[k]) && !is_in(&read[(k + m - 1) % m]))
            .count();
        let ins = read.iter().filter(|x| is_in(x)).count();
        let undecided = read.iter().filter(|x| x.is_none()).count();
        let up = (0..m / 2).filter(|&k| is_in(&read[k])).count();
        let down = (m / 2..m).filter(|&k| is_in(&read[k])).count();
        // A touch from outside F: one half-disc wholly F's material, the
        // other holding the edge's wedge. Anything else: F's material
        // side is cut, so F passes through a material sector's bounds.
        let full = up == m / 2 || down == m / 2;
        out.push_str(&format!(
            "r={r:e}: {rises} arcs {ins} in, up {up}/{h} down {down}/{h}, {undecided} unread; ",
            h = m / 2
        ));
        best = best.max(if full { 1 } else { 2 });
    }
    (best, out)
}

/// Every decided `FaceThroughEdgeWedge` in `errors`, read by [`arcs`];
/// prints a line per finding, `REV5431-ONEARC` where the material is one
/// arc (a decided refusal of a possibly valid body).
pub fn report(what: &str, body: &Body<f64>, errors: &[ValidationError], tol: Tol) {
    for e in errors {
        if let ValidationError::FaceThroughEdgeWedge { edge, face, witness } = e {
            let (k, line) = arcs(body, *edge, *face, tol);
            let tag = if k >= 2 {
                "REV5431-RIDERSHAPE"
            } else {
                "REV5431-OUTSIDETOUCH"
            };
            println!("{tag} {what} at {witness}: {line}");
        }
        if let ValidationError::FaceThroughEdgeWedgeUndecided {
            edge, face, witness, what: why,
        } = e
        {
            let (k, line) = arcs(body, *edge, *face, tol);
            let tag = if k >= 2 {
                "REV5431-UNDEC-RIDERSHAPE"
            } else {
                "REV5431-UNDEC-OUTSIDETOUCH"
            };
            let why: String = why.chars().take(40).collect();
            println!("{tag} {what} at {witness} [{why}]: {line}");
        }
    }
}

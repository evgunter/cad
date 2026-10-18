//! **The census's edge-face lane reads where a touch could be before
//! it escalates a proxy for one**: on JOIN's near-tangent poses
//! ([`NtPrism::near_normal`]) every op in both orders builds a body
//! whose tier-3′ census reads, against a face of the other operand,
//!
//! - a face's boundary vertex within the band of an edge's LINE, a
//!   metre past the edge's end (`pm_census_ef_cut_gap`), and
//! - an edge's end within the band of a face's PLANE, half a metre
//!   outside the face (`pm_census_ef_residual`),
//!
//! both of which once escalated, and the boundary edge leaving the
//! edge's end the other way, nearly collinear, which the crossing
//! reading of the two segments escalated (`curve_pair_pose_offset`).
//! None of the three can make a cut, a pierce or an overlap there, so
//! the lane passes them by. What the census still escalates on these
//! bodies is the edge-edge lane's.
//!
//! The poses are per ε row, each reproducing the escalations there.
//!
//! [`NtPrism::near_normal`]: crate::common::differential::NtPrism::near_normal

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::Tol;
use topo::ValidationError;

use crate::common::differential::{nt_cube, nt_prisms};

/// `(ε, prism, edge, sixteenths, tilt)`: the census's `"{prism} nt
/// e{edge} a{sixteenths} d{tilt}"` poses whose bodies escalated the
/// edge-face lane at that ε row.
const POSES: [(f64, &str, usize, f64, f64); 7] = [
    (1e-9, "Ltop", 2, 8.0, 1e-9),
    (1e-9, "vee300", 0, 0.0, 1e-8),
    (1e-9, "vee300", 0, 0.0, -1e-8),
    (1e-12, "vee300", 0, 0.0, 1e-11),
    (1e-12, "vee300", 0, 0.0, -1e-11),
    (1e-6, "Lbot", 0, 0.0, -1e-6),
    (1e-6, "vee300", 0, 0.0, 1e-5),
];

/// Whether `e` is an escalation of the edge-face or vertex-face lane:
/// a residual, the point-on-span reading a vertex cut is made by
/// (`curve_carrier_*`, `curve_pair_span_*`), or the crossing reading
/// (`curve_pair_*`).
fn edge_face_lane(e: &ValidationError) -> bool {
    matches!(e, ValidationError::CensusEscalated { cause }
    if cause.predicate.is_some_and(|p| {
        p.starts_with("pm_census_ef_")
            || p == "pm_census_vf_residual"
            || p.starts_with("curve_carrier_")
            || p.starts_with("curve_pair_")
    }))
}

#[test]
fn the_edge_face_lane_escalates_no_proxy_on_the_near_tangent_poses() {
    let tol = Tol::witness();
    let prisms = nt_prisms();
    let mut wrong = Vec::new();
    let mut built = 0;
    for &(eps, name, edge, a, d) in &POSES {
        if (tol.eps() - eps).abs() > eps * 1e-9 {
            continue;
        }
        let pr = prisms.iter().find(|p| p.name == name).unwrap();
        let body = pr.body(tol);
        let cube = nt_cube(pr.v, pr.near_normal(edge, a, d), tol);
        for (op, r) in [
            ("prism ∪ cube", topo::union(&body, &cube, tol)),
            ("cube ∪ prism", topo::union(&cube, &body, tol)),
            ("prism ∩ cube", topo::intersect(&body, &cube, tol)),
            ("cube ∩ prism", topo::intersect(&cube, &body, tol)),
            ("prism ∖ cube", topo::subtract(&body, &cube, tol)),
            ("cube ∖ prism", topo::subtract(&cube, &body, tol)),
        ] {
            let Some(bb) = r.as_ref().ok().and_then(|r| r.body()) else {
                continue;
            };
            built += 1;
            let errors = topo::validate_pseudomanifold(&bb.body, &bb.contacts, tol)
                .err()
                .unwrap_or_default();
            let lane: Vec<_> = errors.iter().filter(|e| edge_face_lane(e)).collect();
            if !lane.is_empty() {
                wrong.push(format!("{name} nt e{edge} a{a} d{d:e}, {op}: {lane:?}"));
            }
        }
    }
    assert!(built > 0, "no pose built at ε = {:e}", tol.eps());
    assert!(
        wrong.is_empty(),
        "{} runs escalate the edge-face lane:\n{}",
        wrong.len(),
        wrong.join("\n")
    );
}

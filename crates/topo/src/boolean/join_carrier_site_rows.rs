//! **The join's sites refuse a spiric or spline edge typed, naming the
//! edge and the face** ([`BooleanError::EdgeCarrierUnsupported`]): the
//! germ frame along an edge of either solid, and the ring run.
//!
//! The rows drive each site directly on the sweep rows' sheets
//! (`reduce::planar_lane_carrier_rows`): a planar sheet in `z = 0`
//! bounded by a quadratic Bézier arc and its chord, and one in `x = ½`
//! bounded by a spiric arc and its chord.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use super::super::reduce::planar_lane_carrier_rows::{arc_sheet, spiric_cap};
use super::super::{BooleanError, Branch, EdgeCarrierSite, HalfGerm, Locus, Operand};
use super::{IslandClosing, germ_section_loops, ring_run_ccw};
use crate::chord_join::SegmentCurve;
use crate::{Body, EdgeKey, FaceKey, HalfEdgeKey};
use geom_core::{Band, Point3, Tol, UnitVec3, Vec3};

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

/// The face on `edge`'s plus side, and that half.
fn plus_side(body: &Body<f64>, edge: EdgeKey) -> (FaceKey, HalfEdgeKey) {
    let he = body.get_edge(edge).unwrap().he_plus;
    (body.face_of_linked(he), he)
}

fn assert_site(
    got: Result<impl core::fmt::Debug, BooleanError>,
    (operand, edge, face): (Operand, EdgeKey, FaceKey),
    site: EdgeCarrierSite,
    what: &str,
) {
    match got {
        Err(BooleanError::EdgeCarrierUnsupported {
            operand: o,
            edge: e,
            face: f,
            face_of,
            site: s,
        }) => {
            assert_eq!(
                (o, e, f, face_of, s),
                (operand, edge, face, operand, site),
                "{what}"
            );
        }
        other => panic!("{what}: expected the {site:?} refusal, got {other:?}"),
    }
}

/// **A germ along a spline or spiric edge of either solid has no
/// frame.** Read for the face pair instead, a coplanar pair's
/// straight-chord test would take a curved edge for its chord. Each
/// sheet stands as the edge's solid on each side, the other sheet as
/// its partner.
#[test]
fn a_germ_along_a_curved_edge_refuses_at_the_germ_frame() {
    for (name, (body, edge)) in [("spline", arc_sheet(1.0, 1.0)), ("spiric", spiric_cap())] {
        let (other, _) = arc_sheet(1.0, 0.5);
        let (face, _) = plus_side(&body, edge);
        let germ = |on: Operand| HalfGerm {
            he: HalfEdgeKey::default(),
            a_face: if on == Operand::A {
                face
            } else {
                FaceKey::default()
            },
            b_face: if on == Operand::B {
                face
            } else {
                FaceKey::default()
            },
            a_locus: if on == Operand::A {
                Locus::OnEdge(edge)
            } else {
                Locus::InFace(FaceKey::default())
            },
            b_locus: if on == Operand::B {
                Locus::OnEdge(edge)
            } else {
                Locus::InFace(FaceKey::default())
            },
            dir: Vec3::new(1.0, 0.0, 0.0),
            branch: Branch::Only,
        };
        assert_site(
            germ_section_loops((&body, &other), &germ(Operand::A), band()),
            (Operand::A, edge, face),
            EdgeCarrierSite::GermFrame,
            &format!("a {name} edge of A"),
        );
        assert_site(
            germ_section_loops((&other, &body), &germ(Operand::B), band()),
            (Operand::B, edge, face),
            EdgeCarrierSite::GermFrame,
            &format!("a {name} edge of B"),
        );
    }
}

/// **A ring run holding a spline edge refuses before any closing but a
/// carried chord's winds it**: the planar closing, whose chord winding
/// has no row for a spline; the wall closing, whose chart has no
/// closed-form image of one; and the quadric closing, which reads no
/// spline it holds no evidence of lying on the face for. The run is the
/// arc alone, closed by its chord. The refusal precedes any reading of
/// the face, so the sheet's plane face stands for the wall and the
/// quadric too: reverted, each closing answers its own way instead
/// (`SectionInvariant` on the plane, `RingIslandUnread` on the others).
/// The carried closing (`IslandClosing::Carried`) reads a spline run on
/// a cylinder, sphere or torus face from its edges' certificates
/// (`ring_path::SplineArc`): `cylinder_sphere_frame`'s turned-chart pose
/// reaches it on the drum's wall and its two-loop poses on the ball,
/// `germ_sphere_torus_placements` on the donut, and
/// `ring_path::spline_fuzz` searches its readings.
#[test]
fn a_ring_run_along_a_spline_edge_refuses_at_the_ring_run() {
    let (body, edge) = arc_sheet(1.0, 1.0);
    let (face, he) = plus_side(&body, edge);
    let chord = SegmentCurve::of((he, he), None);
    let closings = [
        (
            "planar",
            IslandClosing::Planar(Vec3::new(0.0, 0.0, 1.0), &chord),
        ),
        ("quadric", IslandClosing::Quadric(&chord)),
        (
            "wall",
            IslandClosing::Wall((
                Point3::new(0.0, 0.0, 0.0),
                UnitVec3::new(Vec3::new(0.0, 1.0, 0.0), "ring_run_rows", band()).unwrap(),
            )),
        ),
    ];
    for (name, closing) in closings {
        for operand in [Operand::A, Operand::B] {
            assert_site(
                ring_run_ccw((&body, operand), face, ((he, he), None), closing, band()),
                (operand, edge, face),
                EdgeCarrierSite::RingRun,
                &format!("{name} closing, operand {operand:?}"),
            );
        }
    }
}

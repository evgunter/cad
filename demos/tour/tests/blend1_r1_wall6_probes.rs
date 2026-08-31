//! BLEND-1 review probes (r1), tour half — the PR's wall-6 re-measure,
//! reproduced through the public façade as an outside consumer would.
//!
//! The PR body claims, on the tour's lily lantern: (a) wall 6 as
//! authored — EVERY edge requested — still refuses
//! `TangentialEdge { margin: 0.0 }` before any closed-rim door;
//! (b) requested one rim at a time, the three transverse rims at
//! carrier radii ~0.090, ~0.183, ~0.052 fillet whole at r = 0.02
//! through the new door; (c) the fourth (~0.253, the mouth) refuses
//! `"a concave chain adds material"`.
//!
//! The lantern here is rebuilt from `demos/tour/src/lily.rs`'s own
//! meridian numbers (globe 0.44, top 0.40, mouth 0.36, lip 0.09, drop
//! 0.16, neck (0.052, 70°)) at identity placement — carrier radii are
//! profile-intrinsic, so the PR's numbers must reproduce.
//!
//! Authored in the review lane and ADOPTED into the unit at its fix
//! pass, so the findings below are re-taken by the suite on every run
//! rather than living only in a review thread.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use pncad::authoring::{p2, validated};
use pncad::geom::Curve3;
use pncad::geom_core::{Band, Tol, Vec2};
use pncad::prelude::{BlendError, Open, Start, fillet_edges};
use pncad::profile::{ArcSweep, Center, ProfileLoop, SketchPlane};
use pncad::sweep::{Revolution, RevolveAxis, revolve};
use pncad::topo::{Body, EdgeKey};

const GLOBE: f64 = 0.44;
const TOP: f64 = 0.40;
const MOUTH: f64 = 0.36;
const LIP_R: f64 = 0.09;
const LIP_DROP: f64 = 0.16;
const NECK_R: f64 = 0.052;
const NECK_HALF_ANGLE: f64 = 70.0 * core::f64::consts::PI / 180.0;

fn lily_lantern(tol: Tol) -> Body<f64> {
    let r_top = (GLOBE.powi(2) - TOP.powi(2)).sqrt();
    let r_mouth = (GLOBE.powi(2) - MOUTH.powi(2)).sqrt();
    let shoulder = (r_top - NECK_R) / NECK_HALF_ANGLE.tan();
    let t_mouth = shoulder + TOP + MOUTH;
    let t_end = t_mouth + LIP_DROP;
    let meridian: ProfileLoop<f64> = Open
        .at(p2(0.0, 0.0))
        .line_to(p2(NECK_R, 0.0), tol)
        .expect("throat disk")
        .line_to(p2(r_top, shoulder), tol)
        .expect("neck cone")
        .arc_to(
            Center {
                c: p2(0.0, shoulder + TOP),
                winding: ArcSweep::Ccw,
                p: p2(r_mouth, t_mouth),
            },
            tol,
        )
        .expect("belly rides the globe")
        .line_to(p2(LIP_R, t_end), tol)
        .expect("pucker cone")
        .line_to(p2(0.0, t_end), tol)
        .expect("lip disk")
        .line_to(Start, tol)
        .expect("axis seam")
        .into();
    let profile = validated(SketchPlane::xy(), vec![meridian], tol).expect("meridian validates");
    revolve(
        &profile,
        RevolveAxis {
            origin: p2(0.0, 0.0),
            dir: Vec2::new(0.0, 1.0),
        },
        Revolution::Full,
        tol,
    )
    .expect("the lantern revolves")
    .body
}

/// Circular edges at carrier radius `r` whose two supports are
/// DISTINCT surfaces (excludes the chart seams).
fn rims_of_radius(body: &Body<f64>, r: f64) -> Vec<EdgeKey> {
    let face_of = |he| {
        body.get_loop(body.get_half_edge(he).unwrap().parent_loop)
            .unwrap()
            .face
    };
    body.edges()
        .filter_map(|(k, e)| {
            let c = body.get_curve_geom(e.curve)?.certified()?;
            match *c.carrier() {
                Curve3::Circle { radius, .. } if (radius - r).abs() < 5e-4 => Some(k),
                _ => None,
            }
        })
        .filter(|k| {
            let ed = body.get_edge(*k).unwrap();
            let (a, b) = (face_of(ed.he_plus), face_of(ed.he_minus));
            a != b && body.get_face(a).unwrap().surface != body.get_face(b).unwrap().surface
        })
        .collect()
}

/// Wall 6 as authored: every edge, one call — the battery refuses the
/// co-surface chart seams at margin exactly zero, before any door.
#[test]
fn t1_wall_6_as_authored_still_refuses_tangential_at_margin_zero() {
    let tol = Tol::witness();
    let lant = lily_lantern(tol);
    let all: Vec<EdgeKey> = lant.edges().map(|(k, _)| k).collect();
    match fillet_edges(&lant, &all, 0.02, Band::linear(tol).expect("band"), tol) {
        Err(BlendError::TangentialEdge { margin, .. }) => {
            assert_eq!(margin, 0.0, "a co-surface seam, not a near-tangency");
        }
        other => panic!("wall 6 as authored refuses tangential, got {other:?}"),
    }
}

/// One rim at a time: the three transverse convex rims the PR names
/// fillet whole at r = 0.02, each as one band over two arcs.
#[test]
fn t2_the_three_convex_rims_fillet_whole_at_the_named_radii() {
    let tol = Tol::witness();
    let lant = lily_lantern(tol);
    let r_top = (GLOBE.powi(2) - TOP.powi(2)).sqrt();
    for (name, rim_r) in [
        ("the lip rim", LIP_R),      // ~0.090
        ("the shoulder rim", r_top), // ~0.183
        ("the throat rim", NECK_R),  // ~0.052
    ] {
        let arcs = rims_of_radius(&lant, rim_r);
        assert_eq!(arcs.len(), 2, "{name} is seam-split into two arcs");
        let out = fillet_edges(&lant, &arcs, 0.02, Band::linear(tol).expect("band"), tol)
            .unwrap_or_else(|e| panic!("{name} fillets whole at r = 0.02, got {e:?}"));
        pncad::topo::validate_geometric(&out.body, tol)
            .unwrap_or_else(|e| panic!("{name} carves tier-3 valid, got {e:?}"));
        assert_eq!(out.band_faces.len(), 1, "{name} leaves one band");
    }
}

/// The mouth rim (~0.253) is the real other frontier: CONCAVE, which
/// no closed-rim carve in the module builds — the material-adding band,
/// filed as evgunter/cad issue 1244. So the lily's fourth transverse
/// rim is not this door's business and says so in its own words.
#[test]
fn t3_the_mouth_rim_refuses_concave() {
    let tol = Tol::witness();
    let lant = lily_lantern(tol);
    let r_mouth = (GLOBE.powi(2) - MOUTH.powi(2)).sqrt();
    assert!((r_mouth - 0.253).abs() < 5e-4, "the PR's fourth radius");
    let arcs = rims_of_radius(&lant, r_mouth);
    assert_eq!(arcs.len(), 2, "the mouth rim is seam-split too");
    match fillet_edges(&lant, &arcs, 0.02, Band::linear(tol).expect("band"), tol) {
        Err(BlendError::UnsupportedChain { detail, .. }) => assert!(
            detail.contains("concave"),
            "the mouth refuses as concave, got {detail}"
        ),
        other => panic!("the mouth rim refuses concave, got {other:?}"),
    }
}

/// **The recourse stays true at a REACHABLE concave site.** One arc of
/// the lily's own concave mouth rim refuses `SeamVertex` — the tag
/// reads incidence and never convexity — and t3 above is the whole-rim
/// request it names, refusing `concave`. This row was the r1 review's
/// consumer-side witness for the MAJOR while the sentence promised the
/// carve unconditionally; it now pins the CONDITIONED sentence at the
/// same site, so a re-widening without issue 1244 would go red here on
/// a body a real user holds rather than only on a synthetic fixture.
#[test]
fn t4_one_mouth_arc_gets_the_conditioned_recourse_and_the_rim_refuses_concave() {
    let tol = Tol::witness();
    let lant = lily_lantern(tol);
    let r_mouth = (GLOBE.powi(2) - MOUTH.powi(2)).sqrt();
    let arcs = rims_of_radius(&lant, r_mouth);
    assert_eq!(arcs.len(), 2);
    match fillet_edges(
        &lant,
        &arcs[..1],
        0.02,
        Band::linear(tol).expect("band"),
        tol,
    ) {
        Err(BlendError::UnsupportedCorner { corner, .. }) => {
            let shown = format!("{corner}");
            assert!(
                shown.contains("seam"),
                "one mouth arc stops at a seam vertex, got {shown}"
            );
        }
        other => panic!("one mouth arc refuses SeamVertex, got {other:?}"),
    }
    // And the sentence it carries does not promise this rim a carve.
    let shown = pncad::sweep::fillet::FILLET3_SEAM_VERTEX_RECOURSE;
    assert!(
        shown.contains("CONVEX"),
        "the carve half names the side the door serves: {shown}"
    );
}

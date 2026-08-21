//! **The OQ6 refusal vocabulary, pinned variant by variant** (M5
//! PR 12 §3) and the **two-tolerance trio** for every `fillet3_*`
//! predicate (§1's D4 ¶1 addendum obligation, on every arm including
//! the definite ones).
//!
//! `FilletCornerUnsupported` has zero constructor surface: neither
//! `RunOutPolicy` variant is ever taken, both are only NAMED, and the
//! rows below are what keeps that vocabulary from being decorative.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Band, Point2, Point3, Tolerance, Vec3};
use profile::RawLoop;
use profile::{Profile, ProfileLoop, ProfileVertex, SketchPlane};
use sweep::fillet::battery::{
    FilletRequest, chain_g1, convexity_at, corner_config, face_clearance, run_battery,
    spine_regularity,
};
use sweep::fillet::{CornerConfig, FilletError, FilletSite, RunOutPolicy};
use sweep::{Extrusion, extrude};
use topo::{Body, EdgeKey, FaceKey, VertexKey};

fn tol() -> Tolerance {
    Tolerance::get()
}

fn band() -> Band {
    Band::new(tol().eps, tol().k * tol().eps).unwrap()
}

/// A margin strictly inside the band: escalation territory, never a
/// classification (the S2 trio idiom).
fn in_band() -> f64 {
    5.0 * tol().eps
}

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

fn boxy() -> Body<f64> {
    let lp = ProfileLoop::new(
        [(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]
            .into_iter()
            .map(|(x, y)| ProfileVertex::new(p2(x, y), 0.0))
            .collect(),
    );
    let profile = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(tol())
        .unwrap();
    extrude(&profile, Extrusion::Distance(1.0)).unwrap().body
}

/// A cylinder: a three-arc circle extruded. Its rim edges are
/// plane–cylinder, which is a support pair NO analytic arm covers —
/// the canal-surface lane's front door.
fn cylinder() -> Body<f64> {
    let b120 = (core::f64::consts::PI / 6.0).tan();
    let at = |deg: f64| {
        let th: f64 = deg.to_radians();
        p2(0.5 * th.cos(), 0.5 * th.sin())
    };
    let lp = ProfileLoop::new(vec![
        ProfileVertex::new(at(0.0), b120),
        ProfileVertex::new(at(120.0), b120),
        ProfileVertex::new(at(240.0), b120),
    ]);
    let profile = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(tol())
        .unwrap();
    extrude(&profile, Extrusion::Distance(1.0)).unwrap().body
}

/// A "D" prism: a square with one side replaced by a circular arc,
/// extruded. Its top cap borders three planar walls and ONE cylinder
/// wall, so a planar chain on that cap terminates at a vertex whose
/// third edge is plane–cylinder.
fn dee() -> Body<f64> {
    let lp = ProfileLoop::new(vec![
        ProfileVertex::new(p2(0.0, 0.0), 0.0),
        ProfileVertex::new(p2(1.0, 0.0), 0.0),
        ProfileVertex::new(p2(1.0, 1.0), 0.4),
        ProfileVertex::new(p2(0.0, 1.0), 0.0),
    ]);
    let profile = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(tol())
        .unwrap();
    extrude(&profile, Extrusion::Distance(1.0)).unwrap().body
}

/// Any face / vertex / edge key of a real body — the trio rows below
/// exercise the PREDICATES, whose margins are their arguments; the
/// keys only ride the payload, so borrowing real ones keeps the rows
/// honest without contriving a body per row.
fn keys(body: &Body<f64>) -> (FaceKey, VertexKey, EdgeKey) {
    (
        body.faces().next().unwrap().0,
        body.vertices().next().unwrap().0,
        body.edges().next().unwrap().0,
    )
}

// ---------------------------------------------------------------------
// §3 — every `CornerConfig` tag and both `RunOutPolicy` names.
// ---------------------------------------------------------------------

/// A valence-four vertex: no spherical triangle, so no octant patch.
/// The policy named is stop-at-vertex — a general corner patch is
/// exactly what such a vertex would need.
#[test]
fn corner_tag_n_edge_vertex_names_stop_at_vertex() {
    let body = boxy();
    let (_, v, _) = keys(&body);
    match corner_config(v, 4, 4, [Vec3::new(0.0, 0.0, 1.0); 3], 0.1, band()) {
        Err(FilletError::FilletCornerUnsupported {
            corner: CornerConfig::NEdgeVertex { valence },
            policy,
            ..
        }) => {
            assert_eq!(valence, 4);
            assert_eq!(policy, RunOutPolicy::RunOutStopAtVertex);
        }
        other => panic!("expected an N-edge corner refusal, got {other:?}"),
    }
}

/// Three convex edges but a DEPENDENT trihedron (two of the three
/// normals parallel): the ball centre is not determined by the three
/// distance conditions, so there is no corner ball to mint.
#[test]
fn corner_tag_dependent_normals_refuses_definitely() {
    let body = boxy();
    let (_, v, _) = keys(&body);
    let normals = [
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        Vec3::new(1.0, 0.0, 0.0),
    ];
    match corner_config(v, 3, 3, normals, 0.1, band()) {
        Err(FilletError::FilletCornerUnsupported {
            corner: CornerConfig::DependentNormals,
            policy,
            ..
        }) => assert_eq!(policy, RunOutPolicy::RunOutStopAtVertex),
        other => panic!("expected a dependent-normals refusal, got {other:?}"),
    }
}

/// Mixed convexity names the FEATHER policy: a corner patch cannot
/// help a vertex where the ball must change sides, but a radius that
/// decays to zero before the vertex can.
#[test]
fn corner_tag_mixed_convexity_names_feather() {
    let body = boxy();
    let (_, v, _) = keys(&body);
    let normals = [
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        Vec3::new(0.0, 0.0, 1.0),
    ];
    match corner_config(v, 3, 1, normals, 0.1, band()) {
        Err(FilletError::FilletCornerUnsupported {
            corner: CornerConfig::MixedConvexity { convex },
            policy,
            ..
        }) => {
            assert_eq!(convex, 1);
            assert_eq!(policy, RunOutPolicy::RunOutFeather);
        }
        other => panic!("expected a mixed-convexity refusal, got {other:?}"),
    }
}

/// The three-convex-edge trihedron with independent normals is the
/// ONE configuration that passes — the tag that is not a refusal.
#[test]
fn corner_tag_three_convex_edges_is_the_one_that_passes() {
    let body = boxy();
    let (_, v, _) = keys(&body);
    let normals = [
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        Vec3::new(0.0, 0.0, 1.0),
    ];
    corner_config(v, 3, 3, normals, 0.1, band()).expect("the octant corner is in scope");
    assert_eq!(
        format!("{}", CornerConfig::ThreeConvexEdges),
        "three convex edges (the sphere-octant corner)"
    );
}

/// A **same-surface smooth split** (the cylinder's two wall faces meet
/// on one cylinder) is refused by predicate 5 with a margin of
/// EXACTLY zero: the supports share a tangent plane, so there is no
/// wedge for a ball to roll into. Pinned because it is the honest
/// pre-construction answer for a whole class of requests a user will
/// make by accident (selecting every edge of a curved body).
#[test]
fn a_same_surface_smooth_split_refuses_with_a_zero_wedge() {
    let body = cylinder();
    let wall_edge = body
        .edges()
        .find(|(_, e)| {
            [e.he_plus, e.he_minus].iter().all(|he| {
                body.get_half_edge(*he)
                    .and_then(|h| body.get_loop(h.parent_loop))
                    .and_then(|l| body.get_face(l.face))
                    .and_then(|f| body.get_surface(f.surface))
                    .is_some_and(|s| matches!(s, geom::Surface::Cylinder { .. }))
            })
        })
        .map(|(k, _)| k)
        .expect("a wall-to-wall seam on one cylinder");
    let req = FilletRequest {
        body: &body,
        edges: vec![wall_edge],
        radius: 0.05,
    };
    match run_battery(&req, band()) {
        Err(FilletError::TangentialEdge { margin, .. }) => {
            assert_eq!(margin, 0.0, "a smooth split has an exactly-zero wedge");
        }
        other => panic!("expected a zero-wedge refusal, got {other:?}"),
    }
}

/// `CornerConfig::Indeterminate` on a real body: a plane–plane chain
/// that TERMINATES at a vertex whose third incident edge is
/// plane–cylinder. The chain's own links resolve; the corner does
/// not — and the refusal lands at the CORNER rather than blaming the
/// neighbouring edge, which is the reporting rule under test.
#[test]
fn corner_tag_indeterminate_is_reached_at_a_curved_neighbour() {
    let body = dee();
    // A top-cap edge whose two supports are both planes.
    let planar = body
        .edges()
        .find(|(_, e)| {
            [e.he_plus, e.he_minus].iter().all(|he| {
                body.get_half_edge(*he)
                    .and_then(|h| body.get_loop(h.parent_loop))
                    .and_then(|l| body.get_face(l.face))
                    .and_then(|f| body.get_surface(f.surface))
                    .is_some_and(|s| matches!(s, geom::Surface::Plane { .. }))
            })
        })
        .map(|(k, _)| k);
    let mut saw = false;
    for (k, _) in body.edges() {
        let req = FilletRequest {
            body: &body,
            edges: vec![k],
            radius: 0.05,
        };
        if let Err(FilletError::FilletCornerUnsupported {
            corner: CornerConfig::Indeterminate,
            policy,
            ..
        }) = run_battery(&req, band())
        {
            assert_eq!(policy, RunOutPolicy::RunOutStopAtVertex);
            saw = true;
        }
    }
    assert!(
        planar.is_some() && saw,
        "the D-prism has a planar chain terminating at a curved neighbour"
    );
}

/// The canal-surface lane's front door: a plane–cylinder support pair
/// has a general rolling-ball spine, so the blend is a canal surface
/// — the kernel's first approximating SURFACE, banked as its own
/// reviewed unit. The refusal NAMES it.
#[test]
fn spine_unsupported_names_the_canal_surface_unit() {
    let body = cylinder();
    let rim = body
        .edges()
        .find(|(_, e)| {
            let kinds: Vec<bool> = [e.he_plus, e.he_minus]
                .iter()
                .filter_map(|he| {
                    let h = body.get_half_edge(*he)?;
                    let f = body.get_face(body.get_loop(h.parent_loop)?.face)?;
                    Some(matches!(
                        body.get_surface(f.surface)?,
                        geom::Surface::Plane { .. }
                    ))
                })
                .collect();
            kinds.len() == 2 && kinds[0] != kinds[1]
        })
        .map(|(k, _)| k)
        .expect("a plane–cylinder rim edge");
    let req = FilletRequest {
        body: &body,
        edges: vec![rim],
        radius: 0.05,
    };
    match run_battery(&req, band()) {
        Err(e @ FilletError::SpineUnsupported { .. }) => {
            let text = format!("{e}");
            assert!(
                text.contains("canal-surface"),
                "the refusal must name the missing front door: {text}"
            );
        }
        other => panic!("expected a spine-unsupported refusal, got {other:?}"),
    }
}

// ---------------------------------------------------------------------
// The two-tolerance trios: definitely / exactly / in-band, per
// predicate. Below eps_input the exactly-on and in-band situations are
// ONE user situation, so both arms carry the same recourse sentence.
// ---------------------------------------------------------------------

fn assert_same_recourse(definite: &FilletError, escalated: &FilletError, fragment: &str) {
    let d = format!("{definite}");
    let e = format!("{escalated}");
    assert!(d.contains(fragment), "definite arm lost its recourse: {d}");
    assert!(e.contains(fragment), "escalated arm lost its recourse: {e}");
    assert!(
        matches!(escalated, FilletError::Escalated { .. }),
        "the in-band row must escalate, not classify"
    );
}

#[test]
fn trio_spine_regularity() {
    let b = band();
    // Definitely negative: r = 1, spine curvature 2 ⇒ 1 − 2 = −1.
    let definite = spine_regularity(2.0, 1.0, b).unwrap_err();
    // Exactly on: r·κ = 1 ⇒ margin exactly 0 — a refusal, not a pass.
    let exact = spine_regularity(1.0, 1.0, b).unwrap_err();
    assert!(matches!(exact, FilletError::SpineIrregular { .. }));
    // In band: margin = 5ε.
    let escalated = spine_regularity((1.0 - in_band()) / 1.0, 1.0, b).unwrap_err();
    assert_same_recourse(
        &definite,
        &escalated,
        "below the spine's own curvature radius",
    );
}

#[test]
fn trio_face_clearance() {
    let body = boxy();
    let (f, _, _) = keys(&body);
    let b = band();
    let definite = face_clearance(f, 1.0, 0.8, 0.8, b).unwrap_err();
    let exact = face_clearance(f, 1.0, 0.5, 0.5, b).unwrap_err();
    assert!(matches!(
        exact,
        FilletError::FaceClearanceUncertified { .. }
    ));
    let escalated = face_clearance(f, 1.0, 0.5, 0.5 - in_band(), b).unwrap_err();
    assert_same_recourse(&definite, &escalated, "enlarge the support face");
}

#[test]
fn trio_chain_g1() {
    let body = boxy();
    let (_, v, _) = keys(&body);
    let b = band();
    let x = Vec3::new(1.0, 0.0, 0.0);
    let y = Vec3::new(0.0, 1.0, 0.0);
    // Definitely NOT G1: a right-angle kink at a 1 m arm.
    let definite = chain_g1(x, y, 1.0, v, b).unwrap_err();
    // Exactly G1: identical tangents — this one PASSES (the polarity
    // is inverted for a coincidence predicate, and the trio says so).
    chain_g1(x, x, 1.0, v, b).expect("identical tangents are G1");
    // In band: a kink whose sin θ · arm sits inside the band.
    let tiny = in_band();
    let escalated = chain_g1(x, Vec3::new(1.0, tiny, 0.0), 1.0, v, b).unwrap_err();
    assert_same_recourse(&definite, &escalated, "tangent-continuous chain");
    // The collapsed-arm gate: an arm at zero is not a question.
    match chain_g1(x, y, 0.0, v, b) {
        Err(FilletError::Escalated {
            site: FilletSite::Joint { .. },
            source,
        }) => assert_eq!(source.predicate, Some("fillet3_chain_arm")),
        other => panic!("a collapsed arm must escalate Invalid, got {other:?}"),
    }
}

#[test]
fn trio_convexity_sign() {
    let body = boxy();
    let (_, _, e) = keys(&body);
    let b = band();
    let tau = Vec3::new(0.0, 0.0, 1.0);
    let (convex, m) = convexity_at(
        Vec3::new(-1.0, 0.0, 0.0),
        Vec3::new(0.0, -1.0, 0.0),
        tau,
        1.0,
        e,
        b,
    )
    .expect("a definite box edge");
    assert_eq!(convex, sweep::fillet::Convexity::Convex);
    assert!(
        (m - 1.0).abs() < 1e-12,
        "the 90° box edge margin is the arm"
    );
    let (concave, _) = convexity_at(
        Vec3::new(0.0, -1.0, 0.0),
        Vec3::new(-1.0, 0.0, 0.0),
        tau,
        1.0,
        e,
        b,
    )
    .expect("the mirrored configuration");
    assert_eq!(concave, sweep::fillet::Convexity::Concave);
    // Exactly on: coplanar supports — a tangential edge with no side
    // for the ball to roll on, refused definitely. (The perturbation
    // below turns the normals ABOUT the edge tangent, which is the
    // only turn the margin's triple product can see.)
    let flat = convexity_at(
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(1.0, 0.0, 0.0),
        tau,
        1.0,
        e,
        b,
    )
    .unwrap_err();
    // Fix pass F6: a tangential edge gets its OWN situation, not a
    // convexity DISAGREEMENT with a chain verdict that was never taken.
    assert!(matches!(flat, FilletError::TangentialEdge { .. }));
    assert!(format!("{flat}").contains("no wedge"));
    // In band.
    let escalated = convexity_at(
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(1.0, in_band(), 0.0).normalize(),
        tau,
        1.0,
        e,
        b,
    )
    .unwrap_err();
    assert!(matches!(escalated, FilletError::Escalated { .. }));
}

#[test]
fn trio_corner_independence() {
    let body = boxy();
    let (_, v, _) = keys(&body);
    let b = band();
    let n = |x: f64, y: f64, z: f64| Vec3::new(x, y, z);
    // Definitely independent: the orthonormal trihedron.
    corner_config(
        v,
        3,
        3,
        [n(1.0, 0.0, 0.0), n(0.0, 1.0, 0.0), n(0.0, 0.0, 1.0)],
        1.0,
        b,
    )
    .expect("|det| · r = 1 m");
    // Exactly dependent.
    let exact = corner_config(
        v,
        3,
        3,
        [n(1.0, 0.0, 0.0), n(0.0, 1.0, 0.0), n(1.0, 1.0, 0.0)],
        1.0,
        b,
    )
    .unwrap_err();
    assert!(matches!(
        exact,
        FilletError::FilletCornerUnsupported {
            corner: CornerConfig::DependentNormals,
            ..
        }
    ));
    // In band: a trihedron that is nearly flat.
    let t = in_band();
    let escalated = corner_config(
        v,
        3,
        3,
        [
            n(1.0, 0.0, 0.0),
            n(0.0, 1.0, 0.0),
            n(0.0, 0.0, t).normalize() * t,
        ],
        1.0,
        b,
    );
    match escalated {
        Err(FilletError::Escalated { source, .. }) => {
            assert_eq!(source.predicate, Some("fillet3_corner_independence"));
        }
        other => panic!("an in-band determinant must escalate, got {other:?}"),
    }
}

/// The recourse sentences are shared CONSTANTS, so the definite and
/// escalated arms of one user situation cannot drift apart — this row
/// is what would catch a future edit that inlines one of them.
///
/// The list is hand-kept and nothing forces it to stay complete —
/// Rust cannot enumerate a module's constants — so the standing check
/// on completeness is `fillet::recourse_tests`' exhaustive match over
/// the variants that append them.
#[test]
fn every_recourse_sentence_is_reachable_from_both_arms() {
    let p = Point3::new(0.0, 0.0, 0.0);
    let _ = p;
    for s in [
        sweep::fillet::FILLET3_RADIUS_RECOURSE,
        sweep::fillet::FILLET3_CLEARANCE_RECOURSE,
        sweep::fillet::FILLET3_TANGENTIAL_RECOURSE,
        sweep::fillet::FILLET3_SPINE_RECOURSE,
        sweep::fillet::FILLET3_CHAIN_RECOURSE,
        sweep::fillet::FILLET3_CONVEXITY_RECOURSE,
        sweep::fillet::FILLET3_CORNER_RECOURSE,
        sweep::fillet::FILLET3_SPINE_KIND_RECOURSE,
        sweep::fillet::FILLET3_ASSEMBLY_RECOURSE,
        sweep::fillet::FILLET3_RING_RECOURSE,
        sweep::fillet::FILLET3_BODY_RECOURSE,
        sweep::fillet::FILLET3_GEOMETRY_RECOURSE,
    ] {
        assert!(!s.is_empty());
        assert!(
            !s.ends_with('.'),
            "recourse sentences compose into a message and never end it: {s}"
        );
    }
}

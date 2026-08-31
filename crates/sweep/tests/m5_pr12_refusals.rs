//! **The OQ6 refusal vocabulary, pinned variant by variant** (M5
//! PR 12 §3) and the **two-tolerance trio** for every `fillet3_*`
//! predicate (§1's D4 ¶1 addendum obligation, on every arm including
//! the definite ones).
//!
//! `UnsupportedCorner` has zero constructor surface: neither
//! `RunOutPolicy` variant is ever taken, both are only NAMED, and the
//! rows below are what keeps that vocabulary from being decorative.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::Tol;
use geom_core::{Band, Point2, Vec3};
use profile::RawLoop;
use profile::{Profile, ProfileLoop, ProfileVertex, SketchPlane};
use sweep::blend::battery::{
    BlendRequest, chain_g1, convexity_at, corner_config, face_clearance, run_battery,
    spine_regularity,
};
use sweep::blend::{BlendError, BlendKind, BlendSite, CornerConfig, RunOutPolicy};
use sweep::{Extrusion, extrude};
use topo::{Body, EdgeKey, FaceKey, VertexKey};

fn tol() -> Tol {
    Tol::witness()
}

fn band() -> Band {
    Band::new(tol().eps(), tol().k() * tol().eps()).unwrap()
}

/// A margin strictly inside the band: escalation territory, never a
/// classification (the S2 trio idiom).
fn in_band() -> f64 {
    5.0 * tol().eps()
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
    extrude(&profile, Extrusion::Distance(1.0), Tol::witness())
        .unwrap()
        .body
}

/// A **toroidal spool**: an annular meridian whose outer wall is an
/// off-axis 60° ARC, revolved about the sketch y-axis. That wall is
/// a TORUS, and a torus support is outside every analytic arm's table —
/// the canal-surface lane's front door, where the rolling ball's spine
/// is neither a line nor a circle.
fn spool(rev: sweep::Revolution<f64>) -> Body<f64> {
    // A 60° arc about (1.5, 0) of radius 0.5, so it meets the base at a
    // square corner and the top at a 30° one — neither joint tangent,
    // which is what keeps the profile's own validator out of the way.
    let bulge = (core::f64::consts::FRAC_PI_6 / 2.0).tan();
    let (ex, ey) = (1.75, 0.25 * 3.0f64.sqrt());
    sweep::test_support::revolved_about_y(
        vec![
            ProfileVertex::new(p2(0.5, 0.0), 0.0),
            ProfileVertex::new(p2(2.0, 0.0), bulge),
            ProfileVertex::new(p2(ex, ey), 0.0),
            ProfileVertex::new(p2(0.5, ey), 0.0),
        ],
        rev,
        tol(),
    )
}

/// A cylinder: a three-arc circle extruded.
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
    extrude(&profile, Extrusion::Distance(1.0), Tol::witness())
        .unwrap()
        .body
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
// §3 — every `CornerConfig` tag `corner_config` itself can reach, and
// both `RunOutPolicy` names. The one tag that is NOT reachable here is
// `SeamVertex`: it is recognized from the vertex's own structure before
// any valence is read as a corner configuration, so it is pinned
// through the front door instead (`verbs_arms3`).
// ---------------------------------------------------------------------

/// A valence-four vertex: no spherical triangle, so no octant patch.
/// The policy named is stop-at-vertex — a general corner patch is
/// exactly what such a vertex would need.
#[test]
fn corner_tag_n_edge_vertex_names_stop_at_vertex() {
    let body = boxy();
    let (_, v, _) = keys(&body);
    match corner_config(
        v,
        4,
        4,
        [Vec3::new(0.0, 0.0, 1.0); 3],
        0.1,
        BlendKind::Fillet,
        band(),
    ) {
        Err(BlendError::UnsupportedCorner {
            corner: CornerConfig::NEdgeVertex { valence },
            policy,
            ..
        }) => {
            assert_eq!(valence, 4);
            assert_eq!(policy, Some(RunOutPolicy::RunOutStopAtVertex));
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
    match corner_config(v, 3, 3, normals, 0.1, BlendKind::Fillet, band()) {
        Err(BlendError::UnsupportedCorner {
            corner: CornerConfig::DependentNormals,
            policy,
            ..
        }) => assert_eq!(policy, Some(RunOutPolicy::RunOutStopAtVertex)),
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
    match corner_config(v, 3, 1, normals, 0.1, BlendKind::Fillet, band()) {
        Err(BlendError::UnsupportedCorner {
            corner: CornerConfig::MixedConvexity { convex },
            policy,
            ..
        }) => {
            assert_eq!(convex, 1);
            assert_eq!(policy, Some(RunOutPolicy::RunOutFeather));
        }
        other => panic!("expected a mixed-convexity refusal, got {other:?}"),
    }
}

/// **The uniform CONCAVE trihedron is where the two verbs' corner
/// doors part.** One configuration, one set of normals, one call each:
/// the chamfer's flat patch carves it and the classifier passes, while
/// the fillet's octant is derived convex-only and it refuses with the
/// tag that says so — never as a "mixed" corner, which it is not.
#[test]
fn corner_tag_three_concave_edges_splits_the_two_verbs() {
    let body = boxy();
    let (_, v, _) = keys(&body);
    let normals = [
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        Vec3::new(0.0, 0.0, 1.0),
    ];
    corner_config(v, 3, 0, normals, 0.1, BlendKind::Chamfer, band())
        .expect("the chamfer's flat patch carves a concave trihedron");
    match corner_config(v, 3, 0, normals, 0.1, BlendKind::Fillet, band()) {
        Err(BlendError::UnsupportedCorner {
            corner: CornerConfig::ThreeConcaveEdges,
            policy,
            ..
        }) => assert_eq!(policy, Some(RunOutPolicy::RunOutStopAtVertex)),
        other => panic!("expected a concave-trihedron refusal, got {other:?}"),
    }
}

/// The three-convex-edge trihedron with independent normals is the
/// ONE configuration that passes for BOTH verbs — the convex tag that
/// is not a refusal.
#[test]
fn corner_tag_three_convex_edges_is_the_one_that_passes() {
    let body = boxy();
    let (_, v, _) = keys(&body);
    let normals = [
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        Vec3::new(0.0, 0.0, 1.0),
    ];
    corner_config(v, 3, 3, normals, 0.1, BlendKind::Fillet, band())
        .expect("the octant corner is in scope");
    assert_eq!(
        format!("{}", CornerConfig::ThreeConvexEdges),
        "three convex edges (the built corner configuration)"
    );
}

/// A **same-surface smooth split** (the cylinder's two wall faces meet
/// on one cylinder) is refused by predicate 5 with a margin of
/// EXACTLY zero. Here the supports really do share a tangent plane —
/// both sides are the same surface by construction, so the dihedral
/// sine is structurally zero and there is no wedge for a ball to
/// roll into. Pinned because it is the honest
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
    let req = BlendRequest {
        body: &body,
        edges: vec![wall_edge],
        size: 0.05,
    };
    match run_battery(&req, band()) {
        Err(BlendError::TangentialEdge { margin, .. }) => {
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
    // A PARTIAL revolve of the spool: its sweep-end caps are planar, and
    // a planar chain on one of them terminates where the TORUS wall's
    // meridian arrives — an edge no analytic arm resolves, which makes
    // the CORNER unclassifiable rather than that edge's own refusal.
    let body = spool(sweep::Revolution::Partial(1.0));
    // A cap edge whose two supports are both planes.
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
        let req = BlendRequest {
            body: &body,
            edges: vec![k],
            size: 0.05,
        };
        if let Err(BlendError::UnsupportedCorner {
            corner: CornerConfig::Indeterminate,
            policy,
            ..
        }) = run_battery(&req, band())
        {
            assert_eq!(policy, Some(RunOutPolicy::RunOutStopAtVertex));
            saw = true;
        }
    }
    assert!(
        planar.is_some() && saw,
        "the partial spool has a planar chain terminating at a torus neighbour"
    );
}

/// The canal-surface lane's front door: a plane–TORUS support pair is
/// outside the analytic-arm table, so its blend needs the canal surface
/// — the kernel's first approximating SURFACE, banked as its own
/// reviewed unit. The refusal NAMES it.
#[test]
fn spine_unsupported_names_the_canal_surface_unit() {
    let body = spool(sweep::Revolution::Full);
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
                        geom::Surface::Torus { .. }
                    ))
                })
                .collect();
            kinds.len() == 2 && kinds[0] != kinds[1]
        })
        .map(|(k, _)| k)
        .expect("a plane–torus rim edge");
    let req = BlendRequest {
        body: &body,
        edges: vec![rim],
        size: 0.05,
    };
    match run_battery(&req, band()) {
        Err(e @ BlendError::SpineUnsupported { .. }) => {
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

fn assert_same_recourse(definite: &BlendError, escalated: &BlendError, fragment: &str) {
    let d = format!("{definite}");
    let e = format!("{escalated}");
    assert!(d.contains(fragment), "definite arm lost its recourse: {d}");
    assert!(e.contains(fragment), "escalated arm lost its recourse: {e}");
    assert!(
        matches!(escalated, BlendError::Escalated { .. }),
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
    assert!(matches!(exact, BlendError::SpineIrregular { .. }));
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
    let definite = face_clearance(f, 1.0, 0.8, 0.8, false, b).unwrap_err();
    let exact = face_clearance(f, 1.0, 0.5, 0.5, false, b).unwrap_err();
    assert!(matches!(exact, BlendError::FaceClearanceUncertified { .. }));
    let escalated = face_clearance(f, 1.0, 0.5, 0.5 - in_band(), false, b).unwrap_err();
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
        Err(BlendError::Escalated {
            site: BlendSite::Joint { .. },
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
    assert_eq!(convex, sweep::blend::Convexity::Convex);
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
    assert_eq!(concave, sweep::blend::Convexity::Concave);
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
    assert!(matches!(flat, BlendError::TangentialEdge { .. }));
    assert!(format!("{flat}").contains("no definite wedge side"));
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
    assert!(matches!(escalated, BlendError::Escalated { .. }));
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
        BlendKind::Fillet,
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
        BlendKind::Fillet,
        b,
    )
    .unwrap_err();
    assert!(matches!(
        exact,
        BlendError::UnsupportedCorner {
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
        BlendKind::Fillet,
        b,
    );
    match escalated {
        Err(BlendError::Escalated { source, .. }) => {
            assert_eq!(source.predicate, Some("fillet3_corner_independence"));
        }
        other => panic!("an in-band determinant must escalate, got {other:?}"),
    }
}

/// **Every recourse sentence composes into a message.** A recourse is
/// appended to a sentence the `Display` impl has already started, so a
/// constant that is empty or that closes with a full stop renders a
/// refusal that reads wrong wherever it appears.
///
/// That is the whole of what this row checks. It reads the constants
/// and renders no refusal, so it cannot see which variant appends
/// which sentence, nor whether the definite and escalated arms of one
/// user situation still agree; a name promising either would be a name
/// this body cannot go red for. **Coverage of the list lives in
/// `fillet::recourse_tests`'
/// `every_recourse_sentence_is_rendered_by_some_variant`**, which
/// renders one value of every `BlendError` variant and requires each
/// sentence to appear in some rendering.
///
/// The list below is hand-kept — Rust cannot enumerate a module's
/// constants — and so is the private `ALL` it mirrors, so a constant
/// added to neither is checked by nothing in either crate.
#[test]
fn every_recourse_sentence_composes_into_a_message() {
    for s in [
        sweep::blend::CHAMFER_ARM_RECOURSE,
        sweep::blend::FILLET3_RADIUS_RECOURSE,
        sweep::blend::FILLET3_CLEARANCE_RECOURSE,
        sweep::blend::FILLET3_TANGENTIAL_RECOURSE,
        sweep::blend::FILLET3_SPINE_RECOURSE,
        sweep::blend::FILLET3_CHAIN_RECOURSE,
        sweep::blend::FILLET3_CONVEXITY_RECOURSE,
        sweep::blend::FILLET3_CORNER_RECOURSE,
        sweep::blend::FILLET3_SEAM_VERTEX_RECOURSE,
        sweep::blend::FILLET3_SPINE_KIND_RECOURSE,
        sweep::blend::FILLET3_ASSEMBLY_RECOURSE,
        sweep::blend::FILLET3_RING_RECOURSE,
        sweep::blend::FILLET3_BODY_RECOURSE,
        sweep::blend::FILLET3_GEOMETRY_RECOURSE,
    ] {
        assert!(!s.is_empty(), "a recourse sentence is never empty");
        assert!(
            !s.ends_with('.'),
            "recourse sentences compose into a message and never end it: {s}"
        );
    }
}

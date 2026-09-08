//! **The OQ6 refusal vocabulary, pinned variant by variant** (M5
//! PR 12 §3) and the **two-tolerance trio** for every `fillet3_*`
//! predicate (§1's D4 ¶1 addendum obligation, on every arm including
//! the definite ones).
//!
//! `UnsupportedCorner` has zero constructor surface: neither
//! `RunOutPolicy` variant is ever taken, both are only NAMED, and the
//! rows below are what keeps that vocabulary from being decorative.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common::approx::band;
use geom_core::Tol;
use geom_core::{Point2, Sign, Vec3};
use profile::RawLoop;
use profile::{Profile, ProfileLoop, ProfileVertex, SketchPlane};
use sweep::blend::battery::{
    BlendRequest, chain_g1, convexity_at, corner_config, face_clearance, run_battery,
    spine_regularity,
};
use sweep::blend::{BlendError, BlendSite, CornerConfig, RunOutPolicy};
use sweep::{Extrusion, extrude};
use topo::{Body, EdgeKey, FaceKey, FaceSurface, VertexKey};

fn tol() -> Tol {
    Tol::witness()
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
    match corner_config(v, 4, 4, [Vec3::new(0.0, 0.0, 1.0); 3], 0.1, band()) {
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
    match corner_config(v, 3, 3, normals, 0.1, band()) {
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
    match corner_config(v, 3, 1, normals, 0.1, band()) {
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

/// **The classifier reads the CORNER, and a uniform trihedron is one
/// configuration on either side of the material.** It admits both and
/// refuses only the mixed signs, with no verb anywhere in it — which
/// is what lets one predicate serve two bands, both of which now
/// carve both uniform sides (the concave-chamfer and concave-fillet
/// suites each carve the all-concave corner through their own front
/// door).
#[test]
fn corner_config_admits_either_uniform_trihedron() {
    let body = boxy();
    let (_, v, _) = keys(&body);
    let normals = [
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        Vec3::new(0.0, 0.0, 1.0),
    ];
    for convex in [0, 3] {
        corner_config(v, 3, convex, normals, 0.1, band())
            .unwrap_or_else(|e| panic!("a uniform trihedron is a configuration, got {e:?}"));
    }
    for convex in [1, 2] {
        match corner_config(v, 3, convex, normals, 0.1, band()) {
            Err(BlendError::UnsupportedCorner {
                corner: CornerConfig::MixedConvexity { convex: got },
                ..
            }) => assert_eq!(got, convex),
            other => panic!("a mixed trihedron is out of scope for every band, got {other:?}"),
        }
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
    corner_config(v, 3, 3, normals, 0.1, band()).expect("the octant corner is in scope");
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
            assert_eq!(margin.predicate, "fillet3_convexity_sign");
            assert_eq!(margin.sign, Sign::Zero);
            assert_eq!(
                margin.value(),
                Some(0.0),
                "a smooth split has an exactly-zero wedge"
            );
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
    let body = sweep::test_support::spool(sweep::Revolution::Partial(1.0), tol());
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
    let body = sweep::test_support::spool(sweep::Revolution::Full, tol());
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
    let collapsed = chain_g1(x, y, 0.0, v, b).unwrap_err();
    match &collapsed {
        BlendError::Escalated {
            site: BlendSite::Joint { .. },
            source,
        } => assert_eq!(source.predicate, Some("fillet3_chain_arm")),
        other => panic!("a collapsed arm must escalate Invalid, got {other:?}"),
    }
    // `fillet3_chain_arm` never refuses definitely — it is the gate on
    // the junction question, so it only ever escalates — and it
    // carries the sentence its gated predicate's definite refusal
    // carries: a caller whose junction arm collapsed and a caller
    // whose junction kinked both need a chain the door can take.
    assert_same_recourse(&definite, &collapsed, "tangent-continuous chain");
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
    assert_eq!(m.predicate, "fillet3_convexity_sign");
    assert_eq!(m.sign, Sign::Positive);
    assert!(
        m.value().is_some_and(|v| (v - 1.0).abs() < 1e-12),
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
    // In band: the same site, one wedge too small to call. It carries
    // the decided-Zero arm's sentence — a chain flip is a different
    // refusal at a different site and nothing here decided one.
    let escalated = convexity_at(
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(1.0, in_band(), 0.0).normalize(),
        tau,
        1.0,
        e,
        b,
    )
    .unwrap_err();
    assert_same_recourse(&flat, &escalated, "meet at a definite angle");
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
        b,
    );
    let escalated = escalated.unwrap_err();
    match &escalated {
        BlendError::Escalated { source, .. } => {
            assert_eq!(source.predicate, Some("fillet3_corner_independence"));
        }
        other => panic!("an in-band determinant must escalate, got {other:?}"),
    }
    assert_same_recourse(&exact, &escalated, "FULLY REQUESTED trivalent vertices");
}

/// A cylinder whose top cap sits on a plane tilted off the rim
/// circle's axis, so the cap–wall pair's two stored axes part by
/// `departure` meters at the rim's own lever arm — the quantity
/// `fillet3_support_coaxiality` meters. Returns the body and its
/// whole raised rim.
///
/// **The rim is TWO semicircular arcs**, so the whole rim is a closed
/// two-link chain the battery admits and the trio's exact leg is a
/// BUILD on the same body the other two legs refuse on. A three-arc
/// rim is not: `walk_chains` lists a closed chain's junctions against
/// links that do not all touch them, so the junction check reads a
/// far-end tangent and refuses `ChainNotG1` at 120°
/// (`review_blend1_r1_probes::r1_a_three_arc_rim_refuses_chain_g1_at_a_junction_where_a_two_arc_rim_builds`).
///
/// The tilt is written through `topo`'s public face-surface door
/// because no BUILDER mints a parted curved pair: extrude derives the
/// wall's axis and the cap's normal from one sketch normal, so every
/// pair it builds is coaxial exactly and the predicate reads Zero.
/// `FaceSurface::New` mints a FRESH surface key, so the rim arcs'
/// stored intersection descriptions name the old one and tier 3
/// reports `DescriptionNotAdjacent` per arc at every departure — the
/// zero leg included, which is therefore a re-keyed body and not the
/// untouched extrusion. The body stays tier-2 closed, and the battery
/// reads each edge's own certified curve rather than the face's
/// surface key, so the departure is what the legs below meter; both
/// facts are measured in
/// `review_blend1_r1_probes::r1_tilted_cap_is_tier2_valid_and_tier3_names_the_tilt`
/// and `::r1_tilted_cap_departure_is_the_meridian_reading`.
fn tilted_rim(departure: f64) -> (Body<f64>, Vec<EdgeKey>) {
    let lp = ProfileLoop::new(vec![
        ProfileVertex::new(p2(0.5, 0.0), 1.0),
        ProfileVertex::new(p2(-0.5, 0.0), 1.0),
    ]);
    let profile = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(tol())
        .unwrap();
    let mut body = extrude(&profile, Extrusion::Distance(1.0), tol())
        .unwrap()
        .body;
    // The raised rim: every arc whose stored carrier circle is the
    // raised one. Its lever arm is that circle's own radius.
    let raised: Vec<(EdgeKey, f64)> = body
        .edges()
        .filter_map(|(k, e)| {
            let c = body.get_curve_geom(e.curve)?.certified()?;
            match c.carrier() {
                geom::Curve3::Circle { center, radius, .. } if center.z > 0.5 => Some((k, *radius)),
                _ => None,
            }
        })
        .collect();
    assert_eq!(raised.len(), 2, "the raised rim of a two-arc extrusion");
    let (rim, lever) = raised[0];
    let arcs: Vec<EdgeKey> = raised.iter().map(|(k, _)| *k).collect();
    let sides = {
        let e = body.get_edge(rim).expect("the rim resolves");
        [e.he_plus, e.he_minus]
    };
    let cap = sides
        .into_iter()
        .find_map(|he| {
            let h = body.get_half_edge(he)?;
            let f = body.get_loop(h.parent_loop)?.face;
            matches!(
                body.get_surface(body.get_face(f)?.surface)?,
                geom::Surface::Plane { .. }
            )
            .then_some(f)
        })
        .expect("the cap side of the rim");
    let geom::Surface::Plane {
        origin,
        normal,
        u_ref,
    } = *body
        .get_surface(body.get_face(cap).expect("the cap resolves").surface)
        .expect("the cap's plane")
    else {
        panic!("the cap side is a plane")
    };
    // A turn about the plane's own `u_ref` keeps the frame orthonormal
    // and right-handed, and parts the normal from the rim's axis by
    // exactly `departure` at the lever arm.
    let theta = (departure / lever).asin();
    let tilted = geom::Surface::Plane {
        origin,
        normal: normal * theta.cos() + u_ref.cross(normal) * theta.sin(),
        u_ref,
    };
    body.set_face_surface(cap, FaceSurface::New(tilted))
        .expect("a plane for a planar cap");
    (body, arcs)
}

/// The battery's verdict on the whole raised rim of [`tilted_rim`]'s
/// body at one departure.
fn tilted_rim_verdict(departure: f64) -> Result<(), BlendError> {
    let (body, arcs) = tilted_rim(departure);
    run_battery(
        &BlendRequest {
            body: &body,
            edges: arcs,
            size: 0.05,
        },
        band(),
    )
    .map(|_| ())
}

#[test]
fn trio_support_coaxiality() {
    // Exactly on: the cap's normal IS the rim circle's axis, the
    // hypothesis holds, and the whole rim BUILDS — the pair is not a
    // coaxiality question at all (the polarity a coincidence
    // predicate inverts, as `trio_chain_g1` records).
    tilted_rim_verdict(0.0).expect("an exactly coaxial rim resolves");
    // Definitely parted: a millimetre off the axis at the rim's lever.
    let definite = tilted_rim_verdict(1e-3).unwrap_err();
    assert!(
        matches!(definite, BlendError::SpineUnsupported { .. }),
        "a definitely non-coaxial pair is refused, not escalated: {definite:?}"
    );
    // In band: a departure strictly inside (ε, K·ε).
    let escalated = tilted_rim_verdict(in_band()).unwrap_err();
    match &escalated {
        BlendError::Escalated {
            site: BlendSite::Chain,
            source,
        } => assert_eq!(source.predicate, Some("fillet3_support_coaxiality")),
        other => panic!("an in-band departure must escalate at the chain, got {other:?}"),
    }
    assert_same_recourse(&definite, &escalated, "canal-surface approximating blend");
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
/// The list it reads is `sweep::blend::ALL_RECOURSES`, the one home
/// for the roster: a constant added there is checked here, and a
/// reader of this row does not have to ask whether a copy has drifted.
#[test]
fn every_recourse_sentence_composes_into_a_message() {
    for (name, s) in sweep::blend::ALL_RECOURSES {
        assert!(!s.is_empty(), "the {name} recourse sentence is never empty");
        assert!(
            !s.ends_with('.'),
            "recourse sentences compose into a message and never end it: {name}: {s}"
        );
    }
}

//! **Closed rims meter an honest lever arm** (#554's acceptance
//! rows). The battery's lever is the maximum pairwise chord over the
//! samples `{t0, (t0+t1)/2, t1}`, so a full revolve's latitude rim —
//! a CLOSED edge whose endpoint chord is structurally zero — meters
//! ~its diameter and its dihedral is decided honestly.
//!
//! The rows pin the change from both sides:
//! - the #554 pair: the same cone×cylinder corner, full and partial
//!   revolve, refuses `SpineUnsupported` on BOTH (the closed rim no
//!   longer misreports `TangentialEdge` on a transverse corner);
//! - the differential: a co-surface seam meridian — dihedral sine
//!   exactly zero — still refuses `TangentialEdge` at a margin of
//!   exactly 0.0, at a lever that is now definitely nonzero (the fix
//!   removed the false positive, not the detector);
//! - convexity classification on closed rims: a dome's equator
//!   decides Convex, a boss's root rim decides Concave, each at a
//!   lever of ~the rim diameter.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::Surface;
use geom_core::{Band, Point2, Tol, Vec2};
use profile::RawLoop;
use profile::{Profile, ProfileLoop, ProfileVertex, SketchPlane};
use sweep::fillet::battery::{FilletRequest, run_battery};
use sweep::fillet::build::fillet_edges;
use sweep::fillet::{Convexity, FilletError};
use sweep::{Revolution, RevolveAxis, revolve};
use topo::{Body, EdgeKey};

fn tol() -> Tol {
    Tol::witness()
}

fn band() -> Band {
    Band::new(tol().eps(), tol().k() * tol().eps()).unwrap()
}

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

/// Revolve a closed sketch loop about the sketch y-axis.
fn revolved(verts: Vec<ProfileVertex<f64>>, rev: Revolution<f64>) -> Body<f64> {
    let profile = Profile::new(SketchPlane::xy(), vec![ProfileLoop::new(verts)])
        .validate(tol())
        .unwrap();
    let axis = RevolveAxis {
        origin: p2(0.0, 0.0),
        dir: Vec2::new(0.0, 1.0),
    };
    revolve(&profile, axis, rev, tol()).unwrap().body
}

/// The surface kind on each side of an edge, plus whether the edge is
/// closed (start vertex == end vertex).
fn edge_sides(body: &Body<f64>, edge: EdgeKey) -> (Surface<f64>, Surface<f64>, bool) {
    let e = body.get_edge(edge).unwrap();
    let surf = |he| {
        let l = body.get_half_edge(he).unwrap().parent_loop;
        let f = body.get_loop(l).unwrap().face;
        body.get_surface(body.get_face(f).unwrap().surface)
            .unwrap()
            .clone()
    };
    let start = body.get_half_edge(e.he_plus).unwrap().start;
    let end = body.half_edge_end(e.he_plus).unwrap();
    (surf(e.he_plus), surf(e.he_minus), start == end)
}

/// The widest edge matching a two-sided support predicate and a
/// closedness requirement (widest by the lever functional, so a body
/// with two matching rims hands back the deliberate one).
fn find_edge(
    body: &Body<f64>,
    closed: bool,
    pair: impl Fn(&Surface<f64>, &Surface<f64>) -> bool,
) -> EdgeKey {
    let hit = body
        .edges()
        .map(|(k, _)| k)
        .filter(|k| {
            let (a, b, c) = edge_sides(body, *k);
            c == closed && (pair(&a, &b) || pair(&b, &a))
        })
        .max_by(|x, y| lever_of(body, *x).total_cmp(&lever_of(body, *y)));
    hit.expect("an edge matches the requested supports")
}

/// The battery's lever functional, restated here as the test's own
/// oracle: the maximum pairwise chord over `{t0, mid, t1}` of the
/// edge's carrier.
fn lever_of(body: &Body<f64>, edge: EdgeKey) -> f64 {
    let e = body.get_edge(edge).unwrap();
    let c = body.get_curve_geom(e.curve).unwrap().certified().unwrap();
    let (t0, t1) = c.params();
    let mid = (t0 + t1) / 2.0;
    let carrier = c.carrier();
    let (p0, pm, p1) = (carrier.eval(t0), carrier.eval(mid), carrier.eval(t1));
    (p1 - p0).norm().max((pm - p0).norm()).max((p1 - pm).norm())
}

/// A neck-and-flare ring: a cylinder wall meeting a cone wall at a
/// 30° dihedral on a latitude rim of radius 1. The profile stays OFF
/// the axis: an on-axis profile revolves into two half-bands whose
/// latitude rims are open semicircles, while this annular profile —
/// the same authoring shape as a shell wall — is what mints the
/// CLOSED rims these rows are about.
fn neck_flare(rev: Revolution<f64>) -> Body<f64> {
    let t30 = (30.0f64).to_radians().tan();
    revolved(
        vec![
            ProfileVertex::new(p2(0.2, 0.0), 0.0),
            ProfileVertex::new(p2(1.0, 0.0), 0.0),
            ProfileVertex::new(p2(1.0, 1.0), 0.0),
            ProfileVertex::new(p2(1.0 - t30, 2.0), 0.0),
            ProfileVertex::new(p2(0.2, 2.0), 0.0),
        ],
        rev,
    )
}

/// **The #554 pair, pinned back to back.** The same cone×cylinder
/// corner at a 30° dihedral refuses `SpineUnsupported` — the missing
/// analytic arm, named — whether the rim is CLOSED (full revolve) or
/// open (partial). Before the lever fix the closed form falsely
/// refused `TangentialEdge` at a ~0 margin: its lever was the
/// endpoint chord, structurally zero on a closed rim.
#[test]
fn full_and_partial_revolve_refuse_the_same_honest_spine_unsupported() {
    let is_pair = |a: &Surface<f64>, b: &Surface<f64>| {
        matches!(a, Surface::Cone { .. }) && matches!(b, Surface::Cylinder { .. })
    };
    let full = neck_flare(Revolution::Full);
    let rim = find_edge(&full, true, is_pair);
    assert!(
        lever_of(&full, rim) > 1.9,
        "a closed radius-1 rim meters ~its diameter"
    );
    match fillet_edges(&full, &[rim], 0.05, band(), tol()) {
        Err(FilletError::SpineUnsupported { .. }) => {}
        other => panic!("closed rim: expected the honest SpineUnsupported, got {other:?}"),
    }

    let part = neck_flare(Revolution::Partial(1.0));
    let arc = find_edge(&part, false, is_pair);
    match fillet_edges(&part, &[arc], 0.05, band(), tol()) {
        Err(FilletError::SpineUnsupported { .. }) => {}
        other => panic!("open rim: expected SpineUnsupported unchanged, got {other:?}"),
    }
}

/// **The differential row: genuine tangency is still detected.** A
/// full revolve's seam meridian is a co-surface edge — the same
/// sphere on both sides, dihedral sine exactly 0 — and it still
/// refuses `TangentialEdge` at a margin of exactly 0.0 even though
/// its lever is now definitely nonzero. The zero comes from the
/// sine, not from a collapsed arm.
#[test]
fn a_co_surface_seam_meridian_still_refuses_tangential_at_exactly_zero() {
    let ball = revolved(
        vec![
            ProfileVertex::new(p2(0.0, -1.0), 1.0),
            ProfileVertex::new(p2(0.0, 1.0), 0.0),
        ],
        Revolution::Full,
    );
    let seam = find_edge(&ball, false, |a, b| {
        matches!(a, Surface::Sphere { .. }) && matches!(b, Surface::Sphere { .. })
    });
    assert!(
        lever_of(&ball, seam) > 1.9,
        "a pole-to-pole meridian levers ~the ball's diameter"
    );
    match fillet_edges(&ball, &[seam], 0.05, band(), tol()) {
        Err(FilletError::TangentialEdge { margin, .. }) => {
            assert_eq!(margin, 0.0, "a co-surface seam's sine is structurally zero");
        }
        other => panic!("expected TangentialEdge at exactly zero, got {other:?}"),
    }
}

/// **Convex classification on a closed rim.** A dome ring — a
/// unit-sphere zone rising off a flat base annulus, bored through so
/// the profile stays off-axis — has a plane–sphere equator rim: ONE
/// closed edge of radius 1, decided Convex at a lever of ~its
/// diameter.
#[test]
fn a_dome_equator_rim_decides_convex_at_an_honest_lever() {
    // Sphere zone from the equator (1, 0) up 45° along the unit
    // circle; the top annulus and inner cylinder close the ring.
    let a45 = core::f64::consts::FRAC_1_SQRT_2;
    let bulge = (core::f64::consts::FRAC_PI_4 / 4.0).tan();
    let dome = revolved(
        vec![
            ProfileVertex::new(p2(0.5, 0.0), 0.0),
            ProfileVertex::new(p2(1.0, 0.0), bulge),
            ProfileVertex::new(p2(a45, a45), 0.0),
            ProfileVertex::new(p2(0.5, a45), 0.0),
        ],
        Revolution::Full,
    );
    let rim = find_edge(&dome, true, |a, b| {
        matches!(a, Surface::Plane { .. }) && matches!(b, Surface::Sphere { .. })
    });
    let req = FilletRequest {
        body: &dome,
        edges: vec![rim],
        radius: 0.05,
    };
    let verdict = run_battery(&req, band()).expect("a plane–sphere closed rim resolves");
    let link = verdict.chains[0].first();
    assert_eq!(link.convexity, Convexity::Convex);
    assert!(
        (link.arm_len - 2.0).abs() < 1e-9,
        "the closed radius-1 rim levers its diameter, got {}",
        link.arm_len
    );
}

/// **Concave classification on a closed rim.** A spherical boss
/// rising out of a plate's top face meets it in a crevice: the
/// plane–sphere rim is decided Concave. Bored through on-axis so the
/// profile stays annular. (The minimal concave closed-rim fixture
/// #644 notes the corpus lacks; it is a fixture here, not corner
/// work.)
#[test]
fn a_boss_root_rim_decides_concave_at_an_honest_lever() {
    // Unit sphere centred at the sketch origin; the plate's top plane
    // y = 0.5 cuts it at rim radius √3/2, and the bore x = 0.2 cuts
    // it at height √(1 − 0.04).
    let rim_r = (3.0f64).sqrt() / 2.0;
    let bore_y = (1.0f64 - 0.04).sqrt();
    let bulge = (((0.2f64).acos() - core::f64::consts::FRAC_PI_6) / 4.0).tan();
    let boss = revolved(
        vec![
            ProfileVertex::new(p2(0.2, 0.0), 0.0),
            ProfileVertex::new(p2(2.0, 0.0), 0.0),
            ProfileVertex::new(p2(2.0, 0.5), 0.0),
            ProfileVertex::new(p2(rim_r, 0.5), bulge),
            ProfileVertex::new(p2(0.2, bore_y), 0.0),
        ],
        Revolution::Full,
    );
    let rim = find_edge(&boss, true, |a, b| {
        matches!(a, Surface::Plane { .. }) && matches!(b, Surface::Sphere { .. })
    });
    let req = FilletRequest {
        body: &boss,
        edges: vec![rim],
        radius: 0.05,
    };
    let verdict = run_battery(&req, band()).expect("a plane–sphere closed rim resolves");
    let link = verdict.chains[0].first();
    assert_eq!(link.convexity, Convexity::Concave);
    assert!(
        (link.arm_len - 2.0 * rim_r).abs() < 1e-9,
        "the closed radius-(√3/2) rim levers its diameter, got {}",
        link.arm_len
    );
}

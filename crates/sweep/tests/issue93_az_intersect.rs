//! Issue #93 acceptance: the A×Z silhouette intersect (the #91 demo
//! probe). Z's lower bay crossing A's crossbar-bottom strip completes
//! an isolated 6-vertex seam hexagon as a cookie-cutter null-face pair
//! whose flanking regions are bounded ENTIRELY by seam vertices — the
//! vertex-only anchor in `resolve_roles_geometric` found nothing and
//! refused `JoinDesync` ("neither section loop's regions hold a
//! classifiable vertex"). The edge-midpoint anchor tier classifies the
//! regions from their non-seam edges' interiors; all three A variants
//! must now build and pass tiers 1/2/3′ with exact-fraction volumes.
//!
//! Coincidence decoupling (the C2 rule, audited in #91): an A-plane
//! (normal (a,b,0)) and a Z-plane (normal (0,c,d)) can only coincide as
//! y = const planes; A's {0, 1, 1.4375, 2.5} and Z's {-0.0625, 2.5625}
//! are disjoint, caps straddle by 1/16 — no flush pair anywhere.
//!
//! Volume oracles, derived independently for this test (exact Fraction
//! integration, scratch script re-run 2026-07-25; matches the #91
//! probe's precomputed values):
//!   V = ∫₀^{5/2} w_A(y)·h_Z(y) dy   (each prism is constant along its
//! own extrusion axis and each extrusion strictly covers the partner's
//! extent), with piecewise-linear cross-sections
//!   w_plain(y) = 5/4 − 13y/40 on [0,1] (two legs),  2 − 7y/10 on [1,5/2]
//!   counter    = w_plain − (3/16 − (y−23/16)/3) on [23/16, 2]
//!   stencil    = counter − 1/16 on (1, 23/16)      (the crossbar slot)
//!   h_Z(y)     = 7/8 + clamp(min(25/16, 3y/5+19/40) − max(7/16, 3y/5+1/40))
//! integrated exactly per breakpoint {0, 11/16, 1, 23/16, 29/16, 2, 5/2}:
//!   counter 880383/327680,  stencil 868511/327680,  plain 903207/327680.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Decide, Point2, Point3, Vec3};
use profile::RawLoop;
use profile::{Profile, ProfileLoop, SketchPlane, ValidatedProfile};
use sweep::{Extrusion, extrude};
use topo::{
    Body, BooleanBody, BooleanResult, mass_properties, validate, validate_closed,
    validate_pseudomanifold,
};
use geom_core::Tol;

/// 880383/327680: A with the counter as a true inner loop (genus 1).
const ORACLE_COUNTER: f64 = 880383.0 / 327680.0;
/// 868511/327680: stencil A (counter + 1/16 crossbar slot).
const ORACLE_STENCIL: f64 = 868511.0 / 327680.0;
/// 903207/327680: plain A (no counter).
const ORACLE_PLAIN: f64 = 903207.0 / 327680.0;

/// Shared A skeleton: feet at y = 0, crossbar band y ∈ [1, 1.4375],
/// flat apex at y = 2.5; outer slants (0,0)→(0.875,2.5) mirrored, leg
/// inner slants (0.625,0)→(0.8125,1) mirrored.
const A_OUTLINE: [(f64, f64); 8] = [
    (0.0, 0.0),
    (0.625, 0.0),
    (0.8125, 1.0),
    (1.1875, 1.0),
    (1.375, 0.0),
    (2.0, 0.0),
    (1.125, 2.5),
    (0.875, 2.5),
];

/// Triangular counter above the crossbar (true inner loop; its slants
/// are deliberately NOT on the leg inner-slant carriers).
const A_COUNTER: [(f64, f64); 3] = [(0.90625, 1.4375), (1.09375, 1.4375), (1.0, 2.0)];

/// Stencil A: the counter opened to the crossbar by a 1/16 slot
/// (x ∈ [0.96875, 1.03125]) — one simply-connected loop.
const A_STENCIL: [(f64, f64); 15] = [
    (0.0, 0.0),
    (0.625, 0.0),
    (0.8125, 1.0),
    (0.96875, 1.0),
    (0.96875, 1.4375),
    (0.90625, 1.4375),
    (1.0, 2.0),
    (1.09375, 1.4375),
    (1.03125, 1.4375),
    (1.03125, 1.0),
    (1.1875, 1.0),
    (1.375, 0.0),
    (2.0, 0.0),
    (1.125, 2.5),
    (0.875, 2.5),
];

fn lp<T: Decide>(poly: &[(f64, f64)]) -> ProfileLoop<T> {
    ProfileLoop::polygon(
        poly.iter()
            .map(|&(x, y)| Point2::new(T::from_f64(x), T::from_f64(y)))
            .collect::<Vec<_>>(),
    )
}

fn validated<T: Decide>(plane: SketchPlane<T>, loops: Vec<ProfileLoop<T>>) -> ValidatedProfile<T> {
    Profile::new(plane, loops)
        .validate(Tol::witness())
        .expect("profile validation")
}

/// The A prism: profile on world xy, extruded z ∈ [-1/16, 2 + 1/16]
/// (strictly covers Z's z-extent [0, 2]).
fn a_prism<T: Decide>(loops: Vec<ProfileLoop<T>>) -> Body<T> {
    let plane = SketchPlane::from_frame(
        Point3::new(T::from_f64(0.0), T::from_f64(0.0), T::from_f64(-0.0625)),
        Vec3::new(T::from_f64(1.0), T::from_f64(0.0), T::from_f64(0.0)),
        Vec3::new(T::from_f64(0.0), T::from_f64(1.0), T::from_f64(0.0)),
    );
    extrude(
        &validated(plane, loops),
        Extrusion::Distance(T::from_f64(2.125)),
        Tol::witness(),
    )
    .expect("extrude A")
    .body
}

/// The Z prism: profile in (u, v) = (y, z) — bars z ∈ [0, 0.4375] and
/// [1.5625, 2] spanning y ∈ [-0.0625, 2.5625] (strictly covers A's
/// y-extent), diagonal at slope 3/5 — extruded x ∈ [-1/16, 2 + 1/16]
/// (strictly covers A's x-extent).
fn z_prism<T: Decide>() -> Body<T> {
    let z_poly = [
        (-0.0625, 0.0),
        (2.5625, 0.0),
        (2.5625, 0.4375),
        (0.6875, 0.4375),
        (2.5625, 1.5625),
        (2.5625, 2.0),
        (-0.0625, 2.0),
        (-0.0625, 1.5625),
        (1.8125, 1.5625),
        (-0.0625, 0.4375),
    ];
    let plane = SketchPlane::from_frame(
        Point3::new(T::from_f64(-0.0625), T::from_f64(0.0), T::from_f64(0.0)),
        Vec3::new(T::from_f64(0.0), T::from_f64(1.0), T::from_f64(0.0)),
        Vec3::new(T::from_f64(0.0), T::from_f64(0.0), T::from_f64(1.0)),
    );
    extrude(
        &validated(plane, vec![lp(&z_poly)]),
        Extrusion::Distance(T::from_f64(2.125)),
        Tol::witness(),
    )
    .expect("extrude Z")
    .body
}

/// Tiers 1/2/3′ + exact-oracle volume (1e-12: the oracles are exact
/// rationals with denominator 5·2^16, not representable dyadically).
fn check_success(bb: &BooleanBody<f64>, oracle: f64, label: &str) {
    assert_eq!(validate(&bb.body), Ok(()), "{label}: tier 1");
    assert_eq!(validate_closed(&bb.body), Ok(()), "{label}: tier 2");
    assert_eq!(
        validate_pseudomanifold(&bb.body, &bb.contacts, Tol::witness()),
        Ok(()),
        "{label}: tier 3′"
    );
    let v = mass_properties(&bb.body, Tol::witness()).expect("mass properties").volume;
    assert!(
        (v - oracle).abs() < 1e-12,
        "{label}: volume {v} vs exact oracle {oracle}"
    );
}

fn intersect_success(a: &Body<f64>, oracle: f64, label: &str) -> BooleanBody<f64> {
    match topo::intersect(a, &z_prism(), Tol::witness()) {
        Ok(BooleanResult::Body(bb)) => {
            check_success(&bb, oracle, label);
            bb
        }
        Ok(BooleanResult::Empty) => panic!("{label}: nonempty overlap returned Empty"),
        Err(e) => panic!(
            "REGRESSION (issue #93 acceptance): {label} refused {e:?} — \
             the A×Z seam-region anchor gap has reopened"
        ),
    }
}

/// The originally-refusing case (issue #93's headline): counter-hole
/// A × Z, pinned as now-succeeding with the exact oracle.
#[test]
fn az_counter_hole_intersects_exact() {
    let a = a_prism(vec![lp(&A_OUTLINE), lp(&A_COUNTER)]);
    intersect_success(&a, ORACLE_COUNTER, "A with counter hole");
}

/// Stencil variant: the 20-vertex confined seam loop through the slot,
/// still 100% seam vertices pre-fix.
#[test]
fn az_stencil_intersects_exact() {
    let a = a_prism(vec![lp(&A_STENCIL)]);
    intersect_success(&a, ORACLE_STENCIL, "stencil A");
}

/// Plain variant: no counter at all — the gap was never about the
/// hole (the hexagon seam alone defeats vertex anchors).
#[test]
fn az_plain_intersects_exact() {
    let a = a_prism(vec![lp(&A_OUTLINE)]);
    intersect_success(&a, ORACLE_PLAIN, "plain A");
}

/// The COUPLED variant — Z's bars shifted flush onto A's y-extent
/// (bar ends at y = 0 and y = 2.5, coinciding with A's feet and apex
/// planes; diagonal slope re-derived, 18/29). Pin history (issue
/// #93, 2026-07-25): pre-#93 main refused through the gap arm itself
/// ("…no classifiable vertex"); with the #93 anchors alone it
/// succeeded EXACTLY (same-side tangential contact, the
/// contact-records class); after M4 PR 5 (Declare + GeomSource, N6)
/// undeclared value-equal flush planes refuse typed at the
/// coincidence door — a designed narrowing, ruled 2026-07-25. Pinned
/// LIVE: undeclared → `UndeclaredCoincidence`; DECLARED (the three
/// value-equal y-plane pairs, PR 5 vocabulary) → exact success
/// through the #93 anchor tiers, independent oracle
///   ∫ w_plain·h_Zflush dy = 2562165/950272 (same Fraction method).
#[test]
fn az_coupled_flush_refuses_undeclared_succeeds_declared() {
    let z_poly_flush = [
        (0.0, 0.0),
        (2.5, 0.0),
        (2.5, 0.4375),
        (0.6875, 0.4375),
        (2.5, 1.5625),
        (2.5, 2.0),
        (0.0, 2.0),
        (0.0, 1.5625),
        (1.8125, 1.5625),
        (0.0, 0.4375),
    ];
    let plane = SketchPlane::from_frame(
        Point3::new(0.0_f64, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        Vec3::new(0.0, 0.0, 1.0),
    );
    let z_flush = extrude(
        &validated(plane, vec![lp(&z_poly_flush)]),
        Extrusion::Distance(2.125),
        Tol::witness(),
    )
    .expect("extrude flush Z")
    .body;
    let a = a_prism(vec![lp(&A_OUTLINE)]);
    // Undeclared: the N6 coincidence door refuses typed.
    match topo::intersect(&a, &z_flush, Tol::witness()) {
        Err(topo::BooleanError::UndeclaredCoincidence { .. }) => {}
        Err(e) => panic!("undeclared coupled flush refused OFF the coincidence door: {e:?}"),
        Ok(_) => panic!(
            "undeclared coupled flush succeeded — the N6 door (M4 PR 5) \
             stopped guarding value-equal flush planes"
        ),
    }
    // Declared: pair the value-equal flush planes (both bodies' pure
    // ±y carriers at equal signed offsets — dyadic sketch data, so
    // offsets are exact bit-for-bit), then the intersect runs on the
    // #93 anchors and is exact.
    let mut decls = topo::BooleanDeclarations::none();
    let y_planes = |body: &Body<f64>| -> Vec<(topo::FaceKey, f64)> {
        body.faces()
            .filter_map(|(k, f)| match body.get_surface(f.surface) {
                Some(&geom::Surface::Plane { origin, normal, .. })
                    if normal.x == 0.0 && normal.z == 0.0 =>
                {
                    Some((k, origin.y))
                }
                _ => None,
            })
            .collect()
    };
    for &(fa, da) in &y_planes(&a) {
        for &(fb, db) in &y_planes(&z_flush) {
            if da == db {
                decls
                    .coincident_faces
                    .push(topo::FacePairDeclaration::rest(fa, fb));
            }
        }
    }
    assert_eq!(
        decls.coincident_faces.len(),
        6,
        "A's two feet × Z's two y=0 bar ends + A's apex × Z's two y=2.5 bar ends"
    );
    match topo::intersect_with(&a, &z_flush, &decls, Tol::witness()) {
        Ok(BooleanResult::Body(bb)) => {
            check_success(&bb, 2_562_165.0 / 950_272.0, "declared coupled-flush A×Z");
        }
        Ok(BooleanResult::Empty) => panic!("declared coupled flush returned Empty"),
        Err(e) => panic!("declared coupled flush refused: {e:?}"),
    }
}

/// Interval lane: conservatism acceptable, wrongness never (the
/// demo_tripwires pattern). A success must pass tier 2 and enclose
/// the exact oracle.
#[cfg(feature = "interval")]
#[test]
fn az_plain_interval_refuses_or_encloses() {
    use geom_core::{Bounds, Interval};
    let a = a_prism::<Interval>(vec![lp(&A_OUTLINE)]);
    match topo::intersect(&a, &z_prism::<Interval>()) {
        Err(_) => {} // conservative refusal: acceptable
        Ok(BooleanResult::Body(bb)) => {
            assert_eq!(validate_closed(&bb.body), Ok(()), "interval tier 2");
            let v = mass_properties(&bb.body)
                .expect("interval mass properties")
                .volume;
            assert!(
                v.lo() <= ORACLE_PLAIN && ORACLE_PLAIN <= v.hi(),
                "interval volume enclosure [{}, {}] excludes the oracle {ORACLE_PLAIN}",
                v.lo(),
                v.hi()
            );
        }
        Ok(BooleanResult::Empty) => panic!("nonempty overlap returned Empty on Interval"),
    }
}

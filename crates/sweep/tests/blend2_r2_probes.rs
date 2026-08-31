//! **BLEND-2 (#935) — independent review probes, lane r2.** These rows
//! re-derive the PR's equality claims off the PR's own fixture radii,
//! and push the served door onto shapes the unit's suite did not touch:
//! a pair of annulus rims whose SHARED support is a PLANE CAP (not a
//! revolution wall), a four-rim request whose sharing closes a CYCLE
//! through two caps and two walls, and the mixed-arm reachability
//! measurement reproduced through the public boolean door.
//!
//! Probe rows, not unit rows: they pin what the review MEASURED, and a
//! future door that widens or narrows any of it should flip them
//! deliberately, with the same disclosure discipline the unit's own
//! flipped pins carry.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Band, Point2, Tol, Vec3};
use profile::ProfileVertex;
use sweep::Revolution;
use sweep::blend::build::{Filleted, fillet_edges};
use sweep::test_support::{revolved_about_y, rim_arcs_at};
use topo::{Body, EdgeKey, mass_properties, validate_geometric};

fn tol() -> Tol {
    Tol::witness()
}

fn band() -> Band {
    Band::new(tol().eps(), tol().k() * tol().eps()).unwrap()
}

fn v(x: f64, y: f64, bulge: f64) -> ProfileVertex<f64> {
    ProfileVertex::new(Point2::new(x, y), bulge)
}

/// The #935 zone at the issue bore — `test_support`'s fixture (homed
/// there in the fix pass; this suite's independent authoring moved
/// with it).
fn zone() -> Body<f64> {
    sweep::test_support::sphere_zone(0.6, Revolution::Full, tol())
}

/// The BLEND-1 lantern — `test_support`'s fixture.
fn lantern() -> Body<f64> {
    sweep::test_support::lantern(tol())
}

/// The zone's four closed rims as `(radius, latitude)` selectors:
/// the two sphere rims and the two bore rims.
const ZONE_SPHERE_LO: (f64, f64) = (1.936_491_673_103_708_5, -0.5); // sqrt(4-0.25)
const ZONE_SPHERE_HI: (f64, f64) = (1.732_050_807_568_877_2, 1.0); // sqrt(3)
const ZONE_BORE_LO: (f64, f64) = (0.6, -0.5);
const ZONE_BORE_HI: (f64, f64) = (0.6, 1.0);

fn one_edge_rim(body: &Body<f64>, sel: (f64, f64)) -> EdgeKey {
    let hits = rim_arcs_at(body, sel.0, sel.1);
    assert_eq!(hits.len(), 1, "one closed rim at {sel:?}");
    hits[0]
}

fn volume(body: &Body<f64>) -> f64 {
    let p = mass_properties(body, tol()).expect("mass properties");
    assert_eq!(p.volume_pad, 0.0, "closed-form faces only");
    p.volume
}

/// Fillet rims one call per selector, in order, on the running result.
fn sequential(src: &Body<f64>, order: &[(f64, f64)], r: f64) -> f64 {
    let mut body = src.clone();
    for &sel in order {
        let arcs = rim_arcs_at(&body, sel.0, sel.1);
        assert!(!arcs.is_empty(), "rim {sel:?} still selectable");
        body = fillet_edges(&body, &arcs, r, band(), tol())
            .unwrap_or_else(|e| panic!("rim {sel:?} fillets sequentially at r = {r}, got {e:?}"))
            .body;
    }
    validate_geometric(&body, tol()).unwrap_or_else(|e| panic!("sequential tier 3, got {e:?}"));
    volume(&body)
}

/// **P1 — the zone-pair equality, OFF the unit's fixture radius.** The
/// unit pins bit-equality at r = 0.08. MEASURED here: at r = 0.11 the
/// equality is still bit-level in both orders, but at r = 0.3 the
/// lo-then-hi sequential order lands ONE ULP away
/// (1.57569308007029445e1 vs …463e1) while hi-then-lo stays bit-equal —
/// the summation-order mechanism the PR discloses on the bud demo
/// reaches the kernel's own fixture at an untested radius. So the
/// bit-level claim is a per-fixture measurement, not a door property,
/// exactly as the PR body's "not universally" hedge states; this row
/// pins the measured boundary: exact at 0.11, within 2 ε_machine
/// relative at 0.3.
#[test]
fn r2_p1_zone_pair_equality_off_the_fixture_radius() {
    let body = zone();
    let (lo, hi) = (
        one_edge_rim(&body, ZONE_SPHERE_LO),
        one_edge_rim(&body, ZONE_SPHERE_HI),
    );
    for (r, exact) in [(0.11, true), (0.3, false)] {
        let one = fillet_edges(&body, &[lo, hi], r, band(), tol())
            .unwrap_or_else(|e| panic!("the pair builds at r = {r}, got {e:?}"));
        validate_geometric(&one.body, tol()).unwrap_or_else(|e| panic!("tier 3, got {e:?}"));
        let v1 = volume(&one.body);
        let v_ab = sequential(&body, &[ZONE_SPHERE_LO, ZONE_SPHERE_HI], r);
        let v_ba = sequential(&body, &[ZONE_SPHERE_HI, ZONE_SPHERE_LO], r);
        if exact {
            assert!(
                v1 == v_ab && v1 == v_ba,
                "bit-equal both orders at r = {r}: {v1:.17e} vs {v_ab:.17e} / {v_ba:.17e}"
            );
        } else {
            let ulp = 2.0 * f64::EPSILON * v1;
            assert!(
                (v1 - v_ab).abs() <= ulp && (v1 - v_ba).abs() <= ulp,
                "within a summation ulp at r = {r}: {v1:.17e} vs {v_ab:.17e} / {v_ba:.17e}"
            );
            assert!(
                v1 == v_ab || v1 == v_ba,
                "one call still lands ON one sequential order at r = {r}"
            );
        }
    }
}

/// **P2 — the lantern triple's equality holds off the fixture radius.**
#[test]
fn r2_p2_lantern_triple_equality_off_the_fixture_radius() {
    let rims = [(1.0, 0.0), (0.8, 0.6), (0.2, 1.2)];
    let body = lantern();
    let r = 0.04;
    let mut all: Vec<EdgeKey> = Vec::new();
    for sel in rims {
        all.extend(rim_arcs_at(&body, sel.0, sel.1));
    }
    let one = fillet_edges(&body, &all, r, band(), tol())
        .unwrap_or_else(|e| panic!("the triple builds at r = {r}, got {e:?}"));
    validate_geometric(&one.body, tol()).unwrap_or_else(|e| panic!("tier 3, got {e:?}"));
    let v1 = volume(&one.body);
    let v3 = sequential(&body, &rims, r);
    assert!(v1 == v3, "bit-equal at r = {r}: {v1:.17e} vs {v3:.17e}");
}

/// **P3 — two annulus rims sharing a PLANE CAP compose in one call.**
/// The unit's fixtures all share revolution walls (sphere, cone); a
/// full-revolution CAP has a radial seam meridian too, and the zone's
/// top cap is shared by the top sphere rim and the bore's top rim. The
/// refresh code is support-kind-agnostic, and MEASURED here it does
/// serve the cap-sharing pair — this row is the measurement the unit
/// did not take. The composition lands bit-equal on the bore-first
/// sequential order and one summation ulp off the sphere-first order
/// (1.59657466438555087e1 vs …051e1), the same integrator mechanism as
/// P1's off-radius point.
#[test]
fn r2_p3_two_rims_sharing_a_plane_cap_compose_in_one_call() {
    let body = zone();
    let r = 0.08;
    let (sph, bore) = (
        one_edge_rim(&body, ZONE_SPHERE_HI),
        one_edge_rim(&body, ZONE_BORE_HI),
    );
    let one = fillet_edges(&body, &[sph, bore], r, band(), tol())
        .unwrap_or_else(|e| panic!("the cap-sharing pair builds in one call, got {e:?}"));
    assert_eq!(one.band_faces.len(), 2, "one band per rim");
    validate_geometric(&one.body, tol()).unwrap_or_else(|e| panic!("tier 3, got {e:?}"));
    let v1 = volume(&one.body);
    let v_ab = sequential(&body, &[ZONE_SPHERE_HI, ZONE_BORE_HI], r);
    let v_ba = sequential(&body, &[ZONE_BORE_HI, ZONE_SPHERE_HI], r);
    let ulp = 2.0 * f64::EPSILON * v1;
    assert!(
        (v1 - v_ab).abs() <= ulp && (v1 - v_ba).abs() <= ulp,
        "within a summation ulp both orders: {v1:.17e} vs {v_ab:.17e} / {v_ba:.17e}"
    );
    assert!(
        v1 == v_ab || v1 == v_ba,
        "one call lands ON one sequential order"
    );
}

/// **P4 — a four-rim request whose sharing closes a CYCLE.** All four
/// zone rims in one call: sphere-lo shares the sphere wall with
/// sphere-hi and the bottom cap with bore-lo; sphere-hi shares the top
/// cap with bore-hi; the two bore rims share the bore cylinder. Every
/// later rim's refresh runs against a body carrying up to three earlier
/// bands, on caps and walls both.
#[test]
fn r2_p4_four_rims_in_a_sharing_cycle_compose_in_one_call() {
    let body = zone();
    let r = 0.08;
    let all: Vec<EdgeKey> = [ZONE_SPHERE_LO, ZONE_SPHERE_HI, ZONE_BORE_LO, ZONE_BORE_HI]
        .into_iter()
        .map(|sel| one_edge_rim(&body, sel))
        .collect();
    let one = fillet_edges(&body, &all, r, band(), tol())
        .unwrap_or_else(|e| panic!("the four-rim cycle builds in one call, got {e:?}"));
    assert_eq!(one.band_faces.len(), 4, "one band per rim");
    validate_geometric(&one.body, tol()).unwrap_or_else(|e| panic!("tier 3, got {e:?}"));
    let v1 = volume(&one.body);
    let v_a = sequential(
        &body,
        &[ZONE_SPHERE_LO, ZONE_SPHERE_HI, ZONE_BORE_LO, ZONE_BORE_HI],
        r,
    );
    let v_b = sequential(
        &body,
        &[ZONE_BORE_HI, ZONE_SPHERE_LO, ZONE_BORE_LO, ZONE_SPHERE_HI],
        r,
    );
    assert!(
        v1 == v_a && v1 == v_b,
        "bit-equal, two sequential orders: {v1:.17e} vs {v_a:.17e} / {v_b:.17e}"
    );
}

/// The partition identity of `blend_tworims::partition_check`, carried
/// here (a review-probe copy of that suite's private helper, declared)
/// so the CAP-seam retire path — which the unit's own partition rows
/// never reach — is held to the same records-are-a-partition contract.
fn partition_check(src: &Body<f64>, out: &Filleted<f64>) {
    let rec = out.naming.as_ref().expect("the surgery keeps its records");
    let mut minted_e: Vec<EdgeKey> = rec
        .rim_trims
        .iter()
        .map(|(e, _, _)| *e)
        .chain(rec.meridian_remnants.iter().map(|(e, _)| *e))
        .chain(rec.slits.iter().map(|(e, _)| *e))
        .collect();
    let n = minted_e.len();
    minted_e.sort_unstable();
    minted_e.dedup();
    assert_eq!(n, minted_e.len(), "a mint was recorded twice");
    let fragments: Vec<_> = rec
        .meridian_remnants
        .iter()
        .chain(rec.slits.iter())
        .collect();
    for e in &minted_e {
        match fragments.iter().find(|(k, _)| k == e) {
            Some((_, parent)) => assert!(
                e == parent || src.get_edge(*e).is_none(),
                "a split fragment carries a key that is neither its parent's nor fresh"
            ),
            None => assert!(src.get_edge(*e).is_none(), "a minted edge reused a key"),
        }
    }
    for (e, _) in out.body.edges() {
        assert!(
            minted_e.contains(&e) || src.get_edge(e).is_some(),
            "an output edge is neither minted nor a survivor"
        );
    }
    for e in &rec.dead.edges {
        assert!(
            src.get_edge(*e).is_some(),
            "a retired edge is not a source key"
        );
        assert!(out.body.get_edge(*e).is_none(), "a retired edge survived");
    }
}

/// **P3b/P4b — the naming records stay a partition on the cap-sharing
/// pair and the four-rim cycle**, where the retire/re-cover path runs
/// on a plane cap's RADIAL seam and on up to three earlier bands.
#[test]
fn r2_p34_cap_and_cycle_carves_keep_the_records_a_partition() {
    let body = zone();
    let pair: Vec<EdgeKey> = [ZONE_SPHERE_HI, ZONE_BORE_HI]
        .into_iter()
        .map(|sel| one_edge_rim(&body, sel))
        .collect();
    let out = fillet_edges(&body, &pair, 0.08, band(), tol()).expect("the cap pair builds");
    partition_check(&body, &out);

    let all: Vec<EdgeKey> = [ZONE_SPHERE_LO, ZONE_SPHERE_HI, ZONE_BORE_LO, ZONE_BORE_HI]
        .into_iter()
        .map(|sel| one_edge_rim(&body, sel))
        .collect();
    let out = fillet_edges(&body, &all, 0.08, band(), tol()).expect("the four-rim cycle builds");
    partition_check(&body, &out);
}

/// **P5 — the mixed-arm unreachability measurement, reproduced.** The
/// PR's stated fence: the only public construction of a plane face
/// carrying both a pip ring and a revolution-wall cycle is a boolean of
/// a ball against a revolve, and the operand gate refuses it. Here: a
/// die-style pip ball sunk into the zone's top cap, through the public
/// `subtract` door.
#[test]
fn r2_p5_the_mixed_fixture_still_refuses_at_the_boolean_door() {
    use topo::boolean::{BooleanError, subtract};
    let zone_body = zone();
    // A pole-touching ball of radius 0.12, revolved at the origin then
    // translated onto the cap: center (1.15, 1.07, 0), so it dips
    // 0.05 below the cap plane y = 1 — a die pip's shape.
    let rb = 0.12f64;
    let ball = revolved_about_y(
        vec![v(0.0, -rb, 1.0), v(0.0, rb, 0.0)],
        Revolution::Full,
        tol(),
    );
    let map = geom_core::Affine3::translation(Vec3::new(1.15, 1.0 + rb - 0.05, 0.0));
    let placed = topo::transform_rigid(&ball, &map, tol()).expect("a rigid translate");
    match subtract(&zone_body, &placed, tol()) {
        Err(BooleanError::CurvedPierceUnsupported { .. }) => {}
        Err(other) => panic!(
            "the pip-on-a-revolve-cap fixture refuses, but not at the curved-pierce \
             frontier the PR measured: {other:?} — re-examine the mixed-arm \
             reachability claim"
        ),
        Ok(_) => panic!(
            "the pip-on-a-revolve-cap fixture BUILT: the mixed ladder+annulus arm is \
             reachable through the public doors and its refusal fence needs a row"
        ),
    }
}

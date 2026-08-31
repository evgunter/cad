//! **Two closed rims sharing a support face, in ONE call** (#935): the
//! annulus carve refreshes each later rim's crossing SEAM KEYS against
//! the partially-carved body immediately before that rim's own phase —
//! identity only, every decision stays in the plan, resolved against
//! the source. The served sharing is any pair of ANNULUS rims — on a
//! revolution WALL (this file's rows) or on a full-revolve PLANE CAP,
//! whose radial seam is the same shape (`blend2_r2_probes`' cap-pair
//! and four-rim-cycle rows). These rows are what makes the widened
//! door's claims measured rather than hoped:
//!
//! - **The one-call result IS the sequential composition**, bit-level:
//!   the volume of the one-call body equals the sequential result's in
//!   BOTH sequential orders, exactly (`==` on `f64`, no tolerance) —
//!   on the zone's one-edge rim pair, on the lantern's seam-split rim
//!   pair over shared HALF-BAND walls, and on the lantern's three
//!   chained rims (neck+shoulder share the sphere, shoulder+lip the
//!   cone). Red if the composition drifts past the integrator's own
//!   summation ulp — a wrong carve, OR a summation-order change
//!   reaching these fixtures (measured elsewhere at other radii and on
//!   the bud; `blend2_r2_probes` rows carry the measured boundary), so
//!   a red here is a signal to re-measure, not automatically a carve
//!   bug.
//! - **Order independence is structural and stays pinned**: the plan
//!   sorts rims by first edge before carving, so the request's order
//!   cannot reach the carve — the two request orders produce identical
//!   key sets, and the sequential composition is itself order-blind
//!   (bit-equal both ways). Red if a carve-order dependence appears.
//! - **The naming records stay a partition** when one source meridian
//!   is split by TWO bands: the later band's re-split of a recorded
//!   remnant supersedes that row and re-covers both pieces, so every
//!   output entity is exactly one of {recorded mint, survivor}, every
//!   split fragment's key is its own parent's or fresh, and every
//!   retired key is a source key gone from the output. This is the
//!   contract `editor-core`'s emitter refuses to work without.
//! - **Bands that would collide on the shared wall refuse UPFRONT**,
//!   typed `FaceClearanceUncertified` out of the battery's clearance
//!   metering, never mid-carve on a stale key; a cross-chain pair's
//!   refusal names the SPLIT recourse and the pin executes it (the
//!   issue-1278 rule). The r = 0.749 one-call/sequential gap is the
//!   screen's pre-existing chord-vs-arc conservatism surfacing on a
//!   sphere wall — not a one-call metering penalty (identical refusal
//!   radius on a plane×cylinder spool, `review_blend2_r1_probes::p5`).
//!
//! The refresh itself has no row here by name: rows 1–3 die without it
//! (the later rim's plan keys are stale the moment the first band
//! splits the shared seam), which is the red-under-revert that pins it.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Band, Tol};
use sweep::Revolution;
use sweep::fillet::FilletError;
use sweep::fillet::build::{Filleted, fillet_edges};
use sweep::test_support::{lantern as test_support_lantern, rim_arcs_at, sphere_zone};
use topo::{Body, EdgeKey, mass_properties, validate_geometric};

fn tol() -> Tol {
    Tol::witness()
}

fn band() -> Band {
    Band::new(tol().eps(), tol().k() * tol().eps()).unwrap()
}

// ------------------------------------------------------------------
// Fixtures — `test_support`'s, not local copies: the #935 zone
// (one-edge rims) and the BLEND-1 lantern (seam-split rims).
// ------------------------------------------------------------------

/// The #935 zone at its issue bore. Two one-edge sphere rims sharing
/// the sphere wall; two cap rims sharing the caps with them.
fn zone() -> Body<f64> {
    sphere_zone(0.6, Revolution::Full, tol())
}

/// The BLEND-1 lantern. Its three latitude rims chain the sharing:
/// neck and shoulder share the sphere wall, shoulder and lip the cone.
fn lantern() -> Body<f64> {
    test_support_lantern(tol())
}

/// The lantern's three rims as `(radius, latitude)` selectors, in
/// bottom-up order.
const LANTERN_RIMS: [(f64, f64); 3] = [(1.0, 0.0), (0.8, 0.6), (0.2, 1.2)];

/// The zone's two sphere rims as `(radius, latitude)` selectors —
/// one selector spelling for every rim in this suite, `rim_arcs_at`'s.
const ZONE_SPHERE_LO: (f64, f64) = (1.936_491_673_103_708_5, -0.5); // sqrt(3.75)
const ZONE_SPHERE_HI: (f64, f64) = (1.732_050_807_568_877_2, 1.0); // sqrt(3)

/// The one closed rim matching a `(radius, latitude)` selector.
fn one_rim(body: &Body<f64>, sel: (f64, f64)) -> EdgeKey {
    let hits = rim_arcs_at(body, sel.0, sel.1);
    assert_eq!(hits.len(), 1, "exactly one closed rim at {sel:?}");
    hits[0]
}

fn volume(body: &Body<f64>) -> f64 {
    let p = mass_properties(body, tol()).expect("mass properties");
    assert_eq!(p.volume_pad, 0.0, "closed-form faces only");
    p.volume
}

/// Fillet the lantern's rims one call per rim, in the given selector
/// order, and return the final volume.
fn lantern_sequential(order: &[(f64, f64)], r: f64) -> f64 {
    let mut body = lantern();
    for &(rim_r, rim_y) in order {
        let arcs = rim_arcs_at(&body, rim_r, rim_y);
        assert_eq!(
            arcs.len(),
            2,
            "each lantern rim is two arcs before its carve"
        );
        body = fillet_edges(&body, &arcs, r, band(), tol())
            .unwrap_or_else(|e| {
                panic!("the ({rim_r}, {rim_y}) rim fillets sequentially, got {e:?}")
            })
            .body;
    }
    validate_geometric(&body, tol()).unwrap_or_else(|e| panic!("sequential tier 3, got {e:?}"));
    volume(&body)
}

// ------------------------------------------------------------------
// Rows.
// ------------------------------------------------------------------

/// **Request order cannot reach the carve** — the plan sorts rims by
/// first edge before any mutation, so the two spellings of the zone
/// pair produce IDENTICAL bodies: the same key sets, the same volume
/// bits.
#[test]
fn the_request_order_of_a_shared_wall_pair_is_structurally_inert() {
    let r = 0.08;
    let body = zone();
    let (lo, hi) = (
        one_rim(&body, ZONE_SPHERE_LO),
        one_rim(&body, ZONE_SPHERE_HI),
    );
    let a = fillet_edges(&body, &[lo, hi], r, band(), tol()).expect("the pair builds");
    let b = fillet_edges(&body, &[hi, lo], r, band(), tol()).expect("the reversed pair builds");
    let keys = |body: &Body<f64>| {
        let mut vs: Vec<_> = body.vertices().map(|(k, _)| k).collect();
        let mut es: Vec<_> = body.edges().map(|(k, _)| k).collect();
        let mut fs: Vec<_> = body.faces().map(|(k, _)| k).collect();
        vs.sort_unstable();
        es.sort_unstable();
        fs.sort_unstable();
        (vs, es, fs)
    };
    assert_eq!(
        keys(&a.body),
        keys(&b.body),
        "identical key sets either way"
    );
    let (va, vb) = (volume(&a.body), volume(&b.body));
    assert!(va == vb, "volumes bit-equal: {va:.17e} vs {vb:.17e}");
}

/// **A seam-split rim pair on shared HALF-BAND walls composes in one
/// call**, bit-equal to the sequential composition in both orders —
/// the multi-arc twin of the zone row, on BLEND-1's rim shape. The
/// shared wall is TWO faces of one sphere here, and each rim's two
/// crossings sit on the wall's two seam meridians, both of which the
/// earlier band splits.
#[test]
fn a_seam_split_rim_pair_on_shared_half_band_walls_composes_in_one_call() {
    let r = 0.05;
    let body = lantern();
    let mut both = rim_arcs_at(&body, LANTERN_RIMS[0].0, LANTERN_RIMS[0].1);
    both.extend(rim_arcs_at(&body, LANTERN_RIMS[1].0, LANTERN_RIMS[1].1));
    assert_eq!(both.len(), 4, "two rims, two arcs each");
    let one = fillet_edges(&body, &both, r, band(), tol())
        .unwrap_or_else(|e| panic!("neck + shoulder build in one call, got {e:?}"));
    assert_eq!(one.band_faces.len(), 2, "one band per rim");
    validate_geometric(&one.body, tol()).unwrap_or_else(|e| panic!("tier 3, got {e:?}"));
    let v1 = volume(&one.body);
    let v_ns = lantern_sequential(&[LANTERN_RIMS[0], LANTERN_RIMS[1]], r);
    let v_sn = lantern_sequential(&[LANTERN_RIMS[1], LANTERN_RIMS[0]], r);
    assert!(
        v1 == v_ns,
        "one call == neck-then-shoulder: {v1:.17e} vs {v_ns:.17e}"
    );
    assert!(
        v1 == v_sn,
        "one call == shoulder-then-neck: {v1:.17e} vs {v_sn:.17e}"
    );
}

/// **Three rims whose sharing CHAINS carve in one call** — the middle
/// rim shares a wall with each of the other two, so the refresh runs
/// against a body carrying one and then two earlier bands. Bit-equal
/// to the three sequential calls.
#[test]
fn three_chained_shared_wall_rims_carve_in_one_call() {
    let r = 0.05;
    let body = lantern();
    let mut all: Vec<EdgeKey> = Vec::new();
    for (rim_r, rim_y) in LANTERN_RIMS {
        all.extend(rim_arcs_at(&body, rim_r, rim_y));
    }
    assert_eq!(all.len(), 6, "three rims, two arcs each");
    let one = fillet_edges(&body, &all, r, band(), tol())
        .unwrap_or_else(|e| panic!("all three lantern rims build in one call, got {e:?}"));
    assert_eq!(one.band_faces.len(), 3, "one band per rim");
    validate_geometric(&one.body, tol()).unwrap_or_else(|e| panic!("tier 3, got {e:?}"));
    let v1 = volume(&one.body);
    let v3 = lantern_sequential(&LANTERN_RIMS, r);
    assert!(
        v1 == v3,
        "one call == three sequential: {v1:.17e} vs {v3:.17e}"
    );
}

/// **The naming records stay a partition when one source meridian is
/// split by TWO bands.** The earlier band records the seam's far piece
/// as its meridian remnant; the later band re-splits exactly that
/// piece, so its record must supersede the earlier row — every output
/// entity exactly one of {recorded mint, survivor}, every fragment key
/// its own parent's or fresh, every retirement a source key gone from
/// the output. `editor-core`'s emitter refuses duplicates and unnamed
/// entities, so a violation here is a downstream naming failure.
///
/// Checked on both rim shapes: the zone's one-edge pair and the
/// lantern's chained seam-split triple.
#[test]
fn a_shared_wall_carve_records_every_birth_and_every_death_once() {
    let zone_body = zone();
    let (lo, hi) = (
        one_rim(&zone_body, ZONE_SPHERE_LO),
        one_rim(&zone_body, ZONE_SPHERE_HI),
    );
    let zone_out =
        fillet_edges(&zone_body, &[lo, hi], 0.08, band(), tol()).expect("the zone pair builds");
    partition_check(&zone_body, &zone_out);

    let lantern_body = lantern();
    let mut all: Vec<EdgeKey> = Vec::new();
    for (rim_r, rim_y) in LANTERN_RIMS {
        all.extend(rim_arcs_at(&lantern_body, rim_r, rim_y));
    }
    let lantern_out =
        fillet_edges(&lantern_body, &all, 0.05, band(), tol()).expect("the lantern triple builds");
    partition_check(&lantern_body, &lantern_out);
}

/// The rim-phase half of `m6_5_fillet_naming`'s partition identity,
/// applied to a shared-wall result (no open chains here, so the blank
/// phase's channels are empty and asserted so), in BOTH directions:
/// every output entity is a mint or a survivor, and every recorded
/// mint is present in the output. What no row anywhere yet does is
/// drive these records through `editor-core`'s `emit_fillet` on an
/// annulus body — issue #1294 tracks that end-to-end drive.
fn partition_check(src: &Body<f64>, out: &Filleted<f64>) {
    let rec = out.naming.as_ref().expect("the surgery keeps its records");
    assert!(
        rec.blends.is_empty() && rec.corners.is_empty() && rec.trims.is_empty(),
        "a rim-only request fills the rim channels only"
    );

    let mut minted_f: Vec<_> = rec.bands.iter().map(|(f, _)| *f).collect();
    let mut minted_e: Vec<EdgeKey> = rec
        .rim_trims
        .iter()
        .map(|(e, _, _)| *e)
        .chain(rec.meridian_remnants.iter().map(|(e, _)| *e))
        .chain(rec.slits.iter().map(|(e, _)| *e))
        .collect();
    let mut minted_v: Vec<_> = rec
        .rim_feet
        .iter()
        .map(|(v, _)| *v)
        .chain(rec.meridian_splits.iter().map(|(v, _)| *v))
        .collect();
    fn dedup<K: Ord + Copy>(v: &mut Vec<K>) {
        let n = v.len();
        v.sort_unstable();
        v.dedup();
        assert_eq!(n, v.len(), "a mint was recorded twice");
    }
    dedup(&mut minted_f);
    dedup(&mut minted_e);
    dedup(&mut minted_v);

    // No mint is a survivor — except a split FRAGMENT, whose key may
    // be its own parent's (`split_edge` hands the parent key to one
    // child), never an unrelated survivor's.
    for f in &minted_f {
        assert!(src.get_face(*f).is_none(), "a minted face reused a key");
    }
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
    for v in &minted_v {
        assert!(src.get_vertex(*v).is_none(), "a minted vertex reused a key");
    }

    // Every recorded mint is PRESENT in the output — the direction the
    // retire-superseded-remnant machinery exists for: a stale row
    // names an entity a later split subdivided away.
    for f in &minted_f {
        assert!(
            out.body.get_face(*f).is_some(),
            "a recorded face mint is absent from the output"
        );
    }
    for e in &minted_e {
        assert!(
            out.body.get_edge(*e).is_some(),
            "a recorded edge mint is absent from the output (a superseded row survived)"
        );
    }
    for v in &minted_v {
        assert!(
            out.body.get_vertex(*v).is_some(),
            "a recorded vertex mint is absent from the output"
        );
    }

    // Every output entity is a recorded mint or a survivor; every
    // retirement is a source key that is really gone.
    for (f, _) in out.body.faces() {
        assert!(
            minted_f.contains(&f) || src.get_face(f).is_some(),
            "an output face is neither minted nor a survivor"
        );
    }
    for (e, _) in out.body.edges() {
        assert!(
            minted_e.contains(&e) || src.get_edge(e).is_some(),
            "an output edge is neither minted nor a survivor"
        );
    }
    for (v, _) in out.body.vertices() {
        assert!(
            minted_v.contains(&v) || src.get_vertex(v).is_some(),
            "an output vertex is neither minted nor a survivor"
        );
    }
    for e in &rec.dead.edges {
        assert!(
            src.get_edge(*e).is_some(),
            "a retired edge is not a source key"
        );
        assert!(out.body.get_edge(*e).is_none(), "a retired edge survived");
    }
    for v in &rec.dead.vertices {
        assert!(
            src.get_vertex(*v).is_some(),
            "a retired vertex is not a source key"
        );
        assert!(
            out.body.get_vertex(*v).is_none(),
            "a retired vertex survived"
        );
    }
}

/// **Bands that would collide on the shared wall refuse UPFRONT, the
/// refusal names the SPLIT that works, and the pin FOLLOWS it.** The
/// zone's sphere wall spans `y ∈ [−0.5, 1]`; at `r = 0.75` the two
/// bands' contact circles meet (`−0.5 + r = 1 − r`). The refusal is
/// the battery's face-clearance metering — typed, before any mutation,
/// NEVER the mid-carve stale-seam death #932 found — and because its
/// two setbacks belong to two different chains it renders the
/// SPLIT recourse (`FILLET3_CLEARANCE_SPLIT_RECOURSE`), which this row
/// then executes at `r = 0.749`: the sequential composition builds.
///
/// **What the r = 0.749 gap is, said precisely** (measured in review):
/// it is NOT a one-call metering penalty — on a plane×cylinder spool,
/// one call and the sequential composition refuse at the IDENTICAL
/// radius (margin −2.22e-16;
/// `review_blend2_r1_probes::p5`), because the screen's straight-line
/// gap equals the wall's own meridian there. The gap is the screen's
/// pre-existing DIRECTION-conservatism (chord vs arc) surfacing on a
/// SPHERE wall, where sequential calls re-meter the second band
/// against the first carve's actual trim circle. A property of the
/// fixture's geometry, not of one-call metering.
#[test]
fn colliding_bands_on_a_shared_wall_refuse_upfront() {
    let body = zone();
    let rims = [
        one_rim(&body, ZONE_SPHERE_LO),
        one_rim(&body, ZONE_SPHERE_HI),
    ];
    for r in [0.749, 0.8] {
        match fillet_edges(&body, &rims, r, band(), tol()).map_err(|r| r.error) {
            Err(
                e @ FilletError::FaceClearanceUncertified {
                    margin,
                    cross_chain,
                    ..
                },
            ) => {
                assert!(
                    margin < 0.0,
                    "the metering names a real interference at r = {r}"
                );
                assert!(
                    cross_chain,
                    "the two setbacks are two different rims' — the refusal knows"
                );
                let text = e.to_string();
                assert!(
                    text.contains(sweep::fillet::FILLET3_CLEARANCE_SPLIT_RECOURSE),
                    "the refusal names the split recourse, got: {text}"
                );
            }
            Err(other) => panic!(
                "a colliding pair refuses through the clearance metering, upfront; \
                 got {other:?} at r = {r}"
            ),
            Ok(_) => panic!("a colliding pair built at r = {r}; re-examine the metering"),
        }
    }
    // The recourse, EXECUTED (the issue-1278 rule: a named recourse is
    // pinned by following it): at r = 0.749 the split the refusal
    // names really builds.
    let r = 0.749;
    let first = fillet_edges(&body, &[rims[0]], r, band(), tol())
        .expect("the bottom rim alone builds at r = 0.749");
    fillet_edges(
        &first.body,
        &[one_rim(&first.body, ZONE_SPHERE_HI)],
        r,
        band(),
        tol(),
    )
    .expect("the split the refusal names composes at r = 0.749");
}

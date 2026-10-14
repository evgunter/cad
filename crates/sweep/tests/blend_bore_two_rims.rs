//! **Both rims of a through-bore, in ONE fillet.** A bore's two rims are
//! LADDER rims — each a ring of its own cap — that share the bore WALL
//! as their mate: the wall's seam lines run from one rim to the other,
//! so the first band splits each seam at its trimline and the second
//! band splits the surviving piece of the same seam. The rows pin what
//! that composition owes:
//!
//! - **It builds**, tier-3 valid, two bands, and removes exactly two
//!   plane–cylinder corner tori (the Pappus closed form, the rims being
//!   far enough apart that the two carves never meet) — on the extruded
//!   bore at N = 2…4 arcs per rim and on the box-minus-cylinder boolean.
//! - **The one-call result is the sequential composition**: its volume
//!   equals the two single-rim calls in sequence, in both orders.
//! - **Every birth row names a SOURCE key**: each wall seam is split
//!   by both bands and both `meridian_splits` rows name the seam the
//!   caller handed in, told apart by their band — never the fragment
//!   the first band minted (the surgery's debug postcondition panicked
//!   on exactly that) — and the naming records stay total.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point2, Tol};
use sweep::blend::build::{Filleted, fillet_edges};
use sweep::test_support::{
    assert_naming_totality, bored_block_of_arcs, circle_arcs_at_z, cube, cylinder_of_arcs_at,
    realized, wedge_fill,
};
use topo::boolean::BooleanOp;
use topo::{Body, EdgeKey, mass_properties, validate_geometric};

fn tol() -> Tol {
    Tol::witness()
}

const R: f64 = 0.5;
const RHO: f64 = 0.1;
const L: f64 = 2.0;

fn volume(body: &Body<f64>) -> f64 {
    let p = mass_properties(body, tol()).expect("mass properties");
    assert_eq!(p.volume_pad, 0.0, "closed-form faces only");
    p.volume
}

/// The convex plane–cylinder corner torus a bore rim's band removes:
/// the meridian corner `(R, 0)`, the cap running away from the axis and
/// the wall running into the material.
fn bore_corner() -> f64 {
    wedge_fill((R, 0.0), (1.0, 0.0), (0.0, -1.0), RHO)
}

/// A bore through a block, with its two rims at stations `lo` and `hi`.
struct Bore {
    what: String,
    body: Body<f64>,
    lo: f64,
    hi: f64,
}

fn bores() -> Vec<Bore> {
    let mut v: Vec<Bore> = (2..=4)
        .map(|n| Bore {
            what: format!("the {n}-arc extruded bore"),
            body: bored_block_of_arcs(n, L, 1.0, R, tol()),
            lo: 0.0,
            hi: 1.0,
        })
        .collect();
    let drill = cylinder_of_arcs_at(2, R, Point2::new(L / 2.0, L / 2.0), -0.5, L + 1.0, tol());
    v.push(Bore {
        what: "the box minus a cylinder".to_owned(),
        body: realized(BooleanOp::Subtract, &cube(L, tol()), &drill, tol()),
        lo: 0.0,
        hi: L,
    });
    v
}

fn both_rims(b: &Bore) -> Vec<EdgeKey> {
    let (lo, hi) = (
        circle_arcs_at_z(&b.body, b.lo),
        circle_arcs_at_z(&b.body, b.hi),
    );
    assert!(!lo.is_empty() && !hi.is_empty(), "{}: two rims", b.what);
    [lo, hi].concat()
}

fn one_call(b: &Bore) -> Filleted<f64> {
    fillet_edges(
        &sweep::test_support::at_rest(&b.body, tol()),
        &both_rims(b),
        RHO,
        tol(),
    )
    .unwrap_or_else(|e| panic!("{}: both rims fillet in one call, got {e:?}", b.what))
}

/// **Both rims build in one call**, valid, two bands, and the volume is
/// the source's less two corner tori.
#[test]
fn both_rims_of_a_bore_fillet_in_one_call_and_remove_two_corner_tori() {
    for b in bores() {
        let out = one_call(&b);
        assert_eq!(out.band_faces.len(), 2, "{}: two bands", b.what);
        validate_geometric(&out.body, tol())
            .unwrap_or_else(|e| panic!("{}: tier-3 valid, got {e:?}", b.what));
        let (v0, v1) = (volume(&b.body), volume(&out.body));
        let want = -2.0 * bore_corner();
        assert!(
            ((v1 - v0) - want).abs() < 1e-13,
            "{}: V₁ − V₀ = {} against two corner tori {want}",
            b.what,
            v1 - v0
        );
    }
}

/// **The one-call result is the sequential composition**, in both
/// orders.
#[test]
fn the_one_call_bore_is_the_sequential_composition_in_both_orders() {
    for b in bores() {
        let v_one = volume(&one_call(&b).body);
        let operand = sweep::test_support::at_rest(&b.body, tol());
        for (first, second) in [(b.lo, b.hi), (b.hi, b.lo)] {
            let mid = fillet_edges(&operand, &circle_arcs_at_z(&b.body, first), RHO, tol())
                .unwrap_or_else(|e| panic!("{}: the z = {first} rim alone, got {e:?}", b.what))
                .body;
            let end = fillet_edges(
                &sweep::test_support::at_rest(&mid, tol()),
                &circle_arcs_at_z(&mid, second),
                RHO,
                tol(),
            )
            .unwrap_or_else(|e| panic!("{}: then the z = {second} rim, got {e:?}", b.what))
            .body;
            let v_seq = volume(&end);
            assert!(
                (v_one - v_seq).abs() < 1e-13,
                "{}: one call {v_one} against z = {first} then z = {second} {v_seq}",
                b.what
            );
        }
    }
}

/// **Each wall seam's two splits name the source seam**, once per band,
/// and the records stay total.
#[test]
fn each_wall_seam_is_split_once_per_band_and_named_after_its_source() {
    for b in bores() {
        let requested = both_rims(&b);
        let out = one_call(&b);
        let rec = out.naming.as_ref().expect("the surgery keeps its records");
        assert_eq!(rec.bands.len(), 2, "{}: two band rows", b.what);
        let seams: Vec<EdgeKey> = {
            let mut s: Vec<EdgeKey> = rec.meridian_splits.iter().map(|(_, m, _)| *m).collect();
            s.sort_unstable();
            s.dedup();
            s
        };
        assert_eq!(
            rec.meridian_splits.len(),
            2 * seams.len(),
            "{}: every wall seam split by both bands",
            b.what
        );
        for m in &seams {
            assert!(
                b.body.get_edge(*m).is_some(),
                "{}: a split row names {m:?}, which the source does not carry",
                b.what
            );
            let mut bands: Vec<&Vec<EdgeKey>> = rec
                .meridian_splits
                .iter()
                .filter(|(_, s, _)| s == m)
                .map(|(_, _, band)| band)
                .collect();
            bands.sort();
            bands.dedup();
            assert_eq!(
                bands.len(),
                2,
                "{}: {m:?}'s two splits, one per band",
                b.what
            );
        }
        assert_naming_totality(&b.body, &out, &requested, &b.what);
    }
}

//! **The rim door at the certified scalar** (feature `interval`) — the
//! interval twin of `rim_of_rows`' order and end-to-end rows.
//!
//! What this lane is FOR: the door's numeric comparison is bit
//! equality, and at an enclosing scalar "the same stored value" means
//! BOTH bracket ends agree — two enclosures that merely overlap are
//! different circles to it. Nothing here decides and no band is read,
//! so the door's answer at `Interval` must be the `f64` lane's answer
//! exactly: the same arcs, in the same order, and the same carve.
//! The waisted revolve's coordinates are dyadic, so its stored
//! enclosures are points and the two lanes are comparing the same bits.

#![cfg(feature = "interval")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Interval, Real, Tol};
use sweep::blend::build::fillet_edges;
use sweep::test_support::{arcs_at, waisted, waisted_at};
use topo::query::rim_of;
use topo::validate_geometric;

#[test]
fn the_rim_door_answers_identically_at_the_certified_scalar_and_the_answer_carves() {
    let tol = Tol::witness();
    let source = waisted_at::<Interval>(tol);
    let plain = waisted(tol);

    for (name, r, y) in [
        ("the concave waist", 0.5, 0.5),
        ("the convex base", 1.0, 0.0),
        ("the convex top", 1.0, 1.0),
    ] {
        let seed = arcs_at(&source, r, y)[0];
        let rim = rim_of(&source, seed)
            .unwrap_or_else(|e| panic!("{name}: one rim at the certified scalar, got {e}"));
        assert_eq!(rim.len(), 2, "{name} is seam-split");
        assert_eq!(rim[0], seed, "{name}: the seed comes first");

        // The same body at `f64` mints its arenas through the same doors
        // in the same order, so the two lanes name the same keys — which
        // is the claim that the exact match read the same bits on both.
        let twin_seed = arcs_at(&plain, r, y)[0];
        assert_eq!(twin_seed, seed, "{name}: the two lanes seed on one key");
        assert_eq!(
            rim_of(&plain, twin_seed).expect("the f64 lane's rim"),
            rim,
            "{name}: the certified lane's rim is the f64 lane's rim"
        );
    }

    let seed = arcs_at(&source, 0.5, 0.5)[0];
    let rim = rim_of(&source, seed).expect("the waist rim");
    let out = fillet_edges(&source, &rim, Interval::from_f64(0.05), tol)
        .unwrap_or_else(|e| panic!("the door's answer carves at Interval, got {e:?}"));
    assert_eq!(out.band_faces.len(), 1, "one annulus band");
    validate_geometric(&out.body, tol).unwrap_or_else(|e| panic!("tier-3 valid, got {e:?}"));
}

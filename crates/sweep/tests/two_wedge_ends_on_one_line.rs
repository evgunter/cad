//! **Two wedge ends on one line refuse at tier 3′, named.** A cusp
//! edge's two sheets meet at a zero angle and a knife slit's at a full
//! turn, so the first-order reading of two coincident edges' wedges
//! (`topo`'s census, through `line_wedges`) has no interior direction
//! for either, and refuses the pair
//! `CoincidentWedgesOverlapUndecided { why: Degenerate }` naming the
//! edge it could not read. The body here is the failed re-pairing that
//! reading guards against, as the void door builds it: two lune
//! cavities inserted one after the other, whose cusps kiss on one line.
//! Each cavity is reverted alone and keeps its own cusp edge, now a
//! full-turn slit, so the two slits' material overlaps where D1 pairs
//! each sheet of one with a sheet of the other (TOPO's
//! `revert-re-pairs-the-sheets-on-a-line-where-coincident-edges-meet`).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point2, Tol};
use profile::{Open, Profile, ProfileLoop, SketchPlane, Start};
use sweep::{ExtrudeSide, Extrusion, extrude};
use topo::{
    Body, Cell, CensusContact, ContactRecords, SolidContainment, TouchUnread, ValidationError,
    VoidContainment, VoidEvidence, insert_void, validate_geometric, validate_pseudomanifold,
};

fn tol() -> Tol {
    Tol::witness()
}

/// A lune with one cusp at the origin: from `(0, 4s)` along the y-axis
/// to `(0, 2s)`, round the circle about `(0, s)` to the origin, cusp,
/// and back round the circle about `(0, 2s)`. `s = 1` opens its horn
/// along `+x` above the x-axis; `s = −1` is that turned a half turn
/// about the origin, its horn along `−x` below.
fn lune(s: f64) -> ProfileLoop<f64> {
    let t = tol();
    Open.at(Point2::new(0.0, 4.0 * s))
        .angle(-s * std::f64::consts::FRAC_PI_2, t)
        .unwrap()
        .line(2.0, t)
        .unwrap()
        .turn(std::f64::consts::FRAC_PI_2, t)
        .unwrap()
        .tangent_arc_to(Point2::new(0.0, 0.0), t)
        .unwrap()
        .cusp()
        .tangent_arc_to(Start, t)
        .unwrap()
        .into()
}

/// [`lune`] extruded over `z ∈ [0, 1]`: its cusp edge lies on the
/// z-axis.
fn lune_prism(s: f64) -> Body<f64> {
    let profile = Profile::new(SketchPlane::xy(), vec![lune(s)])
        .validate(tol())
        .expect("the lune validates");
    extrude(
        &profile,
        Extrusion::Distance {
            depth: 1.0,
            side: ExtrudeSide::Along,
        },
        tol(),
    )
    .unwrap()
    .body
}

/// The census's unrecorded touches of `body` as records.
fn recorded(body: &Body<f64>) -> ContactRecords {
    let unrecorded = validate_pseudomanifold(body, &ContactRecords::default(), tol())
        .expect_err("the cavities' kiss is unrecorded");
    topo::test_support::records(
        body,
        unrecorded.iter().filter_map(|e| match e {
            ValidationError::UnrecordedContact {
                contact: CensusContact::VertexVertex { a, b },
                ..
            } => Some((Cell::Vertex(*a), Cell::Vertex(*b))),
            ValidationError::UnrecordedContact {
                contact: CensusContact::EdgeEdgeOverlap { a, b },
                ..
            } => Some((Cell::Edge(*a), Cell::Edge(*b))),
            _ => None,
        }),
    )
}

/// A brick holding both lunes as cavities, through the void door one
/// at a time: tiers 3 and 3′ pass one cavity alone (its slit is one
/// edge on its line), and with both the line's reading refuses the pair
/// `Degenerate`, naming one of its two slit edges, and nothing else
/// refuses once the kiss is recorded.
#[test]
fn two_lune_cavities_kissing_at_their_cusps_refuse_their_wedge_ends() {
    let mut both = sweep::test_support::brick((-4.0, 4.0), (-6.0, 6.0), (-1.0, 2.0), tol());
    let (solid, _) = both.solids().next().unwrap();
    for s in [1.0, -1.0] {
        let cavity = lune_prism(s);
        let evidence = VoidEvidence {
            shells: cavity
                .shells()
                .map(|(k, _)| (k, VoidContainment::Probed(SolidContainment::In)))
                .collect(),
        };
        insert_void(&mut both, solid, cavity, &evidence).expect("the void door takes the lune");
        if s > 0.0 {
            assert_eq!(
                validate_geometric(&both, tol()),
                Ok(()),
                "one cavity: tier 3"
            );
            assert_eq!(
                validate_pseudomanifold(&both, &ContactRecords::default(), tol()),
                Ok(()),
                "one cavity: tier 3′"
            );
        }
    }
    assert_eq!(validate_geometric(&both, tol()), Ok(()), "both: tier 3");
    let errors = validate_pseudomanifold(&both, &recorded(&both), tol())
        .expect_err("the two slits on one line refuse");
    let [
        ValidationError::CoincidentWedgesOverlapUndecided {
            edges,
            unread: Some(unread),
            why: TouchUnread::Degenerate,
            ..
        },
    ] = &errors[..]
    else {
        panic!("only the line's wedge ends refuse: {errors:?}");
    };
    assert!(
        edges[0] != edges[1] && edges.contains(unread),
        "the unread edge is one of the pair: {errors:?}"
    );
}

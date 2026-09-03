//! **R1 review probes for MESH-10** (issue 1562) — rows written to
//! falsify the torus meridian fold, at the props band, on hand-built
//! loops.
//!
//! The unit's own rows pin: a meridian in pieces folding into its
//! edge; pieces from distinct edges (and from no edge) never folding;
//! a corner refusing. The rows here ask what those leave open:
//!
//! * the fold's "adjacency" is LOOP adjacency plus id plus direction —
//!   there is no test that the pieces' intervals actually meet. What
//!   does the fold answer for a chain that leaves a parameter GAP, or
//!   whose pieces overlap, or whose endpoints do not chain at all?
//! * `fold_torus_meridians` picks the chain-break start with a `find`
//!   that falls back to index 0 when EVERY loop-adjacent arc pair
//!   continues. Does the answer then depend on which rotation of the
//!   same loop it is handed?
//! * a chain's carrier frame is taken from the `t0`-end piece only.
//!   Do the other pieces' stored circles enter the answer at all?
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::shared::surf;
use crate::shared::tol::band;
use crate::shared::topo;
use crate::shared::topo::edge;
use geom::Surface;
use geom_brep::props::{CarrierId, LoopEdge, PropsError, curved_face, require_iso_rectangle};

const RR: f64 = 0.020;
const R0: f64 = 0.005;

fn torus() -> Surface<f64> {
    surf::torus(RR, R0)
}

fn trim(v: f64, u0: f64, u1: f64, a: u32, b: u32) -> LoopEdge<f64> {
    edge(topo::torus_rim_circle(RR, R0, v), u0, u1, a, b)
}
fn tmer(u: f64, v0: f64, v1: f64, a: u32, b: u32, id: Option<u64>) -> LoopEdge<f64> {
    LoopEdge {
        carrier_id: id.map(CarrierId::minted),
        ..edge(topo::torus_meridian_circle(RR, R0, u), v0, v1, a, b)
    }
}

const V0: f64 = 0.2;
const V1: f64 = 1.2;
const U0: f64 = -1.0;
const U1: f64 = 1.0;

/// The unit's control rectangle: each meridian one edge.
fn control() -> Vec<LoopEdge<f64>> {
    vec![
        trim(V0, U0, U1, 0, 1),
        tmer(U1, V0, V1, 1, 2, Some(1)),
        trim(V1, U1, U0, 2, 3),
        tmer(U0, V1, V0, 3, 0, Some(2)),
    ]
}

/// The same rectangle with the `u1` meridian stated as TWO pieces of
/// one edge whose intervals do not meet: `[V0, m0]` and `[m1, V1]`.
/// With `m0 == m1` this is the honest split the unit pins.
fn gapped(m0: f64, m1: f64) -> Vec<LoopEdge<f64>> {
    vec![
        trim(V0, U0, U1, 0, 1),
        tmer(U1, V0, m0, 1, 2, Some(1)),
        tmer(U1, m1, V1, 2, 3, Some(1)),
        trim(V1, U1, U0, 3, 4),
        tmer(U0, V1, V0, 4, 0, Some(2)),
    ]
}

fn flux_bits(loop_: &[LoopEdge<f64>]) -> Result<(u64, u64), PropsError> {
    curved_face(&torus(), loop_, 1.0, band()).map(|c| (c.flux.to_bits(), c.area.to_bits()))
}

/// **The pieces of a chain must MEET.** This row was written to show
/// the fold testing loop adjacency, an id and a direction and never
/// that the pieces' intervals meet — a gapped or overlapping chain
/// folded to `[lowest t0, highest t1]` all the same, and the row
/// asserted that. Inverted: an honest split is still the control,
/// bitwise, and every chain whose pieces leave a parameter GAP or
/// OVERLAP refuses `props_meridian_pieces_meet` at the door and at the
/// flux lane.
#[test]
fn a_gap_between_the_pieces_of_a_chain_refuses_typed() {
    let honest = flux_bits(&gapped(0.7, 0.7));
    let ctl = flux_bits(&control());
    println!("control            : {ctl:?}");
    println!("honest split @0.7  : {honest:?}");
    assert_eq!(
        honest, ctl,
        "the unit's own claim: an honest split is the control"
    );
    assert_eq!(
        require_iso_rectangle(&torus(), &gapped(0.7, 0.7), band()),
        Ok(())
    );
    let meet = PropsError::NotIsoRectangle {
        what: "props_meridian_pieces_meet",
    };
    for (name, m0, m1) in [
        ("gap [0.5, 0.9]", 0.5, 0.9),
        ("gap [0.3, 1.1]", 0.3, 1.1),
        ("overlap [0.9, 0.5]", 0.9, 0.5),
    ] {
        let got = flux_bits(&gapped(m0, m1));
        println!("{name}: {got:?}");
        assert_eq!(got, Err(meet.clone()), "{name}: the pieces do not meet");
        assert_eq!(
            require_iso_rectangle(&torus(), &gapped(m0, m1), band()),
            Err(meet.clone()),
            "{name}: refused at the door by the same name"
        );
    }
}

/// **A chain that closes on itself refuses by one name, whatever
/// rotation it arrives in.** This row was written to show the fold
/// cutting such a loop wherever index 0 happened to fall, so that the
/// verdict depended on the rotation the caller handed in. Inverted: a
/// loop that is one full minor circle in pieces, with no rim, has no
/// chain boundary, and every rotation refuses at the flux lane and at
/// the door by the same name.
#[test]
fn a_chain_that_closes_on_itself_refuses_by_one_name_at_every_rotation() {
    // One full minor circle at u = 0, in three pieces of one edge, all
    // traversed forward: a loop with no chain break anywhere.
    let full = |k: usize| -> Vec<LoopEdge<f64>> {
        let pieces = [
            tmer(0.0, 0.0, 2.0, 0, 1, Some(7)),
            tmer(0.0, 2.0, 4.0, 1, 2, Some(7)),
            tmer(0.0, 4.0, core::f64::consts::TAU, 2, 0, Some(7)),
        ];
        let mut v: Vec<_> = pieces[k..].to_vec();
        v.extend_from_slice(&pieces[..k]);
        v
    };
    let closed = PropsError::NotIsoRectangle {
        what: "torus meridian pieces close a loop with no rim",
    };
    for k in 0..3 {
        let got = curved_face(&torus(), &full(k), 1.0, band())
            .map(|c| (c.flux.to_bits(), c.area.to_bits()));
        println!("rotation {k}: {got:?}");
        assert_eq!(got, Err(closed.clone()), "rotation {k}: the flux lane");
        assert_eq!(
            require_iso_rectangle(&torus(), &full(k), band()),
            Err(closed.clone()),
            "rotation {k}: the door"
        );
    }
}

/// **Only the `t0`-end piece's stored circle reaches the answer.**
/// `fold_chain` copies `n_c` and `c_c` from the piece at the interval's
/// `t0` end and reads `t1` off the other end; the intervening pieces
/// contribute nothing but their identity. The row moves a middle
/// piece's carrier to a DIFFERENT minor circle (same id, so it still
/// folds) and measures whether any consumer notices.
#[test]
fn only_the_t0_end_pieces_carrier_reaches_the_answer() {
    let mut lying = gapped(0.7, 0.7);
    // Piece [0.7, V1] of the u1 meridian, restated on the u = 0.5
    // minor circle — a different carrier, the same lineage id.
    lying[2] = tmer(0.5, 0.7, V1, 2, 3, Some(1));
    let got = flux_bits(&lying);
    println!("middle piece on another minor circle: {got:?}");
    println!(
        "door: {:?}",
        require_iso_rectangle(&torus(), &lying, band())
    );
}

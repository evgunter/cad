//! R2 review probes for MESH-10 (issue 1562): the torus meridian fold
//! on hand-built loops, at the edges of the `LoopEdge` contract the
//! unit's rows do not reach. The fold keys on `carrier_id` equality
//! and traversal direction ONLY — there is no parameter-contiguity
//! test between loop-adjacent pieces (none is needed for pieces topo
//! stamps, whose adjacency is structural). These rows print what the
//! fold does when a hand-built loop stamps one id on pieces that are
//! not a partition, and on a loop with no rim at all.
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::print_stdout
)]

use crate::shared::point::{p3, v3};
use crate::shared::surf;
use crate::shared::tol::band;
use geom::Curve3;
use geom::Surface;
use geom_brep::props::{CarrierId, LoopEdge, PropsError, curved_face, require_iso_rectangle};

const RR: f64 = 0.020;
const R0: f64 = 0.005;
fn torus() -> Surface<f64> {
    surf::torus(RR, R0)
}
/// **Deliberately not `shared::topo::edge`.** Stamping a carrier id is
/// this suite's whole subject, and `LoopEdge::hand_built` mints none —
/// so this one builds the struct itself and takes the id as an
/// argument. The interval ordering below is the shared builder's, and
/// deliberately identical to it.
fn edge(
    carrier: Curve3<f64>,
    a: f64,
    b: f64,
    start: u32,
    end: u32,
    id: Option<u64>,
) -> LoopEdge<f64> {
    let (t0, t1, forward) = if a < b { (a, b, true) } else { (b, a, false) };
    LoopEdge {
        carrier_id: id.map(CarrierId::minted),
        carrier,
        t0,
        t1,
        forward,
        start,
        end,
    }
}
fn trim(v: f64, u0: f64, u1: f64, a: u32, b: u32) -> LoopEdge<f64> {
    edge(
        Curve3::Circle {
            center: p3(0.0, 0.0, R0 * v.sin()),
            axis: v3(0.0, 0.0, 1.0),
            radius: RR + R0 * v.cos(),
            u_ref: v3(1.0, 0.0, 0.0),
        },
        u0,
        u1,
        a,
        b,
        None,
    )
}
fn tmer(u: f64, v0: f64, v1: f64, a: u32, b: u32, id: Option<u64>) -> LoopEdge<f64> {
    edge(
        Curve3::Circle {
            center: p3(RR * u.cos(), RR * u.sin(), 0.0),
            axis: v3(u.sin(), -u.cos(), 0.0),
            radius: R0,
            u_ref: v3(u.cos(), u.sin(), 0.0),
        },
        v0,
        v1,
        a,
        b,
        id,
    )
}
const V0: f64 = 0.2;
const V1: f64 = 1.2;
const U0: f64 = -1.0;
const U1: f64 = 1.0;
fn control() -> Vec<LoopEdge<f64>> {
    vec![
        trim(V0, U0, U1, 0, 1),
        tmer(U1, V0, V1, 1, 2, Some(1)),
        trim(V1, U1, U0, 2, 3),
        tmer(U0, V1, V0, 3, 0, Some(2)),
    ]
}

/// **The contiguity test.** This row was written to show two pieces
/// stamped with one id whose intervals leave a GAP (`[V0, 0.5]` then
/// `[0.7, V1]`, tags chained as if adjacent) folding to `[V0, V1]`,
/// the door admitting and the flux lane answering the control's
/// numbers bitwise. Inverted: the pieces must meet exactly, and a gap
/// or an overlap refuses `props_meridian_pieces_meet` at the door and
/// at the flux lane.
#[test]
fn m10r2_a_gap_between_same_id_pieces_refuses_at_the_contiguity_test() {
    let s = torus();
    let ctl = curved_face(&s, &control(), 1.0, band()).unwrap();
    let gapped = vec![
        trim(V0, U0, U1, 0, 1),
        tmer(U1, V0, 0.5, 1, 2, Some(1)),
        tmer(U1, 0.7, V1, 2, 3, Some(1)),
        trim(V1, U1, U0, 3, 4),
        tmer(U0, V1, V0, 4, 0, Some(2)),
    ];
    let door = require_iso_rectangle(&s, &gapped, band());
    let flux = curved_face(&s, &gapped, 1.0, band());
    println!(
        "M10R2 gapped pieces: door {door:?}, flux {flux:?} vs control ({}, {})",
        ctl.flux, ctl.area
    );
    let meet = PropsError::NotIsoRectangle {
        what: "props_meridian_pieces_meet",
    };
    assert_eq!(door, Err(meet.clone()));
    assert_eq!(flux.map(|_| ()), Err(meet.clone()));
    // The same pieces OVERLAPPING (`[V0, 0.9]` then `[0.7, V1]`)
    // refuse the same way.
    let overlapped = vec![
        trim(V0, U0, U1, 0, 1),
        tmer(U1, V0, 0.9, 1, 2, Some(1)),
        tmer(U1, 0.7, V1, 2, 3, Some(1)),
        trim(V1, U1, U0, 3, 4),
        tmer(U0, V1, V0, 4, 0, Some(2)),
    ];
    assert_eq!(require_iso_rectangle(&s, &overlapped, band()), Err(meet));
}

/// **A loop of arcs only, one id, one direction** — the "no chain
/// boundary" arm of the fold's start search: three pieces of one minor
/// circle covering the full turn, given in an order that starts
/// mid-chain. This row was written to show the walk starting at index
/// 0 (mid-chain), the flux lane refusing `DegenerateFace` on a zero
/// span and the door admitting a loop with no rim. Inverted: a closed
/// chain with no rim has no chain boundary and refuses by one name at
/// both, whatever rotation it arrives in.
#[test]
fn m10r2_a_rimless_loop_of_one_meridians_pieces() {
    let s = torus();
    use core::f64::consts::PI;
    let pieces = vec![
        tmer(U1, 2.0, 4.0, 1, 2, Some(1)),
        tmer(U1, 4.0, 2.0 * PI, 2, 0, Some(1)),
        tmer(U1, 0.0, 2.0, 0, 1, Some(1)),
    ];
    let door = require_iso_rectangle(&s, &pieces, band());
    let flux = curved_face(&s, &pieces, 1.0, band()).map(|c| (c.flux, c.area));
    println!("M10R2 rimless pieces: door {door:?}, flux {flux:?}");
    let closed = PropsError::NotIsoRectangle {
        what: "torus meridian pieces close a loop with no rim",
    };
    assert_eq!(door, Err(closed.clone()));
    assert_eq!(flux, Err(closed));
}

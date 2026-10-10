//! **The evaluation digest** — the ONE home of the
//! feed every verb-migration suite pins its documents with
//! (`seat4_verb_lowering`, `seat7_sweep_lowering`,
//! `seat8_split_lowering`). The constants stay per suite; the feed
//! does not, because three verbatim copies that each claimed to be
//! "byte for byte the SEAT-4 feed" were held to it by nothing — an
//! edit to one would have left every suite green and the claim false.
//!
//! # What it covers, and how that set was chosen
//!
//! Not "everything observable": the channels the migrated lowerings can
//! move, enumerated off their own bodies. `wire_blend` writes exactly
//! three things — the name table the emitter returns, the body the
//! kernel verb returns, and (on the refusal path) a typed error.
//! `wire_swept` writes the same three. `wire_boolean` writes the first
//! two plus the
//! boolean VALUE's other halves — the result classification, the
//! surviving declared contacts, and the typed empty success — fed by
//! the `Boolean` arm, each with a pinned input on which it actually
//! VARIES (the contacts through `kiss_carry`, the empty token through
//! the disjoint intersect; both were measured fed-but-dead before those
//! inputs existed). `wire_split` writes the name table and TWO sides,
//! fed by the `Split` arm: each side
//! under its ROLE token — so a lowering that swapped the halves moves
//! the digest even where every arena is bit-identical — and an EMPTY
//! side as its own token, pinned live by an input that produces it.
//!
//! The refusal path is NOT covered, and that is a real hole, stated: a
//! refusal payload's spelling and the verdict logs are outside this
//! digest, so a change that only altered which `NodeErrorKind` came
//! back would pass it.
//!
//! # Why each half of the body feed is load-bearing, measured
//!
//! - **Point bits alone are not enough.** A unit cube filleted at
//!   radius `r` and a unit cube chamfered at setback `r` have the same
//!   twenty-four vertex positions to the bit, differing only in whether
//!   the faces between them are cylinders and spheres or planes. A
//!   points-only digest gave the two documents ONE identical number.
//! - **Face carriers alone are not enough either**: the edge curve
//!   carriers are fed beside them.
//!
//! Geometry enters through `Debug`, whose `f64` rendering
//! is the shortest round-tripping decimal: a bijection with the bits
//! for every finite value, `-0.0` included. Nothing rendered from a
//! classification band enters, so the constants are eps-independent.

use editor_core::{BooleanValue, Evaluation, SplitSide, ValuePayload};
use topo::Body;

/// FNV-1a 64 over a document's evaluated name tables and values — see
/// the module docs for what is fed and why.
pub fn digest(ev: &Evaluation<f64>) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    let mut feed = |bytes: &[u8]| {
        for b in bytes {
            h ^= u64::from(*b);
            h = h.wrapping_mul(0x1000_0000_01b3);
        }
    };
    for id in &ev.order {
        feed(format!("#{id:?}").as_bytes());
        let Some(value) = ev.value(*id) else { continue };
        for (name, entry) in value.name_table.iter() {
            feed(format!("{name:?}={entry:?}").as_bytes());
        }
        feed(value.payload.kind_name().as_bytes());
        // EXHAUSTIVE, no wildcard: a payload kind added to the value
        // vocabulary is a visit here that says whether a migrated
        // lowering writes it. The kinds fed nothing beyond their name
        // are named for that reason — each is one no migrated lowering
        // produces, and a wildcard would silently unfeed the next one
        // that is.
        match &value.payload {
            ValuePayload::Body(body) => feed_body(&mut feed, body),
            ValuePayload::Boolean(bv) => match bv {
                BooleanValue::Body {
                    body,
                    kind,
                    contacts,
                } => {
                    feed(format!("{kind:?}{contacts:?}").as_bytes());
                    feed_body(&mut feed, body);
                }
                BooleanValue::Empty => feed(b"empty"),
            },
            ValuePayload::Split { above, below } => {
                for (role, side) in [("above", above), ("below", below)] {
                    feed(role.as_bytes());
                    match side {
                        SplitSide::Body(body) => feed_body(&mut feed, body),
                        SplitSide::Empty => feed(b"empty-side"),
                    }
                }
            }
            ValuePayload::Datum(_)
            | ValuePayload::Profile(_)
            | ValuePayload::Instances(_)
            | ValuePayload::Mate(_)
            | ValuePayload::Gauge
            | ValuePayload::Measure { .. }
            | ValuePayload::MeasureUnavailable { .. }
            | ValuePayload::Assertion(_) => {}
        }
    }
    h
}

/// The body half of [`digest`]: points, the curve and surface arenas,
/// the topology's attachment both ways, and the entity census.
pub fn feed_body(feed: &mut impl FnMut(&[u8]), body: &Body<f64>) {
    // Points: their keys and bits.
    for (key, p) in body.points() {
        for c in p.to_array() {
            feed(&c.to_bits().to_be_bytes());
        }
        feed(format!("{key:?}").as_bytes());
    }
    // Curves and surfaces: the arenas themselves.
    for (key, curve) in body.curves() {
        feed(format!("{key:?}{curve:?}").as_bytes());
    }
    for (key, surface) in body.surfaces() {
        feed(format!("{key:?}{surface:?}").as_bytes());
    }
    // The topology's attachment to that geometry, both ways: a face's
    // carrier and an edge's curve. A re-plumbing that kept every arena
    // and re-pointed the topology at it moves these and nothing above.
    for (key, face) in body.faces() {
        let surface = body
            .get_surface(face.surface)
            .expect("a face has a carrier");
        feed(format!("{key:?}{surface:?}").as_bytes());
    }
    for (key, edge) in body.edges() {
        let curve = body
            .get_curve_geom(edge.curve)
            .expect("an edge has a curve");
        feed(format!("{key:?}{curve:?}").as_bytes());
    }
    feed(
        format!(
            "V{}E{}F{}",
            body.vertices().count(),
            body.edges().count(),
            body.faces().count()
        )
        .as_bytes(),
    );
}

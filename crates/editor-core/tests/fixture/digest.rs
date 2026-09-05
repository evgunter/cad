//! **The provenance-extended evaluation digest** — the ONE home of the
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
//! four things — the name table the emitter returns, the body the
//! kernel verb returns, the provenance stamp `stamp_minted` applies to
//! that body, and (on the refusal path) a typed error. `wire_swept`
//! writes the same four. `wire_boolean` writes the first three plus the
//! boolean VALUE's other halves — the result classification, the
//! surviving declared contacts, and the typed empty success — fed by
//! the `Boolean` arm, each with a pinned input on which it actually
//! VARIES (the contacts through `kiss_carry`, the empty token through
//! the disjoint intersect; both were measured fed-but-dead before those
//! inputs existed). `wire_split` writes the name table and TWO sides,
//! each stamped in one index space, fed by the `Split` arm: each side
//! under its ROLE token — so a lowering that swapped the halves moves
//! the digest even where every arena is bit-identical — and an EMPTY
//! side as its own token, pinned live by an input that produces it.
//!
//! The refusal path is NOT covered, and that is a real hole, stated: a
//! refusal payload's spelling and the verdict logs are outside this
//! digest, so a change that only altered which `NodeErrorKind` came
//! back would pass it.
//!
//! **What it deliberately does NOT feed: the per-field parameter
//! sources.** They are SEAT-6's channel, pinned in their own rows
//! through the kernel's evidence door; feeding them here would make the
//! sweeps' differential against their merge base impossible to state.
//!
//! # Why each half of the body feed is load-bearing, measured
//!
//! - **Point bits alone are not enough.** A unit cube filleted at
//!   radius `r` and a unit cube chamfered at setback `r` have the same
//!   twenty-four vertex positions to the bit, differing only in whether
//!   the faces between them are cylinders and spheres or planes. A
//!   points-only digest gave the two documents ONE identical number.
//! - **Carriers alone are not enough either.** With face carriers only,
//!   `stamp_minted` — the line that gives every description a blend
//!   mints its `GeomSource` — could be DELETED from `wire_blend` with
//!   the whole editor-core suite green. So the three provenance source
//!   tables are fed, and so are the edge curve carriers.
//!
//! Deleting a lowering's stamp reds its suite's rows; that is the
//! red-first evidence each suite records for its own constants.
//!
//! Geometry and provenance enter through `Debug`, whose `f64` rendering
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
            | ValuePayload::Declarations(_)
            | ValuePayload::Mate(_)
            | ValuePayload::Measure { .. }
            | ValuePayload::MeasureUnavailable { .. }
            | ValuePayload::Assertion(_) => {}
        }
    }
    h
}

/// The body half of [`digest`]: points with their provenance stamps,
/// the curve and surface arenas with theirs, the topology's attachment
/// both ways, and the entity census.
pub fn feed_body(feed: &mut impl FnMut(&[u8]), body: &Body<f64>) {
    // Points: bits, then the provenance stamp on the same key.
    for (key, p) in body.points() {
        for c in [p.x, p.y, p.z] {
            feed(&c.to_bits().to_be_bytes());
        }
        feed(format!("{key:?}<-{:?}", body.point_source(key)).as_bytes());
    }
    // Curves and surfaces: the arenas themselves plus their stamps.
    for (key, curve) in body.curves() {
        feed(format!("{key:?}{curve:?}<-{:?}", body.curve_source(key)).as_bytes());
    }
    for (key, surface) in body.surfaces() {
        feed(format!("{key:?}{surface:?}<-{:?}", body.surface_source(key)).as_bytes());
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

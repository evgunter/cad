---
id: next-payload-rung-under-the-cur3-cur4-carriages
kind: issue
title: EntityId, GeomRef and ContactFinding: the rung CUR3 and CUR4 both stopped at
status: closed
opened: 2026-09-03
closed: 2026-09-08
refs: [LIB-CUR4]
---


The rung both curation units deliberately stopped at, recorded so the
stop is a decision with a home rather than an omission.

CUR3 carried `DanglingRef` beside `ReadbackError` and fenced its own
scope explicitly: "the payload's own payloads (`EntityId`, `GeomRef`)
are not carried". LIB-CUR4 carried `CensusContact` beside
`ValidationError` and stopped in the same place, at
`CensusContact::ConformalPatch`'s `topo::ContactFinding`
(`crates/topo/src/validate.rs:1257`, the arm; `crates/topo/src/contact.rs:264`,
the struct). Both stops are defensible on the CUR3 rule — a caller
BINDS the inner payload and branches on the DISCRIMINANT, and the
discriminant is what the curated list owes — but the rung is now named
twice and belongs in one place.

The set, all at `topo`'s root and none on a curated list:

- `EntityId` (`topo::entity`) — carried by `DanglingRef::Entity`, and
  directly by `BlendError::{UnsupportedRunOut, UnsupportedGeometry,
  BodyNotIntact}` and by `ValidationError`.
- `GeomRef` (`topo::entity`) — carried by `DanglingRef::Geometry` and
  by `ValidationError`.
- `ContactFinding` (`topo::contact`) — carried by
  `CensusContact::ConformalPatch`. Note the asymmetry that makes this
  one odd rather than merely deep: its siblings `ContactClass`,
  `ContactRefusal` and `ContactVerdict` ARE curated (through
  `crate::select`), so the contact vocabulary is carried three-quarters
  and this is the missing quarter.

`EntityId` and `GeomRef` are the sharper pair, because they are not
only a rung below a carriage — `BlendError` names `EntityId`
DIRECTLY in three arms, which makes them a rung-1 hit of exactly the
LIB-CUR4 shape that unit's fence excluded (its brief named a trio and a
quartet, not the whole `BlendError` payload set).

**What a unit closing it would have to decide.** Whether the entity-key
vocabulary is curated surface. `no_arena_key_is_nameable_through_the_facade_document_surface`
(`crates/pncad/tests/all.rs`) already forbids `EntityRef`/`EntityKey`/
`Entry` on the document surface, so there is a standing rule in this
neighbourhood that a carriage argument has to clear first, and clearing
it is the unit's first job. See also
`work/lib/loop-key-is-uncurated-and-invisible-to-payload-scans.md`,
which is the same question about one more key.

## Closed

LIB-CUR5, under LB17's rule, and the stop clause did NOT fire.

**The guard, read first.** `no_arena_key_is_nameable_through_the_facade_document_surface`
names `EntityRef`, `EntityKey` and `Entry`. All three are
`editor-core`'s, declared in `crates/editor-core/src/names/table.rs`:
`EntityRef` is a `(body index, EntityKey)` pair, `EntityKey` is the
document layer's own sum over three topo keys plus `Body`, and `Entry`
is the name table's forward row. They are body-lineage-scoped against
the evaluation that minted them, and that is the seal. `EntityId` and
`GeomRef` are `topo::entity`'s — the type-erased sums over that
crate's seven arena keys and three geometry keys, declared in the same
module as `VertexKey`/`EdgeKey`/`FaceKey`/`LoopKey`, four of which the
prelude carries deliberately because `topo::query` answers keys from a
`Body`. Different crate, different layer, and the guard's word-boundary
scan matches none of the three names in anything this row adds. So the
row proceeded; the distinction is written into the prelude group that
carries them.

**Carried.** `EntityId` and `GeomRef` into the prelude's group 4
beside the keys they sum over — `DanglingRef`'s two arms are one or
the other, and `BlendError` names an `EntityId` directly in three
arms, so this is a rung-1 hit as well as a rung-2 one. `ContactFinding`
through `crate::select`, the contact vocabulary's missing quarter,
re-exported into the prelude's group 9 with its three siblings. Both
sums are matched exhaustively in `all.rs`; `CensusContact`'s
`ConformalPatch` arm names its payload now instead of binding a value
it could not spell, and the prelude's group-5 "ONE RUNG, AND THE STOP
IS DELIBERATE" comment is rewritten to say where the stop actually is.

**What this does NOT cover, as an argued stop rather than a banked
one.** `SolidKey`, `ShellKey`, `HalfEdgeKey`, `PointKey`, `CurveKey`
and `SurfaceKey` are not carried. A key has no discriminant, so there
is nothing a curated list owes about one that carrying the sum has not
already delivered: a caller binds the key out of the arm it matched
and passes it on. All six are one module hop away at
`pncad::topo::…`, contract clause 1 met by the whole re-export. This
is the `BandField` shape of a stop — argued from what a consumer
branches on — and not a deferral waiting for a unit.

---
id: next-payload-rung-under-the-cur3-cur4-carriages
kind: issue
title: EntityId, GeomRef and ContactFinding: the rung CUR3 and CUR4 both stopped at
status: open
opened: 2026-09-03
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

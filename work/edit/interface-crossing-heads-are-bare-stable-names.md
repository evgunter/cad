---
id: interface-crossing-heads-are-bare-stable-names
kind: issue
title: An interface crossing's two heads are bare StableNames, not FaceNames
status: open
opened: 2026-09-17
refs: [mate-head-entity-kind-is-decided-only-at-assembly]
---


Disclosed by the unit that made a mate head's entity kind a TYPE
(`edit/mate-head-kind`). A mate's heads are now
`crate::node::SitedFace`s over `crate::names::FaceName`, so a mate
naming an edge is a program that does not compile and a file carrying
one refuses at the wire door's own constructor.

**`InterfaceCrossing::Mate` did not follow.** Its `outer` and `inner`
fields (`crates/editor-core/src/node.rs`) are bare `StableName`s,
written by the split (`refactor.rs`'s crossing walk) out of the two
heads it just read — so they are face names by construction and the
type does not say so. The unit read them out through `(**outer)` and
left the record's shape alone, which is the smaller change and not the
honest one:

- the record is SERIALIZED node data carried by every
  `InstantiatePart`, so a file can spell a crossing whose `outer`
  names an edge, and nothing refuses it — the asymmetry the mate head
  itself no longer has;
- the split's re-wrap (the remainder's rebind) has to go back through
  `FaceName::new` at some point, and doing it at the record's boundary
  is one call instead of one per reader.

**What the fix is.** `outer: FaceName`, `inner: FaceName`, the wire
door's constructor asked by `Deserialize` as it is for a head, and the
split's crossing walk writing the heads' own `FaceName`s through
rather than unwrapping them. `crates/pncad-py`'s crossing payload
follows mechanically (LIB's).

**Measured, and WIDER than a file.**
`crates/editor-core/tests/rv_matehead_probes.rs`'s
`probe_an_edge_headed_interface_crossing_inserts_saves_and_loads` is
the measurement and stays until this row is taken:
`Node::instantiate_part_with` is public, so an EDGE-headed crossing is
built IN MEMORY through an ordinary door, and then inserts, saves and
loads. No file is needed — the load door is the third of three places
that admit one, not the only one.

Not a live defect:
`crates/editor-core/tests/fix_pattern_mate_crossing.rs` builds
crossings through the split, and nothing in the tree names a non-face
one. What the row asks for is the type saying what the split already
guarantees.

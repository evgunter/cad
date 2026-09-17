---
id: interface-crossing-heads-are-bare-stable-names
kind: issue
title: An interface crossing's two heads are bare StableNames, not FaceNames
status: spec
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

## Ruled and spec'd (2026-09-17, EDIT orchestrator) — middle tier, branch `edit/crossing-face-heads`

**Ruling: the record says what the split guarantees.**
`InterfaceCrossing::Mate { outer, inner }` (`crates/editor-core/src/node.rs`)
carry `FaceName`s, not bare `StableName`s: a crossing is written out
of two mate heads that are `SitedFace`s, so the two fields are face
names by construction and the type now says so. `Deserialize` goes
through `FaceName::new` as a head's does, so a file whose crossing
names an edge refuses at the wire door as `PersistError::Unreadable`
with the constructor's sentence; `Node::instantiate_part_with` takes
the typed record, so an edge-headed crossing cannot be built in
memory either — the measurement
`rv_matehead_probes::probe_an_edge_headed_interface_crossing_inserts_saves_and_loads`
flips from "inserts, saves and loads" to "does not typecheck / refuses
at load", and is the red-first row (re-headed to the invariant, moved
beside the mate-head load row in `edit_one_predicate`). The split's
crossing walk (`refactor.rs`) writes the heads' own `FaceName`s
through instead of unwrapping them; the remainder's rebind re-wraps
through the typed head, one call at the record's boundary rather than
one per reader. `crates/pncad-py/src/py/refactor.rs`'s crossing
payload (`outer`/`inner` as name text) follows mechanically — LIB's,
announced; no tag moves.

**Rows.** The flipped probe (a `compile_fail` twin where the in-memory
door is the subject, the load refusal where the file is — both, with
the `quantity::units` idiom the mate head used); the round trip of a
face-headed crossing; `fix_pattern_mate_crossing`'s rows unchanged
and green (the split's guarantee, now typed).

**Mutants.** `Deserialize` bypassing the constructor (the load row
reds); the walk unwrapping to a bare name and re-wrapping without the
check (does not compile — say so).

**Territory.** `crates/editor-core/src/{node.rs, refactor.rs,
persist/*}` (EDIT); `crates/editor-core/tests/*` (TCOST/TINT);
`crates/pncad-py/src/py/refactor.rs` (LIB, mechanical). Middle tier:
one opus style review with a correctness arm, then the fix pass.

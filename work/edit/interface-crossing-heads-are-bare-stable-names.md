---
id: interface-crossing-heads-are-bare-stable-names
kind: issue
title: An interface crossing's two references are bare StableNames, not FaceNames
status: closed
pr: 2814
branch: edit/crossing-face-heads
opened: 2026-09-17
closed: 2026-09-17
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

## Built (2026-09-17)

`InterfaceCrossing::Mate`'s `outer` and `inner` are `FaceName`s
(`crates/editor-core/src/node.rs`). `Deserialize` goes through
`FaceName::new` as a head's does, so a file whose crossing names an
edge refuses at the wire door as `PersistError::Unreadable`;
`Node::instantiate_part_with` takes the typed record, so one cannot be
built in memory either. The split's crossing walk (`refactor.rs`)
writes the heads' own face names through.

**Premise corrected.** The ruling above says the remainder's rebind
re-wraps through the typed head. It does not: the remainder's rebind
runs over the DOCUMENT's own names and never touches the record. The
re-wrap the ruling describes is the PART-SIDE remap in the crossing
walk (`refactor::split`, the `inner` that moves into the part's id
space) and `inline`'s dissolve check, which re-derives the same
reference to prove it lands on a spliced local name. One call at the
record's boundary either way; the site is not the one named.

`FaceName::map_derivation` (`names/role.rs`, crate-private) is the one
door a face name is re-derived at inside the crate. It is handed a
name's DERIVATION — the minting node and the role path — and keeps the
kind itself, so a rewrite cannot change a kind and no arm exists for
the case where one did. Its three callers are `refactor::remap_face`
(the split's crossing walk and `inline`'s dissolve check),
`remap_node`'s head arm, and `Node::rebind_payload_names`' mate arm;
the `unreachable!`, the `debug_assert!` and the third `FaceName::new`
they used to spell are gone. `remap_face` answers `RemapMiss::Name`,
and the id miss is its only miss. Where a `FaceName` comes from is one
census in one home (`FaceName`'s doc): three boundaries that turn DATA
into one — the wire, the Python binding's name-from-text door, the
viewer's picked face — plus that one in-crate re-derivation.
`SitedFace`'s copy is a pointer.

Rows. `rv_matehead_probes`'s
`probe_an_edge_headed_interface_crossing_inserts_saves_and_loads` is
deleted and replaced, beside the mate-head load row in
`edit_one_predicate`, by
`a_saved_crossing_reference_that_is_not_a_face_refuses_at_the_load_door`
(both fields, all three non-face kinds), its round-trip control
`a_face_referenced_crossing_round_trips`, and
`a_crossings_references_are_bare_names_on_the_wire`, which pins a
serialized crossing against a JSON literal. `refactor.rs`'s
`remap_keeps_the_kind` module — the reviewer's probe, merged with its
authorship and re-headed to the invariant — pins that a remap carries
every kind through and that the face remap agrees with the bare one.
`InterfaceCrossing`'s doc carries the `quantity::units` twin: a
`compile_fail,E0308` block feeding EDGE names, as its mate-head twin
does, and a RUNNING twin differing only in the two `FaceName::new`
calls. Measured red first: with the fields reverted to `StableName` the
load row answers `Ok(Loaded { … outer: StableName { kind: Body, … } })`.
`fix_pattern_mate_crossing`'s rows are unchanged and green.

Wire bytes unchanged — `FaceName` is `#[serde(transparent)]` — and the
receipt for that is the two rows above, not `wire_rv_bytes`, whose
variant pins carry no interface record and never see a crossing.
`crates/pncad-py`'s crossing payload needed no code change: its getters
reach the name through `Deref`, so what followed there is prose, in
`py/refactor.rs` and by hand in `pncad.pyi` (the stub a Python caller
reads; `py/path.rs` says the two are kept in step by hand). Vocabulary:
a crossing has REFERENCES, a mate has HEADS — the record's own word,
applied to the code and the title here; the id is identity and does not
move.

Filed, not built:
`work/edit/instantiate-part-crossings-are-names-payload-names-does-not-list.md`
— `InstantiatePart` is `name_free_node!()` while its crossings carry
two names each.

## Closed (2026-09-17, EDIT orchestrator)

Built and merged as PR #2814 (middle tier: one opus style review with
a correctness arm, then the union fix pass). `InterfaceCrossing::Mate`'s
`outer` and `inner` are `FaceName`s: the record says what the split
guarantees, `Deserialize` goes through the constructor (a file whose
crossing names an edge refuses at the load door, the six field×kind
cases pinned independently), `Node::instantiate_part_with` takes the
typed record so an edge-referenced crossing cannot be built in memory
(the `compile_fail` twin, fed an edge like its mate-head sibling), and
the wire bytes are unchanged (`serde(transparent)`, pinned against a
JSON literal). One premise corrected: the re-wrap site is the
part-side remap in the crossing walk plus `inline`'s dissolve check,
not the remainder's rebind. The review (0 MAJOR, 2 MINOR, 3 NOTE)
found the unit had minted a third divergent answer to "this face-name
remap cannot fail"; the fix pass replaced all three with one door,
`FaceName::map_derivation`, which keeps the kind by signature so no
impossible arm exists — the `unreachable!`, the `debug_assert!` and
the third `FaceName::new` are gone, and the callers census lives once
on `FaceName`. Vocabulary: a crossing's fields are REFERENCES, a mate's
`SitedFace`s are heads (the title moved; the id did not). Filed:
`instantiate-part-crossings-are-names-payload-names-does-not-list`
(`payload_names`' single-answer claim is false for the variant;
`Rebind` never reaches `outer`; the insert door's liveness check sees
neither reference). Territory crossed by announcement: WIRE (one
token), FIX (`refactor.rs`), TCOST/TINT suites, LIB (`pncad.pyi` and
the crossing pyclass doc, one fixture).

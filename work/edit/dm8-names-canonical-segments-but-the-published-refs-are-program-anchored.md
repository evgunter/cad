---
id: dm8-names-canonical-segments-but-the-published-refs-are-program-anchored
kind: issue
title: DM8 says 'canonical segments' but a program loop's published ProfileEdgeRef is program-anchored
status: open
opened: 2026-09-16
refs: [authored-step-to-canonical-segment-map-has-no-home]
---

Found building `authored-step-to-canonical-segment-map-has-no-home`
(the DM8 door). The door is built and green; this row is about the
WORDING of a ratified clause and of a type's doc, which is Ev's call,
not a lane's.

## The finding

**DM8** (`crates/editor-core/REFERENCES.md`, "The authored-step to
canonical-segment map is composed in `editor-core`") says the map
answers with "the canonical segments it became (`ProfileEdgeRef {
loop_index, segment }`)", composed from the replay's per-step segment
span and "canonicalization's `reversed` and `start` on
`LoopCanonical`".

For a PROGRAM loop those are two different coordinates, and the
published one is not the canonical one:

- `eval/anchor.rs` (`ProfileNaming`, `LoopAnchor`, `remap_table`)
  rewrites every profile ref an emitter minted canonical → program
  before the name table is published. Its module docs state the
  intent: *"`ProfileEdgeRef`/`ProfileVertexRef` indices mean 'the
  segment/vertex the program's step order authored', not 'the
  canonical rotation's position'"*, precisely so a parameter edit
  cannot renumber a selection.
- So the `ProfileEdgeRef` that reaches a face — the coordinate VIEW's
  focus marking and the chain-radius consumer both read — is
  program-anchored, and applying canonicalization's permutation to the
  span moves the answer OFF it. Measured: the acceptance rows in
  `crates/editor-core/tests/edit_step_segments.rs` fail on a reversed
  loop and on a rotated one when the door permutes.
- DM8's composition also cannot produce a canonical `loop_index` at
  all: `LoopCanonical` is indexed per INPUT loop and carries no
  canonical loop position, so a `{loop_index: loop_, segment:
  canonical}` ref would mix the two anchorings.

`crates/editor-core/src/names/role.rs` carries the same conflation one
level down: `ProfileEdgeRef`'s own doc says *"A profile edge (segment)
by canonical combinatorial identity"*, which is true of a hand-built
profile and false of every program loop since the anchor rewrite
landed.

## What the door does today, and why

`ProfileProgram::canonical_segments_of` answers in the published
(program) anchoring, and consumes BOTH records DM8 names: the replay's
span gives the segments, and `LoopCanonical`'s `reversed`/`start` is
checked against the naming anchor's independently bit-matched
permutation, refusing `StepSegmentsError::RecordsDisagree` when the two
derivations of one permutation disagree. That keeps DM8's substance —
the map is composed from the records the evaluation produced, never
re-derived from the geometry — while landing on the vocabulary the
names actually carry.

## What is wanted

Two wordings, neither of which a lane should decide:

1. **DM8's sentence.** Either re-word "the canonical segments it
   became" as "the profile edges the published names carry", naming
   the anchor rewrite as the reason, or rule that the door should
   answer canonically and that its consumers remap. The first is what
   the code does; the second contradicts the acceptance rows above.
2. **`ProfileEdgeRef`'s doc** in `names/role.rs`, which should say
   which anchoring it carries and when.

Both are a change to what a ratified clause decides, so they go to Ev
as an `[ev]` PR rather than riding the unit that found them.

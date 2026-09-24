---
id: dm8-names-canonical-segments-but-the-published-refs-are-program-anchored
kind: issue
title: DM8 says 'canonical segments' but a program loop's published ProfileEdgeRef is program-anchored
status: closed
pr: 2785
branch: edit/dm8-follow-through
opened: 2026-09-16
closed: 2026-09-16
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

`ProfileProgram::profile_edges_of` answers in the published
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

## The method's name is the lane's, and it moved (2026-09-16)

The unit first spelled the door `canonical_segments_of`, which is the
one word in it that the code makes false — the answer is in the
published (program) anchoring, not the canonical one. A METHOD NAME is
not a ratified clause and needed no ruling, so the style review's S1
was taken and the door is now
**`ProfileProgram::profile_edges_of`**: it names what it answers,
`ProfileEdgeRef`s, the coordinate a consumer holds. Recorded here so
that Ev's ruling on DM8's own sentence is read against the name the
code currently carries. The two questions above are unchanged by it —
they are about what the CLAUSE says, not what the method is called.

## Recommendation (2026-09-16, EDIT orchestrator) — item 3 of `[ev]` PR #2764

**Re-word DM8; do not re-shape the door.** The unit's review confirmed
the premise (`sure` for extrude and revolve, by reading `anchored()` at
both sweep call sites and by mutation) with one qualifier: `wire_loft`
anchors every section with section 0's anchor, so a loft's published
anchoring is program-anchored for section 0 only (the door's doc says
so; `work/wire/loft-anchors-every-section-with-section-zeros-map`).
Two wordings change: DM8's "the canonical segments it became" becomes
"the profile edges the published names carry (`ProfileEdgeRef`,
program-anchored for a program loop by `eval/anchor.rs`; for a loft,
by section 0's anchor)", with `LoopCanonical` named as the check on the
anchor's permutation rather than a factor of the answer; and
`ProfileEdgeRef`'s doc in `names/role.rs` says which anchoring it
carries and when. **Alternative:** rule that the door answers
canonically and consumers remap — contradicted by the acceptance rows
and would hand VIEW's focus marking a ref the faces do not carry.

## RULED (Ev, on the `[ev]` PR #2764, 2026-09-16): re-word DM8; the disagreement asserts

DM8's clause now says the door READS the two records — the span gives
the answer in the program's own step order, the numbering the published
names carry; canonicalization's `reversed`/`start` are checked against
the naming anchor's record of the same permutation, never applied — and
that a disagreement between them is the evaluation contradicting itself
and asserts (Ev: kernel bugs panic; the one cost, a caller's mispairing
of two records from different evaluations panicking rather than
refusing typed, is a caller bug with no façade caller today and
disappears by construction once the structure record rides on the
evaluation's own value). What remains is E-class code follow-through
on this row: `ProfileEdgeRef`'s doc in `names/role.rs` gains the same
one-clause fix; `StepSegmentsError::RecordsDisagree` becomes an
assertion with a message naming the invariant, its tag retired (LIB's
file, mechanical); the door's doc cites this clause.

## Spec (2026-09-16, EDIT orchestrator) — E-class: green CI and the orchestrator's read, no review lane

Branch `edit/dm8-follow-through`. Three edits the ruling above names,
each with its shape written here:

1. `ProfileEdgeRef`'s doc in `crates/editor-core/src/names/role.rs`:
   "by canonical combinatorial identity" becomes the one-clause truth
   — canonical for a hand-built profile, the program's own step order
   for a program loop after `eval/anchor.rs`'s rewrite — citing DM8.
2. `StepSegmentsError::RecordsDisagree` becomes an assertion at the
   door with a message naming the invariant (the evaluation's two
   records of one permutation agree) and both records' values; the
   variant is deleted, its F6 census case with it, and its tag retired
   in `crates/pncad-py` (LIB's, mechanical; the binding census follows).
   The row `two_records_describing_different_loops_refuse` becomes a
   `#[should_panic(expected = …)]` row on the message.
3. `ProfileProgram::profile_edges_of`'s doc cites DM8's amended
   sentence and says why the disagreement asserts (a kernel
   contradiction; the two-argument shape's mispairing case disappears
   once the structure record rides on the evaluation's value — cite
   WIRE's `section-of-re-derives-the-whole-f64-precompute-the-profile-node-already-made`).

No other behaviour changes; corpus goldens unmoved.

## Built (2026-09-16) — PR #2785, branch `edit/dm8-follow-through`

All three edits the ruling named landed.

1. `ProfileEdgeRef`'s doc in `names/role.rs` says which anchoring it
   carries and when — canonical as an emitter mints it, the program's
   own step order once `eval/anchor.rs`'s rewrite has published it —
   citing DM8 and naming the loft exception. Its two fields follow, and
   `ProfileVertexRef` with them: `remap_table` rewrites `loop_index`
   too (canonical loop index to `LoopAnchor::program_loop`), so the
   anchoring is not a segment-only property.
2. The two-record check at `profile_edges_of` is an `assert!` naming
   the invariant and both records' values.
   `StepSegmentsError::RecordsDisagree` is deleted with its `Display`
   arm and its F6 census case; the enum's header doc says why there is
   no such arm. `two_records_describing_different_loops_refuse` is
   `two_records_describing_different_loops_assert`, a `should_panic`
   row whose `expected` is the whole message, values included.
3. The door's doc cites DM8's amended sentence — the span GIVES the
   answer, the permutation is CHECKED and never applied — and a new
   section says why a disagreement asserts, with the mispairing cost
   and its citation of WIRE's
   `section-of-re-derives-the-whole-f64-precompute-the-profile-node-already-made`.

Three things the spec did not name, each argued in the PR body:

- **The `crates/pncad-py` tag the spec says to retire does not
  exist.** `StepSegmentsError` has no per-variant spelling there; the
  type is dispositioned whole as `gap: B-STEP-SEGMENTS` because the
  door has no Python binding yet. Nothing under `crates/pncad*` is
  touched.
- **A second row**,
  `a_naming_without_this_loop_refuses_rather_than_asserting`, carries
  the `NoAnchor` assertion the old row held alongside the one that now
  panics; without it `NoAnchor` has no coverage.
- **The sweep** over prose calling a published profile ref canonical
  fixed three more sites in `names/role.rs` and `names/mod.rs` (the
  `# Locators` paragraph, the `band` builder's doc, the module's
  locator sentence) and filed the Python mirror as
  `work/lib/python-selection-builder-docs-call-the-profile-index-canonical`.

Nothing else moved: no golden, no stored bit, no behaviour beyond the
refusal that became an assertion.

## Closed (2026-09-16, EDIT orchestrator)

Built and merged as PR #2785 (E-class: green CI and the orchestrator's
read; no review lane). The three edits the ruling named landed as the
`## Built` section records, with the sweep's three further prose sites
in `names/` taken as the same class. The orchestrator's rulings on the
lane's three questions: the `crates/pncad-py` half of the spec was a
wrong premise (there is no per-variant tag; the type is dispositioned
whole as `gap: B-STEP-SEGMENTS`), so nothing under `crates/pncad*`
moves and the spec is corrected by this note; the three sweep sites
stay; the closed row `authored-step-to-canonical-segment-map-has-no-home`
keeps its `m3` note naming `RecordsDisagree`, since it records a
mutation whose branch still exists and now asserts. One stale inline
comment in the `should_panic` row (it still said the door "cannot tell
which" record lied) was re-worded at close-out to match the row's doc.
Residue for LIB in its own file:
`work/lib/python-selection-builder-docs-call-the-profile-index-canonical`.

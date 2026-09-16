---
id: authored-step-to-canonical-segment-map-has-no-home
kind: unit
title: The authored-step to canonical-segment map has no home: its two halves are DOCM's and BOOL's, and neither owner can site it alone
status: spec
opened: 2026-09-04
refs: [focus-marking-is-per-node-not-per-segment]
branch: edit/step-segment-map
---

The announce VIEW's plan item 4 has owed since 2026-09-03, written as
a file because `work/README.md` is explicit that disclosing a residue
in prose is not scheduling it. `work/view/plan.md` said "two
announces"; a sentence in a plan is not one.

Filed in `work/issues/` because **no single program can own it** —
that is the finding, not an accident of filing.

## What is wanted

`work/view/focus-marking-is-per-node-not-per-segment.md` needs to
light the walls a profile step swept: viewport focus marking is per
`RecipeNodeId`, so a profile step cannot mark the faces it produced.
The missing reading is a map from an **authored step** to the
**canonical segments** it became.

## Why it has no home

The two coordinates are in two programs' territory and the map is
neither:

- the authored `step` is `ProfileProgram::step_args`
  (`crates/editor-core/src/program.rs:653`, read at `:1264`) —
  **DOCM's**;
- the canonical `segment` is `crates/profile`'s canonicalization —
  **BOOL's** (`crates/profile/src/*`, with `path/arc_fillet.rs` shared
  with FILLET).

A door on either side alone does not produce the map: DOCM can say
what steps a program authored and BOOL can say what segments a path
canonicalized to, and the correspondence between them is the thing
that exists in neither crate. So this is not "VIEW needs a door from
DOCM" (which is the shape of
`work/view/next-id-has-no-layer3-door.md`, and is answerable by DOCM
alone). It is a question about **where a value that belongs to two
crates lives**, and VIEW has no standing to answer it — nor, on its
own, does either owner.

## What the answer has to settle

1. **Which side computes it.** Canonicalization is where the
   correspondence is actually known — it is the step that turns
   authored steps into segments — which argues for `crates/profile`
   emitting it and `editor-core` carrying it. The counter-argument is
   that `crates/profile` has no vocabulary for an authored step today
   and would grow one to serve a consumer two layers up.
2. **Whether it survives the lowering** at all, or is re-derived. If
   the map is recomputed by the viewer from both endpoints, no door is
   needed and no crate grows a field — at the cost of a derivation
   that can disagree with the one that produced the geometry, which is
   the shape this project treats as a defect elsewhere.
3. **Whether anything but the viewer wants it.** If a second consumer
   exists (naming, selectors, a check), that decides (1) on its own.

## Who is blocked

`work/view/focus-marking-is-per-node-not-per-segment.md` — VIEW's plan
item 4, which cannot start until this is sited. It is the only known
consumer today, which is why the question has gone unasked: it looks
like VIEW's problem and it is not VIEW's to answer.

Signed: (VIEW orchestrator)

## Re-homed (2026-09-04)

Moved from `work/issues/` to `work/docm/` in the tracker-wide
re-home sweep of 2026-09-04 (Ev's direction, in-chat), which read every
open `work/issues/` file against every open program's `paths` and
against the code-quality K–X fences. Id, body and header are unchanged;
the directory is the claim (`work/README.md`). Any `## Home` section
above naming `work/issues/` is superseded by this line and is kept as
the record of why the file was parked there.

## Re-homed (2026-09-13)

Moved from `work/docm/` to `work/edit/` at DOCM's exit sweep (`docs/DOC-LEDGER.md`,
sweep 14): EDIT is DOCM's successor on the document-model ground (persist, the edit vocabulary, the node and resolver doors). Id, body and header are unchanged; the directory is the
claim (`work/README.md`). Any `## Home` section above is superseded by
this line and is kept as the record of why the file was where it was.

## Question for Ev (2026-09-16, EDIT orchestrator) — with a recommendation

**The question.** Where does the map from an authored profile step
(`SlotId::Profile { loop_, step, arg }`) to the canonical segments it
became (`ProfileEdgeRef { loop_index, segment }`) live, and who
computes it? VIEW's focus marking is the consumer; the row's three
sub-questions are which side computes it, whether it survives the
lowering or is re-derived, and whether a second consumer exists.

**Recommendation: both producers record what they already know, and
`editor-core` composes.** The map is two facts in sequence, each
known exactly by the code that makes it:

1. the replay turns each step into `k` segments in step order
   (`crates/profile`'s verb table; a fillet leg or `circle_split`
   is where `k ≠ 1`). `ReplayStructure` already records the replay's
   structural decisions (its fillet resolutions) and gains the
   per-step segment span — the same kind of record, one field.
2. canonicalization permutes that chain by `reversed` and `start`,
   both already recorded on `LoopCanonical`.

So the door is a function in `crates/editor-core/src/program.rs`
composing the two records into "step → set of canonical segments",
refusing rather than guessing for a loop whose record is absent. It
is derived from the structure record the evaluation itself produced,
so it cannot disagree with the geometry; it is not persisted. The
`program.rs` doc on `LoopProgram`'s radius door already says the
replay owns this map and left it unbuilt for want of a consumer.

**Second consumer, answering sub-question 3.** That same doc: pairing
a chain loop's per-step radii with swept walls needs this map, and
today the radius door answers `None` for chains because of it. So the
map has a consumer inside `editor-core` before VIEW's.

**Rejected: the viewer re-deriving it** from both endpoints — a
second derivation that can disagree with the one that produced the
geometry, the shape the project treats as a defect. **Rejected: the
profile crate owning the whole map** — it has no vocabulary for an
authored step and would grow one for a consumer two layers up.

**Cost and seam.** Item 1 is one field in BOOL's crate, announced to
S-BOOL and built by whichever side lands first; item 2 and the door
are EDIT's. VIEW's row unparks when the door merges.

## RULED (Ev, on the `[ev]` PR, 2026-09-16): the recommendation stands — DM8

Recorded as **DM8** in `crates/editor-core/REFERENCES.md`: the map is
composed in `editor-core` from the replay's per-step segment span and
canonicalization's recorded permutation.

## Spec (2026-09-16, EDIT orchestrator) — middle tier: one opus style review with a correctness arm, no A/B row

Branch `edit/step-segment-map`. Both halves in one PR, the profile
half by announcement (`work/bool/replay-structure-gains-the-per-step-segment-span.md`
says so; S-BOOL was not asked).

1. `crates/profile`: `ReplayStructure` gains the per-step segment span
   — for each authored step in program order, the range of
   pre-canonical segment indices it produced (a fillet leg or a
   `circle_split` is where a step is not one segment). Recorded where
   the replay already records its fillet decisions; the guided pass
   (a recorded structure replayed under a lane scalar) verifies the
   span it reproduces exactly as it verifies the fillet decisions.
2. `crates/editor-core/src/program.rs`: a door `canonical_segments_of(
   loop_, step) -> Result<set of segment, refusal>` composing (1) with
   `LoopCanonical { reversed, start }` (`crates/profile/src/structure.rs`)
   — reversal maps index `i` of `n` to `n-1-i` on the oriented chain,
   then the rotation by `start`; derive it, do not guess, and pin it
   against the geometry (acceptance 4). Refuses typed for a loop with
   no record or a step out of range. Carrier forms (`circle`,
   `circle_split`) pin `step` to 0 and map to every segment.
3. Re-read the `LoopProgram` radius-door doc that says the map is
   unbuilt; make it true again (present tense only). Do NOT widen the
   radius door to chains — that doc states why (the content-key
   attach obligation) and it is a separate row.
4. Rows that go red: for every gallery document, every `(loop, step)`
   maps to segments whose canonical `ProfileEdgeRef` names appear on
   faces the step's authored geometry actually bounds — checked by
   geometry (the wall's boundary contains the step's authored
   endpoints, transformed), not by index arithmetic; a mutant that
   drops the reversal or the rotation is caught by a loop that is
   reversed and one that is rotated.
5. On merge, VIEW's row `work/view/focus-marking-is-per-node-not-per-segment.md`
   unparks — say so there in a `## Unblocked` section in this PR.

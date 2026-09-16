---
id: authored-step-to-canonical-segment-map-has-no-home
kind: unit
title: The authored-step to canonical-segment map has no home: its two halves are DOCM's and BOOL's, and neither owner can site it alone
status: closed
opened: 2026-09-04
refs: [focus-marking-is-per-node-not-per-segment, dm8-names-canonical-segments-but-the-published-refs-are-program-anchored]
branch: edit/step-segment-map
pr: 2759
closed: 2026-09-16
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

## Built (2026-09-16) — superseded by `## Built, after the review` below

Both halves, one PR. Branch `edit/step-segment-map`.

**The profile half (spec item 1).** `ReplayStructure` gains `steps:
Vec<StepSpan>` — per authored step, in program order, the half-open
range of pre-canonical segment indices it produced — beside the fillet
decisions, with `StepSpan`'s own `Display` so a refusal renders it as
words. The chain records it as it lowers: `Core::record`, which every
row calls before it constructs, now also notes the chain length the
step starts from, and `Core::step_spans` turns those boundaries into
spans at the close (segment `k` leaves vertex `k`, so a step produces
the segments whose end vertices it pushed, plus the seam segment when
it is the closing step). The complete-loop carrier forms mint
`ReplayStructure::carrier(n)` — one step, every segment — which is how
`circle`/`circle_split` answer the door with no arm of their own.
`replay_guided` verifies the spans it reproduced against the record,
refusing `Decision::StepSpan { step }` flipped; the check sits there
rather than inside the guide because the carrier forms take no guide at
all.

**The door (spec item 2).** `ProfileProgram::canonical_segments_of(
structure, naming, loop_, step) -> Result<Vec<ProfileEdgeRef>,
StepSegmentsError>` in `crates/editor-core/src/program.rs`. Refusals are
typed for a loop or step the program does not have, a record that does
not cover the loop, an anchor that does not mention it, a span that
reaches past the loop, and — the substantive one — two records that
describe the loop's permutation differently.

**One correction to the spec's premises, filed as its own row.** The
spec (and DM8) has the door composing `LoopCanonical`'s
`reversed`/`start` into its answer. A program loop's published
`ProfileEdgeRef` is program-anchored, not canonical (`eval/anchor.rs`
rewrites every emitted ref canonical → program before the table is
published), so permuting the span moves the answer off the refs the
names carry — measured: the reversed and rotated acceptance rows fail
under exactly that mutant. The door therefore answers in the published
anchoring and consumes `LoopCanonical` as the CHECK on the anchor's
independently bit-matched permutation. The wording of DM8 and of
`ProfileEdgeRef`'s own doc is Ev's call and is filed as
`dm8-names-canonical-segments-but-the-published-refs-are-program-anchored`.

**Rows that go red (spec item 4).** In `crates/profile`,
`common::pinned` — the blanket funnel every closing verb in the suite
goes through — now asserts the spans partition the loop, and
`guided_replay` gains a lying-span row. In `crates/editor-core`,
`tests/edit_step_segments.rs`: every step of every corpus profile is
answered and the answers partition its loop; three extruded prisms
(identity, reversed, rotated — each asserting it IS that case) check by
GEOMETRY that the wall a ref names carries that segment's endpoints
placed into 3-space; and the two-records-disagree refusal. The
permuting mutant is caught by the reversed row, the rotated row and the
corpus row.

**Not done, deliberately (spec item 3).** The `LoopProgram::carrier_radius`
doc is re-worded to present tense — the map is built and named — and the
door is NOT widened to chains: that doc states the content-key attach
obligation as the reason, and it is a separate row.

**Follow-throughs outside this fence, each named in the PR.**
`crates/pncad` carries `StepSegmentsError` beside `RecordedProgramError`
(the document-layer completeness guard requires it), and
`crates/pncad-py`'s binding census dispositions it as
`gap: B-STEP-SEGMENTS` with that family's charter — both LIB's ground,
both mechanical, and the binding itself is filed as
`work/lib/python-has-no-step-to-profile-edge-door`.

**CI.** Run 35086077388 green on head `39024d0b`: twelve `test (…)`
jobs (six default, six interval) and five `k-lint (gate, …)` jobs, every
step green.

## Built, after the review (2026-09-16)

The style review's verdict was APPROVE-WITH-FIXES. The tree as it now
stands differs from the `## Built` above in these ways.

**The door is named `ProfileProgram::profile_edges_of`** (S1). The
answer is in the PUBLISHED anchoring, so "canonical" was the one word
in the old name the code makes false; the name now says what it
answers. A method name is not a ratified clause, so this needed no
ruling, and the rename is recorded on the `[ev]` row
(`dm8-names-canonical-segments-but-the-published-refs-are-program-anchored`)
so Ev's reading of DM8's own sentence sees the current name.

**The rows can see a wrong ATTRIBUTION** (M1). The two review probes
are adopted authorship-preserving and re-documented as rows of the
suites they belong to, and the shape is extended to every step kind the
corpus has: `edit_step_segments.rs` gains a section that reads each
step's OWN AUTHORED ARGUMENTS back — the point a `line_to`/`arc_to`/
`far end` names, the entry point an `at` seeds, the length a `line`
names, the centre and radius a carrier form names — and asserts the
segments the door credited it with are the ones that geometry
describes, with a tally that refuses to pass on a corpus that stopped
containing an entry verb, a state-binding verb, an arrival at an
authored point, a close, a length-authored leg, a carrier form, a step
credited with several segments, or a step credited with an arc.
Measured, with the mutant planted on this tree: a `Core::record` that
shifts every boundary one step later leaves the WHOLE profile suite
green (446 rows) and every one of the unit's original rows green, and
reds exactly the two attribution rows.

**Both partition helpers now say what they cannot catch** (S2, S6):
`profile`'s `assert_spans_partition` states that on a chain
contiguity, cover and count are what `Core::step_spans` makes true by
construction and names the row that does see attribution, and
`edges_by_step` answers while `assert_partition` asserts, so four rows
no longer share one check by accident.

**Every row records before it constructs, entry rows included** (m1).
`Open::at_kernel` and `Open::director` take the step and record it into
the core they mint; the three entry rows no longer record into a path
they already built. The spans are unchanged by it — the old order's
`saturating_sub` happened to absorb the difference — and `Core::record`
now states the invariant, which is what makes `step_starts[j]` mean one
thing for every `j`.

**The refusal prose has no gutters, and every arm is read back** (m2).
The three `\`-continued literals are `concat!`s; `StepSegmentsError`
joins the F6 census in `tests/display_contract.rs`, so a variant added
to it stops that file compiling until it has a case.

**`RecordsDisagree`'s reversed arithmetic is exercised** (m3). A convex
pentagon authored clockwise from a non-minimal corner:
reversed AND rotated, with `2 * start != n`, which is the only fixture
that can tell `(n - start) % n` from `start`. Measured: that mutant
leaves every other row in the suite green and reds this one alone.

**Two more refusal arms and a new one** (N1, N2). Rows for `NoRecord`
and `SpanOffTheLoop` — the latter is the last guard before the door
mints refs for segments a loop does not have — and the record's step
count is now checked against `LoopProgram::authored_steps()`, refusing
`RecordShape { loop_, authored, recorded }`, the shape the profile side
already uses for a record of the wrong length.

**Smaller** (N3, N4, S5): the corpus row's floor is tied to the profile
nodes the corpus evaluates rather than to `20`/`40`; the dead
`end.max(start)` clamp is gone; `StepSpan`'s fields are private and
`StepSpan::new` asserts `start <= end`, so a backwards span cannot be
built rather than being papered over where it is read.

**Stated, not repaired** (Claim 1, m5). `wire_loft` anchors the whole
emitted table on section 0's `LoopAnchor`, so the door's answer is
program-anchored only for section 0 of a loft; that is now on the
door's doc and filed as
`work/wire/loft-anchors-every-section-with-section-zeros-map.md`. The
acceptance rows still REBUILD the structure record, because measurement
says nothing public carries the evaluation's own: `ProfilePre` is
`pub(crate)` and `ProfileValue` holds only `validated` and `naming`.
The rebuild is kept with a comment naming WIRE's row, and the door's
own two-record check is what holds the pairing honest.

**The door has no caller outside its own tests**, and that is the plain
state of it: the records it takes are not reachable from a consumer
until `work/wire/section-of-re-derives-the-whole-f64-precompute-the-profile-node-already-made.md`
lands. That row is cited from VIEW's `## Unblocked` and from the LIB
row rather than duplicated (m4b).

**Rows filed** (m4a, Claim 1):
`work/edit/carrier-radius-door-answers-none-for-chains-though-the-map-now-exists.md`
and `work/wire/loft-anchors-every-section-with-section-zeros-map.md`.

## Closed (2026-09-16, EDIT orchestrator)

DM8 built and merged as PR #2759 after one opus style review with a
correctness arm (APPROVE-WITH-FIXES: one MAJOR — the rows could not
tell a correct step→segment attribution from a wrong one, shown by a
surviving mutant in `Core::record` — five MINOR, four NOTE) and a fix
pass that took every finding: the attribution rows now read each
step's own authored arguments back against the segments it is
credited with, a reversed-and-rotated prism exercises the two-record
check's arithmetic, the record's step count is checked against the
program's, the dead clamp is gone, `StepSpan` cannot be built
backwards, and the door is `profile_edges_of` (the one false word
dropped). The unit corrected a premise of its spec and of DM8 — the
published `ProfileEdgeRef` is program-anchored — and filed the wording
for Ev (`dm8-names-canonical-segments-but-the-published-refs-are-program-anchored`,
on `[ev]` PR #2764 as item 3). Residue in its own files:
`carrier-radius-door-answers-none-for-chains-though-the-map-now-exists`
(EDIT), `work/wire/loft-anchors-every-section-with-section-zeros-map`
(WIRE), and the plumbing that would give the door a caller outside its
tests, which WIRE's
`section-of-re-derives-the-whole-f64-precompute-the-profile-node-already-made`
already covers. VIEW's `focus-marking-is-per-node-not-per-segment` is
open. The reviewer's probe merge carried a proptest regression seed
found under its planted mutant; it pinned nothing the tree ever failed
and was dropped at merge.

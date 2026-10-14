---
id: the-refusal-shape-guard-has-blind-spots
kind: issue
title: chrome: the refusal shape guard (test_utils::refusal and its callers) has blind spots that stay green
status: closed
opened: 2026-09-23
priority: P3
cost: M
closed: 2026-09-29
pr: 3457
---


## Finding

The shape guard for refusals the viewer draws (`crates/test-utils/src/refusal.rs`,
called by `crates/editor-core/tests/refusal_concision_chains.rs` and
`crates/viewer/tests/refusal_concision_edits.rs`) passes over several
shapes it exists to catch. Each blind spot stays green:

1. **Where a prefix may start.** `stage_prefixes` opens a clause only at
   the text's start or after `": "`, `"— "`, `"; "` or `"("`
   (`refusal.rs:36`). A prefix after `". "` or `", "` is missed.
2. **How long a prefix may be.** A clause counts only at one or two
   tokens (`refusal.rs:44`), so a prefix of three or more words is
   missed (`declared-REST union zip:`, `curved-section invariant at
   face …:`).
3. **Capitals.** A token must start lowercase (`refusal.rs:47`), so a
   capitalised prefix is missed (`A/B lockstep invariant violated:`).
4. **Allowed labels are global.** `ALLOWED_LABELS`
   (`refusal_concision_chains.rs:92`) admits `check separation` and the
   rest on every row, not only on the checks-window rows that use them.
5. **Exemptions are never required to fire.** `FILED`, `FILED_DECLARE`
   and `KERNEL_KEYED` are never checked for use. (The span admissions,
   `ADMISSIONS` there and in `refusal_concision_edits.rs`, are:
   `test_utils::refusal::problems_admitting` reds an admission whose
   span its row no longer holds, and `unclaimed_admissions` one whose
   row is gone.) When the owner's fix lands, the stale
   entry stays and the test stays green; it goes red only when someone
   reads the list.
6. **The blend-detail reader.** It has no count floor: it asserts only
   that each helper is read once (`:2830`). It also reads only the four
   helper calls (`:2908`), so a direct
   `BlendError::UnsupportedChain { .. }` construction outside
   `surgery.rs`'s helpers would not be rendered.
7. **BodyNotIntact rows are keyed by prefix.** They are admitted by
   `starts_with("Blend/BodyNotIntact@")` (`:2862`), not by exact id.
8. **Recourses without a marker are not counted.** `recourse_markers`
   counts only `Recourse:` and `There is no way through`
   (`refusal.rs:87-88`). A second recourse phrased without them is
   invisible, for example:
   - "blend in SEQUENTIAL calls";
   - `Indeterminate`'s "— a near-coincidence; declare …" tail;
   - `StructureRefusal`'s "narrow the parameter box and try again".

   So "one recourse per message" is enforced only between the two
   markers.

Found by the CHROME concision delta review (PR #3108).

## Also found while fixing it

`geom::EllipseInvalid` (`crates/geom/src/curves.rs`, in open PR #2861)
opens every arm with `ellipse construction:`. Its `CircularAxes` and
`Escalated` arms offer `COINCIDENCE_RECOURSE` ("declare …"). Under a
split (`SplitJoinError::Section`), a plane×cylinder section at a
near-circular tilt reaches it, and a split takes no declaration. The
chain test admits exactly this on `Split/Join/Section(Carrier)` and
`Boolean/Join/Section(Carrier)` (`FILED`, `FILED_DECLARE`).

## A ninth: a refusal with no recourse is never flagged (2026-09-29)

`test_utils::refusal::problems` (`crates/test-utils/src/refusal.rs`)
flags a message with MORE than one recourse marker and never one with
none, so a refusal that states no recourse at all, and no "There is no
way through" either, stays green in
`crates/editor-core/tests/refusal_concision_chains.rs`. Live instances,
all `PartFault` arms (`crates/editor-core/src/eval/parts.rs`,
`impl core::fmt::Display for PartFault`), each rendered through
`NodeErrorKind::Part`:

- `NoResolver` ("this evaluation carries no part resolver, so a
  referenced document cannot be reached");
- `DepthExceeded`;
- `ReferenceCycle`;
- `PartProduct` (the gather's own sentence, whose recourse is
  whatever the product door's arm states).

`refusal_concision_at_rest.rs` already holds its rows to at least one
marker (`every_at_rest_finding_renders_to_the_standard`); the chain
test does not. Found by `edit/part-root-carried-refusal` (the designer
pair's seam list,
`work/edit/part-root-failure-nests-a-whole-refusal-past-the-budget.md`).

The same unit closed one blind spot this row did not list: `arena_key`
now flags a hex document id as well as `Key(`.

## Closed 2026-09-29 (`chrome/refusal-residue`)

1–3. **The prefix check reads a label's shape, not its length.** A
   clause now opens after `. `, `, ` and a line break as well, a
   parenthetical that closes before the colon never opens one, and a
   clause is a stage prefix when it carries none of the words only a
   sentence has (`SENTENCE_WORDS`: articles, determiners, pronouns,
   auxiliaries, negations, and the wrappers' own "failed", "refused",
   "escalated"). So `declared-REST union zip:`, `A/B lockstep
   invariant violated:` and `path junction classification:` are red,
   and `the Boolean op refused:` and `node 5 failed:` are not. Unit
   rows: `the_prefix_shapes_the_first_reading_missed_are_red`.
   It found, on rows already rendered, every `predicate 'x'
   indeterminate:` (the payload's name; decided in
   `indeterminate-payload-shows-the-viewer-a-predicate-name-and-band-numbers`),
   `shell ShellKey(…):`, and the labels now filed on `paths`, `reach`,
   `carve` and the unowned row (`*-refusals-short-of-the-shape-guard`).
4. **Labels are scoped.** `ALLOWED_LABELS` is `(row namespace,
   label)`: the check labels and `root 4 output 0` on `Check/`,
   `mate 9` on `Mate/`, `at corner` on the corner-pair rows. The
   at-rest and edit suites scope theirs the same way.
5. **Every exemption list must fire.** `KERNEL_KEYED`,
   `ALLOWED_LABELS`, `FILED`, `FILED_NAMESPACES`, `FILED_DECLARE`,
   `FILED_NO_RECOURSE` and `FILED_SUBJECTLESS` are checked by
   `every_admission_admits_a_row_it_is_needed_for`; the at-rest and
   edit suites check their own lists in the same test. The entry
   `("Transform/Certify", "certification")` was already stale on main
   (ENCL's certify pass had removed the prefix); put back, it turns the
   check red.
6. **The blend-detail reader** has a count floor (`BLEND_DETAIL_FLOOR`,
   187) and also reads the four arms built directly with a `detail:`
   field. That found eight raise sites the helper reader never
   rendered: four in `battery.rs` (two `BodyNotIntact`, two
   `UnsupportedRunOut`) and four in `mod.rs`'s sample list. All fit.
7. **`BodyNotIntact` is keyed by the arm**, read from the helper the
   detail was raised through, not by a row-name prefix.
8. **Recourses without a marker.** The shared unlabelled repairs
   (`BARE_RECOURSES`) now count, so `Indeterminate`'s "— a
   near-coincidence; declare …" tail is one recourse, and a second one
   beside it is red.

**What the guard still cannot see.** A bespoke recourse phrased
without a marker and outside the shared vocabulary ("blend in
SEQUENTIAL calls", "narrow the parameter box and try again") is not
counted: by shape it is a clause like any other, and no list of such
phrases would see the next one. A stage label that happens to contain a
sentence word (`the section stage:`) passes. A sentence with no such
word reads as a label: `two instances overlap:` on the at-rest
`InstanceInterference` row, admitted there by name.

**The ninth.** `problems` flags a refusal that states no recourse
(`a_lost_or_doubled_recourse_is_red`), so the chain test now holds
every row to one; the `PartFault` rows above that state none are
admitted by exact id in `FILED_NO_RECOURSE` under
`work/recipe/edit-refusals-short-of-the-shape-guard.md`, where a row still
does. `ADMISSIONS` (the exact-span admissions the same EDIT unit added
for hex ids) carries its own must-fire through `problems_admitting`.

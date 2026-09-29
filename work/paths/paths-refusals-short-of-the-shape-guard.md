---
id: paths-refusals-short-of-the-shape-guard
kind: issue
title: paths: refusals the viewer draws that state no recourse, by the shape guard's census
status: open
opened: 2026-09-29
priority: P2
cost: M
---

(CHROME `refusal-residue`, from the shape guard's zero-recourse check.)

## What

`test_utils::refusal::problems` now flags a refusal that states no
recourse: no `Recourse:`, no "There is no way through" in either case,
and none of the shared unlabelled repairs (`BARE_RECOURSES`). The
standard (`work/chrome/error-and-check-text-overflows-its-region.md`,
"The standard a refusal is rewritten to") says the recourse is the
part never to drop, and where there is no way through the sentence
says so.

These rows, raised through `PathError` (`crates/profile/src/path.rs`), `ProfileError` (`crates/profile/src/validate.rs`) and `StructureRefusal` (`crates/profile/src/structure.rs`), render with none. Each is admitted
by exact id, under the comment naming this file:

- `crates/editor-core/tests/refusal_concision_chains.rs`, `FILED_NO_RECOURSE`:
  33 feature-tree rows.
- `crates/viewer/tests/refusal_concision_edits.rs`, `FILED_NO_RECOURSE`:
  3 status-line rows.

Families: `Edit/ProfileProgramRefused`, `Profile`, `ProfileReplay`.

## Labels the shape guard reads as stage prefixes

The guard now reads a clause of any length, in any case, after `. ` and
`, ` as well; these rows open a clause with a label and are admitted
row by row in `refusal_concision_chains.rs`'s `FILED`:

- `ProfileReplay/Path/CircleSplitCount`: `circle_split needs between 2
  and 4294967295 arcs:` — a function name and `u32::MAX` on screen.
- `ProfileReplay/Path/SeamRetrimsArcFirstSide`: Rust syntax on screen,
  `fillet_arc(r, Center { c, winding, p: Start })`.
- `Profile/TangentialContact`: `tangential contact between loop 0
  segment 1 and loop 0 segment 3:`.
- `Profile/RayCastingExhausted`: `containment of loop 1 in loop 0:`.

## Repair shape

Rewrite each arm at its source to the standard: add the recourse the
raise site supports, or say "There is no way through" where none
exists (`geom_core::KERNEL_DEFECT_ENDING` and its siblings for a
kernel defect). Read the raise sites first: a recourse is a claim.
Then drop the row's entry. The lists carry a must-fire check
(`every_admission_admits_a_row_it_is_needed_for`, and the same check
in the edit and at-rest suites), so an entry left behind after the fix
goes red.

## A predicate's name on screen (CHROME fix pass, PR 3457)

A predicate's static name is routing: `geom_core::IndeterminatePayload`
no longer renders it, and every escalation now says in words what was
being decided (`test_utils::refusal::subjectless_escalations`). These
sites still put a name in the sentence; each wants the decision in
words, with the name left to `Debug`:

- `crates/profile/src/path.rs`, the stored-form fillet arms: "…
  ('{predicate}' classifies …)" (two arms).
- `ProfileError::Escalated` and `PathError::Escalated` now open with
  what the decision decides, from `validate::decision_subject` (the
  `SHARED_CLAUSE_ONLY` descriptions plus the carrier and segment
  checks). A name decided later that is in neither falls back to "a
  validation decision" / "a decision the path door takes".

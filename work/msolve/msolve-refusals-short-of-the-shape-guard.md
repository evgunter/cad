---
id: msolve-refusals-short-of-the-shape-guard
kind: issue
title: msolve: refusals the viewer draws that state no recourse, by the shape guard's census
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

These rows, raised through `MateFault` (`crates/editor-core/src/mate.rs`), in the feature tree and through `EditError::MateRefused` / `MaintenanceRefused` on the status line, render with none. Each is admitted
by exact id, under the comment naming this file:

- `crates/editor-core/tests/refusal_concision_chains.rs`, `FILED_NO_RECOURSE`:
  11 feature-tree rows.
- `crates/viewer/tests/refusal_concision_edits.rs`, `FILED_NO_RECOURSE`:
  22 status-line rows.

Families: `Edit/MaintenanceRefused`, `Edit/MateRefused`, `Mate`.

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

- `crates/editor-core/src/mate.rs`: ": predicate `{predicate}` …".

## The carried placer line (CHROME, PR 3457)

`MateFault::PlacerRefused`'s carrying line, as the feature tree draws it
when the placer's own row is silent, reads "… node 4, which places it,
refuses — repair node 4": an unlabelled recourse, so the shape guard
counts none. Admitted by exact row
(`Carried/PlacerRefused/level-0`) in
`refusal_concision_chains.rs` `every_carried_refusal_draws_within_the_budget_at_every_line`,
with a must-fire check.

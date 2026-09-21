---
id: no-test-can-reach-a-pane-function
kind: issue
title: ViewerBehavior's *_ui functions have no test that can construct one, so a panel's own decisions are unassertable
status: open
opened: 2026-09-20
refs: [the-hide-toggle-is-drawn-over-a-refusal-the-op-will-give, the-range-button-re-mints-the-ratified-affordance]
priority: P3
cost: D
---

Recorded once, in prose, in `work/view/log.md:338` — *"the entire
2,507-line `impl ViewerBehavior` (32 `*_ui` fns) has no direct test at
all"* — and nowhere as a row. A closing program's log is deleted with
its directory, so this files the fact where the sweep that re-homes
residue can see it.

## What it costs, measured on a unit that paid it

`vnews/properties-controls-read-their-refusals` (PR #2961) repaired two
controls whose gates disagreed with their doors. **Neither repair is
mutation-provable at the panel.** `ViewerBehavior` is `pub(crate)`,
carries about twenty borrowed fields (a `DocSession`, a `SceneMesh`, a
`PickIndex`, a `Camera`, four `&mut` draft stores, a `Theme`), and
nothing in the crate constructs one — so reverting `instance_ui`'s
document, or making `range_button` mint a sentence again, leaves every
test green. What that lane could pin was the model values the panel
reads and the identity of the sentences; what it could not pin is that
the panel reads them.

The same gap is why the `session.doc()` / `committed_doc()` defect the
style review found could only be argued, not executed.

## The two shapes, and the one this crate already uses

- **Make the panel constructible in a test** — a builder or a
  `#[cfg(test)]` constructor over defaulted fields, with a headless
  `egui::Ui` (`egui::Context::run_ui`, which `eval_seam.rs` and
  `widgets.rs`'s own test module already drive). Buys assertions on
  what a panel DRAWS and on the ops it pushes; costs a seam that has to
  be kept honest as the struct grows.
- **Put the decision on a value the panel reads** — what the crate
  does today, and what closes this class elsewhere: `CancelDoor` on the
  session, `PartEntry::refusal` on the catalogue,
  `DeleteAffordance::of`. The panel then holds one line and the
  decision is testable where it lives. This is the cheaper and more
  idiomatic direction and it is NOT free: a decision hoisted out of the
  panel has to belong to the door it mirrors, or it becomes a second
  home for a condition (which is what the same PR's review caught in a
  first draft that hoisted the per-instance section's gate into a door
  that swallowed the fault `instance_check`'s doc says the caller must
  discard at its own call site).

## Scope

`impl ViewerBehavior` spans `app.rs` (VSEAM's) and every `pane/*`
module, which four successor programs claim between them — so the ROW
is here, on VNEWS's slate, and a unit cut from it either lands in one
file with the others announced, or is a VIEW-level decision. The
count in the log line is a measurement of `app.rs` on 2026-09-04 and is
not re-derived here: `grep -c 'fn .*_ui' crates/viewer/src/pane/*.rs`
plus `app.rs` is the instrument.

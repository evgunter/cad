---
id: no-test-can-reach-a-pane-function
kind: issue
title: what a pane METHOD decides is still unassertable — the document it reads is the case that cost a defect
status: open
opened: 2026-09-20
refs: [the-hide-toggle-is-drawn-over-a-refusal-the-op-will-give, the-range-button-re-mints-the-ratified-affordance]
priority: P3
cost: D
---

Recorded once, in prose, in `work/view/log.md:338` — *"the entire
2,507-line `impl ViewerBehavior` (32 `*_ui` fns) has no direct test at
all"* — and nowhere as a row until this one.

## What main already has, and what it does not reach

`crate::pane::headless` (`crates/viewer/src/pane.rs`) lays a pane out
in a real `egui::Context` with no window and reads back what it
painted — `painted_text`, `painted_after_clicking`, `landed`. Its own
doc states the rule this row would otherwise have proposed: *"What it
still cannot reach is a pane METHOD … A row a test must drive is
therefore a free function over the `Ui`, and the method's job is to
call it."*

`vnews/properties-controls-read-their-refusals` (PR #2961) took that
route for the hide toggle: `hide_toggle` is a free function, and two
headless rows click it — refused, it cannot be flipped and paints the
door's own sentence; addressable, the same click flips it. Both were
verified red by re-introducing the defect (a toggle enabled
regardless) and by minting a sentence in place of the fault's.

## What stays out of reach

**Whatever a method decides before it calls the free function.** On
that same unit, that was the defect with the most reach:
`instance_ui` read `session.doc()` — the previewed document — where
`set_hidden` reads `history.doc()`. The fix is one line in the method,
and no test can hold it, because the method borrows the whole
application (`ViewerBehavior`: a `DocSession`, a `SceneMesh`, a
`PickIndex`, a `Camera`, four `&mut` draft stores) and nothing in the
crate constructs one. `range_button`'s call to `probe_refusal` is the
same shape, more mildly.

## The two ways to close it

- **Pass the decided VALUE into the free function**, not the session:
  the method computes `display_check(committed_doc(), node)` and hands
  over the `Result`. That is what `hide_toggle` does, and it moves the
  toggle's behaviour into reach — but the choice of DOCUMENT stays in
  the method, which is exactly the part that was wrong.
- **Put the choice where the door is.** `DocSession` could answer
  "would `SetInstanceHidden` accept this node?" itself, from the
  document its own door reads, the way `delete_affordance` and
  `CancelDoor::blocked` already answer for theirs. Then the panel
  cannot pick the wrong document because it picks none, and the
  question is testable on the session. `session.rs` is VSEAM's, so a
  unit cut from this row is a hand-off there;
  `work/vseam/a-panels-gate-reads-the-previewed-document-while-its-door-reads-the-committed-one`
  is its member list.

The count in the log line is a measurement of `app.rs` on 2026-09-04
and is not re-derived here: `grep -c 'fn .*_ui'` over `app.rs` and
`pane/*.rs` is the instrument.

---
id: refusal-nothingtodo-pattern-sites-took-a-payload
kind: issue
title: Four viewer suites' NothingToDo patterns took a payload from a VNEWS unit
status: open
opened: 2026-09-20
priority: P4
cost: E
---

**Announcement, not a defect.** `crates/viewer/tests/*` is this
program's by declaration (VNEWS's `program.md` `keep_out`: *"a
test-mechanism change is announced to both"*), and VNEWS's
`vnews/app-controls-read-their-refusals` (#2960) made one.

## What moved

`viewer::session::Refusal::NothingToDo` grew a payload —
`NothingToDo { direction: Step }` — so that the toolbar's Undo and Redo
buttons can each show the sentence their OWN direction is refused with.
Four suites match on that arm and each now names the direction it
means rather than matching the bare variant:

- `crates/viewer/tests/undo_tree.rs`, in
  `undo_at_the_root_and_redo_at_a_leaf_refuse_rather_than_wrap` — two
  sites, one per direction, and they are the row that gave the payload
  its argument: that fixture is at the root WITH a redo waiting.
- `crates/viewer/tests/creation_ops.rs` — one site, `Step::Undo`.
- `crates/viewer/tests/review_gui3_r2.rs` — one site, `Step::Undo`.
- `crates/viewer/tests/frame_policy.rs` — two sites, which construct a
  `Refusal` by hand to feed `frame::batch_status` / `frame::frame_status`.

Each was `matches!(…, Some(Refusal::NothingToDo))` or a bare
constructor; each is now the same with `{ direction: Step::… }`. **No
row's subject, fixture or assertion count changed**, and every one of
them is strictly stronger than it was: the direction is now asserted
where before any direction would have satisfied it.

`Step` is reachable as `viewer::session::Step`; it is deliberately not
in `crates/viewer/src/lib.rs`'s root re-export list.

`test_utils::f6_variants!` needed no change: the macro writes
`Ty::Variant { .. }`, which is a legal pattern for a unit, tuple or
struct variant alike, so `panel_edits.rs`'s `Refusal` roster is
untouched.

## Track W

VNEWS's `keep_out` names *"S-TCOST's and S-TINT's and Track W's"* for
this file class. There is no `work/` directory for Track W on this
tree, so the announcement could not be filed on its slate; this row and
its twin `nothingtodo-pattern-sites-took-a-payload-tint` on `work/tint/` are the whole of it.

---
id: the-expression-path-edit-cannot-refuse-as-prose
kind: issue
title: SetExpression cannot be bound: path_off_tree Debug-renders its address and the prose gate panics
status: open
opened: 2026-09-09
priority: P3
cost: D
---



Found at LIB-EDITS by binding the door and running it: the door was
written, its refusal was provoked, and the binding PANICKED.

`EditError::PathOffTree`'s `Display` renders the whole address
through `Debug` (`crates/editor-core/src/edit.rs:1033`):

    expression path ExprPath { node: RecipeNodeId(2), slot: Distance,
    path: [0] } runs off the tree

`crates/pncad-py/src/py/mod.rs`'s `typed_err` asserts
`crate::errors::reads_as_prose` on every raise, and that predicate
rejects the field-brace fingerprint `" { "`. The assertion is a
`debug_assert` the workspace keeps ON under release, so this is a
PanicException in a built wheel, not a debug-only tripwire — and it
fires at the arm the door exists to be refused by. `Doc.apply`
raising it was observed directly, with the stack through
`py::doc::edit_err`.

The rendering is already filed as DOCM's
`work/docm/debug-in-prose-residue-after-finding-sink.md` (GitHub 985)
and listed in `crates/pncad-py/src/prose_census.rs`'s `KNOWN_BRACED`,
whose own header says every entry there is an undischarged defect.
What LIB-EDITS adds is that this entry is not a degraded message: it
BLOCKS a binding door, which is why `DocEdit.set_expression` was not
shipped with `set_param` and `rebind`. The `KNOWN_BRACED` note says
so at the site.

Nothing in the bindings can repair it: the message is the kernel's own
`Display` and a boundary that composed its own prose for one arm would
be inventing what the kernel owns.

## What closing it looks like

The prose fix lands where the arm is (`ExprPath`'s three parts read as
a sentence — the node, the slot's word, and the child indices), the
`KNOWN_BRACED` row goes with it, and then the door is mechanical: one
`DocEdit.set_expression(node, slot, path, expr)` taking the three
attributes `EditError` already reads back, its `path_off_tree` and
`dimension` rows, and the `DocEdit::SetExpression` census row leaving
`MEMBERS_NOT_BOUND` under rule 1's namesake spelling.

## Unparked (2026-09-16, EDIT orchestrator)

The trigger fired: `debug-in-prose-residue-after-finding-sink` closed
with PR #2719, whose measurement says this row's site — `EditError`'s
`{path:?}` over `ExprPath` — needs a prose spelling for an expression
path MINTED, not a rendering re-pointed, and that door is LIB's. So
nothing on EDIT's slate gates this row any more; it is open work here.
Header edited from outside LIB's fence only to keep the tracker true
(`work/README.md`: a fired trigger is not a blocker).

## The prose half landed (CHROME concision fix pass, PR #3108, 2026-09-23)

`EditError::PathOffTree` now reads "the expression path [0, 3] in node
5's distance slot runs off the tree" (`crates/editor-core/src/edit.rs`,
the node, the slot's `label()` and the child indices, no `Debug`), and
its `KNOWN_BRACED` entry in `crates/pncad-py/src/prose_census.rs` is
struck. The feature tree's shape guard
(`viewer/tests/refusal_concision_edits.rs`, via
`test_utils::refusal::debug_struct`) found it independently. What
remains here is the door itself: `DocEdit.set_expression` and its rows.

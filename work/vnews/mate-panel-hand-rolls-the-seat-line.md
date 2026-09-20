---
id: mate-panel-hand-rolls-the-seat-line
kind: issue
title: The mate panel composes its own seat line, with the list mark and the empty-state sentence spelled a second time
status: open
opened: 2026-09-19
---



Found by the sweep of
`seat-line-spells-the-list-mark-as-a-literal`, whose class was *a site
that spells a mark `frame` already names as a constant*. That row's own
site is fixed; this is the only other member in `crates/viewer/src/`.

## The site

`pane::create`'s mate-tool panel (`crates/viewer/src/pane/create.rs`,
the `MateToolState` match under `ToolKind::Mate.says(...)`) renders the
held picks itself, in three arms:

    MateToolState::Idle => ui.weak("no picks yet"),
    MateToolState::One(a) => ui.weak(format!("pick a: face of node {}", a.node.0)),
    MateToolState::Two { a, b } =>
        ui.weak(format!("pick a: node {}; pick b: node {}", a.node.0, b.node.0)),

**Two things are spelled a second time there**, and both have one home
already in `seats`:

- the **list mark**. `"pick a: node …; pick b: node …"` is the items of
  a list one line carries, joined with a literal `"; "` — which is
  `frame::LIST_SEPARATOR`'s value. This is exactly the two-copies-drift
  the sibling row is about: a change to `LIST_SEPARATOR` now moves the
  withdrawal join and `seat_line` and leaves this one behind.
- the **empty-state sentence**. `"no picks yet"` is `seat_line`'s
  own early return, word for word, in a second file.

## Why it was filed rather than swept in

The sibling row is one constant substituted for its own value and has
no wrong answer. This site does: substituting `LIST_SEPARATOR` into the
format string is not obviously the right fix, because the line should
plausibly not be composed here at all. `seats`' module doc names the
mate tool as **the deliberate exception** — its picks are an
interchangeable pair and are faces rather than nodes, so it shares
neither the state nor the pick rule — and that argument is about the
STATE, not about the sentence. The panel's line has `seat_line`'s
shape: an empty-state sentence, one `role: value` item per pick, joined
on the list mark.

So the fix has to decide between giving the line the constant (cheap,
and leaves two compositions), routing the mate panel through
`seat_line` or a composer beside it (which needs `seat_line` to take
something other than `(Seat, Option<RecipeNodeId>)`, since these are
faces), and lifting just the empty-state sentence and the mark.

**A third inconsistency is visible at the same three lines and is not
the same defect**: the one-pick arm says *"pick a: face of node N"* and
the two-pick arm says *"pick a: node N"*, so the same pick is described
two ways depending on how many there are. Whichever fix lands should
settle that word too.

## Home

VNEWS's: `crates/viewer/src/pane/create.rs`. The file is a double
claim (also `chrome`, `view`, `vseam`).

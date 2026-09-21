---
id: mate-panel-hand-rolls-the-seat-line
kind: issue
title: The mate panel composes its own seat line, with the list mark and the empty-state sentence spelled a second time
status: open
opened: 2026-09-19
priority: P3
cost: E
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

## The noun for a pick drifts, and it has two members

**Not the same defect as the duplication, and it outlives it**: the
word for what a seat holds is spelled three ways across what the docs
call one vocabulary. The fix that lands should settle the word, not
just the composition.

- **In this panel**, the one-pick arm says *"pick a: **face of node**
  N"* and the two-pick arm says *"pick a: **node** N"*, so the same
  pick is described two ways depending on how many there are.
- **Inside `seats.rs` itself**, which the module doc calls one
  vocabulary composed in one place so the two cannot drift:
  `seat_line` renders a held pick as *"{role}: **feature** N"* while
  `SeatEvent::PickLost`'s `Display` — three hundred lines up, and the
  sentence `seat_line`'s own doc names as the reason the line is
  composed here — says *"the {role} pick (**node** N) is no longer in
  the document"*. Same value, same module, same `RecipeNodeId`, two
  nouns.

The second is the stronger evidence: the file that exists to stop this
drift has it internally, so a rule stated in a doc comment is doing
the work a shared composer should. Population re-derived at
2026-09-20 and stated as of then; a third spelling may have landed
since.

## Home

VNEWS's: `crates/viewer/src/pane/create.rs`, and
`crates/viewer/src/seats.rs` for the noun-drift half.
`pane/create.rs` is a double claim (also `chrome`, `view`, `vseam`).

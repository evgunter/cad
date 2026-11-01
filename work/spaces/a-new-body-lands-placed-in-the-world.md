---
id: a-new-body-lands-placed-in-the-world
kind: issue
title: a new body lands placed in the world rather than unplaced
status: open
opened: 2026-10-31
priority: P0
cost: H
refs: [6209]
parent: a-new-body-lands-unplaced
---


## Finding

Every creation gesture inserts its body together with a `Place` of it
into the world with the identity bundle (`editor_core::stand`), as the
viewer README states ("**"Place part" writes the identity bundle.**",
in *The mate tool commits into a placement's bundle*), through
`place_in_world` (`crates/viewer/src/session.rs`). Reproduced with
`examples/drive.rs`: Add feature → rectangle → Add profile → select it
→ Extrude adds `Extrude 2` **and** `Place 07a90beb9a38`, six `0` value
rows, the body "in the product".

So a new body is never unplaced, and free-move, which G3 defines over
the shown spaces of an assembly "before a mate exists"
(`crates/viewer/GUI-DESIGN.md`, G3; IDENTITY DI5), has nothing to move:
the probe refuses a copy in the world (`AdmissionFault::InTheWorld`,
`crates/viewer/src/display.rs`).

## Ruling

Ev (in chat, 2026-10-31): **a new body lands unplaced** — in a space of
its own, which the picture shows as a shown space (DI5), not placed into
the world. The kernel model already reads this way: in
`docs/DESIGN.md` `[placement.world]`'s example, `base = extrude(profile,
t)` lives in `base.space`, and placing it in the world is an explicit
`place … {target world; …}`.

The README paragraph quoted above is ratified design text and changes
with this row; the ruling is Ev's, so the README edit cites it.

## Also needed

The free-move probe's door is `display::instance_check`, which admits
`Node::InstantiatePart` only (its comment: "a new kind has none until G3
gives it some"). G3 and DI5 key free-move on the shown space, so the
probe has to be offered for any copy whose space is shown, not only an
instance's. Without that, a body that lands unplaced still cannot be
free-moved.

## Also found

A new body's automatic world placement is not added to the product,
so every creation immediately raises `check unused: Place <tag>: used
by nothing: not the product, not an assertion, and nothing reads it`
(reproduced with `examples/drive.rs`: Add profile → Extrude, and the
toolbar reads `checks: 1 finding(s)`). Whatever an unplaced landing
does about the product, a fresh body should not open with a finding.

## Two copies of one part land on each other

Second walk (2026-10-31; Ev: "put this in along with actual handling for free-move"). Two **Add part…** of the same file give two instances placed identically, one inside the other, indistinguishable in the viewport until a slide value is edited. Whatever lands a new body unplaced and free-movable should also keep a second copy from landing exactly on the first.

## A duplicate or a projected copy gets two placements

Third walk (2026-11-01; Ev: add here unless covered). The duplicate tool's copy is a `Place "Part 1"`, and the automatic world placement then places that Place again, so the tree shows the copy with twelve free values: its own six and the automatic six. The projection tool's copy does the same. Landing bodies unplaced removes the automatic half; the row's fix should check these two gestures.

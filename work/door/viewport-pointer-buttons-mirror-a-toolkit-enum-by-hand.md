---
id: viewport-pointer-buttons-mirror-a-toolkit-enum-by-hand
kind: issue
title: The viewport maps egui's three pointer buttons to the viewer's three by hand, and nothing forces either side
status: open
opened: 2026-09-11
---



Filed by the `BooleanOp::ALL` unit. The finding is not new — it is the
fourth instance recorded in the body of
`hand-maintained-mirrors-of-a-kernel-enum-are-unforced`, which that
item declined to file separately because it WAS the file. That item
closes with this unit, so the instance needs its own.

## The finding

`crates/viewer/src/pane/viewport.rs:85-87` maps `egui::PointerButton`'s
three buttons to `crate::input::PointerButton`'s three
(`crates/viewer/src/input.rs:63`), by hand, in production:

    (egui::PointerButton::Primary, PointerButton::Primary),
    (egui::PointerButton::Secondary, PointerButton::Secondary),
    (egui::PointerButton::Middle, PointerButton::Middle),

Both sides are complete at three today and nothing forces either. A
fourth button on either side is silently never produced: no compile
error, no red row.

## Why it is the class's hardest instance

It is the same class as the closed item — a complete list of one
enum's variants, written where no compiler can read it against the
declaration — with the mirrored enum in the TOOLKIT rather than in a
crate this repo owns. The fix that closed the boolean case (ask the
declaring crate to publish its own `ALL`) is unavailable in the
`egui` direction, so this is the instance that most likely wants the
"a row rather than a mechanism" answer: a test that matches
exhaustively over `input::PointerButton` and asserts each arm is
produced by some `egui::PointerButton`, red on the day either side
grows.

The list is not a `const` item, so
`scripts/gates/viewer-vocab-declared-once.sh` does not see it — that
blind spot is stated in the gate's own header ("A LIST THAT IS NOT A
`const` OR A `static` ITEM").

## The mirror is not unforced — it is already WRONG (2026-09-12)

Corrected by the DOOR orchestrator, from Ev's question on PR #2410
("why is there a hand mirrored enum there"). The row was filed as an
instance of "a vocabulary mirrored where no compiler sees the mirror",
which is the wrong classification twice over.

**Why the viewer has its own `PointerButton`, and why that is right.**
`crates/viewer/src/input.rs` is toolkit-free BY DESIGN: it imports only
`crate::camera`, and its module doc states the rule — *"Module kind:
**vocabulary** — it names no driver type and no `app`-only crate."* That
is what lets `InputMap` and the bindings (`orbit_button`,
`alt_orbit_button`, `pan_button`, `select_button`) exist without an
`egui` dependency. `pane/viewport.rs:84-88` is the adapter that
translates at the edge. Keeping the toolkit's types out of the
vocabulary layer is the same instinct as the kernel/chrome split, and
the mirror's EXISTENCE is not the defect.

**The defect is that the adapter is already stale.**
`egui::PointerButton` has **five** variants —
`Primary`, `Secondary`, `Middle`, `Extra1`, `Extra2`, with
`pub const NUM_POINTER_BUTTONS: usize = 5`
(`egui-0.36.1/src/data/input/pointer_button.rs:4-23`). The viewport's
loop polls three. A drag or a click on a mouse side button therefore
produces **no `ViewportEvent` at all**, silently — and the viewer's own
`PointerButton` has no variant that could name one, so no binding could
reach them even if the adapter tried.

So this row is not "nothing forces it, and it could go stale". It went
stale and nothing reported it.

**And it is a fourth kind, not one of `crates/viewer/README.md`'s
three.** Completeness here is a PRODUCT decision — which buttons the
viewport binds — which is nearer the surviving *"deliberately partial"*
kind than the retired mirror kind. But it is not cleanly that either,
because nothing at the site says three-of-five is deliberate. What it
actually is: **an adapter silently incomplete against its upstream, with
no statement either way.**

**What the row now wants**, in order: decide whether binding the side
buttons is wanted; then either add the two variants and their bindings,
or say at the site that three-of-five is the product decision and why.
Only after that does the question of which README kind covers it have an
answer.

**Consequence for the ruling.** This row was the orchestrator's
counterexample against the paragraph #2387 put in place of the retired
kind. The counterexample was misclassified, so it does not support that
argument; see `readme-ratification-amendments-need-ev`.

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

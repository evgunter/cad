---
id: the-chrome-vocabulary-is-not-held-to-the-target-the-panel-draws
kind: issue
title: Nothing holds a gesture vocabulary to the target its own panel row is drawing
status: open
opened: 2026-09-21
---



Found by `the-two-drags-name-their-gestures-in-two-shapes`'s unit
(2026-09-21), by a mutation its own new rows did not catch.

## What it is

`widgets::value_gesture` and `widgets::free_move_gesture` mint a
gesture's whole vocabulary from one name, so no two operations in it
can disagree. **Which name the panel hands them is still unheld.**
`PropertiesPane::instance_ui` passes the `node` whose probe it is
drawing and `slot_value_ui` passes `row.slot`; nothing asserts that
either is the row under the field.

## The receipt

On the fixed tree, changing `free_move_gesture` to name
`RecipeNodeId(instance.0 + 1)` — so every operation the probe emits
drives the NEXT instance's probe — left the whole viewer suite green
when this row was filed: `cargo test -p viewer --features app --test all
--no-fail-fast` 634 passed, 0 failed, 1 ignored, and `--lib`'s
`widgets::tests` 6 passed (`cargo test … --lib widgets::tests` prints
`6 passed; 111 filtered out`; the 15 that `widgets::` prints is that 6
plus `field_tests`' 9).

The rows that unit added are green under it, and correctly so: they pin
that one control's operations name ONE gesture, and a name shifted
uniformly still satisfies that. What none of them can reach is which
name the panel chose.

The same mutation was available before that unit: the free-move block
of `instance_ui` spelled its target six times at that merge base.

## Why it is not closed

**The type system settles the reachability outright**, which is
stronger than the citation this row first gave for it: `ViewerBehavior`
is `pub(crate)` (`crates/viewer/src/app.rs:1929`) and `instance_ui` is
`pub(crate)` (`crates/viewer/src/pane/properties.rs:388`), so a test in
`crates/viewer/tests/` — a separate crate — cannot name either, at that
base or at any head. No test file there can be the one that closes
this.

`panel_display.rs` says the same thing in prose, at
`a_parameter_field_is_written_the_way_its_declaration_says`: *"the
panel's `DragValue` lives inside a private `ViewerBehavior` method over
an `egui::Ui`, and this crate carries no headless egui harness"*.
`widgets.rs` has a harness (`widgets::tests::Probe`) and it builds its
own vocabulary rather than the panel's, so it cannot answer for the
panel either.

Closing this means a headless harness over `ViewerBehavior`'s panel
methods, sited inside `crates/viewer/src/` where `pub(crate)` reaches —
a facility, not a row — and `pane/properties.rs` is ground VNEWS also
works.

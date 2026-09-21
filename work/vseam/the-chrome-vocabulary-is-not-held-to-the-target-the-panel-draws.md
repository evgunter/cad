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
gesture's four operations from one name, so the four cannot disagree
with each other. **Which name the panel hands them is still unheld.**
`PropertiesPane::instance_ui` passes the `node` whose probe it is
drawing and `slot_value_ui` passes `row.slot`; nothing asserts that
either is the row under the field.

## The receipt

On the fixed tree, changing `free_move_gesture` to name
`RecipeNodeId(instance.0 + 1)` — so every operation the probe emits
drives the NEXT instance's probe — leaves the whole viewer suite green:
`cargo test -p viewer --features app --test all --no-fail-fast` is
634 passed, 0 failed, 1 ignored, and `--lib`'s `widgets::tests` rows
are 15 passed. The four new rows this unit added are green too, and
correctly so: they pin that one drag's operations name ONE gesture,
which a uniformly shifted name still satisfies.

The same mutation was available before this unit — `properties.rs`
spelled `instance: node` four times inline — so this is the hole the
unit narrowed rather than one it opened.

## Why it was not closed here

Nothing in `crates/viewer/tests/` reaches the panel. `panel_display.rs`
says so at `a_parameter_field_is_written_the_way_its_declaration_says`:
*"the panel's `DragValue` lives inside a private `ViewerBehavior`
method over an `egui::Ui`, and this crate carries no headless egui
harness"*. `widgets.rs` has one (`widgets::tests::Probe`) and it builds
its own vocabulary rather than the panel's, so it cannot answer for the
panel either. Closing this means a headless harness over
`ViewerBehavior`'s panel methods — which is a facility, not a row —
and `pane/properties.rs` is ground VNEWS also works.

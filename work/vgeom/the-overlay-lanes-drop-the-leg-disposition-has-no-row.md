---
id: the-overlay-lanes-drop-the-leg-disposition-has-no-row
kind: issue
title: The overlay lanes' drop-the-leg disposition is asserted nowhere and its public door needs a window
status: closed
opened: 2026-09-21
priority: P3
cost: D
closed: 2026-09-22
branch: vgeom/seam-refusals
pr: 3065
---



Filed by `vgeom/f32-seam`, from its own mutation receipts.

## The measurement

Two of the display seam's three dispositions are held by a row and one
is held by nothing. Planted on the unit's own branch, each mutation
run as `cargo nextest run -p viewer --features app --no-fail-fast`
with the three slowest pick rows excluded (803 of 806 rows):

| mutation | rows red, besides the standing Vulkan one |
|---|---|
| `Narrow for f64` stops refusing | 4 (`narrowing::tests`) |
| `SceneMesh::build` falls back to `[0.0; 3]` | 1 (`scene::tests::a_corner_past_the_display_seam_refuses_the_whole_scene`) |
| `Camera::view_projection_f32` stops refusing | 1 (`camera_ops::the_narrowed_view_projection_refuses_what_a_gpu_cannot_hold`) |
| `marks::segments_of` **and** `pane::viewport::push_segment` keep a leg whose end does not narrow | **0** |

So the drop-the-leg rule is documented at both sites and asserted at
neither, and a later unit can delete it and stay green.

## Why the unit did not write the row

Because the rule that says how to write it forbids the cheap version.
`work/view/plan.md`'s rule register (deleted 2026-09-21 by Ev's ruling;
recoverable at `66d7357417`): *drive a guard through the PUBLIC
door* — a row against a private helper proves the arithmetic and
nothing about reachability, and reachability is the whole question
here. `marks::segments_of` is private and its public door
(`marks::edge_id_segments`) takes a built `PickIndex`, so the row
needs a body tessellated past `f32::MAX`;
`pane::viewport::push_segment` is private in an `app`-gated DRIVER and
its public door is `viewport_ui`, which takes an `egui::Ui` and ends
in a wgpu paint callback.

The honest options, for whoever takes this:

1. Build the `PickIndex` over a far-out body and assert
   `edge_id_segments` returns the legs it can draw and not the ones it
   cannot. Covers `marks` through its public door; says nothing about
   `viewport`.
2. Lift the pair-narrowing out of `push_segment` into a value
   `viewport_ui` merely appends, so the rule has a headless door of
   its own. That is a shape change and wants an argument.
3. Decide the drop is not worth holding and delete the documentation
   that says it happens. That register has the
   precedent both ways (same register, same sha): *before writing an
   item's suggested row, mutate the code the row would guard and read
   which rows red* — here
   nothing does, which is the argument FOR a row rather than against
   one.

Read with `the-display-seams-refusal-is-drawn-and-never-said`, which
is the other half: this row is about the rule not being held, that one
about the refusal not being heard.

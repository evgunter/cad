---
id: the-viewer-composes-a-copy-on-the-cpu-not-the-gpu
kind: issue
title: The viewer composes a copy's frame into its mesh on the CPU at scene build, where §6 asked for a GPU multiply
status: open
opened: 2026-10-27
---


Found by INTENT stage 3 E (deviation 2 of PR 5811). §6
(`docs/INTENT-STAGE3-SPEC.md`) says "The viewer multiplies each shown
body by its frame on the GPU, so a value edit moves a frame, not a
mesh." E applies the frame on the CPU when the scene is built
(`crates/viewer/src/scene.rs` `ScenePart::placed`, applied to the
positions in `f64` beneath the free-move probe before the `f32` cast;
`crates/viewer/src/pickindex.rs` `drawn`). "A value edit moves a frame,
not a mesh" holds at the pick memo: a copy whose values moved is served
the same tessellation `Arc` under its new frame
(`a_value_edit_moves_a_copys_frame_and_not_its_mesh`). The vertex
buffer is still rebuilt.

Done when the frame reaches the draw as a per-part transform (a uniform
or an instance attribute) and the positions are uploaded once per
tessellation, or §6 is restated to the CPU composition with its reason
(one `f64` rounding beneath the probe, the same as an undisplaced part
takes).

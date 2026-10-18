---
id: a-cap-normal-reads-the-height-s-width
kind: issue
title: A cap's normal reads with an enclosure twice the height's width
status: open
opened: 2026-10-17
---


## Finding

In `docm2_part_interval::a7_the_width_ladder_of_the_split_of_a_widened_box`,
the corpus `part_select` document places its lifted pattern copy with
the `place_at` gesture: a plane mate reading the copy's cap
(`crates/editor-core/src/eval/wire/pose.rs` `read_off`, the face
carrier's stored plane) against a world plane. With the height `H`
widened by `w` (an `Interval` run), the copy's swung `x`
(`crates/editor-core/src/mate/value.rs` `Chart::of`, `copy_x`, read off
the cap's normal) carries an enclosure about `2w` wide, though no
rotation in the recipe reads `H`: an extrude's cap normal is the sweep
direction. The solve levers that tilt over the copy's reach (≈5.1), the
solved map's translation is ≈`12w` wide, and certifying the moved edges
escalates (`carrier_endpoint_start` / `_end`) through ε/64, where the
same document's split and union certify from ε/32.

Measured at ε/10 (w = 1e-10): rotation entries off the diagonal 2.0e-10
wide, translation 1.2e-9 wide; the target chart is exact.

## What would close it

The cap's carrier normal at `Interval` is the sweep direction exactly
(point intervals) wherever the sweep direction does not read a widened
parameter — or the plane read takes the normal from the carrier's
definition rather than from a frame built through the height. Then the
ladder's placement rungs go green again: re-baseline
`docm2_part_interval`'s ladder and `A7_DIVISOR` to the measured floor.

**No longer exercised by the ladder** (PR 5267, after the orchestrator's interim). The corpus places its copies at the identity by the named interim's world pin, which reads no cap, so `docm2_part_interval`'s ladder and `A7_DIVISOR` are main's again (ε/32 green, 128). The finding stands for any reader of an extrude cap's carrier normal at `Interval`, among them `place_at` at a pose off the identity (`a-pin-read-off-a-copys-geometry-carries-its-width-into-the-copy`). A row pinning it directly is what this item still needs.

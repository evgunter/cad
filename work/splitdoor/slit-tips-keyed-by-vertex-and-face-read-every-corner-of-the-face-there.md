---
id: slit-tips-keyed-by-vertex-and-face-read-every-corner-of-the-face-there
kind: issue
title: SlitTips is keyed by (vertex, face): a cusp between two other edges of the face at a slit's tip would read as the full turn
status: open
opened: 2026-10-29
priority: P3
cost: M
refs: [a-slit-tip-on-a-curved-face-or-at-a-cone-apex-reads-its-full-turn-as-a-spike]
---

The residue of
`a-slit-tip-on-a-curved-face-or-at-a-cone-apex-reads-its-full-turn-as-a-spike`
(branch `splitdoor/slit-tip-readers`), which threaded the tips to
`vertex_section` and the split's apex arm and left the keying as it was.

## What

`sector_shape::SlitTips` holds `(vertex, face)` pairs, so at a slit's
tip every corner of `face` there reads `CornerBounds::SlitTip`. Both
readers take the full turn only where the corner's bounds read along
one ray (`corner_shape`: the straight rung rejected positive;
`apex_corner_shape`: an apex window of no width), so a corner between
edges apart reads as it would anywhere
(`sector_shape::tests::an_apex_window_of_no_width_is_a_slit_tips_whole_turn`
pins that at an apex). The one corner still misread is a cusp: two
other edges of `face` leaving the tip tangent, which reads as the full
turn rather than refusing as a spike. Unmeasured: no witness is known
(a cusp between tangent curves at a slit's far end in the same face).

## Why it is not keyed by edge

The Boolean reads its tips on the operand at rest and its sectors on
the carved body (`boolean::mod`, `SlitTips::of` before the reduction;
`sectors::build_sectors` after it). The reduction may split a slit's
two edges, so the edges at the tip in the carved body need not carry
the at-rest keys. A corner key that survives the reduction has to
follow the split pieces back to the slit's edges; nothing records that
lineage today.

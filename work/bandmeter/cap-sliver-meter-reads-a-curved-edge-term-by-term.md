---
id: cap-sliver-meter-reads-a-curved-edge-term-by-term
kind: issue
title: blend: the cut-off's cap meter reads a circle or ellipse edge one enclosure face at a time, and an ellipse section only through its minor disc point by point
status: open
opened: 2026-10-07
priority: P3
cost: M
---


## Finding

`CapSliver::clearance` (`crates/sweep/src/blend/open/end_face.rs`)
reads a STRAIGHT end-face edge point by point
(`CapSliver::line_clearance`: the least over the segment of the largest
enclosure term, at closed-form candidates), so a line that misses the
enclosure `Ω` by leaving it through different faces carves. Two
residues of that change:

1. **A circle or ellipse edge is still read term by term**: each of
   reach, the section, and each floor must clear the WHOLE edge on its
   own, so a bore ring that is partly inside the band's section and
   partly short of a floor refuses `RingClearance` though it misses
   `Ω`. Point by point on a circle, an affine term against a radial
   one is a quartic in the half-angle, which is why the line case did
   not take it. Measured on the D-rod at `ROD_FILLET` (a grid of 1371
   bores clear of the sliver by ≥ 0.003, centres over
   `[0.1, 0.3] × [0.25, 0.45]`, radii 0.01–0.08): none refuses
   `RingClearance`, so no fixture witnesses this today. The ruled
   band's other fixtures (keyhole, oblique cap) were not gridded.
2. **On an elliptic section the pointwise read uses the disc of the
   minor semi-axis** in place of the ellipse (the ellipse against the
   reach circle would be a quartic too). The whole-edge ellipse term
   is kept beside it, so the meter never loses ground, but a line
   leaving `Ω` through the ellipse's far reach and a floor reads
   not-clear.

Both are in the conservative direction (refuse, never a silent pass).

## Close

A witness fixture first (an oblique-cap rod with a bore or a slot
spanning the section and a floor); then the quartic crossings in
closed form, or a sound split of the arc at the section's nearest
point, with the row flipped to a carve at the closed form.

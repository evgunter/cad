---
id: rim-wedge-decides-bare-cosines-on-the-metre-band
kind: issue
title: rim_wedge decides three bare cosines against the metre band (seam_rim_traversal, seam_line_traversal, seam_rim_axis_sense), and the dimension audit has no row for them
status: open
opened: 2026-10-06
priority: P3
cost: E
---


Found by the `shell/planar-gate-misses` lane's sweep for bare-cosine
margins (the shape of `shell-walls-antiparallel-decides-a-cosine-on-the-metre-band`):
`Margin::of(… .dot(…))` and a cosine bound to a local first. Not
measured on a fixture.

`crates/topo/src/boolean/rim_wedge.rs`:

- the traversal read (near the `let cos = along.dot(tangent) / …` line)
  decides `seam_rim_traversal` / `seam_line_traversal` on
  `Margin::of(cos)`, a cosine of two normalized directions;
- `curve_sense` decides `seam_rim_axis_sense` on
  `Margin::of(axis.dot(rim.axis))`, a cosine of two unit axes.

Both are dimensionless against the metre band, so their Zero window is
an angle of `ε` radians at every model size. The sense read is about
`±1` on every reachable rim, so the band is unlikely to bite there; the
traversal read is a sign of a cosine that is zero when the walk runs
perpendicular to the locus. `docs/predicate-dimension-audit.md` has no
row for any of the three. The fix is a lever (the rim radius, the seam's
extent) or a row saying why a cosine is the right comparand.

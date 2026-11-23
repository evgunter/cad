---
id: the-r3-box-sweeps-split-the-widest-axis-whatever-the-implicit-forms-read
kind: issue
title: The SSI box sweeps split a cell's widest axis, whatever the excluding forms depend on
status: open
opened: 2026-11-22
---


Found by BOXFINISH's sweep for the shape its split unit fixed in `editor_core::drive` (`work/boxfinish/a-drive-splits-a-box-along-a-variable-its-undecided-node-does-not-read.md`): a refinement step chosen without asking what the undecided thing depends on. **Read, not measured**: the choice rule below is quoted from the code; that splits are wasted is an argument from what the test reads, and no cell count was taken. (BOXFINISH split-reads lane)

- **`ssi::enclose::Box3::split`** (used by `ssi::exhaust`'s `sweep_r3`, `seed_r3`, `account_r3`, and by
  `ssi::one_arc`'s `face_walk` on the two free axes of a box face): the widest raw axis, ties x, y, z.
  The cell is excluded when either surface's implicit form excludes zero over it. A cylinder's form
  does not read its axial coordinate, a plane's reads one direction: for two parallel-axis cylinders a
  split along the axis changes neither enclosure, and the floor is the cell's widest side, so that
  axis is refined to the floor before the sweep may refuse.
- **`ssi::exhaust`'s `SweepCell for UvRect::split`** (`seed_chart_partner`, `account_chart_partner`,
  `sweep_chart_regular`, and `pair_section::spline_implicit::sweep`): the larger raw parameter width,
  tie to u, with no chart speed (`PairCell::split` beside it weighs width by speed). The regular test
  is "a slope along u or along v is off zero", and the split is not told which was close.
- **`SweepCell for PairCell::split`** (`four::heaviest`) and **`ssi::four`'s `face_walk`**
  (`PairBoxes::widest`): metric-widest of four (three) coordinates; which chart's enclosure blocks the
  exclusion is not asked. Weaker: both charts enter the test.
- **`ssi::pair`'s `boundary_pass` corner rungs**: all four coordinates shrink together.

What a dependency-aware choice looks like in the same crate: `pair_section::spline_implicit`'s
`pad_of` (tries both cuts, keeps the better), `ssi::certify`'s `Hull::to_halve` (halves only the
spans that are not cleared).

---
id: the-spline-face-and-carrier-walks-cut-the-widest-side-whatever-holds-the-box-open
kind: issue
title: The curve-on-face and carrier walks cut a box's widest side, whatever keeps the box undecided
status: open
opened: 2026-11-22
---


Found by BOXFINISH's sweep for the shape its split unit fixed in `editor_core::drive` (`work/boxfinish/a-drive-splits-a-box-along-a-variable-its-undecided-node-does-not-read.md`): a refinement step chosen without asking what the undecided thing depends on. **Read, not measured**: the choice rule below is quoted from the code; that splits are wasted is an argument from what the test reads, and no cell count was taken. (BOXFINISH split-reads lane)

- **`curve_face::spline_face`'s walk** (`pair.widest(x)`, `Cell::halves(k)`) over a curve × chart box
  `(t, u, v)`: the widest side in metres. A box touching a crease (`cut_lines.touch`) is cut on to the
  escalation length along whatever side is widest, and only a cut of the creased coordinate separates
  it from the crease.
- **`topo::spline_region`'s `carrier_walk`** (`Patch::quartered`): every open cell is quartered, both
  directions, whichever direction its tests (`cell.floor`, `place`, the centre distance) are open in.

---
id: the-blend-reach-bounds-split-a-cells-longest-side-in-three-dimensions
kind: issue
title: The blend reach bounds split a 3-D cell's longest side, though a circular reach does not read the azimuth
status: open
opened: 2026-11-22
---


Found by BOXFINISH's sweep for the shape its split unit fixed in `editor_core::drive` (`work/boxfinish/a-drive-splits-a-box-along-a-variable-its-undecided-node-does-not-read.md`): a refinement step chosen without asking what the undecided thing depends on. **Read, not measured**: the choice rule below is quoted from the code; that splits are wasted is an argument from what the test reads, and no cell count was taken. (BOXFINISH split-reads lane)

- **`sweep::blend::reach`'s `face_bound` and `inside_bound`** (`halves(lo, hi)`): the longest of x, y,
  z. The bounds are distances to lines and circles. `face_bound_in_sheet` in the same file already
  drops the azimuth for a circular reach and splits only r or z; the 3-D path has no such reduction.

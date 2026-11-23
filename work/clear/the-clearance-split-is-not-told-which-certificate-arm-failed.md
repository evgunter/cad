---
id: the-clearance-split-is-not-told-which-certificate-arm-failed
kind: issue
title: The clearance sweep picks the cell and the axis to halve without the certificate arm that failed
status: open
opened: 2026-11-22
---


Found by BOXFINISH's sweep for the shape its split unit fixed in `editor_core::drive` (`work/boxfinish/a-drive-splits-a-box-along-a-variable-its-undecided-node-does-not-read.md`): a refinement step chosen without asking what the undecided thing depends on. **Read, not measured**: the choice rule below is quoted from the code; that splits are wasted is an argument from what the test reads, and no cell count was taken. (BOXFINISH split-reads lane)

- **`topo::clearance`'s `split_by` / `split_axis(cell, root, axes)` / `split_self`**: the cell of the
  pair with the larger 3-D enclosure (tie to a), then `Axes::Relative` (width against the root window,
  tie to u) or `Axes::Metric` (metres at the centre, relative where the chart's speed collapses 8×).
  On the touching lane `Outcome::Split { arm, shared }` names the certificate arm that failed, and
  `split_by` is not given it; on the distance lane neither choice asks which direction moves the
  separation's lower bound.

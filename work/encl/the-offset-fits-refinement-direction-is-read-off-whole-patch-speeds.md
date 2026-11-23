---
id: the-offset-fits-refinement-direction-is-read-off-whole-patch-speeds
kind: issue
title: The offset fit picks u or v for a failing cell from whole-patch speeds, not from the cell's error
status: open
opened: 2026-11-22
---


Found by BOXFINISH's sweep for the shape its split unit fixed in `editor_core::drive` (`work/boxfinish/a-drive-splits-a-box-along-a-variable-its-undecided-node-does-not-read.md`): a refinement step chosen without asking what the undecided thing depends on. **Read, not measured**: the choice rule below is quoted from the code; that splits are wasted is an argument from what the test reads, and no cell count was taken. (BOXFINISH split-reads lane)

- **`offset_fit`'s `refine_rounds_at` / `refine_schedule` / `directional_mark`**: the cells refined are
  the failing ones (attributed), and the direction is the larger `h·sup‖S_d‖` at whole-patch speeds,
  both directions on an idle round. The code's own comment calls the speed ratio a prediction; the
  error can be anisotropic for curvature the extent does not show.

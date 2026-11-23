---
id: nurbs-iso-derive-spells-its-iso-line-column-three-times-and-names-its-probe-by-caller
kind: issue
title: nurbs_iso_derive spells an IsoLine column image three times and names its one probe predicate by the arm that calls it
status: open
opened: 2026-10-14
priority: P3
cost: E
---


Found by the review of PR 5042 (`pcert/wall-seam-column-pick`, its S3,
S9 and S10). Style only: nothing here changes a verdict.

## What is there

In `crates/topo/src/pcurves.rs`, `nurbs_iso_derive`:
- **One image, three spellings.** A column image `IsoLine { p0: (x,
  v_at(t0) − slope·t0), pl: (0, slope) }` is built by hand in
  `iso_schedule`'s closed-form columns, again in its measured-map
  columns, and again in the wall-seam arm's `column` closure. The rows
  are spelled twice the same way. A constructor (`Pcurve::column(x,
  v_at_t0, slope, t0)` or a local helper) would leave one spelling.
- **One predicate, three labels.** `probe_pick` decides the same
  comparand at every site: the carrier a quarter span in, against the
  candidate image there. It is labelled by the arm that calls it:
  `pcurve_iso_line_rim`, `pcurve_iso_seam_column`,
  `pcurve_iso_wall_seam`, and `pcurve_iso_arc_row_direction` for the
  arc rim. `docs/predicate-dimension-audit.md` therefore carries one row
  per caller for one decision.
- **The schedule is described several times**: `nurbs_iso_derive`'s doc
  header, the boundary-iso arms' note and `iso_schedule`'s own comments
  each restate it.

## Fix

Give the column and row images one constructor. Decide whether the
probe's label names the decision (one label) or the arm (as now), and
make the audit rows follow. Keep one statement of the schedule, in the
doc header, and point the others at it.

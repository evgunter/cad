---
id: a-retimed-straight-row-in-a-cap-plane-refuses-at-the-section
kind: issue
title: a spline wall's re-timed straight boundary row in a cap plane refuses at the plane x wall section's limb 2
status: open
opened: 2026-11-20
priority: P3
cost: M
---


Found by EXCH unit B's fix pass (PR 7170). The committed M7-8 cube
(`crates/step-import/tests/fixtures/m7-8-cube/m7_8_cube.step`) with its
bowed wall's net transposed (rows for columns, the face's `same_sense`
flipped to `.F.`) and the two straight boundaries' middle controls moved
along their lines to `z = 0.2` (#82 and #88, off the Greville `0.5`):
the straight rims #95 and #101 are now the wall's ROWS in the cap
planes, and the import refuses edge #95 at the plane × wall section,
limb 2 (`7.49e-4` m at a break, "the certificate's own image of the
carrier misses the carrier's foot path there"). The untransposed file
(`tests/fixtures/retimed-rim/side_column.step`) imports, carried on the
wall's column.

The row is a polynomial boundary of the wall, so `build::cap_row` (its
straight arm, `line_on_column`) is tried only where the section does
not build; the section builds and then refuses. Whether a wall's own
boundary row in a cap plane should be read as that row before the
section, as a rational row is (`cap_row`'s `rational_only` pass), is
this row's question.

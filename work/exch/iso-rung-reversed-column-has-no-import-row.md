---
id: iso-rung-reversed-column-has-no-import-row
kind: issue
title: the iso rung now carries an edge on a wall's reversed column (Running::Mirror), and no import row exercises it
status: open
opened: 2026-11-20
priority: P3
cost: E
---


Found by EXCH unit B's fix pass (PR 7170). `build::column_carrier` is
now the one home of "carry the edge on the wall's column, or on the
wall's reversed column (`Running::Mirror`) where the edge runs against
it", shared by the iso rung (`iso_of_wall` via `column_match`) and a
cap's rim (`cap_row`, where the reversed arm came from). The iso rung's
circle and line arms now match a column the edge runs against
(`circle_column_match` about the reversed axis, `line_on_column` with
its ends the other way round), so they reach the reversed arm; the
bitwise spline match is forward by construction.

`circle_column_match`'s reversed reading is pinned at unit level
(`build::tests::a_column_matches_the_printed_circle_at_a_metre_scale_band`),
but no import row carries an edge on a reversed column through the iso
rung. What would close it: a fixture whose printed circle or line runs
against its wall's column on both walls (or beside one), imported, with
the edge's chart image read off the mirrored column.

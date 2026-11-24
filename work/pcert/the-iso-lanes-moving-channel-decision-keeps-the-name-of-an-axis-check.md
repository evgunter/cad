---
id: the-iso-lanes-moving-channel-decision-keeps-the-name-of-an-axis-check
kind: issue
title: The iso lane's moving-channel decision is still named pcurve_iso_axis_u/_v, the name of the held-axis check it no longer makes
status: open
opened: 2026-11-23
priority: P4
cost: C
---


Found by the review of BOXFINISH's iso-image unit (PR 7393, 2026-11-23).

`run_iso_checks` (`crates/geom-brep/src/pcurve_cache.rs`) decides that an
iso line's MOVING channel moves over its span under the names
`pcurve_iso_axis_v` (a column: `line.axis == U`, so `v` moves) and
`pcurve_iso_axis_u` (a row). Before PR 7393 the same two names also
decided that the HELD channel does not move (a slope's exact zero);
that decision is gone, the held axis being the image's `ChartLine`. The
names stayed, so the recourse roster, the thread-count digests and
`docs/predicate-dimension-audit.md` did not move for a rename.

What is left reads oddly: a decision named for an axis check that
checks no axis, and named `_v` on a column. A name that says what it
decides would be the moving channel's extent (`pcurve_iso_moves`, or a
`_u`/`_v` pair of it). A rename moves every digest and census that
keys on a decision's name; it is one mechanical change with its
re-cuts, not part of the representation change.

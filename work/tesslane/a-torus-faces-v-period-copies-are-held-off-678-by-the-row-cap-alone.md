---
id: a-torus-faces-v-period-copies-are-held-off-678-by-the-row-cap-alone
kind: issue
title: A torus face's vertex copies a v period apart are held off #678's one-row fan by the row step cap alone; repeat_columns reads u spans only
status: open
opened: 2026-11-23
priority: P3
cost: M
---


Found by the LATHE lane `lathe/seam-twice-remnant` (PR 7473) and its
review, 2026-11-23.

## What

#678's non-manifold fan needs a mesh id at two UV locations with exactly
one lattice line between its copies. `mesh::trimmed::repeat_columns`
doubles the column count `nu` until no repeated id has exactly one
COLUMN between two of its `u`s. It reads `u` spans only.

A torus face whose loop walks its `v` seam (the minor circle) both ways
holds each vertex on that seam at two `v`s a period apart. Between those
copies stand ROWS (`AnalyticRow::lattice_rows`), which nothing counts.
What keeps one row from standing alone between them is the row step
cap: `AnalyticRow::lattice_steps` caps the torus's `h_v` at `π/4`
(`sizing::cap_angular`), so a whole `v` turn holds at least eight gaps.
That is an argument, not a check, the same shape the cylinder's `nu`
argument was before `repeat_columns` took it over.

The emit pass's debug census (`walk::overused_identified_edge_in`) would
still see the defect's conclusion in a debug build.

## Done when

The rule `repeat_columns` states for columns holds for rows on every
chart with a `v` period: either the row layout is doubled on the same
rule, or the step cap is named at the claim site as the guard and a
unit row pins it against a torus band walking its `v` seam.

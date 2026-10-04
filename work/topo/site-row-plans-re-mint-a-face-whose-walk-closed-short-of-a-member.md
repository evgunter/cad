---
id: site-row-plans-re-mint-a-face-whose-walk-closed-short-of-a-member
kind: issue
title: the site-row plans prove their walk stays in its loop but not that it reaches every member: a walk closed short re-mints the face without that member's row
status: open
opened: 2026-10-04
priority: P4
cost: E
---


## What

Found by the row-walk unit (PR 4016). That unit routed the pcurve
rows walk (`pcurves::loop_rows`) and the site-row walks
(`Body::site_cycle`, `Body::site_cycle_from`,
`crates/topo/src/euler.rs`) through `Body::loop_cycle_of`
(`crates/topo/src/body.rs`), so no walk hands another loop's members
to a plan. The doors that move or re-chart a whole loop also prove the
walk misses no member (`Body::whole_cycle`, `Body::face_cycles`).

The site-row plans of the make and kill operators (`mev_fan_site`,
`mef`'s rows plan, `mekr`'s target side, `kef`'s surviving loop,
`kev_describing`'s `kev_loops_after`) prove only the first half. A
`next` torn to close past a member still claiming the loop hands the
plan a cycle without it. The face is then re-minted with every other
member's row, and the skipped member keeps the row it had before the
re-mint. That row is on the same chart, but it was not certified with
the face's new window. Measured on PR 4016's split sheet
(`row_walk_proofs::sheet`, uncommitted probe): `mev_line` and
`mef_chord` return `Ok` at every site of the wall with its walk closed
past one member, re-minting three of its four rows.

## The shape to give

Either the whole proof (`RunExtent::Whole`), which reads the whole
half-edge arena on every make operator that re-mints, or the O(walk)
predecessor check `Body::orbit_inverts` gives vertex orbits, applied
to the loop (`prev(m)` is the walk's member before `m`), with that
check's stated limit (a paired `next` + `prev` tear). Measure the
cost of each on the sweep corpus before choosing.

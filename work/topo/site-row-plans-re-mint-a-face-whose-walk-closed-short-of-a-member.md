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
(`Body::site_cycle`, `crates/topo/src/euler.rs:4522`;
`Body::site_cycle_from`, `euler.rs:4469`) through `Body::loop_cycle_of`
(`crates/topo/src/body.rs:1457`), so no walk hands another loop's members
to a plan. The doors that move or re-chart a whole loop also prove the
walk misses no member (`Body::whole_cycle`, `euler.rs:4537`; `Body::face_cycles`,
`crates/topo/src/euler_ring.rs:1523`).

The site-row plans of the make and kill operators (`mev_fan_site`,
`euler.rs:2698`;
`mef`'s rows plan, `mekr`'s target side, `kef`'s surviving loop,
`kev_describing`'s `kev_loops_after`,
`crates/topo/src/euler_kill.rs:1199`) prove only the first half. A
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

## The paired tear past the `Whole` proof

The moving doors' `Whole` proof has a limit of its own. Divert the
loop's walk through another loop's member `x` (`next(a) := x`,
`next(x) := next(a)`) and re-point `x.parent_loop` at the loop: every
member the walk reaches claims the loop and no half-edge outside it
does, so `Body::loop_cycle_of` and `Body::whole_cycle` both take `x`
as a member. `kfmrh` then returns `Ok` and drops `x`'s row with the
loop's. Only `x`'s untouched `prev` disagrees. Pinned as it stands by
`row_walk_proofs::a_diversion_paired_with_a_parent_loop_tear_passes_the_proof`
(`crates/topo/src/row_walk_proofs.rs`); stated in `loop_cycle_of`'s doc
(`crates/topo/src/body.rs:1442`). The predecessor check above catches
it too, until the `prev` tear is paired as well.

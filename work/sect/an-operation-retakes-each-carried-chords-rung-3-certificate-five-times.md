---
id: an-operation-retakes-each-carried-chords-rung-3-certificate-five-times
kind: issue
title: An operation re-takes each carried chord's rung-3 certificate five times, and the repeats are a fifth of a ball × cylinder boolean
status: open
opened: 2026-10-30
priority: P3
cost: M
---


Found by SECT's `sect-seamslow` lane (branch `sect/seam-region-speed`,
2026-10-30), profiling the seam-region door rows
(`crates/sweep/tests/the_door_reads_seam_regions_as_tier_3_does.rs`).

## Measured

`unit u rod` there (the unit ball ∪ the upright rod, opt-level 2, main
`25d3906bca` with `sect/seam-region-speed`'s span memo) cuts eight
distinct chord arcs from the carried section. `geom_brep::analytic_rung3`
(`edge_nurbs.rs`) runs 40 times in the operation, each arc five times
on bit-identical input (carrier piece, interval, both surfaces, band):
four in one operand order and one in the other. Sampled with gdb, the
calls sit under the join's edge door (`euler::mef_chords` →
`certify_edge_spec` → `policy_lane::certifying_attach`, for the chord
and for its copy on the partner), the zip (`zip_seam` → `fuse_by_joint`
→ `kev_describing`), the output's description
(`finish_output` → `describe_minted_edges` → `restate_edge`), the graft
(`setopfinish` → `graft_door`) and the result gate
(`validate` → `recertify_stated`).

The first call on an arc costs 0.37 s at the default ε and 1.9 s at
1e-12. Limb 2's round memo (`pcurve_cache::limb2_memo`) serves the
repeats' rounds, and what is left of a repeat is the tube
(`intersect::tube_bounds`, `ssi::certify::certify_branch`'s limb 3) and
limb 2's own decisions over the memoized hull: 0.04 s at the default ε
and 0.24 s at 1e-12. Thirty-two repeats are 1.3 s of a 7 s operation
at the default ε and 7.7 s of a 45 s one at 1e-12.

## What closing it takes

Either the stages after the join read the certificate the join took
(an edge whose carrier and surfaces have not changed since its door),
or the rung-3 verdict is memoized by its whole input. The second skips
decisions on a hit, which limb 2's memo was written not to do ("the
chart's decisions are taken outside the memo, every call"), so it is a
design question for C2's owner before it is a change; the first is the
pipeline's.

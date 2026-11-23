---
id: census-pass-5-reads-two-lines-through-its-own-reader-beside-the-tables-line-line-arm
kind: issue
title: census pass 5 reads two line edges through its own reader beside the curve-pair table's LineLine arm
status: open
opened: 2026-11-22
priority: P1
cost: M
---


Found by the PLANAR weighing (fork log row 299, 2026-11-21), filed by
PROFILE's cut of the plane ruling (PR 7290), on ONECOPY as PROFILE's log
of 2026-11-21 directs. Read, not run.

**Two readings of one question.** Tier 3′'s pass 5
(`crates/topo/src/census.rs` `sweep_edge_edge` → `pair_edge_edge` →
`ee_meeting`, near :5188–5290) decides where two line edges' interiors
meet through its own chain: the parallel screen `parallel_reading`
(:5472), the crossing `crossing_reading` (:5821), the collinear arm
`collinear_overlap` (:6154) and `settle_in_band` (:5440). The curve-pair
table answers the same question in `geom_brep::curve_pair::pose`
`line_line` (`crates/geom-brep/src/curve_pair/pose.rs` :1489: `route`'s
parallel and gap arguments, the tilt, the offset, `one_line`), which pass
5′ already reads for every pair with a curved side
(`census::curved::interiors_meeting` → `meet`). Two line edges in pass 5
and a line against an arc in pass 5′ are therefore read by two different
rules in one census.

**Scope against PLANAR.** PLANAR's
`the-census-reads-one-plane-records-pairs-through-the-plane-instance`
moves pass 5's pairs on one plane record onto the plane instance (C5,
PR 7290). This row is the rest: two line edges on different records,
which are a curve pair for `meet` and should read its `LineLine` arm
rather than pass 5's chain. Whether pass 5's `Meeting::Overlap` margin
and its region-door lane (`ee_crossing_lane`) read anything `meet`'s
answer does not carry is the first thing to check.

**Not measured:** whether the two readings disagree anywhere.

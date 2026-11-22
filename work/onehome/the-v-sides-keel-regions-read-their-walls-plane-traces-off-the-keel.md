---
id: the-v-sides-keel-regions-read-their-walls-plane-traces-off-the-keel
kind: issue
title: Read from a keel's V side, each piece's section boundary is its wall's plane trace, slope × offset off the keel, so at ε 1e-6 the two regions overlap and each reads 1.5 ε over its annulus
status: open
opened: 2026-11-15
priority: P3
cost: E
---

Split off from `a-closed-keel-in-the-section-plane-divides-the-section-into-two-regions`
when that row closed: it is the witness's ε 1e-6 red, which is not the
closed keel's reading.

**What is there.** `crates/sweep/tests/a_curved_keel_in_the_plane.rs`,
`a_round_keel_in_the_plane_reads_the_side_it_is_read_from`. It reads
the round keel's section from the V's side (the plane's normal `−y`),
where that side is two pieces touching along the keel circle and the
section is the two annuli `0.5..2` and `2..3`. At `CAD_TOLERANCE_EPS=1e-6`,
with the V's tip `0.6 ε` off the plane, it reads `the V's tip 0.6 ε off
the plane, the plane's normal -1 y: the section's regions hold
[11.780973958926847, 15.707964775914]` against the annuli
`[11.780972450961723, 15.707963267948966]`. Each region is `1.508e-6`
over its annulus, where the row holds it to `1e-8`. The row is pinned red
there on that message.

**Why, by arithmetic.** The keel is decided on the plane, `0.6 ε` off it.
Each piece's section boundary is its own V wall's trace on the plane, and
the walls run `dr/dy = 0.3/1.5 = 0.2`. So the inner piece's rim lies at
`2 + 0.2 · 0.6 ε` and the outer piece's hole at `2 − 0.2 · 0.6 ε`, and
the two regions overlap by `0.24 ε` radially. Each area moves
`2π · 2 · 0.12 ε = 1.508 ε`, which is the reading.

**Owed.** Say whether a region may stand off the keel by its wall's
`slope × offset` (a sub-ε move in position read as about `2πr` times
that in area), and state the row's bound from it at every ε. Otherwise,
read both pieces' boundaries at the keel the vertex was decided on.
Take the pin off with either.


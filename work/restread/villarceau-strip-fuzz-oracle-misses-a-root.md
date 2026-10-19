---
id: villarceau-strip-fuzz-oracle-misses-a-root
kind: issue
title: The Villarceau strip fuzz's numeric oracle finds no root at seed 0xaaece85fd941a95f
status: closed
opened: 2026-10-19
pr: 5363
closed: 2026-10-19
priority: P1
cost: E
rides_with: main-red-torus-region-probe-fuzz-reads-on-boundary-near-the-tube
---

`topo boolean::torus_region::tests::a_villarceau_strip_reads_its_side_at_random_points`
(`crates/topo/src/boolean/torus_region.rs`) fails at the default ε with
`CAD_FUZZ_SEED=0xaaece85fd941a95f` (effort 1): `oracle (inf, 0.0)`. The
panic is the test's own numeric oracle (`fam`), not the region read: its
20000-step sign scan of `g(φ) = n(φ)·w` found no bracket whose root lies
on a circle, so `best` stays at infinity. A sampled point whose family
parameter sits at a tangential (double) root of `g`, or a root the
`g(a)·g(b) ≤ 0` scan misses between two steps, would do it.

Seen on PR 5323's CI (run 39204266518, job `test`), reproduced locally on
that branch; the PR does not touch `torus_region.rs` or anything its test
reads. The fix is the oracle's (a root search that does not rely on a sign
change, or a sample rejection that is logged), then pin this seed beside it.

## Closed

The seed's point sits near a bitangent tangency: its two plane roots
lie `2.2e-4` apart, inside one `3.1e-4` step of the sign scan. The
family parameter is now in closed form (`villarceau_family`, the clamped
`acos` root whose circle holds the point), and the seed's point is
pinned as `a_bitangent_point_reads_the_strips_side`.

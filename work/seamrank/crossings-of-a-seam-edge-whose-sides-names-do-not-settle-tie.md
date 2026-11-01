---
id: crossings-of-a-seam-edge-whose-sides-names-do-not-settle-tie
kind: issue
title: Crossings of a seam edge whose faces' names do not settle the pair's first side tie, where N2 rules only that an equal pair ties
status: closed
opened: 2026-10-01
priority: P3
cost: M
closed: 2026-10-26
---

## What

A seam edge's crossings are ranked as the loop of the pair's first side
runs along the edge (N2). `emit_topo::crossed_edge_orientation`
(`crates/editor-core/src/names/emit_topo.rs:1895`) finds that side by
matching the edge's two faces' names against the pair
(`seam_pair::a_side_is_first`, line 1924). Where the names do not settle
it — neither face descends from a side, or both faces are merged across
both sides — it answers `None`, and the crossings tie.

N2 rules only that an EQUAL pair ties. The undecided case is a
departure, as `SeamLineSides` (retired with
`seam-line-sides-is-a-missing-rule`) was a refusal for it.

## Fix shape

Record the seam's first side structurally at its mint, in a form the
pass-throughs carry, so the orientation does not depend on names
matching. No fixture reaches it.

## Closed (2026-10-26, tracker sweep)

Superseded. The departure this row records (crossings that tie where
N2 ruled only that an equal pair ties, because the seam's first side
did not settle) no longer departs from anything: FORK-ORIENT (fork-log
row 197; Ev, 2026-10-26; PR 5705, `964083e5`) made N2 read a seam along
each of its sides (`Through { entering }`) and rank nothing, so no
crossing needs a seam's first side and crossings the names do not tell
apart tie by rule. The fix shape here (record the first side at mint)
is no longer wanted. The code still orients by `seam_pair::a_side_is_first`;
that goes with the build of
`work/wire/a-unions-line-of-several-edges-is-oriented-by-its-least-edge.md`.

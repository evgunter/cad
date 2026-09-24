---
id: a-faces-loops-are-walked-by-hand-in-thirty-four-places
kind: issue
title: once(face.outer).chain(face.rings) is written out at every face-loop walk in topo/src — 33 copies at last count, each re-deciding what a non-Cycle boundary means
status: open
opened: 2026-09-14
priority: P1
cost: D
---

Filed by PR 2531's fix pass, on its reviewers' Q1 finding: the unit
closed a duplication by giving the stored-rows chart window one home
(`pcurves.rs`'s `stored_rows`), and the walk UNDER that window is
itself written out everywhere.

**The shape.** `core::iter::once(face.outer).chain(face.rings.iter().copied())`,
then per loop a `LoopBoundary::Cycle { first }` let-else and a
`body.loop_cycle(first)`. Counted by that spelling
(`once(face.outer)`, `once(f.outer)`, `once(face_data.outer)`) over
`crates/topo/src`: **33 sites in 21 files** —

`pcurves.rs` (4), `splitting/finish.rs` (3), `seqgen.rs` (3),
`review_m1_pr4.rs` (3), `boolean/contain.rs` (3), `boolean/join.rs` (2),
and one each in `validate.rs`, `replace_face.rs`, `offset_together.rs`,
`movefac.rs`, `iso.rs`, `euler.rs`, `coherence.rs`, `chart_region.rs`,
`census.rs`, `boolean/rest.rs`, `boolean/rim_wedge.rs`,
`boolean/ops.rs`, `boolean/mod.rs`, `boolean/finish.rs`,
`boolean/boxes.rs`.

**What the pattern cannot match**: a walk that names the two loop
kinds some other way (a `match` over `face.rings` first, a helper that
takes `&[LoopKey]`, a `for lp in loops` over a vector built two
statements earlier), and everything outside `crates/topo/src`. The
count is therefore a floor.

**Why it is a defect and not a style note.** Every copy re-decides
what a non-`Cycle` loop boundary means — skip it, report it, or treat
it as empty — and `scripts/gates/loop-boundary-discards.sh` exists
because those decisions were found to differ and to be unaudited. One
walk with the disposition stated once is what would let that register
shrink instead of grow with each new site.

Not swept by the PR that filed it: its unit's fence is `split_edge`'s
row carry, and a 34-site refactor across the boolean pipeline, the
validator and the census is a unit of its own.

## Re-counted, and the inner half narrowed (2026-09-14, PR 2549)

**The 34 was one too many.** Re-taken by that PR's R1 reviewer and
again by its fix pass with the row's own spelling, on a head with the
filing PR merged: **33 sites in 21 files**, with `boolean/rest.rs`
holding one rather than two. The file list above is corrected. The
floor caveat is unchanged — the spelling still cannot see a walk that
names the two loop kinds another way, and still stops at
`crates/topo/src`.

**The inner half has one home now, for two of the sites.** PR 2549
gave the per-loop part of the walk — the `LoopBoundary::Cycle`
let-else plus `loop_cycle` — one function, `pcurves::loop_rows`, whose
answer is a three-way `LoopRows` (`Cycle`, `NoCycle`, `Corrupt`) so a
caller states its disposition instead of re-deciding it.
`pcurves::stored_rows` and the loop-re-parenting doors' drop
(`Body::drop_rows_on_chart_change`) both call it, and the discard
register carries one `audited` entry there instead of an `unaudited`
one per caller. That does not shrink the count above, which is of the
OUTER `once(outer).chain(rings)` chain; it is the shape the rest of
this row's sweep would take, demonstrated on two callers.

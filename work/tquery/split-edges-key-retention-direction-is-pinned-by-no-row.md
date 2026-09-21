---
id: split-edges-key-retention-direction-is-pinned-by-no-row
kind: issue
title: topo: split_edge's key-retention DIRECTION is pinned by no row
status: open
opened: 2026-09-13
priority: P3
cost: E
---


## Finding

`Body::split_edge` (`crates/topo/src/split.rs`) documents which child
keeps the parent key: *"the parent edge survives as the first child
(`start(he_plus)` → the new vertex …); the new edge is the second child
(new vertex → the parent's old end)"*. Consumers depend on the
DIRECTION of that retention — `sweep::blend`'s band carves name a split
fragment's provenance off it, and BLEND unit 8 exists because one of
them read it the other way round.

Nothing in `crates/topo/tests` pins it. A reviewer read every
`split_edge` call site and every row that touches one: the rows check
the Euler vector, the arena delta, the emanating rules, the degenerate
splice cases, the interiority trilean, the curve minting and kill order,
and the tier-3 caveat — and every one of them is symmetric in which
child got which key. Reverse the retention in `split_edge` (hand the
parent key to the second child and the new key to the first) and no
`topo` row goes red; the failure surfaces two crates away, in
`sweep`'s naming totality, and only on fixtures whose meridian runs the
way that exposes it.

## The shape of the row it wants

Split one edge whose `he_plus` start and end are distinguishable, then
assert, off the keys alone: the PARENT key's `he_plus` still starts at
the original start; `created.new_edge`'s `he_plus` starts at the new
vertex and ends at the original end; and the parent key is NOT the piece
touching the original end. `sweep`'s
`review_ladder_split_key_r2_probes::r2_the_split_rule_is_measured_by_splitting_not_read_off_prose`
measures the same fact from outside, on real bodies of both
orientations, and is the nearest thing the tree has — but it is a
consumer's row in another crate, which is exactly the wrong home for
`topo`'s own direction convention.

## Provenance

Found by the v6 dual review of BLEND unit 8 (PR 2505), which re-derived
the rule from `split.rs` rather than trusting the unit's census.

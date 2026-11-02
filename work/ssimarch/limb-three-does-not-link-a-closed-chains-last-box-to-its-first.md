---
id: limb-three-does-not-link-a-closed-chains-last-box-to-its-first
kind: issue
title: Limb 3 does not link a closed chain's last box to its first; the marched winding presumes they share a point
status: open
opened: 2026-10-31
priority: P3
cost: C
---


Filed by SECT's `sect-onesix` lane (PR 6153), from its confirmation review.

## Measured

`one_arc_r3` (`crates/geom-brep/src/ssi/one_arc.rs`) checks that the
pieces of consecutive boxes `k − 1` and `k` share a root, for
`k = 1 … n − 1`. It does not link box `n − 1` back to box `0`. For a
closed carrier, `r3_reaches_end` then puts each end piece within ε of
the start point, but nothing proves the two end pieces share a point.

`pair_section::winds` (`crates/geom-brep/src/pair_section/winds.rs`,
module docs, "The premise at the seam") rests on that link. So do its
homotopy from the walk to the carrier, `once_round`'s wrap adjacency
and `gated`'s `(i + n − 1) % n` neighbour. The premise is stated
there, and nothing checks it.

## What closing it takes

For a closed carrier, run the same shared-root test between the last
box and the first in `one_arc_r3`, and refuse `Unlinked` where it
fails. Then measure what it moves: every closed ℝ³ loop is
re-certified through it.

---
id: euler-site-mint-re-walks-the-rewired-loop-on-every-op
kind: issue
title: The Euler operators' site mint re-walks and re-certifies every rewired loop on every op, so N ops on one minted face cost O(N²)
status: open
opened: 2026-09-29
priority: P3
cost: M
---

Found by the dual review of PR 3160
(`half-edge-minting-euler-ops-leave-a-minted-curved-face-incomplete`),
measured by both reviewers and re-measured by its fix pass.

`mev`, `mef` and `mekr` re-mint a complete face they add half-edges to
before they mutate (`crates/topo/src/pcurves.rs`, `site_rows`, planned
by `Body::plan_site_rows` in `crates/topo/src/euler.rs`). Per op that
costs one closed-form derivation, branch pin and certification per
half-edge of every loop the op REWIRES, plus one presence read per
half-edge of the face's other loops (`site_rows_from`, through
`stored_rows`). The rewired loop is re-walked whole, not just its two
new halves, because `mef` and `mekr` re-anchor the loop's `first` at the
new half and, on a loop that wraps a periodic chart, that moves the
joint where the walk parks its one-period offset — re-walking is what
keeps the rows byte for byte the pass's.

So a pipeline that runs N operators on ONE minted face whose loop grows
with each op costs O(N²) walk-and-certify steps where the merge base
cost O(N) (the base minted nothing and left the face half-minted).

**Measured** (debug build, one box shared with two other lanes, so
read the shape, not the seconds), on `topo::test_support::cyl_wall_sheet`:

| N | struts on the outer loop, `n` rim splits first | rings: split, two struts, `kemr` per step, before the kept-ring cut | the same, after the cut |
|---|---|---|---|
| 25 | 0.034 s | 0.046 s | 0.035 s |
| 50 | 0.102 s | 0.157 s | 0.137 s |
| 100 | 0.471 s | 0.585 s | 0.414 s |
| 200 | 1.263 s | 3.201 s | 1.523 s |

The first column is every strut on the outer loop, so the cut does not
reach it. The review measured the ring bench at 4.3× the merge base at
N = 200. The fix pass stopped re-walking the loops an op does not touch
(kept rings keep their rows), which halved the ring bench; what remains
is the rewired loop itself, which grows by one half-edge per rim split.
One review measured the fillet suite at +11% on the review head; the
other found no suite delta.

**Shapes a fix could take.** (a) Mint only the two new halves where the
splice leaves `first` where it was and the loop does not wrap the chart
(`mev` at a `Fan` site does both), pinning them against their stored
neighbours' exits; the pass's byte-equality then needs an argument per
chart that the neighbours' pins do not move. (b) Carry the offset the
pass would park on a re-anchored wrapping loop instead of re-walking
it. The production caller that runs operators on minted faces today is
the fillet surgery (`crates/sweep/src/blend/surgery.rs`), with a handful
of operators per face, which is where the one measured suite delta
came from; a producer that grows one minted face by many operators is
where the quadratic would bind.

---
id: the-spiric-enclosing-net-on-a-long-oval-halves-to-clear-limb-2
kind: issue
title: The spiric's enclosing net on a long oval: limb 2 clears only after up to five halving rounds as the plane nears the inner equator
status: open
opened: 2026-10-25
priority: P2
cost: E
parent: closed-form-edge-carriers-pass-on-samples-with-no-stated-bound-between-them
---

Filed by the unit's implementer lane (2026-10-25), as the unit asked: the
designers were unsure whether `carrier_net`'s Taylor enclosure of a
`Curve3::Spiric` (`crates/geom-brep/src/carrier_net.rs`, degree
`ORDER = 6` per span) is tight enough for limb 2 on a long oval.

Measured on branch `offset/limb2-for-every-carrier` (release, ε = 1e-9,
torus `R = 0.75`, `r = 0.3`, axis-parallel plane at stand-off `d`; first
net cut into spans of at most a thirty-second of a turn,
`pcurve_cache::closed_form_grid`), limb 2 against the torus:

| d | span | rounds | first bound | final bound | time |
|---|---|---|---|---|---|
| 0.1 | full oval | 2 | 1.3e-8 | 6.7e-10 | 17 ms |
| 0.4 | full oval | 3 | 7.9e-7 | 5.8e-10 | 36 ms |
| 0.44 | full oval | 3 | 1.5e-4 | 6.6e-10 | 39 ms |
| 0.449 | full oval | 5 | 9.3e7 | 6.2e-10 | 73 ms |
| 0.449 | inner half `[π/2, 3π/2]` | 5 | 9.3e7 | 3.3e-10 | 37 ms |
| 0.449 | outer half `[−π/2, π/2]` | 2 | 1.3e-8 | 6.2e-10 | 8 ms |

So the net certifies every oval up to `d = 0.449` against `R − r = 0.45`
inside limb 2's budget (`SSI_HULL_WORK`): the first round's bound is
useless near the inner pinch, where the root's argument
`(R + r cos t)² − d²` nears zero and the Taylor remainder over a span
grows, and halving recovers it. No corpus spiric was found refusing; the
corpus was not run end to end on the branch (see the sibling cost row).
What is open: whether a first grid graded by the root's argument (finer
where `(R + r cos t)² − d²` is small) should replace the uniform one, and
where between `0.449` and `0.45` the budget runs out.

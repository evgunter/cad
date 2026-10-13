---
id: rejoining-a-tangent-seam-cut-refuses-the-union
kind: issue
title: the union of the two pieces of a cut across a tangent seam refuses at tilted poses, large scales and eps 1e-12
status: open
opened: 2026-10-12
priority: P2
cost: M
---


## Finding

Found by the review probes of the TANG PR that routed `Body::split_edge`'s
tangent children through the must-carry rule (branch
`tang/split-edge-review-probes`, `crates/sweep/tests/tang_split_review_probes.rs`,
`rejoin`). The orchestrator's review measured it on main `06bd6180`;
the payloads below are re-measured on the PR head.

Fixture: `wedge_end_doors.rs`'s shoulder scaled by `s` (height `s`),
its `y < s` piece (seam `TangentIntersection` along the cylinder's
ruling), posed by a rigid map, cut by `z = s − d` or `z = d` with
`d = c·√(2εs)`. `topo::union` of the two pieces, which should give back
the piece at volume `(2 + π/4)·s³`, refuses.

**On main too.** The rows at `c ≥ 1.01·√K` (4, 8, 1.01·√K) cut on main,
both pieces storing the intrinsic tangency, and refuse the same way.
The zero-side rows (c = 0.25, 0.99) cut only with the TANG fix, and add
nothing new: the same four refusals.

| ε | s | pose | refusal |
|---|---|---|---|
| 1e-9 | 1 | offset∘tilted, normal −z | `ResultVolumeImplausible { "vol(A ∪ B) ≤ vol(A) + vol(B)" }`, over by 2e-12 … 6e-11 |
| 1e-9 | 1e3 | offset∘tilted | `Euler(RebasedCarrier { ResidualExceeded { EndpointStart } })`, 3e-8 … 7.5e-7 |
| 1e-9 | 1e3 | offset∘tilted, c = 4 and 8 | `Escalated { Coincidence(Sectors, Moot) }`, margin ±2.6e-9 … 2.9e-9 |
| 1e-6 | 1, 1e3 | offset∘tilted, normal −z | `ResultVolumeImplausible` |
| 1e-12 | 1 | offset∘tilted, and identity with the profile turned π/2 | `Escalated { Coincidence(Sectors, Moot) }`, margin ±2e-12 … 1e-11 |
| 1e-12 | 1, 1e-3 | offset∘tilted, identity turned π/2 and π | `Merge(Pcurve { Certify { ResidualExceeded { MapResidual / EnvelopeTerm(Centre) } } })` |

The poses: `offset` translates by `(3, −7, 11)·s`; `tilted` turns 1.1 rad
about `(1, 2, 3)` through `(−0.2, 0.1, 0.4)·s`. The identity pose at
`s = 1` rejoins at every row at ε 1e-9.

The pieces are tier-3 valid and carry the closed-form volumes; the
union's own lanes refuse. Which lane owns each refusal (the rebase
gate, the sector coincidence, the merge's pcurve certify, the volume
backstop) is not measured, and the four may share a cause: they
appear only once the coplanar cut faces are glued back across a
tilted frame.

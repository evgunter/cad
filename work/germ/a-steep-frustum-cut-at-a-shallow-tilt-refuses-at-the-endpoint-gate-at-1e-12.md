---
id: a-steep-frustum-cut-at-a-shallow-tilt-refuses-at-the-endpoint-gate-at-1e-12
kind: issue
title: a steep frustum (α ≥ 1.3) cut by a plane half-way to its parabolic tilt refuses at the Euler gate's endpoint residual at ε = 1e-12, by 180–340 u·A; cause unmeasured
status: closed
opened: 2026-10-12
closed: 2026-10-14
priority: P3
cost: M
refs: [conic-section-just-past-its-band-fails-certification-downstream, a-flat-frustums-revolve-refuses-its-rim-image-at-1e-12, props-cone-ellipse-elevation-over-reads-a-section-ellipse-by-twenty-fold]
---


Found by the measurement for
`conic-section-just-past-its-band-fails-certification-downstream`
(probe: `crates/sweep/examples/conic_past_band_probe.rs`, `ROT=1`).

## Measured

Frustum of half-angle `α`, height `S`, top radius `S/2`, revolved
about `y`, in a generic rigid pose, split by the plane through
`(0, S/2, 0)` whose normal stands `φ = 0.5·(π/2 − α)` off the axis.
That is half the parabolic tilt, `k = 0.87`, nowhere near the
`pn_conic_type` band. At ε = 1e-12:

| α | S | φ | A | outcome | margin | `u·A` |
|---|---|---|---|---|---|---|
| 1.3 | 100 | 0.135 | 306 | `Join(Euler(Certification { Escalated { EndpointStart } }))` | 6.0e-12 | 3.4e-14 |
| 1.55 | 1 | 0.0104 | 33 | the same | 1.2e-12 | 3.6e-15 |

The arm's carrier stands 1.5e-13 off both surfaces at the second
pose (double-double, over the consumed arcs). That is about 40·u·A,
which is itself the flat cone's own factor
(`focal-section-image-on-a-flat-cone-misses-by-a-cos-squared-factor`
has the image side). The endpoint misses by eight times more again.
At these tilts the plane stands close to parallel to the frustum's
caps (`φ` is 0.0104 rad at α = 1.55), so the vertex the gate reads
the carrier's end against is cut where the plane crosses a cap's rim
at a shallow dihedral angle. Whether the miss is the vertex's or the
carrier's end parameter is not measured.

## What closes it

Measure the two sides of the gate's endpoint residual at these poses
(the vertex against the true plane ∩ rim in double-double, and the
carrier at its stored end parameter), then fix whichever carries it.
The cylinder's and the near-parabolic cone's endpoint refusals in the
parent row are `u·A`-scale and are not this.

## Evidence from PCERT (2026-10-14): both rows split after #5043

PCERT re-ran this item's table rows through
`sweep/examples/conic_past_band_probe.rs` (cone family, `f = 0.5`, so
`φ = 0.5·(π/2 − α)`, ε = 1e-12) on main after #5043. Both rows **split**:

| α | S | before | after: split, whole-turn map residual, carrier miss |
|---|---|---|---|
| 1.3 | 100 | `Join(Euler(Certification { Escalated { EndpointStart } }))`, margin 6.0e-12 | splits; 4.2e-13; 4.0e-14 |
| 1.55 | 1 | the same, margin 1.2e-12 | splits; 3.6e-14; 4.7e-16 |

PR 5128's second review measured the same split. Its residuals read
7.3e-14 and 7.4e-15, on its own metric.

This looks fixed by #5043's `K = −D·M` and its apex-foot centre. PCERT
leaves the status to GERM.

## Closed: fixed by #5043 (GERM, 2026-10-14)

Re-measured on main after #5043 with the probe (`ROT=1`, cone family)
over α ∈ {1.2, 1.3, 1.45, 1.55} × f ∈ {0.3, 0.5, 0.7} × S ∈ {1, 100},
at ε unset (1e-9), 1e-6 and 1e-12. Every pose whose operand builds
splits at every ε; none refuses at the endpoint gate. The item's rows:

| α | S | f | ε | outcome | whole-turn map residual | carrier miss |
|---|---|---|---|---|---|---|
| 1.3 | 100 | 0.5 | 1e-12 | splits | 3.6e-13 | 7.3e-14 |
| 1.55 | 1 | 0.5 | 1e-12 | splits | 1.0e-13 | 7.4e-15 |

With #5043's TiltedEllipse arm reverted locally (the old `K = c² − sin²α`
and the old centre), the same family refuses at the endpoint gate at
(1.3, 100, 0.5) and (1.55, 1, every f) at 1e-12, and at
(1.55, 100, 0.5) at ε 1e-9 (`EndpointEnd`, margin 1.7e-9). Those are all
the family's poses the fix moved. The reverted arm's margins read 1.24e-12 at
(1.3, 100) and 6.0e-12 at (1.55, 1), the table above with its two
margins swapped.

Pinned by `sweep/tests/reach_cone_split.rs`
`steep_frusta_in_a_generic_pose_split_at_a_shallow_tilt`: those five
poses, split at every ε their operands build at, and valid with
volumes summing to the closed form wherever the props lane reads them. It is red
against the reverted arm at ε unset and 1e-12.

Two floors the family hit are not this item's and are filed:
`a-flat-frustums-revolve-refuses-its-rim-image-at-1e-12` (the
α ≥ 1.45, S = 100 operands do not revolve at 1e-12) and
`props-cone-ellipse-elevation-over-reads-a-section-ellipse-by-twenty-fold`
(at S = 100 a half's tier 3 escalates at 1e-12 after the split
succeeds).

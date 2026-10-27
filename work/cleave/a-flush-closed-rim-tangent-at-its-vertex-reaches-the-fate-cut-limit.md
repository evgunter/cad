---
id: a-flush-closed-rim-tangent-at-its-vertex-reaches-the-fate-cut-limit
kind: issue
title: A closed rim lying flush in a partner face, tangent inside its boundary at a vertex, reaches the fate cut's ClassificationInvariant (no corner of the dropped face)
status: closed
opened: 2026-10-27
priority: P1
cost: M
closed: 2026-10-27
pr: 5852
---

Found by PR 5808's re-review (probe `rev5808r`, scenes S7 and S10), on
`main` and the PR alike.

## The poses

- **S10**: `A` the one-segment cylinder `r = 0.3` about the origin,
  `z ∈ [0, 1]`; `B` a one-segment cylinder `r_B = 0.5` about
  `(0.3 − r_B, 0)`, so `A` lies inside `B`, its wall tangent to `B`'s
  along the ruling through `(0.3, 0)` and its top rim lying flush in
  `B`'s top, tangent inside `B`'s rim at `(0.3, 0, 1)`. With `A`'s vertex
  at azimuth 1 and `B`'s at 0, `A ∪ B` refuses
  `ClassificationInvariant { "a fate cut's end lies in no corner of the
  dropped face" }` for `B` over `z ∈ [0, 1]` and over `z ∈ [−0.5, 1]`,
  at rest and turned. The same holds for `r_B` within `1e-4` of `0.3`.
- **S7**: two upright one-segment rods, a hole's rod inside the boss's
  tangent at azimuth π, `z0 = 0`: `B ∪ A` refuses the same, at rest and
  turned.

The other operand order builds `SOUND`, and so does the two-arc form,
so the true body is within reach. Counted over the probe: 26 cells at
the default ε, 14 at 1e-6, 32 at 1e-12 on `main`.

## Where

`boolean/fate.rs` `place` (via `places` and `corner_holds`): the kept
rim leaves the cut's end along the dropped face's boundary edge at
first order (tangent to it), so the tangent reading finds only
`Place::Along`, which the end's far vertex does not continue, and the
secant reading finds no corner either. `Limit::NoCorner` is a placement
limit of the cut, but the section path renders it as
`ClassificationInvariant` (`impl From<CutError> for BooleanError`), the
voice of a kernel bug. It needs the second-order reading `runs_into`
now uses (`sectors.rs`, the bound's bend against the edge's), or a typed
refusal for the limit.

PR 5808 keeps a closed rim flush in such a face out of the join
(`boolean/join.rs` `stands_on`), so the one-segment forms with a vertex
at the touch refuse typed before reaching this, as they did on `main`.

## Closed (PR 5852, 2026-10-27)

Measured, the S10 cells had a different cause from the one read above.
At the cut's end, `B`'s dropped top already held the join's copies of
`A`'s rim, and the end's corner was bounded by `B`'s closed rim:
`fate.rs` `leaving` levered a bound at its chord, which a closed edge's
one vertex makes zero, so every corner it bounds read `Zero` on every
sign and the copy the kept rim runs along was never read `Along`. A
bound is now levered at its extent (`geom_brep::edge_extent`, the
diameter for a closed circle), as the sector reader's bounds are.

The two-arc `A ∩ B` with `B` over `z ∈ [0, 0.5]` (the floors flush) and
S7's `B ∪ A` were the cause read above: the kept rim leaves the end
along the dropped face's boundary edge at first order. `corner_holds`
now reads a kept edge leaving along a bound one order down,
`(κ − κ_b)·û` at the shorter reach (`bool_fate_corner_bend`, metres,
`runs_into`'s reading): into the corner past the bound, out of it, or
along it where the bends tie. The secant reading stays for a bound with
no certified bend (a spline, a spiric).

The cells listed above build `SOUND` at the closed form in one shell,
point-sampled, at rest and turned, with `r_B` at 0.5 and from `1e-6` to
`1e-4` over 0.3; nearer ties refuse typed, as on `main` (in band, or
`FallbackExtentUnsupported`, or the volume backstop's
`ResultVolumeImplausible`). The same lever moves the
containment fallback's holed-plate cut
(`work/flush/a-fallback-cut-refuses-beside-a-ring-a-seam-or-a-pinch.md`).
`A ∪ B` and `B ∩ A` with `A`'s vertex at the touch still refuse at the
join (`stands_on`); lifted, they do not build, for a reason of the
join's own: `a-flush-closed-rim-at-its-vertex-is-joined-reversed`.

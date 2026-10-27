---
id: a-flush-closed-rim-tangent-at-its-vertex-reaches-the-fate-cut-limit
kind: issue
title: A closed rim lying flush in a partner face, tangent inside its boundary at a vertex, reaches the fate cut's ClassificationInvariant (no corner of the dropped face)
status: dispatched
opened: 2026-10-27
priority: P1
cost: M
branch: cleave/tangent-crescent-builds
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

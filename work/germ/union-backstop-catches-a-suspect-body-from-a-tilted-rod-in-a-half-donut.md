---
id: union-backstop-catches-a-suspect-body-from-a-tilted-rod-in-a-half-donut
kind: issue
title: A tilted rod entering a half donut's cap and poking an oval out of the inner equator reaches the volume backstop on a tier-valid planar body: the pipeline's upstream result is suspect
status: closed
branch: germ/tilted-rod-remeasure
pr: 3428
opened: 2026-09-28
priority: P1
cost: M
refs: [union-with-a-tilted-cylinder-boss-refuses-as-classification-invariant]
closed: 2026-09-29
---


## What

Found by PR 3336's review (2026-09-28). A tilted rod enters the half donut's
planar cap and pokes an oval out of the inner equator; no events lie on the
oval. `union` refuses at `ClassificationInvariant "volume backstop: mass
properties refused on a tier-valid planar body"`, with the interior-loop
guard on, off or mutated. The backstop catches it, so no wrong answer
reaches a caller. But the body the pipeline built before the backstop is
suspect, and the backstop is the last line. Measure what the pipeline built,
and whether it is the interior-loop class meeting the backstop first.

## Home

GERM: the torus boolean lane.

## Closed (re-measured on `4882ec4c9`, 2026-09-29)

**No op returns a body, and the backstop was never the last line.** The
section certificate refuses the pair on reach; it was decided before
the join but raised after the volume backstop, so the backstop's
refusal reported first. It is now raised after `gate` and before
`volume_backstop` (`ops.rs` `boolean_op_recut`, and the rest door's
`rest.rs` `try_rest_union`), so every op names the pair.

The fixture: the `r = 0.15` rod, its axis from `(1.8, 0, 0)` along
`(−sin β, 0, −cos β)` over `t ∈ [−0.3, 1.4]`, spun about its own axis,
against the half donut. ∪, both ∩ and both ∖:

| β | spin | stop, every op |
|---|---|---|
| 0.5 | π/2 | certificate, `Err(Reach)` on every torus × rod-wall pair (was: the backstop) |
| 0.3, 0.4 | 0, π/2 | the same |
| 0.5 | 0, 0.5, 1.0, 2.5 | the reduction, `CurvedSectorSideUnsupported` |
| 0.6, 0.7 | 0, π/2 | the reduction, `CurvedSectorSideUnsupported` |

The certificate's pairs at the reaching poses: torus × the rod's end
cap `Essential(F)` twice, the donut's cap × rod wall `Essential(G)`,
torus × rod wall `Err(Reach)`. The backstop's cause (a logging patch, reverted):
`mass_properties_closed_form` on the RESULT refuses `NotIsoRectangle
"cylinder boundary carries an ellipse arc"` on all five ops. The body
itself was not re-measured: letting it past the backstop was not run.

**The backstop's cause is not this pair's.** A box `[0.5, 3] × [−1, 1]
× [−2, 0]` against the same rod, where every pair certifies, refuses the
same `ClassificationInvariant` on all four ops, at both spins. That is
REACH's P0 `union-with-a-tilted-cylinder-boss-refuses-as-classification-invariant`,
where the evidence is added.

Pinned by `crates/sweep/tests/germ_tilted_rod.rs`: the reaching poses
refuse every op at `InteriorLoopGuard` with the kinds named and every
torus × wall pair `Err(Reach)`; the other poses refuse in the
reduction; the lens point is in the rod and outside the half donut. Red
against the old raise order, and against `Section::Intractable`
certified `Ok` (both reach the backstop's `ClassificationInvariant`).

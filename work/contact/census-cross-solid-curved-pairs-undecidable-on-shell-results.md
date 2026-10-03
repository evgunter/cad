---
id: census-cross-solid-curved-pairs-undecidable-on-shell-results
kind: issue
title: The census's cross-solid backstop answers CensusUndecidable for curved solids within reach of each other, so 26 multi-solid shell results fail the empty-contact tier 3' they should pass
status: open
opened: 2026-10-03
priority: P2
refs: [boolean-door-tier-3-waits-on-the-description-gap]
---


Found by REACH's census-fold measurement for the finished-body bar
(`analysis/reach-measure/census-fold`, `report.md` §5 class (ii);
the [ev] PR 3870).

## Measured

At ε 1e-9, 26 `shell` results pass tier 3 and fail
`validate_pseudomanifold(&body, &ContactRecords::default(), tol)` with
`CensusUndecidable`: "a curved face of one is within reach of the
other". They come from `sweep::shell7/8/9/10_*`:
- a hollow body shelled again gives nested thin curved solids;
- multi-solid operands give curved solids side by side.

Minimal repro: `census_fold_repro_nested_shell` (in the branch's
`probes/census-fold/repros/`). It revolves a 1×2 cylinder, shells it
at 0.2, then shells that at 0.05. The result has 2 solids, 4 shells and
16 faces. Tier 3 gives `Ok`; empty 3′ gives 33 × `CensusUndecidable`.

Related: the census does not examine same-solid distinct-key curved
pairs at all (C9/C6). So the same geometry passes when the two parts
are shells of one solid and refuses when they are two solids.

## Why it matters now

Ev's ruling on PR 3870 makes a finished body's bar tier 3′ against its
own declared contacts, empty for most bodies. `topo::shell` gates its
result with `validate_geometric` today. It cannot adopt the bar without
refusing these 26 results until the cross-solid backstop can decide
curved × curved and curved × planar pairs: either certify them apart, or
find the coincidence. The lane has to land before or with shell's
adoption of the bar.

## 2026-10-03 — booleans can reach it too (FUSE, PR 3891)

Under Ev's ruling that a solid is one piece of material (PR 3901),
every boolean result is sorted one solid per piece, and pieces that
only touch are distinct solids. So a union whose pieces touch with a
curved face in reach of the other piece lands in this class as well:
the "same geometry passes as shells of one solid" asymmetry above is
gone from the verbs' outputs, which no longer put two pieces under one
solid. Measured on PR 3891's branch with a box declared `Tangent` to a
plate's fillet (two solids, tier 3 green, 3′ `CensusUndecidable` alone);
main has since refused that union (`TangentSlitArmUnbuilt`), so no row
pins it today.

## 2026-10-03 — a row reaches it on main (JOIN-2, PR 3880)

`sweep` `join2_r1_probes::join2_r1_grid` (ignored, release) prints 804
`BAD` union lines on main `82b9ceb2b` and on PR 3880's head alike, with the
same set on both trees: every one is `t2=true t3p=false cert=true
operand=true` with the volume right. They are unions of an axis-aligned box
beside or diagonally off a rounded plate (e.g. `grid r=0.5 x=[-1,0]
y=[-1,0]`, the box clear of the fillet). The result has two solids, and 3′
gives `CensusUndecidable` "a curved face of one is within reach of the
other" between the plate's fillet and the box's faces. At main `66bbdaa6b`
the same battery printed 0 `BAD`, which fits the one-solid-per-piece sort
landing in between. So the grid's assertion (`bad == 0`) stays red until
this lane decides curved × planar pairs across solids.

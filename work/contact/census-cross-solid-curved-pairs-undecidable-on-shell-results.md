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

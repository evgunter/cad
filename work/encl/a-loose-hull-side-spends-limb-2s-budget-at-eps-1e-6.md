---
id: a-loose-hull-side-spends-limb-2s-budget-at-eps-1e-6
kind: issue
title: ssi_limb3_one_arc's loose-hull grid row spends limb 2's subdivision budget at eps 1e-6 on main
status: closed
opened: 2026-10-11
priority: P1
cost: E
refs: [hull-sup-limb-reads-its-bound-as-the-miss-at-the-import-door, a-composite-span-straddling-a-wall-knot-reads-the-neighbours-extension]
branch: encl/loose-hull-budget
pr: 4784
closed: 2026-10-12
---


## Measured

Found by FLUX's `flux/sphere-arm` (PR 4573) running geom-brep's suite on
CI's eps matrix after merging main; reproduced on `origin/main` alone
(main at `781cd435`, a worktree with only geom-brep built):

```text
CAD_TOLERANCE_EPS=1e-6 cargo nextest run -p geom-brep --test all \
  -E 'test(a_side_whose_hull_is_loose_reads_clear_over_the_grid)'
```

fails at `crates/geom-brep/tests/ssi_limb3_one_arc.rs:850`:

```text
P 0.6ε, N 0.2ε, k 10, m 256, ε 1e-6: limb 2's subdivision budget is
spent, and it refuses on its bound:
Err(TubeNotOneArc { rungs: 14, cause: Count { solutions: 0 } })
```

It passes at the default ε and at 1e-12. The row last moved in
`81cb6c34c` (encl: too-close-to-call sentences fold into undecided) and
`444364ddf` (limb 2 subdivides its composite until the bound decides).
Deterministic: same failure on rerun. A PR whose diff seeds geom-brep's
eps rows meets it as red CI that is not its own.

## Root cause

The m = 256 row's expectation held only where the pcurve's values at
limb 2's composite breaks land on the wall's knots. The breaks in `t`
are fixed; the pcurve is the projected foot points, and at ε 1e-6 the
feet sit about 6e-13 off the identity (an O(h·h′) offset that rounds
away at the default ε and at 1e-12), so every break's image in `v`
leaves its knot. `tensor::cells_touched` read a cell by closed
overlap, so on the knots each span ending on a knot was bounded on its
neighbour's polynomial extended across the whole span too, and needed
about twice the halving; off them, the spans inside one wall span
clear at once. So m = 256 (0.6ε/0.2ε) spent the cut cap at the default
ε and cleared at 1e-6 (986 cuts of 1024). The bound is not
undecidable, the idle-round stop does not fire early, and no ε-scaled
floor sits above the band (every row clears at 0.825ε or 0.990ε at every
ε).

Fixed: `cells_touched` reads a cell only where the window meets its
interior (the surface is continuous across a knot), and the rows assert
what holds at every ε: m = 64 passes limb 2 and refuses by its tube;
m = 2048 needs 2016 cuts at any alignment and m = 1024 measured at least
1148, both past the cap, so they refuse on limb 2's bound; m = 256 takes
either refusal. Filed the straddle cost off the knots as
`ssiarith/a-composite-span-straddling-a-wall-knot-reads-the-neighbours-extension`.

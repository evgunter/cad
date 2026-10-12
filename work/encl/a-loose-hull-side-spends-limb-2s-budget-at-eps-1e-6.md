---
id: a-loose-hull-side-spends-limb-2s-budget-at-eps-1e-6
kind: issue
title: ssi_limb3_one_arc's loose-hull grid row spends limb 2's subdivision budget at eps 1e-6 on main
status: dispatched
opened: 2026-10-11
priority: P1
cost: E
refs: [hull-sup-limb-reads-its-bound-as-the-miss-at-the-import-door]
branch: encl/loose-hull-budget
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

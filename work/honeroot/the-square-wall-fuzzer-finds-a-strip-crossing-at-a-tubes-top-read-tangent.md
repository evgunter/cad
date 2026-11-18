---
id: the-square-wall-fuzzer-finds-a-strip-crossing-at-a-tubes-top-read-tangent
kind: issue
title: section_cert's square-wall fuzzer finds a strip crossing at a tube circle's top read Tangent(section_torus_square_wall_fold_half) at the default eps, on seed 0x022535f5aad7ede2
status: open
opened: 2026-11-18
priority: P2
cost: M
---

Found by lane `embed/conic-vertex` (EMBED, 2026-11-18) at the PR 7105
reviewer's prompt, on a checkout of main `d0579ebd` with that PR merged.
The PR touches only `topo::injective` and a sweep test, which this arm
does not read, so the red is main's (a counterexample this seed finds).

`topo boolean::section_cert::square_search::the_square_wall_arm_agrees_with_the_traced_section`
at the default ε, `CAD_FUZZ_EFFORT=1`:

```text
[fuzz] section_cert_square_search: seed=0x022535f5aad7ede2 effort=1
1 mismatches (reproduce with CAD_FUZZ_SEED=0x022535f5aad7ede2 CAD_FUZZ_EFFORT=1):
a strip crossing at a tube circle's top: R 26.66550072636465, r 14.591006396843138,
(e, z0, rc) (27.607069375956343, 13.88583701914881, 1.1763568242880624), tilt 0:
R-tan at Degeneracy { tangent: 0.5030737054907064, top: 2.090087436457111e-9 }:
Tangent("section_torus_square_wall_fold_half")
```

The shape looks like GERMTAIL's
`square-wall-crossing-at-a-tubes-top-refuses-r-order` (the wall crossing
the tube at its top parallel), here read `Tangent` at `top` 2.1e-9 from
the degeneracy. Not traced: whether the fuzzer should classify this
case as that row's known refusal, or the arm misreads it. Pin the seed
beside whichever fix.

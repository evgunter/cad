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

Moved from HONEROOT to GERM on 2026-11-20, by `git mv`, when HONEROOT merged into GERM: HONEROOT held no P0 or P1 row, so no sitting would dispatch it on its own (`work/README.md`, Track size), and its charter is one GERM carries.

**More seeds (2026-11-20, ONEROUND's `oneround/lever-doors` lane).** On a clean worktree of `origin/main`, `CAD_FUZZ_SEED=0xcd26a5f82fb7b695 CAD_FUZZ_EFFORT=1` gives 2 mismatches, both `Tangent("section_torus_square_wall_fold_half")`. On that lane's branch (which does not touch this arm) seeds `0x1` and `0x2` fail and `0x3` passes; those three were not replayed on main. So the counterexample is not rare: three of the four seeds tried find it, and the row's fuzzer, which varies its seed, is red on most runs of the slow set.

**Another seed (2026-11-20, ONEROUND's `oneround/lever-sites-topo` lane).** `CAD_FUZZ_SEED=0x7268e17b993b86d7 CAD_FUZZ_EFFORT=1` gives 2 mismatches, both on that lane's base (`oneround/lever-doors` merged with `origin/main` at `3c300524`) and on its head; the lane does not touch this arm.

**Another seed (2026-11-21, GERM's `germ/crossing-consumers` lane, U3b).** `CAD_FUZZ_SEED=0x5d5157bb6763023d CAD_FUZZ_EFFORT=1` gives 1 mismatch on the lane's base, `germ/crossing-pose-cross` at `4303b022e8` (the lane's diff stashed): a strip crossing at a tube circle's top, `R 0.0173, r 0.00329, (e, z0, rc) (-0.01474, -0.000586, 0.00372)`, tilt 1.8e-14, `R-tan at Degeneracy { tangent: 1.57e-4, top: 1.04e-9 }: Tangent("section_torus_square_wall_fold_half")`.

## A second seed, red on main (SECTCONE, 2026-11-22)

PR 7354's CI hit this row at seed `0x505cee7548d8cc42` (`CAD_FUZZ_EFFORT=1`), run 45030556881, five mismatches of this class. Replayed on `origin/main` at `bed968e40c` with `CAD_TOLERANCE_EPS=1e-12`, it is red there too, with two mismatches:

- `R 0.021898704449694648, r 0.018203982349515643, (e, z0, rc) (-0.06811568974321162, -0.021356084056413805, 0.06083591524589301), tilt 1.598e-16`: R-tan at `Degeneracy { tangent: 0.008280659414815934, top: 1.1733392035750967e-12 }`, `Tangent("section_torus_square_wall_fold_half")`;
- `R 0.8637907423392572, r 0.18053832653197224, (e, z0, rc) (-0.7884287255576242, 0.19244301189147156, 0.07629649472069315), tilt 5.373e-14`: R-tan at `Degeneracy { tangent: 0.050161778987343975, top: 1.5452916724001398e-12 }`, the same predicate.

2026-11-23 — HELDRECORDS `nary-lineage` lane (PR 7516, head fe80aa31ec): red locally at `CAD_FUZZ_SEED=0x2f68e664d3d325c9 CAD_FUZZ_EFFORT=1`, 2 mismatches, both "a strip crossing at a tube circle's top … Tangent("section_torus_square_wall_fold_half")" (R 15.451, r 3.026; R 0.946, r 0.717).

## 2026-11-22 — a second seed (PR 7395's slow-set run)

The same shape on seed `0xcfa453b4ae9f0a91` (`CAD_FUZZ_EFFORT=1`, default
ε), in CI on PR 7395, which does not touch the square-wall arm:

```text
a strip crossing at a tube circle's top: R 10.682974235606407, r 3.026841427789205,
(e, z0, rc) (-10.56320449591688, 3.1214447215157057, 0.15262559901396516),
tilt 0.00000000000000641608758379722: R-tan at Degeneracy { tangent: 0.0557253697477349,
top: 8.348044200356952e-10 }: Tangent("section_torus_square_wall_fold_half")
```

And a third, seed `0xe6f724928c971c91`, on PR 7395's next run: three
mismatches of the same shape, `top` 3.28e-9 and 3.38e-9, read
`Tangent("section_torus_square_wall_fold_half")`. The fuzzer is red on
main's nightly too (`work/green/nightly-1115-residue-the-reds-no-lane-holds.md`,
seed `0x96ec838c8a0d073e` at 1e-12).

**Another seed (2026-11-22, HELDRECORDS' `heldrecords/nesting` lane, whose diff this arm does not read).** `CAD_FUZZ_SEED=0x127a80287c19c581 CAD_FUZZ_EFFORT=1` at the default ε gives 2 mismatches of this class: `R 38.834, r 26.194, (e, z0, rc) (35.239, 30.287, 5.4475)`, tilt 1.2e-14, `Degeneracy { tangent: 1.142, top: 3.74e-9 }`; and `R 0.36623, r 0.32703, (e, z0, rc) (0.39027, 0.26587, 0.065705)`, tilt 8.0e-14, `Degeneracy { tangent: 0.00564, top: 5.69e-10 }`; both `Tangent("section_torus_square_wall_fold_half")`.

**Another seed, red on main (2026-11-23, ONEROUND's `oneround/conic-readers` lane).** `CAD_FUZZ_SEED=0x28ab5920d3c632ea CAD_FUZZ_EFFORT=1` at the default eps gives 1 mismatch on a clean worktree of `origin/main` at `2b9cd8b24`, and the same one on the lane's branch (which changes no formula this arm reads): a strip crossing at a tube circle's top, `R 3.7722725905900796, r 1.0650391083001807, (e, z0, rc) (-4.8298983194402485, 3.5969163843253504, 2.743897725051316)`, tilt 0, `R-tan at Degeneracy { tangent: 0.059752967741008245, top: 4.306466294679012e-10 }: Tangent("section_torus_square_wall_fold_half")`.

Another, seed `0x6007c6a9f3e13c4d` (ORBIT's `orbit/torus-touch-build`,
2026-11-23, at the default eps; the square-wall arm is untouched there):
one mismatch of the same shape, `R 1.210, r 0.846`, `top` 2.98e-9, read
`Tangent("section_torus_square_wall_fold_half")`.

Another seed, from CI's slow set on lane `orbit-tilt`'s PR 7617
(`CAD_FUZZ_SEED=0x71703484f672850c`, `CAD_FUZZ_EFFORT=1`, CI run
45347387574, 2026-11-24): two mismatches of the same shape, both read
`Tangent("section_torus_square_wall_fold_half")`. The first is at
`R 13.60, r 11.48`, tilt 0, `top` 3.79e-9. The second is at
`R 0.0239, r 0.0201`, tilt 4.7e-12, `top` 1.57e-9. The PR leaves the
square-wall arm untouched. The seed reproduces both mismatches on
main `341d62e9a6` (a local replay).

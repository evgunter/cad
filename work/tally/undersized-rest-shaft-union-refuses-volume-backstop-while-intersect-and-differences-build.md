---
id: undersized-rest-shaft-union-refuses-volume-backstop-while-intersect-and-differences-build
kind: issue
title: A shaft declared Rest 1e-10 under its bore: the union refuses ResultVolumeImplausible while ∩ and ∖ build, so the four ops disagree on whether it is a mate
status: open
opened: 2026-10-03
priority: P2
cost: M
---


Found by PR 3980's dual review (`analysis/reach-dual/3980-r2`, NOTE-4);
re-measured on `reach/rest-mate-intersect-diff`. Older than that PR:
the union's refusal is main's.

## Measured

`mate2_common::collar_of(0.5, 1.5, 0, 1, 1)` (bore 0.5) against
`peg_of(0.5 + dr, 0, 0.5, 2)`, bore × shaft walls declared `Rest`
(9 pairs), at ε 1e-9:

- `dr = −1e-10`: `∪` refuses
  `ResultVolumeImplausible { which: "vol(A ∪ B) ≤ vol(A) + vol(B)", got: 7.853981633660325, bound: 7.853981633346163 }`;
  `∩` is `Empty`; `collar ∖ shaft` = 6.283185307179585 (2π);
  `shaft ∖ collar` = 1.5707963261665776 (`π(0.5 − 1e-10)²·2`).
- `dr = +1e-10`: all four build (∪ 7.853981634288644).

The declaration door bridges the 1e-10 radius difference (in band),
so all four ops read the pair as a mate. The union's body measures
3.1e-10 (`2π·0.5·1e-10` over the bore's length 1) above the sum of
the operands' own volumes, and the volume backstop (`ops.rs`
`volume_backstop`) refuses the excess. ∩ and ∖ keep or drop whole
shells and never meet the backstop. One mate, four ops, two answers to
"is it a mate"; the reviewer measured the same at scale 1e-3 with
`dr = −1e-8` and `−1e-10`.

Whether the backstop's bound should carry the bridged displacement
(the declared pair's band, times its area) or the zip should build to
the smaller operand is the open question.

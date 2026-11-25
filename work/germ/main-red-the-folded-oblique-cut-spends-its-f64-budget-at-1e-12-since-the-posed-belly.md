---
id: main-red-the-folded-oblique-cut-spends-its-f64-budget-at-1e-12-since-the-posed-belly
kind: issue
title: Since PR 7553's posed belly, the folded oblique cut's f64 body spends its budget at ε 1e-12 where its Interval twin reads the fold
status: open
opened: 2026-11-24
priority: P1
cost: M
---


Filed by GREEN's U21 lane (2026-11-24), from the closed GREEN row
`main-red-the-folded-oblique-cut-reads-budget-at-interval-at-1e-12`.

## Finding

`sweep::all self_intersection_proves_injectivity::a_folded_oblique_cut_reads_violated_off_its_corners`
fails at ε 1e-12 in `same_class`: the **`f64`** body reads `Budget` and
the `Interval` twin reads `Violated`. (The GREEN row had the two
scalars the other way round.) The panic is
`the folded cut: the Interval body reads the f64 body's verdict class`,
with `left: "Violated"` and `right: "Budget"`. The row's own print, on main `341d62e9a6`:

- `f64`: `Budget`, `CellReceipt { candidates: 32, discharged: 29904,
  violated: 0, refused: 1, splits: 65504, outside: 1422, certified: 116,
  abandoned: 35631 }`, cornered 28;
- `Interval`: `Violated`, `{ candidates: 32, discharged: 8, violated: 1,
  splits: 11, abandoned: 34 }`, cornered 0.

So the `f64` lane splits about 65k cells without finding the fold that
`Interval` finds in 11 splits, and it refuses one. The row passes at the
default ε and at 1e-6.

## Bisect (hosted CI, first parent, ε 1e-12)

Probe runs 45369489326 and 45377944124 (throwaway branch
`green/u21-probe`). The row passes at `1cf86c9e96`, at `401ffb83b5`
(PR 7626, which carries the two suite-file commits `ae95ea6c5` and
`8c32db16b` the GREEN row suspected), at `a7b44d8eb2`, `25084f4fd6` and
`6ea0abeb7d`. It fails from **`80a3bf53fe` (PR 7553,
`germ/harmonic-belly`)** onward: `583a682298`, `83bbe105b6` and
`341d62e9a6`. PR 7553's gate did not run this row at 1e-12, because its
diff does not edit the suite file.

## Not yet read

Which `f64` reading spends the budget. The likely candidate is
`Real::harmonic_belly` at `f64` returning NaN outside its exact range,
which the PR's body says "a decision reads as `Invalid`, so it escalates".
A fold whose poses put an exact zero component in a normal or an axis
would land there in every cell. That is a guess, not a measurement.
A temporary print of the verdicts behind the one `refused` and the
abandoned cells would settle it.

## Held

At ε 1e-12 the row is held with `test_utils::pinned_red::row`, which
names this file. It must stop on the class assertion with
`left: "Violated"` and `right: "Budget"`. **Done when** the `f64`
body reads `Violated` at 1e-12. Then drop the pin.

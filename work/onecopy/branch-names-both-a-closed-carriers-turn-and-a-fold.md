---
id: branch-names-both-a-closed-carriers-turn-and-a-fold
kind: issue
title: "Branch" names both a closed carrier's representative turn and a fold of the curve
status: open
opened: 2026-11-09
priority: P4
cost: C
---

Found by the branch-band fork (PR 6618).

`geom_brep::curve_pair::span`'s `Hit::On.t` and `Reading::On.t` are documented as "the parameter, on the branch about the span's middle", and `Span::branch` returns `Curve3::branch_near`. Here "branch" means the periodic representative of a closed carrier's angle: `t` renamed by whole turns to within half a turn of a reference (`geom::curves`, `branch_near`, via `Period::near`).

Beside these, the on-carrier rows (`geom::curves::on_carrier::rows`, the hairpin rows) and `a-carriers-branches-within-the-band-of-each-other` use "branch" for a fold of the curve: two parameter runs whose images lie close. A reader of "the branch `On` certifies" cannot tell which is meant, and the fork's question turned on exactly that difference.

Rename one. The periodic sense has the narrower home: `branch_near` → `turn_near`, or `representative_near`, with `Span::branch` and the two field docs following it. That leaves "branch" for a fold, which is the geometry's sense.

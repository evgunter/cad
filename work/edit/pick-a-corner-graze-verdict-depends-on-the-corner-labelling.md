---
id: pick-a-corner-graze-verdict-depends-on-the-corner-labelling
kind: issue
title: whether a corner graze on a near-coplanar candidate survives INFORM depends on which corner the tessellator labelled tri[0]
status: open
opened: 2026-09-16
priority: P0
cost: H
---


## The finding

`ray_triangle` (`crates/editor-core/src/resolve/pick.rs`) refuses a
barycentric whose rounding interval covers `[0, 1]`. The interval's
width comes from `triple_bound`, which sums MAGNITUDES of the
operands — `|s|`, `|d|`, the edges — and so does not shrink where the
barycentric's own value does. A graze through a corner has `u` and `v`
exactly `0` or `1`; its bound is whatever `|s|` and the edges make it,
and on a candidate at the certification's floor that is wider than the
range. The graze is refused.

**That much is a cost, and is stated at the door.** What is a defect
is that it is not symmetric. The bound for `u` is
`triple_bound(s, d, e2)` and for `v` is `triple_bound(d, s, e1)`, so
which terms vanish depends on which corner is `tri[0]` — the corner
`s = origin − tri[0]` is measured from. Relabel the same triangle and
the same ray gets a different verdict.

Pinned by `a_corner_graze_is_admitted_or_refused_by_its_label`
(`crates/editor-core/src/resolve/pick.rs`, adopted from review lane
pick2-r2). On the `near_tangent` fixture at `k = 32`, a ray through
corner `b`:

| labelling | `u`, `v` | `ray_triangle` |
| --- | --- | --- |
| `(a, b, c)` | `1`, `0` exactly | `Some(1.0)` |
| `(b, c, a)` | `0`, `0` exactly | `None` |

One geometry, one determinant to the bit, two answers. At `k = 8` no
labelling admits a corner graze at all.

The corpus instance is `cut_cylinder` at open, `+z` through
`(-0.4842915805643155, 0.12434494358242767, 0.0595152840731647)` at
all three of the wide aim's reaches: `main` answers the aimed vertex
at `t = reach` exactly, the tree answers `0.5356375566584823` further
(`crates/viewer/tests/review_pick2_r1.rs`, the `aim_lost` column).

## Why it is a defect and not a fact

The project's ruled position is that a pick answer depends on the ray
and the mesh, not on an incidental ordering: the tie-break is total
and documented precisely so that two candidates that are genuinely
indistinguishable are separated by a stated rule rather than by
whichever the loop met first. A verdict that turns on corner
labelling is the same class — an answer decided by a datum the caller
cannot see and the tessellator did not intend to carry — and the
tessellator is free to emit a triangle's corners in any rotation.

## Shapes

- **Symmetrise the bound**: take each barycentric's bound as the
  maximum over the three labellings, or over the two the barycentric
  is not measured from. Deterministic and labelling-free, at the cost
  of being the widest of three — which refuses more grazes, not fewer.
- **Measure `s` from the nearest corner** rather than `tri[0]`: makes
  the bound smallest where the graze is, but makes the door's `u` and
  `v` no longer Möller–Trumbore's, so `t`'s projection changes with
  it.
- **Answer it inside the `t` ruling**
  (`what-t-the-pick-door-answers-and-with-what-width`): if an
  admitted candidate carries a `t` interval and the tie-break decides
  overlap, a refused graze matters less, because the neighbour that
  answers is no longer silently preferred.

Not resolved by implementing one: it changes what the door refuses on
a class the corpus exercises.

## Measured under the clamp (EDIT-PICK3, 2026-09-16): it survives

The `t` ruling asked whether the asymmetry vanishes once the hit point
is clamped into the closed triangle. It does not, and it cannot: the
clamp changes what the door ANSWERS, not what it ADMITS, and the
verdict this row is about is a refusal at INFORM. The row's own pin,
`a_corner_graze_is_admitted_or_refused_by_its_label`
(`crates/editor-core/src/resolve/pick.rs`), is unchanged and green
under the clamp — `(a, b, c)` admits the graze at corner `b`,
`(b, c, a)` refuses the same geometry.

The third shape (“answer it inside the `t` ruling”) is also measured
and does not close it: the interval order makes the refused graze
matter less only where the neighbour that answers OVERLAPS it, and the
corpus instance (`cut_cylinder` at open, `+z` through
`(−0.4843, 0.1243, 0.0595)`) is not that case — it is the same shape as
the gallery ring's, a candidate certified to a piece of the ray shorter
than its distance from the aimed vertex. This row stays its own, with
the first two shapes untried.

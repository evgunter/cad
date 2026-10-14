---
id: nurbs-curve-interval-eval-loses-the-quotients-correlation-over-a-wide-t
kind: issue
title: NurbsCurve3::eval at a wide Interval t encloses the rational quotient's numerator and denominator independently: poison over a whole span (the denominator holds zero) and ~5-9x loose inside, even on a degree-1 non-rational net
status: open
opened: 2026-10-14
priority: P3
cost: M
refs: [nurbs-interval-ders-at-a-wide-parameter-grows-with-translation]
---

Found by PCERT closing
`projected-row-reads-its-chart-edge-through-the-box-not-its-eval` (topo's
`chart_edge` reads a projected row through `chart_box` because of this).

## What happens

`NurbsCurve3::eval` at an `Interval` `t` goes through
`eval_in_span` (`geom/src/curves/nurbs.rs`, the macro's
`eval_in_span`). It accumulates `Σ Nⱼ(t)·wⱼ·Pⱼ` and `w_acc = Σ Nⱼ(t)·wⱼ`
from the basis at `t` and returns their quotient. At a wide `t` each
`Nⱼ(t)` is enclosed on its own, so the numerator and the denominator
lose the correlation that makes the quotient a convex combination.

Measured on a degree-1 NON-rational net (weights 1), control points
`(0.5, −0.5, 0)` and `(0.5, 0.5, 0)`, whose true points are `x = 0.5`
and `y = t − 0.5`:

| `t` | x | y |
|---|---|---|
| `[0, 1]` | [−inf, inf] (poison) | poison |
| `[0, 0.5]` | [0.167, 1.5] | [−1, 0] |
| `[0.1, 0.9]` | [0.056, 4.5] | [−2, 2] |
| `[0.4, 0.6]` | [0.333, 0.75] | [−0.125, 0.125] |
| `[0.49, 0.51]` | [0.48, 0.52] | [−0.01, 0.01] |

Over the whole span the denominator `(1 − t) + t` encloses to `[0, 2]`.
That holds zero, so the division poisons. Inside the span the
overestimate grows with `t`'s width: about 9× in `x` and 5× in `y` over
`[0.1, 0.9]`. The sibling
`nurbs-interval-ders-at-a-wide-parameter-grows-with-translation` is the
surface `ders` face of the same quotient.

## What it costs

Any consumer reading a spline image over a whole edge gets nothing.
topo's `chart_edge` reads the per-piece control hull instead
(`trim_3_chart_bound::t9b_…` guards that deviation, and goes red when
this is fixed).

## What would close it

A spline evaluation that reads a degree-1 net exactly:
- skip the division when the weight channel is constant (the net is
  polynomial);
- evaluate the polynomial in a form that keeps the correlation, e.g.
  `P₀ + t·(P₁ − P₀)` in difference form, or a Bernstein / convex-hull
  enclosure per span.

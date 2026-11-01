---
id: nurbs-curve-interval-eval-loses-the-partition-of-unity-over-a-wide-t
kind: issue
title: "NurbsCurve3::eval at a wide Interval t encloses each basis function on its own, so its sums lose the partition of unity: the entire line over a whole span of a degree-1 net, and about 5x (degree 1) to 25-50x (degree 2) the true width inside it"
status: open
opened: 2026-10-14
priority: P3
cost: M
refs: [nurbs-interval-ders-at-a-wide-parameter-grows-with-translation]
---

Found by PCERT closing
`projected-row-reads-its-chart-edge-through-the-box-not-its-eval`. topo's
`chart_edge` reads a projected row through `chart_box` because of this.

## What happens

`NurbsCurve3::eval` at an `Interval` `t` reaches `eval_in_span`
(`geom/src/curves/nurbs.rs`) through `located_walk`. `eval_in_span`
accumulates the numerator `Σ Nⱼ(t)·wⱼ·Pⱼ` and the denominator
`w_acc = Σ Nⱼ(t)·wⱼ` from the basis at `t`, and returns their quotient.
At a wide `t` each `Nⱼ(t)` is enclosed on its own, so neither sum keeps
the partition of unity `Σ Nⱼ = 1`. Two things contribute.

1. **The sums.** Take a degree-1, one-span, unit-weight net with
   control points `(0.5, −0.5)` and `(0.5, 0.5)`. Its true points are
   `x = 0.5` and `y = t − 0.5`, and `N₀ = 1 − t`, `N₁ = t`.
   - **The denominator** `(1 − t) + t` encloses `[0, 2]` over `[0, 1]`,
     and the quotient is the entire line, decorated `Trv`.
     `Interval::is_poison` is false for it: it is unbounded, not NaI.
     The numerator is finite there.
   - **The numerator** loses the partition of unity too. Its `x` sum
     `0.5·N₀ + 0.5·N₁` encloses `[0, 1]` over `[0, 1]`, and
     `[0.1, 0.9]` over `[0.1, 0.9]`, where the true value is `0.5`.
     Its `y` sum happens to be exact (`[−0.4, 0.4]` over `[0.1, 0.9]`).
     So skipping the division on a polynomial net leaves `x` wide.
     Both the sums and the quotient need the fix; one does not stand in
     for the other.
2. **The walk.** `located_walk` (`geom/src/curves/nurbs.rs:1447–1456`)
   hands the UNCLIPPED `t` to every span it overlaps. Each span's basis
   is then evaluated past that span's knots, where it goes negative.
   On a two-span degree-1 net (knots `0, 0, 0.5, 1, 1`), the first
   span's `N₀ = 1 − 2t` reads `[−0.8, 0.8]` over `[0.1, 0.9]`. Its
   denominator then holds zero strictly inside the domain, and the
   value is the entire line over `[0.1, 0.9]`, and even over
   `[0, 0.5]`, which touches the second span at its knot.

Measured (`x` / `y`; `[0, 1]` reads the entire line at `Trv` for every
net here):

| net | `t` | `eval` | true |
|---|---|---|---|
| degree 1, one span | `[0, 0.5]` | `[0.167, 1.5]` / `[−1, 0]` | `0.5` / `[−0.5, 0]` |
| degree 1, one span | `[0.1, 0.9]` | `[0.056, 4.5]` / `[−2, 2]` | `0.5` / `[−0.4, 0.4]` |
| degree 1, one span | `[0.4, 0.6]` | `[0.333, 0.75]` / `[−0.125, 0.125]` | `0.5` / `[−0.1, 0.1]` |
| degree 1, one span | `[0.49, 0.51]` | `[0.48, 0.52]` / `[−0.0102, 0.0102]` | `0.5` / `[−0.01, 0.01]` |
| degree 1, two spans | `[0.1, 0.9]`, `[0, 0.5]` | entire / entire | `0.5` / `[−0.4, 0.4]`, `[−0.5, 0]` |
| degree 1, two spans | `[0.1, 0.4]` | `[0.125, 2.0]` / `[−1.0, −0.0625]` | `0.5` / `[−0.4, −0.1]` |
| degree 2, one span, same line | `[0.1, 0.9]` | `[0.006, 40.5]` / `[−10, 10]` | `0.5` / `[−0.4, 0.4]` |
| degree 2, one span, same line | `[0.4, 0.6]` | `[0.222, 1.125]` / `[−0.156, 0.156]` | `0.5` / `[−0.1, 0.1]` |

Over `[0.1, 0.9]` the degree-1 net reads about 5× the true width in `y`
(4.0 against 0.8), and `x` is 4.4 wide where the true width is 0. The
degree-2 net reads about 25× in `y` (20 against 0.8), and `x` is 40.5
wide, about 50× the true `y` width. The sibling
`nurbs-interval-ders-at-a-wide-parameter-grows-with-translation` is the
surface `ders` face of the same loss.

## What it costs

Any consumer that reads a spline over a whole span gets the entire
line, and inside a span it gets several times the true width. A reader
of a projected row's chart extent reads the per-piece control hull
(`Pcurve::chart_box`) instead.
`projected_row_chart_box::a_projected_rows_eval_over_its_edge_is_entire_and_its_box_is_exact`
pins this item's numbers; a red there means re-measure and decide.
`a_projected_edge_on_part_of_its_net_reads_the_tighter_box` pins the case
where `eval` is finite and the box is still tighter.

## What would close it

Read each span over its own clipped sub-range, and enclose the span's
restriction by its control hull. Two in-tree templates do that already,
so a fix should reuse one rather than mint a second per-span hull:

- `geom_brep::pcurve_cache::projected`: `piece_controls`, which
  restricts a span's homogeneous channels over a sub-range through
  `SpanLocate::restrict_window`. `ProjectedImage::frame_box_on` hulls
  them per span over `span_stretches`, never past a span's knots.
- `geom_core::spline::derivative::cell_quotients`, which `geom_brep`'s
  `ssi::enclose::NurbsBoxes` reads per cell on a homogeneous block
  (`CellBlock::Homogeneous`).

Clipping `t` per span (contributor 2) and keeping the partition of unity
in both sums (contributor 1) are both needed. Neither alone closes the
item: clipping leaves the one-span rows above as they are, and the sums
alone leave a multi-span walk evaluating each span past its knots.

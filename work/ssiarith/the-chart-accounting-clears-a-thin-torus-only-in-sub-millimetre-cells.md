---
id: the-chart-accounting-clears-a-thin-torus-only-in-sub-millimetre-cells
kind: issue
title: The chart accounting clears a cell against a thin torus only once it is sub-millimetre, so lily wall 8's torus × spline section exceeds its cell budget before it reaches the section
status: open
opened: 2026-11-12
priority: P0
cost: M
refs: [a-nurbs-face-against-a-torus-has-no-section-arm, the-torus-composite-bounds-a-thin-torus-far-above-its-carriers-rounding, the-nurbs-pair-all-exhausts-its-cell-budget-on-a-small-loop, two-tour-wall-probes-moved-on-main]
---

Found by CLEAVETAIL's lily-wall8 lane
(`cleavetail/two-tour-wall-probes-moved-on-main`), measuring why lily
wall 8 refuses on main `c51fc90f58`.

## What

Lily wall 8 (`demos/tour/src/lily.rs`, `wall_probes`) unions the lower
stem (a `tube_along_arc` torus segment: ring R = 5 m, pipe r = 0.06 m,
R/r ≈ 83, centre (−5, 0, 0), axis −y) with the long leaf
(`lily_leaf_a`, one loft of lens sections). Since PR 6862 the join's
germ mint carries the pair of the torus wall and the leaf's ridge wall
(a NURBS face, 3 × 25 control points, chart [0, 1]², `v` along the
loft) through the section table, which reads it whole: `curved::whole`
(`crates/geom-brep/src/pair_section/spline_implicit/curved.rs`) calls
`ImplicitSection::all` (`crates/geom-brep/src/ssi/implicit_chart.rs`).
That reading's accounting, `exhaust::account_chart_partner`, stops at
`SsiError::CellBudget { budget: 200000 }`. The section value flattens
it to `SectionError::Unaccounted { cause }`
(`pair_section::whole_reading_refused`), and the union refuses
`GermSectionUnbuilt { a_kind: Torus, b_kind: Nurbs, why: Refused(Unaccounted { .. }) }`
after about 55 s at ε 1e-9 and 33 s at 1e-6 (`ci-tour` profile, opt 1).

## Where the cells go

Measured with a scratch tally in `exhaust::sweep`, at ε 1e-9:

- Two accounting walks run in the union, on charts of certified speed
  5.57 and 5.78 m per unit (which face pairs they are was not traced).
  They bank 375 and 584 uniqueness tubes, all along the wall's `v ≈ 0`
  rim (the leaf's lower cap), `u` from 0.16 to 0.84, each about 7e-3
  by 6e-4 of the chart. A 601² grid on the ridge wall puts the section
  there too: the grid points within 1e-4 m of the torus have
  `u ∈ [0.17, 0.83]` and `v ≤ 0.01`.
- Neither walk accounts a single cell: `examined 200000, excluded
  ~99990, accounted 0, refined ~100010, max_depth 28`. The walk is
  depth-first from the chart's `(0, 0)` corner, the leaf's margin at
  the root, which is more than a centimetre from the torus (no grid
  point with `u < 0.105` lies within 1e-2 m of it). The budget runs
  out there, before the walk reaches the tubes.
- The cells refined at each depth double from depth 10 to depth 27
  (1, 2, 4, … 37146), and the exclusions come in bulk only at depths
  26 to 28 (17882, 886, 74292). Doubling at each level is an area, not
  a curve: a whole region of the wall, a centimetre clear of the
  torus, clears only in cells 2⁻¹⁴ of the chart on a side (at most
  3.4e-4 m at the chart's certified speed).

## Why (an estimate, not measured term by term)

The exclusion is `ChartPhi::over` (`crates/geom-brep/src/ssi/enclose.rs`):
the partner's canonical form evaluated in interval arithmetic over the
wall's ℝ³ box for the cell. For a torus that is
`(ρ² + z² + R² − r²)² − 4R²ρ²` (`ImplicitForm::value`,
`crates/geom-brep/src/ssi/residual.rs`). Near the tube the form's
gradient is about `8R²r` ≈ 12 per metre. Its natural interval extension
over a box of width `w` at about `R` from the centre spreads by about
`2·(2R²)·(2Rw) + 4R²·(2Rw) = 16R³w` ≈ 2000·w. The ratio is about
`2R/r` ≈ 170, so a cell at distance `d` from the torus clears only
once `w ≲ d/170`: about a tenth of a millimetre for `d` of a
centimetre or two. This is the conditioning SHELL's
`the-torus-composite-bounds-a-thin-torus-far-above-its-carriers-rounding`
measures in the composite's rounding (a form about the torus's centre
cancels terms of size `R⁴` to leave one of size `R²r·d`), met here in
the exclusion's width.

The classification's hull (`curved::hull`) reads the composite's
Bernstein coefficients per Bézier cell, which tighten under
subdivision; the accounting's exclusion does not read them.

## What would close it

An exclusion whose width over a cell does not grow as `R/r` against
the form's variation there. Candidates: a centred or mean-value form of
the quartic over the cell; the composite's Bernstein hull on the cell's
window, as the hull already reads it; or the form written about the
tube's core, as SHELL's row wants. Then re-measure lily wall 8 and
re-pin it. Its next stop is expected to be the join's frame of the
torus wall against the leaf's lower cap (`GermSectionSpiric`, SECTTORUS's
`c5-plane-torus-cone-cylinder-arms`), where it stopped before PR 6862
made the germ mint read the spline pair.

Banded P0 because it blocks
`sectarms/a-nurbs-face-against-a-torus-has-no-section-arm` (P0), which
waits on lily wall 8's torus × spline section (`work/README.md`,
Priority: a blocker inherits the band of what waits on it).

A smaller point: the budget is typed (`SsiError::CellBudget`) inside
the reading and a string once it reaches the Boolean
(`SectionError::Unaccounted { cause }`), so a consumer can match it
only by its text, which is how lily wall 8's pin matches it now. That
flattening is on ONECOPY's ground, in
`onecopy/the-section-tables-refusal-states-no-standing`
(`Unaccounted`: "mixed").

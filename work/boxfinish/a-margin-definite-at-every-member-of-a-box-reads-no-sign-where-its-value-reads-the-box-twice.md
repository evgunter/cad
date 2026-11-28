---
id: a-margin-definite-at-every-member-of-a-box-reads-no-sign-where-its-value-reads-the-box-twice
kind: issue
title: A margin that is definite at every member of a box reads no sign at the box scalar where its value reads one parameter through two stored values: the expression knows the parameter cancels and no reader asks it
status: open
opened: 2026-11-28
priority: P1
cost: H
design: true
refs: [real-margin-dependency-widening, a-margin-whose-form-is-a-nonzero-constant-escalates-over-a-box]
---


Found by M15 (the probe lane, 2026-11-28; `probe/M15-REPORT.md` on
`boxfinish/probe-side-reading` at `e77112350a`), tracing
`the-embedding-census-is-undecided-on-a-tilted-moving-cap`.

**The fact.** On a brick whose moving cap is tilted, over a box of
± 0.05 in the cap's height, at `Sym<Interval>`:

- `pm_census_ef_residual` (`census::pair_edge_face`) reads a cap
  vertex against the cap's plane: `normal · (p − origin)`, where `p`'s
  height is `top` and the origin's is `top + 0.2`. The value channel
  reads the height twice and encloses ± 0.094 (the normal's `z` times
  the box's width). The margin's expression holds the height as one
  symbol, which cancels: its plain form is a constant with no
  parameter in it, 6.58e-19, the rounding of a normal given as three
  `f64` numbers. At five members of the box the `f64` margin is
  ± 5e-17 and the census has no finding. At a box of no width the
  margin reads zero in the band.
- The tier is asked and answers nothing: it proves the zero form and
  this form is not zero. No budget stop, no missing identity.
- `bool_contact_vertex` (`contain::one_vertex_at`) is undecided only
  downstream of that: the undecided residual takes the pierce branch
  and builds a pierce point from it.
- The three kinds that appear on wider boxes are the same:
  `curve_carrier_gap` encloses `[0, 2.66]` where its form is
  `0.912·|top|`, definite over the box; `pm_census_ee_parallel`
  encloses `[0, 0.95]` where its form is the constant 0.41;
  `pm_census_ef_pierce_lever` encloses `[0, 1.04]` where its form's
  only zero is outside the box.
- On the square brick the same pair reads the same ± 0.1 on the value
  channel; its form is zero and the tier proves it. The whole
  difference between the two bodies is `0.94072087 · 0.2` against
  `0.18814417`: equal to 6.6e-19 and not exactly.

**What it is evidence of.** E12 (`docs/ERROR-DESIGN.md`) gives the
expression one use at a decision: proving a margin zero as an
identity. "Everything else decides numerically as today", on a value
channel that cannot see two occurrences of one parameter as one
number, which is the defect E12 was opened on. So a margin whose
expression is definite, or constant and inside the zero band, over
the whole box is undecided wherever its stored operands each carry
the parameter. This program has so far met that one reading at a time
(the side reader's ray, the wall's roots, the one-plane reading) and
re-spelled each so that the parameter enters once.

Rows of the same shape, each found at its own site:
`a-point-built-on-a-frame-of-normalized-directions-does-not-fold-onto-its-wall`,
`the-census-reads-an-edges-direction-off-its-two-ends`,
`a-holes-finish-climbs-the-ladder-on-eight-pose-disc-decisions-that-end-indeterminate`
(closing in PR 7724), and the margins PR 7712's refuter leaves
undecided after refuting them.

A design question; designers weigh it before a lane builds anything.
(BOXFINISH orchestrator)

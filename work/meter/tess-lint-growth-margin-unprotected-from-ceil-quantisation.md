---
id: tess-lint-growth-margin-unprotected-from-ceil-quantisation
kind: issue
title: The gate's growth margin is unprotected from the split scan's ceil quantisation — D206 closed only the continuous half
status: open
opened: 2026-09-08
---



Filed by METER unit 6's fix pass, correcting what unit 6 itself
recorded. `D206` is closed on the sample-count lever; **its premise is
not**.

**What `D206` complained of.** The split scan's own resolution can move
`span_opt_cells` by more than `tess_lint::GROWTH_TOLERANCE − 1 = 5%`
under a pure geometry change, so the budget gate can fire, or fail to
fire, on where a certified bound sits relative to the scan lattice
rather than on tessellation.

**What unit 6 fixed.** `tess_meter::SPLIT_SCAN_SAMPLES` 321 → 379,
which is the smallest count whose one-sided envelope
`10^(decades/(samples − 1)) − 1` fits inside that 5% —
5.9254% → 4.9939%. That envelope bounds the excess on the CONTINUOUS
objective, which is the half the two constants govern smoothly.

**What is left, and it is the half the gate reads.**
`tools/tess-lint` divides by `span_opt_cells`, which is `ceil`'d, and
no closed form in the tree bounds the excess there. Exhibited in closed
numbers by
`tools/tess-meter/tests/derivations.rs`'s
`the_ceild_excess_can_exceed_the_one_sided_envelope`: for
`muu = 100, muv = 0, mvv = 0.1` over a `1 × 10` box at `δ_s = 1`, the
aspect `t = 26` gives an admissible 13 × 5 grid costing 65 cells; the
shipped lattice does not contain `t = 26` and reports **70**. That is
**7.6923%** — over the envelope and over the gate's entire margin, at
the shipped pair, from the instrument alone.

**It is not a corner case, and the corpus is in the wrong band.** One
missed division out of `n` costs `1/n`, so the smallest excess a miss
can cost is set by the COARSEST AXIS of the answer, and nothing about
how fine the aspect lattice is changes that quantum. `span_opt_cells` is a sum of per-ANALYSIS-CELL
optima, and on `docs/tess-budget-data/tess-budget-baseline.csv` the
median per-cell optimum is **44.4 cells** — near seven divisions an axis
if square — with **56 of the 64 sized faces averaging under 100** and
eleven under 25. A whole division out of seven is 14%, three times the
envelope. That is arithmetic over the committed file, not a draw.

**Three independent random searches corroborate and agree on nothing
finer.** All found exceedances, all found frequency and size falling
together as the division counts rise, all put the tens-of-percent
excesses below a hundred true cells. Two found none above `1e5` true
cells and the third found some, at 6%. None of the three draws is
recorded, which is why they cannot be reconciled — the same defect
`work/meter/tess-meter-sampled-retune-figure-unreproducible` is about.

**What this row is NOT.** It is not a request to raise the sample count
further. `tess_meter::SPLIT_SCAN_DECADES`' docs record why the `ceil`'d
cell count cannot carry a tolerance at all — it moves whole percentage
points between adjacent sample counts (379: 2.94%, 380: 4.11%,
381: 1.95%) and does not converge, and two instruments built against
that quantity have already failed. Raising the count buys a smaller
continuous half and another lottery ticket on the rest.

**Where the answer probably is.** On the CONSUMER side, which is the
side `D206`'s own `## What` said the defect was on: a growth rule whose
margin is not the same order as its instrument's quantisation, or one
that reads a column the instrument can bound. `D105`'s declined lever —
narrowing `SPLIT_SCAN_DECADES` — is still blocked on a
characterisation of what `muu/mvv` ratios real certified bounds
produce, which nothing in the tree has.

Fence: `tools/tess-lint/*` and `tools/tess-meter/*`, METER's.

---
id: one-pass-refinement-would-cut-the-rational-bounds-widening-tail
kind: issue
title: One-pass (Oslo) refinement would cut the rational bound's widening tail: the fold pays a rounding per insertion where A5.4 pays one per degree
status: open
opened: 2026-09-22
---


Filed by TESS-2, which measured the tail and deliberately did not close
it. On ENCL's slate because `crates/geom-brep/src/patch_bound.rs` is the
consumer whose bound the widening is in; the change itself would be in
`geom_core::spline::algebra`.

## The measurement

TESS-2 made `patch_bound`'s rational and refined-integral arms refine IN
THE RING, so the insertion's rounding is part of the enclosure. The cost
is width, measured against the merge base on identical draws (6,000
bilinear trials and 1,500 general ones, forced seeds, in the PR for
`tess/2-refinement-in-the-ring`): median 1.4e-14 to 4.5e-14 relative,
p99 ~7e-13, max ~5e-12.

The top ~1% is above the ~1e-12 the spec named as the point where a
widening becomes a finding about the implementation. It is not: the
arithmetic is already the tight form (the convex `β·x + α·y`, which
costs 16 ulps of the coefficient scale over 30 insertions where the lerp
form costs 355). What is left is the SCHEDULE.

## Why the schedule is the cause

`refine_plan` is a fold of single Boehm insertions — a documented
implementation choice, "a deliberately simple, deterministic composition
of §5.2 rather than the one-pass A5.4". Each insertion contributes its
two ratios' outward rounding, so a 16-fold refinement of one span pays
~16 roundings into every coefficient the chain touches. A one-pass
refinement (Book A5.4, Oslo) computes each refined coefficient directly
as one convex combination of `p + 1` described ones, so it would pay
~`p` — a factor of `splits/p`, which at `RATIONAL_CERT_SPLITS = 16` and
degree 2 is about 8x on the tail.

## What it would cost, and what it would buy

It buys a tighter rational certificate everywhere in the tree, which is
grid density on every rational face. It costs a second refinement
algorithm in `algebra`, and it breaks the "one schedule, two
arithmetics" sharing TESS-2 chose: `CurvePlan`'s step list is the fold's
shape, and a one-pass refinement has a different one. So either the
`f64` path moves to A5.4 as well (evaluation-invariant in ℝ, so no
geometry changes — but every `f64` refined net moves in its last bits,
which is a re-baselining exercise across the tree) or the two paths stop
sharing a schedule and something else holds them in step.

Not urgent: nothing is unsound, the median is 1e-14, and the tail
affects grid counts and not correctness. Recorded here because TESS-2
disclosed it in a PR body and in one paragraph of a row about a
different class, and a residue disclosed is not a residue scheduled.

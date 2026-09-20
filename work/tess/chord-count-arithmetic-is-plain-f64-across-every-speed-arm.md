---
id: chord-count-arithmetic-is-plain-f64-across-every-speed-arm
kind: issue
title: Every chord-count arm multiplies a certified sup by a span in plain f64 and ceils it, with next_up on the sup as the only outward pad
status: open
opened: 2026-09-19
---


## Owner

**TESS** — `crates/mesh/` is its ground. Filed here by TRIM-2 PR-2's
fix pass because that is the PR that found it (the adjudication's
"Recorded, not fixed here"); it is handed to TESS to schedule or
close, not scheduled by TRIM.

## What

`chords::nurbs_tighten` turns a certified per-axis UV speed sup into a
chord count with

```text
n = n.max(ceil_count(su * span, hu)?).max(ceil_count(sv * span, hv)?)
```

Every step of that is plain `f64`. The sup arrives outward-rounded
(`general_uv_speeds` ends in `next_up`, and the `IsoLine`/`IsoArc`/
`Harmonic` arms end in exact `f64` structure), but the multiplication
`su * span`, the division inside `ceil_count` (`(span / step).ceil()`)
and the grid steps `hu`, `hv` from `FaceBounds::grid_steps` are not.
A count one short of what the certificate needs is a mesh whose
boundary UV step exceeds the face's budget.

It is **not specific to the `General` arm** — it is the shape of every
arm in `nurbs_tighten`, and of the `Circle` arm's torus tightening and
`nurbs_chord_count`'s `curvature_step` beside them. That is why it is
a class and not a TRIM-2 defect.

Nothing is known to be wrong today: the sizing targets δ/2, so a
one-ulp short count has a documented margin under it, and no row has
ever caught this. What is missing is the statement of which direction
each rounding runs, at the site, the way the certified enclosures
elsewhere in the kernel carry it.

## Also recorded here — the domination idiom, four times over

`crates/mesh` now spells "a certified sup dominates a dense sampling"
four times: `nurbs_cert::tests::{sample_worst, assert_dominates}`,
`nurbs_cert::tests::first_derivative_sups_dominate_samples_and_refine_upward`,
and (new in TRIM-2 PR-2)
`chords::tests::general_uv_speeds_dominate_the_sampled_image_speeds`.
PR #2848 lands a fifth, `nurbs_cert::tests::Domination`.

The obligation runs TOWARDS the new one: it samples **both sides of
every interior knot** (a maximum attained only at a knot is invisible
to a uniform grid that misses it — the row's degree-2 leg exists to
prove that), it **refuses vacuity** (both axes must move), and it
states per leg whether the bound is attained or convexity-loose. None
of the four siblings does any of these. Propagating that shape — or
unifying on one helper that has it — is TESS's call.

## Fix

Say at each site which way its arithmetic rounds, or take the count
through the ring the way the sups already are. One row that plants a
count one short and shows a certificate exceeded would say whether the
δ/2 margin is the real backstop or an accident.

## Evidence

TRIM-2 PR-2's v6 dual (PR #2863, adjudication comment 5743420032):
R2 NOTE-1 for the arithmetic, R2 style 3 for the idiom count.

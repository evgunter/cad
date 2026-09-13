---
id: certified-hull-padding-is-the-leaf-width-not-the-lane
kind: issue
title: "a stackup's certified worst-case hull pads by the LEAF width: a tier that certifies a study in fewer leaves reports a wider hull at the same leaf budget"
status: open
opened: 2026-09-07
refs: [M10-10, M10-4]
---

**Found by M10-10**, when two of M10-4's padding pins flipped. The
worst case a `Stackup` reports (E5) is the union of the certified
leaves' interval enclosures of the measure, and an enclosure's
dependency padding is proportional to the width of the LEAF it is
taken over — not to ε, and not to the analyzed box. So the padding a
consumer sees depends on how finely the driver had to subdivide to
certify, and a stronger symbolic tier — one that certifies a box in
fewer, wider leaves — reports a WIDER hull at the same leaf budget.
Measured (`m10_10_evidence_interval::m10_10_the_stackup_hulls_under_both_rule_sets`,
default ε; the same at `1e-6` and `1e-12`):

| study (`ε/8` on every axis) | tier | certified leaves | hull padding |
| --- | --- | --- | --- |
| two-hole plate (`depth`, `hole_r`) | A0 + the door (M10-9) | 16 | `2 · half` |
| two-hole plate | + rule D, A/B per node (M10-10) | 4 | `4 · half` |
| two-hole plate | + amendment A1 (the chart phase folds) | 1 — the whole box | `8 · half` |
| bore/pin fit (`r`) | A0 + the door | 4 | `1 · half` |
| bore/pin fit | + rule D, A/B per node | 2 | `2 · half` |
| bore/pin fit | + amendment A1 | 1 — the whole box | `4 · half` |

With A1 both `ε/8` studies are one leaf, so the padding is at the
ceiling this mechanism can reach — a leaf cannot be wider than the
box — and the two pins are re-baselined to `8 · half` and `4 · half`.
Both fixtures' whole-certifying half-widths are now REAL margins at
about 0.018 (the same number on both, which share a 0.2 radius;
`m10_10_the_stackup_hulls_under_both_rule_sets` with
`CAD_M10_10_CEILINGS`), so the bore/pin fit's ±0.05 study certifies.

The hulls all ENCLOSE the true range (the assertions on that hold);
what moved is the slack, exactly in proportion to the leaf width.
M10-4's two pins (`m10_4_stackup_interval::PLATE_PADDING_PER_HALF_WIDTH`,
`m10_4_r2_probes_interval::BORE_PIN_PADDING_PER_HALF_WIDTH`) were
re-baselined to the new leaf counts with this argument in their doc
comments.

## Why it matters, and what is owed

The driver's job (E6) is to certify; it stops subdividing the moment a
leaf certifies, and a coarser certification is a cheaper one. The
stackup's job (E5) is a TIGHT certified worst case, and for that the
leaf is too coarse exactly when certification was easy. The two goals
share one subdivision today. A consumer who wants a tighter hull has
no dial that asks for it: raising `max_leaves` does nothing once every
leaf certifies.

Owed, to whoever owns E5's next unit: a refinement dial for the hull —
subdivide CERTIFIED leaves (each child certifies trivially: the
verdict vector is inherited, only the enclosure is re-taken) until the
hull's padding per analyzed half-width is under a stated target, or
until a leaf budget spent on the hull runs out; and the report saying
which of the two stopped it. Not M10-10's: it is a stackup deliverable,
and the number a user reads today is sound, only looser than it needs
to be.

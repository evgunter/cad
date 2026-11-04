---
id: a-spline-chart-has-no-incidence-test-so-a-lane-choice-refusal-cannot-tell-off-from-uncovered
kind: issue
title: A spline chart has no incidence test, so a lane-choice refusal on it cannot tell a carrier off the chart from one no lane covers
status: open
opened: 2026-11-03
priority: P1
cost: H
refs: [uncovered-chart-classes-have-no-incidence-test]
---

Filed by PCERT unit E5 (PR 6484, review T1). The precedent is the closed
`uncovered-chart-classes-have-no-incidence-test`. That row gave every
analytic chart's uncovered class an incidence test, so a carrier off the
chart ends as its incidence read (`CarrierOffChart`, E1d's door rule)
and not as a lane gap. Spline charts never had one.

## What

On an analytic chart D3's lane choice is reached only behind the
incidence test (`pcurve_cache.rs` `chart_image`: `derive_class` reads
`incidence::incident` first, and only an `Incident` witness can build
`Underived::General`, `class_escalated` or `NoGeneralLane`). On a spline
chart:

- `incidence::incident` and `chart_distance` are `unreachable!`
  (`pcurve_cache.rs`, "a spline chart has no closed-form mint");
- every read of a carrier against a spline chart is an *upper* bound on
  its distance: the probe (`edge_nurbs::probe_pick`), the fitted lane's
  limb 1 (`ssi_on_locus_foot`, a seeded local Newton foot), and the
  General row's check-4 hull. None of them can decide a carrier *off*.

So `topo::pcurves::nurbs_iso_derive`'s arc-rim arm, which E5 made refuse
`NoGeneralLane { Nurbs, Circle, Unwritten }` when no row holds the
circle, gives that "not yet" to a circle on the wall that no row holds
and to one off the wall alike. The second should end as its incidence
read. `LaneGap::Unwritten`'s doc says this at that site.

On a body the kernel built, nothing reaches the off case. The mint
images only certified edges (`half_edge_curve(..).certified()`), and a
spline-adjacent edge's certificate reads the carrier against its wall at
the door (the plane and pair lanes' limbs 1 and 2). Only a damaged body
reaches it: an unvouched re-chart, or an import before EXCH's ε_in
healing lands. E5's rows reach the off pose only through
`set_face_surface_unvouched_for_tests` and `with_surface_moved_for_tests`,
and they pin no ending for it.

## Fix

A certified lower bound on a point's distance from a spline chart, such
as the distance to each span's Bézier control hull (a superset of the
patch), refined where it does not clear. Make it `incident`'s NURBS and
Approx arm, sampled over the carrier's span as `incidence_over_span`
reads the analytic charts. Then route the arc-rim arm, and `iso_schedule`'s
fall-through, through `Incident`, `class_escalated` and `NoGeneralLane`
as `chart_image` does. A row needs a circle decided off a spline wall
that ends `CarrierOffChart`, and one on it, no row holding it, that ends
"not yet".

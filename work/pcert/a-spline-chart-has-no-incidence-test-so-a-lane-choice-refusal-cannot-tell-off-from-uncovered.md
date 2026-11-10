---
id: a-spline-chart-has-no-incidence-test-so-a-lane-choice-refusal-cannot-tell-off-from-uncovered
kind: issue
title: A spline chart has no incidence test, so a lane-choice refusal on it cannot tell a carrier off the chart from one no lane covers
status: closed
closed: 2026-11-04
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

## Closed (2026-11-04): a spline chart has an incidence test

`pcert/spline-chart-incidence`. `pcurve_cache::spline_incidence` is the
spline arm of `incidence::incident` (NURBS, and an `Approx`'s fit): the
chart cut into Bézier cells, each cell's floor the larger of its
distances to its control points' box and slab (a superset of the cell
when every weight is positive), cells quartered where the floor does not
clear the band, and the chart's own point at a kept cell's centre the
upper bound. It is read over the edge's span (a patch is bounded) at
the samples `incidence_over_span` reads, through `read_edge_incidence`.
In `topo::pcurves::nurbs_iso_derive`, the arc-rim arm and the
catch-all read it where they refuse, and `iso_schedule`'s fall-through
reads it before the general image:

- off ends `CarrierOffChart`;
- in band ends `Escalated` (`ChartIncidence`), except at the
  fall-through, where it takes the general lane (D3);
- not shown off takes the lane gap (`NoGeneralLane`, "not yet").

Rows: `spline_incidence::rows`,
`topo/tests/a_circle_not_shown_on_a_spline_row_refuses_alike_in_every_form.rs`,
and the catch-all's own,
`sweep/tests/a_tangent_seam_on_a_spline_wall_ends_as_its_incidence_read.rs`.
The read's cost is still open: a band-scale sample dives (about 8× main on
the in-band seam row at `Interval`), and a thin sheet stops at the visit cap
(`the-spline-incidence-read-dives-where-its-floors-already-decide`, P2). The
sweep's other hits are filed:

- the certificate door
  (`a-spline-charts-certificate-door-reads-no-incidence-so-an-off-carrier-ends-as-its-images-bound`,
  P1);
- the edge certificate's spline lanes
  (`work/isotail/a-plane-or-pair-lane-on-a-spline-wall-ends-an-off-carrier-as-its-limbs-bound.md`,
  P1);
- the seam face's dropped refusal
  (`work/charttail/a-seam-faces-image-refusal-is-dropped-so-off-and-uncovered-both-end-unplaced.md`);
- the spline face placement's uncharged `f64` floor
  (`work/splice/spline-region-decides-off-the-carrier-on-an-uncharged-f64-box-floor.md`,
  P1);
- the loft's section rim, added to
  `a-loft-section-rims-in-band-pick-offers-a-tolerance`;
- the two cell walks (`work/splice/the-spline-chart-cell-walk-is-written-twice.md`).

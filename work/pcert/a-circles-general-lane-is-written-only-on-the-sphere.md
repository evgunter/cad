---
id: a-circles-general-lane-is-written-only-on-the-sphere
kind: issue
title: A circle's general (projected) lane is written only on the sphere: on a cone, torus or cylinder a circle no closed form images refuses not yet, because the projected envelope has no certified circle-to-chart incidence sup there
status: open
opened: 2026-10-20
---


Filed by PCERT unit E1 (`pcert/lane-choices`), the D3 lane-choice
rewrite of the pcurve derivation.

## What

D3 (`docs/DESIGN.md`, `[verdicts.consumers]`) says every class a lane
choice reads has a general lane. For a circle carrier the
general lane is the projected image (`pcurve_cache/projected.rs`). Its
check 4 builds the envelope's `Incidence` term from a certified sup of
the circle's distance from the chart, and that sup is written for the
sphere only: `crate::sphere_circle::off_sphere_sup`. The pairs the
lane is written for have one home, `projected::has_lane`, which the
mint's lane choice (`chart_image`) and check 4's two gates in
`projected.rs` (the circle arm of `projected_envelope`, "a circle's
projected image is written for a sphere", and its carrier gate) read;
E4 widens it there.

So on a cone, torus or cylinder, a circle whose class reading is in
band, or that is on the chart but none of its own circles (a cone
circle that is not a rim, a torus circle that is neither a parallel, a
meridian nor a Villarceau circle), has no general lane. Under D3's
interim rule it refuses lever-only "not yet" (`PcurveCheck::ChartClass`:
`Escalated` with the class reading's cause in band,
`PcurveCertifyError::NoGeneralLane` when definite). Before E1 the
definite case refused as `CarrierGrazesChart`, ending as a defect.

## The invariant E4 owes

`crates/geom-brep/README.md`'s C4 page (the ratified pcurve contract)
states it:

> Every class of carrier a chart can hold has a route into a certified
> row, and a face no route covers refuses at the producer rather than
> reaching rest uncached.

and, of D3's lane choice:

> no class goes unrouted, so the exemption excuses only a scalar that
> holds no certified lane

Since E1, these classes are still unrouted. Each refuses lever-only "not
yet" (`PcurveCheck::ChartClass`, or `NoGeneralLane { gap: Unwritten }`),
not as a defect, but has no route into a certified row:

- **A circle that is none of its chart's classes:** on a cone, a circle
  that is not a rim; on a torus, a circle that is neither a parallel, a
  meridian nor a Villarceau circle (the crest circles). (A circle on a
  cylinder is always a section; only its in-band readings are unrouted,
  below.) This item covers it.
- **A circle whose class reading is in band** on a cone, torus or
  cylinder. Same lane; this item covers it.
- **An in-band line on a cone or cylinder.** The cylinder line is E2's
  (`work/pcert/plan.md`). The cone line's ruling reading in band has no
  lane either; E2 or this item takes it.
- **An in-band ellipse** (a cone section's sense or nappe, a cylinder
  section's orientation). No general lane is written for an ellipse on
  any chart.

The page's sentence stays as written, because it is ratified. This item,
with E2 for the lines, is what makes it true again; until then the
sentence describes the target, not today's code.

## What to build

A certified circle-to-chart incidence sup on each of the three charts,
so `run_projected_checks` admits the circle there. Two shapes:

- the closed-form sup of a harmonic carrier's distance from the chart:
  the cone's elevation (`crate::implicit::cone_elevation` is exact on
  the double cone), and the torus's `|√((ρ − R)² + h²) − r|`, bounded
  over `t` as `off_sphere_sup` bounds the sphere's;
- imaging the circle through its exact rational quadratic net, which
  the projected lane's net path (`FramedCarrier::Net`, `net_incidence`)
  already certifies on every analytic chart, with check 4's carrier gate
  comparing that net with the circle's own.

Rows: a cone circle that is not a rim, a torus crest circle, and an
in-band rim, each minting a projected image with its envelope bounding
the sampled residual, at `f64` and at `Interval`.

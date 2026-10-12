---
id: props-rim-incidence-decides-axis-and-centre-one-at-a-time
kind: issue
title: props' require_rim_incidence decides the rim's axis and centre one at a time, not their sum
status: open
opened: 2026-10-07
priority: P3
cost: E
---

## What

`geom_brep::props::curved`'s `require_rim_incidence`
(`crates/geom-brep/src/props/curved.rs:999`) requires
`props_rim_axis_parallel` (levered at `r_c`) and
`props_rim_center_on_axis` Zero as two rows, and together they certify
that the rim lies on the surface.

Found by the sweep of the TANG unit that closed
`cylinder-axis-rows-decide-tilt-and-gap-one-at-a-time` (`work/tang/`),
and filed on this slate, whose ground it lands on.

## The shape of a fix

Decide the served verdict on one margin carrying both terms, as the
section classifiers' `decide_across` (`crates/geom-brep/src/intersect.rs`)
and `carrier_cyl_reach` (`crates/topo/src/boolean/carrier_eq.rs`) do:
the zero side on `|datum| + tilt·lever`, a definite side on the datum
shrunk toward zero by the tilt, the tilt row kept only to route.

## Re-homed from FLUX to FLUXTAIL (2026-10-10)

(FLUX orchestrator) FLUX measured 125.5 budget points against 30 and was cut on its priority seam: FLUX kept the curved closed-form arms. FLUXTAIL collects the curved arms' numeric honesty rows: rim levels, chord lengths, span folds and bounds that cancel, drop a refusal or depend on the frame, under FLUX's closed forms. The id and the body above are unchanged; the move may have set `priority`, `cost` or `status` in the header.

## A second site in the same file

The sphere-circle inventory (`crates/geom-brep/src/props/curved.rs:2727`)
serves a great circle when `props_circle_axis_class` and
`props_sphere_circle_great` decide Zero, the latter on the MAX of the
centre's offset and the radius's difference: a circle's points stand off
the sphere by up to their sum. Found by the sweep of
`section-cert-axis-pose-decides-tilt-and-offset-one-at-a-time` (branch
`cleave/section-cert-axis-pose`).

## A third site (FLUX cone-tail unit, 2026-10-11)

The cone's vector-area lane decides its own circle incidence
(`require_on_cone`, `crates/geom-brep/src/props/curved.rs`), each term
at the true distance from the cone: `props_rim_fit`,
`props_cone_circle_tilt` (`r·(sin θ·sin α + (1 − cos θ)·cos α)`) and
`props_cone_circle_off_axis` (`o·cos α`), one row each. Each is the
exact distance with the other two held, and the three can sum to up to
three times the band.

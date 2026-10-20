---
id: tier-3-could-check-each-joints-deck-element-is-at-most-one-period
kind: issue
title: Once every image is placed in its chart's principal window, tier 3 could check that each joint's stored deck element is at most one period; no check reads it today
status: open
opened: 2026-10-15
priority: P3
cost: E
design: true
---


Filed with PR 5126 (unit A of the `[ev]` PR 5142 answer).

## What

The walk now reads each joint's deck element from the half-period
marks at zero, with no cap (`geom_brep::Period::decide`), so a stored
`Deck` (`crates/topo/src/joint.rs:69`) can carry any whole number of
periods in `u` and `v`. Once unit B places every image over its edge's
span in the chart's principal window, two neighbouring images sit
within one period of each other, and a joint whose element is more than
one period out names a row that was not placed. Nothing reads that
today: `validate_pcurves` (`crates/topo/src/pcurves.rs:5002`) re-walks
and compares the elements but states no bound on them.

## The design question

Whether "|k| ≤ 1 on each channel" is a C4 invariant of a placed row
that tier 3 checks, or only a fact about today's producers. Unit B
carries the bound itself as a row; this item is whether tier 3 then
enforces it on what a body stores.

## A net's channels (PCERT unit C)

Unit C gives a spline net closed in `v` a `v` period, the knot domain's
length, so the walk decides `Deck.v` there as it decides `Deck.u` on a
net closed in `u`. A net's images are not placed in a principal window:
an image on a net keeps the net's knot domain. So on a net the
"one period" bound is not given by placement. It would hold if every
image on a net lay within the knot domain, so that two neighbouring
images' ends are within one domain length. **That is an unchecked
premise**: the derivations happen to produce such images (an iso row
sits on a boundary or interior knot line; a fitted or general image
interpolates feet that `wall.project` returns inside the domain), but
nothing checks it: no certificate, mint or tier-3 pass refuses an
image on a net that leaves the domain. A tier-3 bound on a net's
elements would rest on that premise, so deciding this item for nets
means checking the premise too, not reading it off C4's placement.

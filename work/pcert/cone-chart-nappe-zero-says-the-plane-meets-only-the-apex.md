---
id: cone-chart-nappe-zero-says-the-plane-meets-only-the-apex
kind: issue
title: geom-brep: pcurve_cone_chart_nappe's Zero refusal says the apex plane meets the cone only at the apex, and prints no margin
status: open
opened: 2026-10-13
priority: P3
cost: E
---

(Filed by the ENCL implementer of `zero-arm-payloads-overclaim-and-print-no-margin`, from PR 4976's review of its sweep's blind spot: a Zero verdict carried as a bare `Sign::Zero`, whose refusal states the zero as an exact fact.)

A `Sign::Zero` from `decide` is a band-decided reading — an exact zero, a straddling enclosure, or a nonzero size inside the zero band — not a proof of equality. The refusals below state it as exact, hedge nothing, and print no margin (D4 ¶1 (i): the margin rides the payload). geom-brep's Zero-arm payloads now say "… is not shown … at this tolerance (margin … m)".

## What

`crates/geom-brep/src/pcurve_cache.rs`, the `decide("pcurve_cone_chart_nappe", Margin::of(h), band)` call (the rim's height above the apex), `Sign::Zero` arm: "the apex plane meets the cone only at the apex, so no rim lies there". A rim at a height inside the zero band is a real circle of sub-tolerance radius, not the apex plane. `work/pred/cone-nappe-is-decided-in-five-places.md` covers where the nappe is decided, not this wording.

## Repair shape

Say the rim's height "is not shown away from the apex at this tolerance (margin … m)", carrying the margin into `off_chart`'s text.

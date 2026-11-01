---
id: the-range-panels-open-sentence-claims-values-it-never-sampled
kind: issue
title: The range panel's Open sentence says nothing new fails across values the probe never sampled, and a bracket's invalid end does not say what refused
status: open
opened: 2026-09-30
priority: P2
cost: E
---


Found by the designer pair on AUTHOR's negative-extrude fork (`[ev]` #3551). Both designers name it as CHROME's to fix whatever Ev rules there.

## The Open sentence overclaims

`crates/viewer/src/bounds.rs`, `Bounds::wording`, renders an Open side as "nothing new fails down to X (as far as it looked)" and "nothing new fails anywhere from X to Y". `Bound::Open`'s doc says "No new failure appeared anywhere out to `probed`".

Both claims cover every value in the span. The probe sampled a doubling ladder, and the module header already concedes that "an invalid sliver narrower than the sampling stride can be stepped over". The measured case: an 8 mm thickness whose only failure is at zero. A 0.8 mm seed samples 7.2, 5.6, 2.4 and −4.0 mm, never 0, and the panel says nothing new fails down to −1.63 m.

The honest sentence says what was sampled, e.g. "no sample failed down to X". The doc on `Bound::Open` should say the same. This is independent of #3551: whatever the extrude ruling, a sliver can sit in any field's span.

## A bracket's invalid end says nothing about what refused

`Bound::Edge` reports where validity ends but not why. The failing-set difference at the nearest invalid sample is already in hand (`bounds::Verdict`), so the reading could name the node that refused there, in the kernel's own words. For example: "valid from ~4 µm (below it: Extrude 2 refuses …)". That is what lets an author judge whether a floor is the model's or an artefact. Both designers recommend it, one as "cheap, build it" and the other as "lean yes".

The two halves share one file and one wording function, so they are one item.

---
id: a-face-read-as-a-plane-has-no-stated-sense
kind: issue
title: "A face reads as a plane" does not fix the plane's sense: angle reads the chart normal, gap and FaceFrame the outward normal
status: open
opened: 2026-10-10
priority: P2
cost: M
---


Found off-question by the FORK-VTX designers. D10's "A face reads as a plane" (FORK-S3P round 10) does not say which sense the plane has, and two readers of a planar face disagree today:

- `angle` reads the chart normal (`crates/editor-core/src/eval/measure.rs`, `Carrier::Plane.normal`; `fn angle`), on the rule that a chart direction is not silently sense-corrected;
- `gap` reads the outward normal (`Carrier::Plane.outward`; `fn gap`), and so does DM1a's `FaceFrame`.

The stage 3 spec's `Plane { face }` reads "its carrier's plane with its outward normal" (`docs/INTENT-STAGE3-SPEC.md` §1), so an oriented `Plane` pose picks the outward sense, while a measure reading the face directly keeps the chart normal for `angle`. Whether the face-as-plane projection should state its sense in D10, and whether `angle` should then read the outward normal, is open. Stage 3 A (`poses-are-variables`) is where the projection is built.

The pose half is settled as outward in INTENT stage 3 A: `Plane { face }` reads the face's outward normal (its material side), and the other sense is `Flip`. The measure's `angle` stays as it is; this row stays open for it.

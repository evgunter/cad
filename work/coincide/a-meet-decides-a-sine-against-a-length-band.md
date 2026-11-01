---
id: a-meet-decides-a-sine-against-a-length-band
kind: issue
title: Meet decides its planes' parallelism as a dimensionless sine against a length band, with no lever
status: open
opened: 2026-10-10
---


## What

`PoseDef::Meet` (`crates/editor-core/src/eval/wire/pose.rs:205`) decides whether its two planes meet with `UnitVec3::new(na × nb, DATUM_UNIT_NORM, band)`. That takes `|na × nb|` — the sine of the planes' angle, a pure number — as if it were a length (`geom-core`'s `UnitVec3::new` documents its norm as a length; a dimensionless direction is `UnitVec3::levered`'s, D4's θ·r form). So the decision does not scale with the model: measured by INTENT stage 3 A's dual review (R1), two planes 1 mm apart tilted 1e-8 are admitted and their line is placed about 100 km away (tilt 1e-6: about 1 km); the meet refuses `PoseDegenerate` only from a tilt near 1e-10 at ε = 1e-9.

## Why it is not fixed in A

The levered form needs an arm — a length over which the line's direction is consumed — and a meet of two planes has none to hand: the planes are unbounded, and the separation of their representative origins is itself a representative (and zero for two planes through one point, where the meet is perfectly defined). `eval::measure`'s parallel test levers at the operands' extent, which a pose read off a face could supply but an `InFrame` plane cannot. Choosing the arm is a design question, not a one-line change.

## Fix shape

Either the meet's readers supply the lever (a construction consuming the line over a known extent), or the meet decides the consequence directly — the line's offset `|nb·(ob − oa)| / |na × nb|²` against a reach — rather than the bare sine. The coset table's parallel decision (`crates/editor-core/src/mate/coset.rs`, levered at the mate's `Arm`) is the precedent. Add the row to `docs/predicate-dimension-audit.md` when it lands.

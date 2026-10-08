---
id: a-mate-relates-two-poses
kind: issue
title: D10 stage 3 PR B: a mate reads two pose variables and its kinds are its primitive; MatePrimitive, MateFrame, FrameBase, the clocking rider and PlanarRest.offset retire into pose definitions
status: parked
opened: 2026-10-08
priority: P0
cost: H
blocked_on: [poses-are-variables, a-mate-reads-face-variables]
refs: [intent-stage3-is-built]
---

INTENT stage 3, PR B. Spec: `docs/INTENT-STAGE3-SPEC.md` §3.

`Mate { a, b, sense, class }`: `a` and `b` are pose variables of one kind, and the kinds are the primitive (FORK-S3-3). `Alignment`, `MateFrame`, `FrameBase`, `MatePrimitive`, the clocking rider, `PlanarRest.offset`, `MateFrame::authored` and `table_gap` (`mate.rs`) retire into pose definitions. The spanning tree and gauges still decide what places until C. Every MSOLVE fixture's `SolvedPoses` is bit-equal.

Closes `a-clocking-rider-is-levered-unreduced`, `a-face-frame-cannot-turn-its-roll`, `a-face-base-puts-its-reference-on-local-y`, `mate-primitive-unit-variants-load-from-a-null-payload` and `a-mate-frame-axis-is-decided-against-a-length-band`. It subsumes MSOLVE-15 (#3681).

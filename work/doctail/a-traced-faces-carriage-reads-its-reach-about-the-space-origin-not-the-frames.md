---
id: a-traced-faces-carriage-reads-its-reach-about-the-space-origin-not-the-frames
kind: issue
title: A profile's traced faces are carried over their reach about the face's space origin, not the frame's origin a placement's copy reach is read about; the frame origin needs the least turn, which Re-posing mints only on a decided pose
status: parked
opened: 2026-11-22
priority: P1
cost: M
blocked_on: [7353]
refs: [a-face-charts-turn-is-levered-over-none-of-what-it-carries]
---


Found by the FULL review of PR 7151 (MAJOR 2, 2026-11-22). The implementer then raised it as a conflict with ratified text and did not build it. The traced carriage stays as PR 7151 leaves it: read about the space origin.

## What

`[placement]` (Ev, `[ev]` 7122) carries "the faces the profile traces … over its own reach, as a placement's copy is". A placement's copy reach is read about the copy's own origin, which the placement carries to the frame origin. PR 7151 reads the traced faces' reach about the face's SPACE origin (`eval/wire/trace.rs`). The review's repros (a 2×2×1 block, cap at 100, a disc of radius 0.25, the frame slid 400) show the verdict disagreeing with a placement both ways, at tilts 2.68e-4 and 2.90e-4. On a two-edged band, too large and too small are both wrong.

The frame origin in the face's space is foot + Σ sᵢ·S·eᵢ, which needs the least turn S. Re-posing (`crates/geom-core/README.md`, "Re-posing, never charging") mints S only on a decided pose. The pole verdict needs the arm, and the arm needs this reach: a circularity. The traced carriage is the profile node's first, so no earlier carriage mints S.

## Done when

The design fork `tracecarry` (two designers, then `[ev]` PR 7353) rules how the traced faces are carried. That ruling is built, with rows that pin the traced carriage against a placement on both edges.

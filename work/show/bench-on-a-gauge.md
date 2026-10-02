---
id: bench-on-a-gauge
kind: unit
title: bench places its shelf subassembly on a gauge whose pose is a document parameter: three poses, one edit each
status: review
opened: 2026-10-02
priority: P3
cost: H
pr: 3840
---

## What

EDIT/PLACE PRs #3497 (P1) and #3676 (P2-core): `Node::Gauge` is a
frame instances sit on — nestable, its rigid steps parameter slots —
and `InstantiatePart` carries `gauge` and `offset`. One document
parameter therefore moves everything placed on a gauge. Evidence:
`crates/editor-core/tests/p2_gauges.rs`,
`p2_gauge_poses_and_doors.rs`, `p2_gauge_offsets_and_spaces.rs`. The
tour never says "gauge".

`bench` is the assembly cell (pinned parts, instances, two mates
solved constructively, the flat-packed pattern beside it). Re-author
its placement onto gauges the way a user of the gauge layer would:
the shelf subassembly sits on a gauge whose lift (or swing) is a
document parameter, and the scene renders three poses through three
`SetDocParamValue` edits — the recipe layer's 5 → 7 → 9 move, said
about placement — with the reused/recomputed node counts in the
caption. A nested gauge is welcome where the bench has a natural one
(a part placed on the shelf).

## Constraints

- Read `docs/EDIT-PLACEMENT-SPEC.md` and ASSEMBLY.md A4/A9/A11 first:
  P2-split is still open on PLACE, so split and inline at a gauge
  refuse (`CutHoldsGauge`, `NeedsAGauge`, `MatePlaced`); the bench's
  split/inline narration must not route through those, or must pin
  them as live walls.
- How mates and gauges interact on one part is PLACE/MSOLVE ground; if
  the natural authoring trips a refusal, that refusal is a library
  finding filed on PLACE, not something to author around.
- Which poses go in the montage cell (one, or three ghosted) is the
  scene's call on the render; the other poses keep standalone renders,
  as heatsink 5/7/9 do.

---
id: JOIN-3
kind: unit
title: A matched section segment carries its chord curve, computed once, and role resolution and the ring-run winding read it
status: closed
opened: 2026-10-02
priority: P1
cost: H
branch: join/3-segment-curve
refs: [JOIN-1, blind-d-pocket-subtract-refuses-with-join-internal-words]
pr: 3895
closed: 2026-10-03
---

Spec: `docs/JOIN-3-SPEC.md`. Carries the second defect of `blind-d-pocket-subtract-refuses-with-join-internal-words`.

## Built

PR 3895. A boolean match computes its segment's chord curve once
(`chord_join::SegmentCurve`, `ChordJoiner::segment_curve`), from the
end and against the run the joiner's first chord is minted with; the
joiner mints both chords on it (`Chords::Segment`), and the ring lane
closes its run with it (`loop_winding::RunClosing`). The blind D pocket
builds from both faces at `4 − 0.5·A_D` (`## Built (JOIN-3)` on the
carried item). Halves either side of their segment's own edge are
ordered structurally: either order mints the same one chord, a sliver
no winding orients. The wall pierce rings' second chords no longer ask
a window of an empty run, which moved TANG's ring door
(`work/tang/pierce-ring-has-no-join-arm`, `## Measured (JOIN-3)`).
The same straight closing made ZIP's engraved one-arc C refuse
`SeamOrientation` past a sweep near 130°; it builds at every sweep, and
the item is claimed and closed here
(`an-engraved-annular-sector-refuses-seam-orientation`).
The split lane keeps its per-chord computation
(`work/cleave/split-lane-second-chord-recomputes-the-first-chords-arc`).

## Closed 2026-10-03 — PR 3895

Merged after a dual review (DR row in `docs/DUAL-REVIEW-LOG.md`), two
fix passes and a delta review (`work/join/log.md`). The pocket poses
that refuse at ring re-homing on a curved face stay open on
`a-pocket-crossing-a-side-face-refuses-at-ring-rehoming-on-a-curved-face`.

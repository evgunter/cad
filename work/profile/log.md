- 2026-10-21 — **Seam (NURBS):** NURBS build A's adjacent-pair PR (`nurbs/adjacent-shared-vertex`) closes `profile/an-adjacent-pair-through-a-joint-straight-within-the-band-escalates-on-chord-side`, retitled "An adjacent line pair escalates at its joint, not its pair" and re-rated P3. `seg::pair_contacts` takes the pair's shared vertex (`validate::judge_pair`, by index). Where `line_line`'s carrier sides escalate, `touches_only_near_the_shared_end` reads the overlap along the first segment, upper bound `extent + noise` on `collinear_overlap`, and within the band answers a Touch at that vertex, which the adjacency discount reads. The collinear arm's straddle is not asked. The joint pass reads the same in-band `sin θ·Ls`, so the escalation moves to the joint (pinned by `seg`'s coupling row) and a crossing later in pass order now refuses `NonSimple`. New rows in `rejections.rs`; `seg_reach_fuzz.rs` passes `None`.
- 2026-10-26 — **Seam (CLEAVE):** PR 5687 closed `a-boss-over-a-hole-tangent-inside-its-rim-at-its-vertex-unites-as-two-shells`, which `circle-lowers-to-one-segment` (PATHS unit 4) was parked on. That PR re-parked the unit on CLEAVE's `one-segment-tangent-rims-refuse-where-their-two-arc-forms-build` (P1). Tangent-outside one-segment rims refuse (`SectionLoopMixed` / `CurvedPierceUnsupported`) where their two-arc forms build, so lowering circles to one segment first would turn those builds into refusals. Re-park it elsewhere if PROFILE reads it differently. (CLEAVE orchestrator)
- 2026-10-27 — **Seam (CLEAVE):** `one-segment-tangent-rims-refuse-where-their-two-arc-forms-build` closes with PR 5808. A one-segment rim tangent to another now reads as its two-arc form at every pose the row measured, whatever the vertices' azimuths, `circle()`'s phase among them (`a_closed_conic_in_the_partners_face.rs` `a_boss_over_a_hole_by_pose`, one cell per pose for both forms, and the tangent poses turned). `circle-lowers-to-one-segment` (PATHS unit 4) is no longer blocked on it. Closing the row fires the unit's only trigger, which `work.py lint` refuses on a `parked` row, so PR 5808 opens it; re-park or defer it if PROFILE reads it differently. Still refused typed in this class, and filed on CLEAVE: one-segment ellipse rims tangent at a point, a one-segment cylinder through a plate with a hole tangent to its wall, and a cone boss over a tangent hole. (CLEAVE lane)

## A note from GREEN (2026-10-29) — `profile::seg` and `one_line` changed under #5952

GREEN's #5952 (merged 2026-10-29) changed two things on this
program's ground, to green the #5542 slow-set reds:

- `profile::seg`'s `line_line` no longer reads `segment_gap` on a pair
  that shares a vertex.
- `geom_core::one_line`'s `apart()` reads an overlap's depth, not its
  length. That is bit-identical where the spans lie apart, so every
  `Positive` outcome is unchanged; only in-band readings of
  overlapping pairs move.

At Interval, a lower `apart()` can turn a definite Zero into an in-band
escalation, which `editor-core`'s driver charges even on an Ok node.
That question, and the range-width cost, are on
`work/cpair/a-line-pairs-gap-reading-is-a-range-boundary-where-no-contact-changes.md`.

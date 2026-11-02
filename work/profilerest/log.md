- 2026-10-26 — (tracker sweep) Every live row checked against main `bb25fa4f` and kept: each cited mechanism still stands (`bulge_carrier`'s `Margin::of(b)`, `circle_circle`'s squared-radius form, `signed_swept` in the arc-side gates, `arc_lever`'s `min(radius, chord)`, `candidates`' `if missed` end read, `RayCastingExhausted`'s recourse-less text, the `FILED_NO_RECOURSE` admissions). Refreshed `arc-lever-caps-a-large-sweep-arcs-arm-at-its-chord`'s `seg.rs` line citations. This program had no `log.md` until this entry.
- 2026-10-26 — (easy-fixes lane) `seg-candidates-skips-its-end-reads-when-the-joints-candidate-is-held`: not easy. Unconditional end reads move three pinned verdicts (escalations on held tangencies, and a different first pair). Reverted, with a note on the row.

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

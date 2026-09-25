# CONTACT — the plan

Touches, overlaps and declared contacts: the ordinary solids the boolean lane will not combine.

Opened 2026-09-20 by REACH's priority-seam cut (`work/README.md`,
Track size). In flight: CONTACT-1 (the touch-kind cone analysis) and
CONTACT-2 (the axis-coincident lap), in parallel.

## The slate

`python3 scripts/work.py status --program contact` is the live table;
this section says only what the table cannot.

- **CONTACT-1** carries `touch-kinds-without-a-local-side-analysis-block-the-material-test`,
  with `declared-faces-has-no-cross-solid-check` riding along.
- **CONTACT-2** carries `axis-coincident-lap-trips-the-planar-join-invariant`.
- Still open: `partial-overlap-with-touch-only-boundaries-clears-at-the-census-gate`,
  `overlap-lane-boundary-crossing-cuts`,
  `area-overlap-contact-admitted-but-unmerged-refuses-at-the-next-step`,
  `point-on-arc-endpoint-zone-compresses-by-sin-half-width`,
  `contact-refusal-prose-outgrows-the-viewer`.

## Order

`touch-kinds-without-a-local-side-analysis-block-the-material-test`
first. Four touch kinds have no local side analysis and that one
absence blocks the material test for all of them, so it is the row the
other refusals in this track sit behind.

Then `partial-overlap-with-touch-only-boundaries-clears-at-the-census-gate`,
which is the same absence seen from the census gate, and
`axis-coincident-lap-trips-the-planar-join-invariant`, which is
independent and can run in parallel.
`declared-faces-has-no-cross-solid-check` rides CONTACT-1, which opens
`census.rs` first.

**The half-overlap gate row waits for CONTACT-1 to land**, not because
of a lint-visible block. Both units edit arm 2 of
`sweep_cross_solid_backstop`, and the gate row's fix, a material test
over the touch findings, reads the analysis CONTACT-1 builds.
`overlap-lane-boundary-crossing-cuts` (the D3 cut schedule) also
edits `census.rs`, so it follows as well.

## Review posture

Named per unit in its item file. CONTACT-1 is dual: a lenient touch
analysis is a confident wrong answer. CONTACT-2 is a single full
review.

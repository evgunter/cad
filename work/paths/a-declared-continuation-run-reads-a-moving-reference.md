---
id: a-declared-continuation-run-reads-a-moving-reference
kind: issue
title: A declared continuation run is checked against a ray the last accepted miss has moved, so it drifts; the 'data gate' that was said to catch it was a side effect of the line-pair defect
status: open
opened: 2026-10-20
priority: P1
cost: M
needs_ev: true
---


## What

`continue_to(p)` checks its target against the *departing point's
ray*: the current vertex `at`, which is the previous accepted target,
along the declared direction (`continuation_ray`, `crates/profile/src/path.rs`).
`emit_straight_leg_at` carries the direction out bitwise, but the anchor
is the authored vertex, which the check just accepted up to the band off
the line. So each accepted miss becomes the next leg's datum. A run of
n continuations can end n·(miss) off the line it started on, with every
per-leg check green.

`docs/PATHS-DESIGN.md` §4 records this as "a limit, not a hole", with
the run-level certifier said to be profile validation, which escalates
on `chord_side`. That escalation was a side effect of `seg::line_line`
reading an in-band carrier question on segments that are definitely
apart (`work/paths/validate-reads-in-band-carriers-before-spans-in-line-line-arc-arc.md`).
CPAIR's gap-first fix for it (`two-spans-projections-on-one-line-are-read-four-ways`,
PR 3 of 4, held at `c389fd0f` on `nurbs/seg-one-line`) makes the pinned
40-leg run (`bool11_probes::the_per_leg_band_composes_and_the_data_gate_catches_the_sum`,
20ε from its first line) validate silently.

Measured on the row's fixture: its 41 vertices are collinear with each
other but turned 0.5ε rad at the first joint, so the declaration "one
line" is false by about two ε_input across the run.

## The question for Ev

What `continue_to` declares, and so what its on-ray check reads. Both
designers recommend checking against the run's carrier line, bound
where the direction was bound and never re-anchored. That would make
the band's guarantee hold per run, with no run-level certifier. See the
`[ev]` PR.

## Done when

Ev has ruled, the ruled datum is implemented with `bool11`'s row
re-pinned, and CPAIR's held PR 3 lands after it.

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

## For the implementer, once ruled (from the designers' notes)

- **Blast radius.**
  - `Incoming` gains the line's anchor, or `carrier` becomes `Line { anchor, dir } | Arc(ArcData)`.
  - The constructors are `leg_end_tip`'s callers.
  - The readers are `continuation_ray` (both `continue_to` kernels), `tangent_kernel`/`cusp_kernel` (inherit) and `director` (rebind).
  - The §2c seal's reader inventory, in the module header, gains the new reader.
  - `resolve_fillet`'s `Pending::Ray.origin` and `junction_check` stay on the vertex.
- **Siblings with the same anchoring shape. Check each one; file any that has it.**
  1. `PendingRunOut::rides`: a fillet run-out ray is anchored at the arrival point, per leg. Does the run's carrier ride through a fillet's run-out extension onto the same line?
  2. The closing seam's `Start.arrives_tangent()` check is weighted by the arriving leg's arm alone. When the closing leg ends a declared run, is the arm that matters the run's length?
  3. `.tangent().line_to(p)`, if it can be spelled, reads its joint at the leg's own length.
- **Rows.**
  - Re-pin the bool11 row: it escalates at the first leg past the zero edge, and is typed `ContinuationTargetOffRay` at leg 21. Don't pin leg 2, which sits at the zero edge, where rounding can land either side.
  - Add an alternating-sides row that passes.
  - Add a row showing a turn starts a new run.
- **Docs to change:** `path.rs` `emit_straight_leg_at` and `on_ray_extent`, and the two `ContinuationTargetOffRay` messages ("off the run's line").

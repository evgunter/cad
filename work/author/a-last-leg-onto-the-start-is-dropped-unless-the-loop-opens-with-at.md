---
id: a-last-leg-onto-the-start-is-dropped-unless-the-loop-opens-with-at
kind: issue
title: A refused loop's last leg onto its start is dropped from the preview unless the loop opens with at and the leg targets a point
status: closed
opened: 2026-09-30
priority: P2
cost: E
branch: author/geometry-close
closed: 2026-09-30
---


Found by AUTH-5's narrow fix-pass review (PR 3440, 2026-09-30), by
running probes; not a regression — these chains dropped the leg before
the PR too.

## What

When a loop's replay refuses and `sketch::prefix_loop` walks back, a
last leg that lands exactly on the loop's start is meant to be drawn as
the close (the PR's "C4"). That only happens when **the loop's first
step is `at` and the last step targets a point**:

- `sketch::prefix_loop` takes the start point only from `steps[0]`
  (`crates/viewer/src/sketch.rs` ~:1287), so an entry that begins
  `angle 0, at(0,0), …` has no start to close onto.
- `sketch::closed_on_start` answers `None` for `Step::Line`, `Turn` and
  `FarEndTo` (~:1324), so a `line`/`turn` square's last leg can't be
  retargeted.

In every other case the provisional close from a tip sitting on the
start is zero-length and replay refuses it `SeamTangent` (margin 0, or
~1e-18). The walk-back then drops the authored last leg and puts the
refusal cross one vertex early, on a step that is fine.

Probes (all run): `at, toward, line, turn(π/2)×3, line, <bad>` draws 3
legs, cross at (0, 0.01); angle-first entry `angle 0, at(0,0), line,
line_to…, line_to(0,0), cusp` draws 3 legs; a last target 5.5e-17 off
the start draws 3 legs; and, same class without the start point, a last
leg collinear with the would-be close (`…, line_to(.01,.01),
line_to(.005,.005), <bad>`) drops that leg because the close is refused
as tangent.

## Shape of a fix

Derive the start from the loop entry's first `At` rather than only
`steps[0]`; decide what `closed_on_start` means for `line`/`turn`/
`far_end_to` (their end point is known after replay, not from the step);
and decide whether a collinear last leg should be kept with no close
drawn. The cross must stay on the step that actually refused.

## The unfinished arm has the same gap

An unfinished chain whose last leg lands on the start draws nothing at
all: its provisional close is zero-length and refused on its geometry.
That is one member of `work/author/a-close-refused-on-its-geometry-draws-nothing.md`,
filed by AUTH-11; the two want one answer to what a last leg onto the
start means.

Dispatched 2026-09-30 with its sibling as **AUTH-13** (`docs/AUTH-13-SPEC.md`, branch `author/geometry-close`). Both rows say they want one answer to what a last leg onto the start means, so they are one unit.

## Built (AUTH-13, `author/geometry-close`, PR 3579)

**The start** is the position the loop's entry binds (`sketch::loop_start`). That is an `at`, alone or after a direction, or a fused entry's incoming arc anchor. It is never a later `at`, which binds an arrival. The kernel seeds its start from the same field and exposes no seed, so the viewer reads the entry as the lattice does. **A last leg onto the start is the close** when its own step names the point it ends on and that point is the start. The same rule applies in both arms: the refused-step walk-back, and the unfinished arm, which walks back through the same `prefix_loop` call when its close is refused.

Probes, measured on the branch:
- **angle-first entry**, last leg `line_to (0,0)`, then a refused step: all four legs drawn, closed, the cross at the start.
- **collinear last leg** `…, line_to (0.01,0.01), line_to (0.005,0.005), <refused>`: the leg is kept, no close drawn, the cross at `(0.005, 0.005)`. The same holds for a close that would arrive continuing the entry's first side (`…, line_to (-0.01, 0)`), whose close is `line_to` the start declared tangent, and for one that does both, `…, line_to (-0.02, 0), line_to (-0.01, 0)`, whose close is `continue_to` the start declared tangent.
- **fused entry** (`arc_fillet` from `(0,0)`, then `at (0.01,0.003)` binding its arrival): a last leg to `(0.01,0.003)` is an ordinary leg, and one to `(0,0)` is the close.
- **`at, toward, line, (turn π/2, line)×3, <refused>`**: still three legs, the cross at `(0, 0.01)`. A `line` names no end point, and `replay` exposes no tip of a chain that does not close. Every close from a tip on the start is refused as having no length, so no replay of the steps written holds that leg.
- **a last target `5.5e-17` off the start**: still dropped. The step names a point that is not the start, and calling it the start is a banded decision the viewer does not make.

The last two, with a last leg every close reverses and one inside the ambiguity band, are filed as `work/author/a-last-leg-no-close-can-follow-is-dropped`, held by `sketch::tests::a_last_leg_no_close_can_follow_is_walked_back`. The kernel's words for a close of no length are filed as `work/paths/a-close-from-a-tip-on-its-start-is-refused-as-a-tangent-seam`.

## Closed 2026-09-30 — PR 3579 merged (`f60af7d1`)

Closed with its sibling as AUTH-13. What is drawn now is in the Built section above. The residue is its own row: `a-last-leg-no-close-can-follow-is-dropped` (the turn square, the 5.5e-17 point, a cusp close, "through the start and on", and the escalation band).

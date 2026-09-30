---
id: a-last-leg-onto-the-start-is-dropped-unless-the-loop-opens-with-at
kind: issue
title: A refused loop's last leg onto its start is dropped from the preview unless the loop opens with at and the leg targets a point
status: review
opened: 2026-09-30
priority: P2
cost: E
branch: author/geometry-close
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

## Built (AUTH-13, `author/geometry-close`)

**The start** is the position the loop entry binds, its first `at` (`prefix_loop`), so an entry that opens with a direction closes onto its last leg too. **A last leg onto the start is the close** when its own step names the point it ends on and that point is the start. That is the same rule in both arms: the refused-step walk-back and the unfinished arm, which now walks back through the same `prefix_loop` call when its close is refused on its geometry.

Probes, measured on the branch:
- angle-first entry, last leg `line_to (0,0)`, then a refused step: all four legs drawn, closed; the cross at the start.
- collinear last leg `…, line_to (0.01,0.01), line_to (0.005,0.005), <refused>`: the leg is kept, no close drawn, the cross at `(0.005, 0.005)`. The provisional close from there is `continue_to Start`, which replays where `line_to Start` is refused as tangent.
- `at, toward, line, (turn π/2, line)×3, <refused>`: **still three legs, the cross at `(0, 0.01)`.** A `line` by length names no end point. `replay` exposes no tip of a chain that does not close, and every close from a tip on the start is refused (length zero). So no replay of the steps written holds that leg, and nothing the viewer can ask the kernel today says the tip is on the start. The same holds for a far-end anchor (no `Start` form) and an arc spec with no target.
- a last target `5.5e-17` off the start: **still dropped.** The step names a point that is not the start, and whether it is the start at the kernel's tolerance is a banded decision the viewer does not make.

The last two are filed on the kernel's slate as `work/paths/a-close-from-a-tip-on-its-start-is-refused-as-a-tangent-seam`, whose typed refusal would answer both. `sketch::tests::a_last_leg_onto_the_start_not_named_by_its_step_is_walked_back` pins them as the row to re-pin.

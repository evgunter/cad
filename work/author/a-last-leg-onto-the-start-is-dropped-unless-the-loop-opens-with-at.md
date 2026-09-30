---
id: a-last-leg-onto-the-start-is-dropped-unless-the-loop-opens-with-at
kind: issue
title: A refused loop's last leg onto its start is dropped from the preview unless the loop opens with at and the leg targets a point
status: open
opened: 2026-09-30
priority: P2
cost: E
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

## The unfinished arm has the same gap (AUTH-11, measured)

Found by AUTH-11's sweep (`author/binder-prefix`), on its blind spot:
a provisional close refused on its GEOMETRY rather than by the
lattice. The chain `at (0,0), line_to (0.01,0), line_to (0.01,0.01),
line_to (0,0)` has not closed, and its tip (a leg end) admits
`line_to`. But the tip sits on the start, so the provisional close
`sketch::preview` retries it under is zero-length, and replay refuses
it as a `Path` refusal. The preview is `Err` (loop 0 never closes),
and nothing is drawn, not even the three authored legs. AUTH-11 walks
back (`sketch::prefix_loop`) only when the close is ill-typed at the
tip. A walk-back here would find the prefix closed on its start
(`closed_on_start`), but the sentence is wrong for it: the tip awaits
no binder, and the chain is closed in all but spelling. So the fix
belongs with this row's question of what a last leg onto the start
means.

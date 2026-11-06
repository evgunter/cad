---
id: an-escalation-returned-by-question-mark-ahead-of-a-later-answer
kind: issue
title: A reading's escalation returned by ? ahead of a later reading that could still answer: pieced_arc::contact's ends, CarrierLoop::place_off
status: dispatched
opened: 2026-11-06
priority: P2
cost: M
branch: onecopy/refusal-standing
---


Found by `kept-refusal-loops-the-ladder-row-never-listed`'s second pass
(`onecopy/refusal-standing`). It is a class the `get_or_insert` and
`= Some(why)` patterns cannot see: nothing is kept. The first
escalation is returned by `?`, so a later reading that would answer
definitely is never read. That breaks the ladder's rule outright
(`geom_brep::ladder` module docs: "a ladder that returned on its first
undecided reading would escalate where a reading it had not yet read
already answers").

## Witnesses

- **`splitting::pieced_arc::PiecedArc::contact`**: each end is read by
  `decide(rows.end, …)?` before the piece walk. An end in band refuses,
  even where the walk would place the point on the arc definitely
  (`ArcHit::On`) or the other end at zero.
- **`splitting::containment::CarrierLoop::place_off`**: edges are read
  in turn by `edge.contact(…)?`, "to the first that places it on the
  edge". The first edge's escalation refuses, even where a later edge
  places the point on itself definitely.

## What closing it takes

1. Port each onto `climb`:
   - `contact`: the two ends then the walk, under `Any`, since each
     definite contact is the answer;
   - `place_off`: under `First`, since which edge holds the point is
     the answer.

   **Both move outcomes** where an in-band reading precedes a definite
   one. Measure the reach and pin each change.
2. **A gate, so the next instance is seen.** A register gate in
   `escalation-discards.sh`'s form is feasible. Its shape: `?` applied to
   a `decide*(…)` call inside a `for`/`while`/`loop` body, read in the
   code view with brace tracking, registered per file at a pinned count.
   - A rough scan at this branch's merge base (single-line calls, test
     mounts excluded) finds **34 hits in 21 files**. Most are in
     `boolean/solid_contain.rs` (7) and `boolean/rim_wedge.rs` (4).
     That is a register a lane can disposition in one pass.
   - **What it cannot see:** a `?` on a call that decides inside its
     callee (`place_off`'s `edge.contact(…)?` is one), and a `?` ahead of
     a later read outside a loop (`contact`'s ends are in a two-element
     `for`, so they are seen). The callee case is not line-greppable. It
     wants the callee's error type to say it is a kept reading (a
     standing: `ladder`'s module docs), which would make a `?` on it the
     visible shape.

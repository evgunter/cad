---
id: radius-arrival-has-no-start-anchored-close
kind: issue
title: paths: whether a Radius arc arrival closes at Start at all, sharp (a Start anchor) and tangent (the token supplying)
status: open
opened: 2026-10-19
priority: P3
cost: E
design: true
---


A residual of the arrival `[ev]` design (`arc-closer-constructed-from-arrival-tangent`), named by its designers and its harmony pass and left outside that question.

**Which arrivals close at `Start` today.**
- The `Via` arrival closes at `Start`: `Via { q, p: Start }` then a director gives the sharp seam (`ViaArrivalStart`, `crates/profile/src/path/family.rs:962`), and under the token half of that design `Via { q, p: Start.arrives_tangent() }` gives the tangent seam, complete at the verb.
- `arc_onto(target)` (`docs/PATHS-DESIGN.md` §2a item 6) does the same: `arc_onto(Start).toward(d)` is the sharp seam and `arc_onto(Start.arrives_tangent())` the tangent one.
- The `Radius { r, side }` arrival has neither. Its anchor comes from the builder's `.at`, which takes only a `Point2` (`verb At(Point2<T>)` in `crates/profile/src/path/program.rs`'s `transition_table!`), and it has no target slot that could hold `Start` or the token.

The closed `at-cannot-bind-start-in-the-path-form` named this "radius-arrival close" as a vocabulary addition. That row closed without adding one, because Ev's report there turned out to be the seam fillet.

**Open question:** should `Radius` arrivals close at `Start` at all, in both halves? The sharp half would be a `Start` anchor with a director. The tangent half would have the token supply the end direction. Either half needs a spelling, and the `Via` precedent puts the anchor in a target rather than in an `.at`. The alternative is to stay without them until a case appears; none asks today.

This is a vocabulary addition to ratified text, so it goes to designers and then an `[ev]` PR.

## Re-homed at PATHS's close (2026-10-21)

Moved from `work/paths/` to PROFILETAIL (`work/profiletail/`) when PATHS closed. PATHS's live rows went to four programs opened at this close and cut along the priority seam (PROFILE, PROFILEREST, PROFILETAIL, PROFILEPROSE; `docs/doc-ledger/paths-leaves-the-tracker.md`). The item's id, content and history are unchanged.

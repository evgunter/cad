---
id: min-clearance-refusal-carried-by-name-across-a-boundary-that-is-gone
kind: issue
title: MinClearanceLane carries the clearance engine's refusal by class name and payload for a feature boundary that no longer exists
status: open
opened: 2026-09-24
priority: P4
cost: E
refs: [ring-4-interval-feature-dropped]
---

## What

`crates/editor-core/src/measure.rs`'s `MinClearanceRefusal` (the struct
above `MinClearanceLane`) carries the clearance engine's refusal as a
stable class name plus a rendered payload rather than as the engine's own
`ClearanceRefusal` enum. Its doc used to give the reason: the engine's
type "lives behind the `interval` feature and this door does not".

RING-4 deleted that feature. `crate::clearance` and its `ClearanceRefusal`
compile in every build, so the reason is gone; RING-4 re-worded the two doc
comments (`MinClearanceRefusal`'s and `MinClearanceLane::min_separation`'s)
to state the shape without the false reason, and left the shape alone —
changing a public payload type is not a mechanical edit.

The question for PROPS: does the door now carry the engine's enum (a typed
refusal a caller can match, per D9), or does the name-and-payload shape
earn its keep on another ground (the goldening form, the wire)? If the
latter, the doc should say which.

## Evidence

- `crates/editor-core/src/measure.rs`, `MinClearanceRefusal` and
  `MinClearanceLane::min_separation`'s `# Errors`.
- `crates/editor-core/src/lib.rs`: `pub mod clearance;` is ungated after
  RING-4.

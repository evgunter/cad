---
id: the-turn-refusal-names-no-corner-point
kind: issue
title: NoCornerForFillet { TurnWithinTolerance } describes one corner but travels the whole-pair channel with no corner point
status: open
opened: 2026-11-06
priority: P3
cost: E
rides_with: anchor-fit-refusal-reports-a-setback-excess-not-a-radius-reduction
---

## Finding

`PathNoCornerReason::TurnWithinTolerance` (`crates/profile/src/path.rs:566`,
the `PathNoCornerReason` enum) carries the factors of ONE corner's
levered turn: its sine, its cosine and its lever. It reaches the caller as
`PathError::NoCornerForFillet`, which is the pair-level, first-wins channel
(`CornerOutcome::Whole` in `crates/profile/src/path/arc_fillet.rs:459`,
`map_refusal`). That channel carries no corner point. On an arc×arc or
line×arc pair with two derived corners, the caller cannot tell which
corner's turn the payload describes.

PR 6544's review raised this as a NOTE.

## Why it was not carried in PR 6544

- The arc door could carry it: `arc_fillet_trims` (`crates/profile/src/sugar.rs:552`) holds `corner`
  when it refuses `ArcTrimRefusal::TurnInBand`.
- The straight pair cannot. Its turn gate runs before the corner is
  derived (`t_ray = w × u2 / cross`, `crates/profile/src/path.rs:4358`, divides by the turn the gate just
  found to be zero at tolerance), so there is no point to carry.

Carrying it on the arc door only would make the field optional. The
cleaner shape may be routing the arc door's turn refusal as an envelope
entry (`CornerReason`) beside its point, which changes the
whole-pair-outranks-envelope precedence for this refusal. That
precedence is a decision for the owner.

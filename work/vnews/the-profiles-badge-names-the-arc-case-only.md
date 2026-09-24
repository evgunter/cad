---
id: the-profiles-badge-names-the-arc-case-only
kind: issue
title: The undrawn-profiles badge says 'with an arc' and the flattener now refuses a vertex too
status: open
opened: 2026-09-21
priority: P3
cost: E
---


## Finding

`crates/viewer/src/frame.rs`, `profiles_badge`, assembles

```
profiles: {undrawn} {noun} with an arc the viewport cannot draw
```

and its doc above it says the cause "is narrower than a datum's …
what empties it is an arc the flattener cannot put a point on".

The count it renders is `CommittedProfiles::undrawn.len()`
(`crates/viewer/src/pane/viewport.rs`, the committed-profiles block),
which is one entry per profile whose `sketch::flatten` refused. As of
`vgeom/sketch-infinity` that refusal has three arms, not one: an arc
whose frame is not numbers, **a vertex whose own position is not a
pair of finite numbers**, and a point along an arc whose frame is
finite and whose far side is past the top of the exponent range. The
badge names the first.

## Reachability, measured rather than argued

**No producer found**, and the row is filed for the prose being
narrower than the mechanism rather than for a wrong sentence on
screen. `committed` draws only profiles the evaluation VALIDATED, and
both producers of a non-finite vertex that were executed for the
vgeom rows fail validation before they reach it — `segment_straightness`
for a leg whose far end overflows, `arc_diameter_clearance` for the
major arc whose far side does. Neither reaches `undrawn`, so today
every profile the badge counts is an arc case and the sentence is
true of every one of them.

What is not established is that validation refuses EVERY loop the
flattener's two new arms refuse; that is a claim about two predicates
in another crate, and nobody has made it. If it is false anywhere,
the badge names the wrong thing for that document.

## Fence

`crates/viewer/src/frame.rs` is chrome's, view's and vnews' — the
badge sentence is vnews' words. Filed rather than fixed from
`vgeom/sketch-infinity` for that reason. `crates/viewer/tests/frame_policy.rs`
pins both spellings of the sentence and moves with it.

---
id: a-lone-site-placeholder-has-no-type-of-its-own
kind: issue
title: a lone site's placeholder circle is an ordinary scaffold, so each reader decides for itself whether a whole-turn scaffold is one
status: open
opened: 2026-10-06
priority: P2
cost: M
---


## What

`EdgeCurveSpec::self_loop_circle_at(p)` (`geom-brep/src/certify.rs`) is
the placeholder `mef` certifies at a lone site (`euler.rs`,
`NewCurve::Chord` with one vertex) and the ring and `mev` sites use too.
It is an ordinary `Scaffold(RevolvedPoint)` description on a unit
circle about `p + x̂`. Nothing in the edge says it is a placeholder, so
each reader of a whole-turn scaffold decides for itself, and they
decide differently:

- `splitting/join.rs` `certify_section_area` reads every scaffold
  self-loop as the placeholder: it adds no area, and a loop of
  placeholders alone refuses `DegenerateSection`. The reading lives in
  `chord_join::lone_site_placeholder`, and its premise is that in the
  split's scratch body every self-loop scaffold is the operation's own
  lone site, because the operand is at rest and tier 3 has refused
  every scaffold.
- `splitting/containment.rs` `point_in_loop` refuses a whole-turn
  scaffold loop (`CorruptLoop`). Its comment says the edge cannot tell
  the placeholder from an honest whole-turn scaffold.
- `validate.rs` (`scaffolds_at_rest` and the transience fence) refuses
  every scaffold at rest. It does not distinguish them, and does not
  need to.

Before `cleave/tube-across-axis` (PR 4120), the split's area check read
the placeholder's circle as real geometry. In a plane with a `ẑ`
component it measured area that does not exist, and a concave graze
reached `Finish(Corrupt)`.

## What the taker owes

Give the placeholder a kind of its own, so every reader reads one
fact. That could be a `CurveGeom` variant beside `NullScaffold` (a lone
site with no carrier, by type), or a marked `MappedCurve` or
`EdgeDescription` variant. Then retire `lone_site_placeholder`'s premise
and containment's refusal in favour of matching on it.

Price: `self_loop_circle_at` has about 34 non-test call sites (11 in
`euler.rs`, plus `euler_kill.rs`, `euler_ring.rs`, `boolean/zip.rs`,
`seqgen.rs`, `revolve/full.rs` and others). A new `CurveGeom` variant
reaches every exhaustive `CurveGeom` match in topo (about 25 sites) and
the certification and pcurve passes that read the placeholder's
carrier today. That is the reason it was not done inside PR 4120.

## Found by

The review of PR 4120 (CLEAVE, `cleave/tube-across-axis`).

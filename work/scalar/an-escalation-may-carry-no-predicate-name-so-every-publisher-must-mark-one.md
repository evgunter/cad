---
id: an-escalation-may-carry-no-predicate-name-so-every-publisher-must-mark-one
kind: issue
title: geom-core: Indeterminate::predicate is optional, so every door that publishes an escalation's name has to mark a nameless one
status: open
opened: 2026-11-06
priority: P3
cost: M
---


## Finding

`geom_core::Indeterminate::predicate` is `Option<&'static str>`
(`crates/geom-core/src/predicate.rs`, the field's doc: "`None` as
produced by `Decide::sign_within` itself, which cannot know its
caller"). The D4 ¶3 convention (`Indeterminate::with_predicate`'s doc)
is that the leaf that decides attaches its name, and the `k_stats`
funnel does so for every row it classifies (`k_stats::classify`'s
`map_err(|e| e.with_predicate(name))`). But nothing holds the
convention in the type, so every door that hands an escalation's name
to a caller as a `&'static str` has to decide what a nameless one is.

Measured on 2026-11-06: 35 `predicate: None` literals outside
`tests/` (15 geom-core, 15 geom-brep, 3 editor-core, 1 topo, 1
step-import — some of them `#[cfg(test)]` fixtures), and 62 raw
`.sign_within(` calls with no `with_predicate` within three lines (52 of
them in geom-core, many the funnel's own internals that name the result
one layer up). More than a handful, and the type is geom-core's.

## What it costs today

The refs lane that closed
`work/refs/flush-invents-a-predicate-name-for-a-nameless-escalation.md`
found these publishers of a name in a `&'static str` slot, each forced
to say something for `None`:

- `topo::clearance::named` (behind `sliver` and the exact-margin
  `EngineRefusal::Sliver`) and `topo::boolean::section_cert`'s
  `UNNAMED` (two `Section::Tangent` sites) mark it `"<unnamed>"`;
- `profile::seg::ShapeIssue::predicate` and
  `geom_core::k_stats::Escalation::predicate` return `"<unnamed>"`;
- `geom_brep::pair_section::torus_arms::first_pinch` and
  `geom_brep::ssi::SsiError::tangency` fall back to the row the verdict
  was decided under, which is the leaf;
- `editor_core::eval::measure::PrimitiveRefusal::Escalated` and
  `NodeErrorKind::MeasureEscalated` now carry the `Option` through.

Each is honest, and each exists only because the field is optional;
`k_stats::Escalation::predicate`'s own doc says its fallback "exists
only because the field is an `Option`".

## Possible shape (SCALAR's call)

Make every escalation that leaves a decision carry its leaf's name —
`sign_within` returns a nameless intermediate that only `with_predicate`
(or the funnel) turns into an `Indeterminate` — and the publishers above
lose their fallbacks. Or rule that `None` is a legitimate state (a
decision nobody labelled, as `geom_core::MissingRecourse` renders it)
and carry `Option` through every payload instead.

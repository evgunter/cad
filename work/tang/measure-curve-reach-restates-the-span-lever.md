---
id: measure-curve-reach-restates-the-span-lever
kind: issue
title: editor-core's measure curve_reach restates the edge span lever, and has drifted from it
status: open
opened: 2026-10-06
priority: P4
cost: E
---


Found by PR 4118's third full review (S2).

## What

`curve_reach` (`crates/editor-core/src/eval/measure.rs:513`) bounds an
edge carrier's distance from a point per carrier. That is the same rule
as `geom_brep::Reach::Span`'s `lever_from`
(`crates/geom-brep/src/extent.rs`), which levers an edge's span for the
section classifiers. The two have drifted:

- the circle's radius and the spiric's two radii are read raw here;
- the span lever reads their magnitudes (`abs`).

The values agree on every certified carrier, whose radii are positive.

## Why it is not folded into PR 4118

Routing `curve_reach` through the span lever adds the `abs` nodes to
the symbolic forms the measure walks build. The plate's pinned
decision-form count in
`crates/editor-core/tests/m10_sym_profile_interval.rs`
(`the_forms_the_walks_build_are_pinned_per_eps_row`) moves from 16228
to 16234, with a new digest. That re-baselines a symbolic-tier pin, and
it belongs with its own review.

## The shape of a fix

Route `curve_reach` through `Reach::Span::lever_from` (or one shared
per-carrier helper), re-baseline the pinned forms, and say what moved.
Alternatively, decide which reading is right for a radius that the
constructor certifies positive and use that reading in both places.

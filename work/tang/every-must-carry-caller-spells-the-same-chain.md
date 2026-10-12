---
id: every-must-carry-caller-spells-the-same-chain
kind: issue
title: every must_carry_over_edge caller spells the same sorted, edge_extent, rule, route, chart chain
status: review
opened: 2026-10-12
priority: P1
cost: M
pr: 4859
branch: tang/one-must-carry-chain
---


## Finding

Raised by the review of the PR that routed `Body::split_edge`'s tangent
children through the must-carry rule.

Every caller that turns `geom_brep::must_carry_over_edge` into a stored
description spells the same five steps by hand:

1. `SurfacePair::sorted(ka, a, kb, b)` to put the surfaces in key order;
2. `edge_extent(carrier, t0, t1, chord)` for the extent;
3. the rule, then `.description(pair, witness)`;
4. `MustCarryRefusal::route` into the caller's three typed arms;
5. `Conventional` mapped to `EdgeDescriptionSpec::chart(<a chart the
   caller picks>)` or `describe_at_rest`.

The spellings: `topo::chord_join::chord_spec` (the tangent ruling),
`topo::splitting::finish::describe_section_boundary`,
`topo::split::Body::tangent_piece`, `sweep::extrude` (cap rim and strut),
`sweep::revolve::upgrade`, `sweep::blend::surgery` (contact edge), and
the copy `shelf/offset-reanchor-keeps-a-tangent-description-on-a-moved-interval-without-the-must-carry-rule`
prescribes for `replace_face`'s re-anchor. The boolean seam ladder
(`topo::boolean::ops`, `must_carry_reading`) reads the verdict
instead, and keeps its own step 1 and 2.

Two implementations of one rule's plumbing, where the steps that vary
are only the error arms and the chart: one helper taking the pair's
keys and surfaces, the carrier and interval, the chord, the band and
the chart, and returning `Result<EdgeDescriptionSpec, MustCarryRefusal>`,
would leave each caller its `route` and nothing else. The chart choice
is the open question in
`five-conventions-pick-a-conventional-smooth-edges-chart`, so the
helper should take it as an argument until that is decided.

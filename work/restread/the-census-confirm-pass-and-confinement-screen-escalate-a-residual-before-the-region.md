---
id: the-census-confirm-pass-and-confinement-screen-escalate-a-residual-before-the-region
kind: issue
title: The census's record confirm pass and its crossing rung's confinement screen escalate a plane residual or carrier gap before reading the face's region, where both readings would refuse the same way
status: open
opened: 2026-10-17
priority: P3
cost: E
---


## What

Found by the edge-face lane's sweep (PR for
`the-census-edge-face-lane-escalates-a-plane-residual-at-an-end-far-outside-the-face`).
That PR moved passes 3 and 4 onto "escalate a residual only where the
readings it could take disagree". Four more sites in
`crates/topo/src/census.rs` decide a plane residual or a carrier gap and
escalate it before reading the face's region. In each one, every reading
of the residual refuses the same way wherever the region read would say
`Out`:

- `confirm_edge_face` (`pm_census_confirm_ef`): an end's residual
  escalates and returns. Read `Zero`, the cells come next, and none
  means stale. Read off, it is stale too.
- `confirm_vertex_on_face`, through `face_holds` →
  `boolean::face_placement` (`pm_census_confirm_vf`): the on-plane
  decide escalates before the trim. `Out` there is stale either way.
- `pair_holds_point` (`pm_census_confined_carrier`, the crossing
  rung's confinement screen, under `ee_cross_backed`): the residual
  escalates before `contain`. `Out` there means "not confined" either
  way, and the crossing then stays an unbacked finding.
  `pair_holds_edges` reads eight residuals with no region at all, so
  only the first refusal's typing is at stake there.
- `census/point.rs` (`pm_census_point_gap`, `pm_census_point_plane`,
  the curved point-touch confirm): the carriers' gap escalates before
  `in_face` reads the trims.

Each site refuses the body either way: an escalation or a
`StaleContactDeclaration` / unbacked finding. So nothing passes that
should not. What is wrong is the typing: an escalation tells the reader
"undecided, try another tolerance", where the region read says the
record is stale or the crossing unbacked.

## The shape to give

Route each site through `census::read_undecided`, the one home of the
rule. Decide the margin into a scratch vector. When it escalates, run
the branch (or branches) its other readings would take as
`read_undecided`'s branches. Push the escalation, with
`census::held`'s share of what the branches met, only where some branch
finds something the off reading does not. `pair_vertex_face`,
`pair_edge_face` and `edge_crossings`' chord-length gate are the three
callers to copy.

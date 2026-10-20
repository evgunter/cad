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

In 3′ each site refuses the body either way, as an escalation or as a
`StaleContactDeclaration` / unbacked finding, and the escalation tells
the reader "undecided, try another tolerance" where the region read
says the record is stale or the crossing unbacked. Through a carry the
difference is behaviour, not typing (below).

## The shape to give

These sites need a disagreement test, not `census::read_undecided`.
`read_undecided` escalates where ANY reading finds something. That is
"the readings differ" only where the margin's off reading finds
nothing. Here the off reading finds something itself: a stale record,
or an unbacked crossing. So `read_undecided` would keep every
escalation.

What is owed: decide the margin, and when it escalates, run each
reading's branch (the off reading's included). Escalate only where the
branches refuse differently: say stale under one and confirmed under
another, or unbacked under one and backed under another. Where every
reading refuses the same way (stale, or unbacked), push that refusal
instead of the escalation. A sibling of `read_undecided` comparing the
branches' answers is the natural home. Carry the escalations the
branches met as `census::Branch::carries` does.

## The carry makes the typing a behaviour (2026-10-19)

From `contacthold/ff-divided-reads-stale` (PR 5390). When an op divides
a recorded cell, `boolean::records::landings` confirms the row on each
pair of the cells' images through `census::RecordConfirm::meets`: a
stale pair is passed over and the row lands on the pair that holds the
touch, while any other finding refuses the op
`CarryRefused { Found, Unread }`. So a piece pair these sites escalate,
where the region read says `Out`, refuses an op that should land the
record on another piece. The sense-first sibling in `confirm_patch` is
its own row:
`a-rest-patch-records-door-one-contradicts-a-piece-its-door-two-would-read-stale`.

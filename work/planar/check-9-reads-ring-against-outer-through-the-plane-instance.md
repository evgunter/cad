---
id: check-9-reads-ring-against-outer-through-the-plane-instance
kind: unit
title: check 9's ring-against-outer contact reads its edge pairs through the plane instance; arms 4 and 5 go
status: parked
opened: 2026-11-22
priority: P1
cost: H
blocked_on: [two-curves-in-one-plane-meet-by-the-in-plane-arms]
---


PLANAR's third unit; runs beside units 4 and 5 once the instance lands.

## The ruled text

C5 (`crates/geom-brep/README.md`): every consumer holding two curves on
one surface reads the co-surface arm, *"the at-rest ring-against-outer
contact reading"* among them, *"and none hands the pair to `meet`"*; on
a plane record the pair meets by the plane instance. The ruling
(PR 7290) keeps `ring_outer_contact` and its callers and deletes its
private reader: check 9's arms 4 and 5.

## What changes (named in the tree, 2026-11-22)

`crates/topo/src/validate.rs`:
- `ring_outer_meeting`'s arm 4 (`circles_of`, `circle_pair`,
  `CirclePair`, read through `contain::loop_circle`) and arm 5
  (`meet_segment`, `MeetSegment`, `segments_meet`, `lines_meet`,
  `meet_at`, `EdgePair`) go. Each ring edge × outer edge pair on the
  face's plane record is read through the instance: a meeting is a
  `RingContact`, a refusal is `RingOuterVerdict::Escalated`.
- Arm 2's trim (`edge_trim`, `vertex_on_edge_interior`) reads
  `MeetSegment` and `window` today; with `MeetSegment` gone it reads the
  edge's span (`Span::place`), which drift 4's correction already
  describes as check 9's arc trim.
- An `Ellipse` edge on a plane face, which `meet_segment` skips today, is
  read like any other (the instance's ellipse arms).
- `ring_outer_contact` and `ring_outer_contact_about_as` keep their
  signatures and their callers: check 9 (`validate_geometric`), the shell
  verb's precondition (`shell.rs`, two calls), and the split's section
  loops (`splitting/section_loops.rs` `outlines_disjoint`).

## Done when

- `tier3_tests.rs`' check 9 rows pass, and every verdict that moves is
  listed in the PR with its reason.
- Rows for the drifts this removes, each a lamina with a ring at `f64`
  and `Interval`: two arcs of nearly one circle (the one-circle reading
  that fell through, `check-9-drops-the-one-circle-escalation`); the
  collinear sliver `check-9-lines-meet-escalates-an-in-band-side-before-its-collinear-arm`
  names; a ring with an elliptic arc crossing its outer loop, which
  today certifies, now refuses.
- No `MeetSegment`, `segments_meet`, `lines_meet`, `meet_at`,
  `circle_pair` remains in `topo` (grep in the PR).
- The section-loop caller reads the same verdicts it reads today on the
  split suites (`splitting` tests green).

## First check, and what to do if it fails

**The section-loop caller has no plane record.**
`ring_outer_contact_about_as` is called by `outlines_disjoint` for loops
*"read before their faces wear the section plane"*, with only a
`normal`. The ruled type has two constructors, two edges of one plane
record and two segments of one sketch plane, and neither fits. Read
whether the split holds a record for its section plane at that point.
If it does not, **stop and report**: which constructor such loops take
is a question for the ruling (listed in the cut's PR), not a third
constructor to add.

## Leaves for later

- `outlines_disjoint`'s own carrier screens for ellipse pairs
  (`split_nest_line_conic`, `split_nest_conic_conic`) are a further
  reading of the same question on splitting ground (ONEHOME); the lane
  files them there, citing this unit, if the instance answers them.
- Arms 1–3 (vertex on vertex, vertex on edge, edge along edge) stay:
  the ruling removes arms 4 and 5.
- Non-planar faces and spiric or NURBS edges stay silent
  (`work/restfront/check-9-meeting-arms-silent-off-a-plane-and-on-ellipse-spiric-nurbs-edges.md`
  keeps them; its ellipse-on-a-plane half closes here, and the lane says
  so on that row).

## Rows

Closes `check-9-drops-the-one-circle-escalation` and
`check-9-lines-meet-escalates-an-in-band-side-before-its-collinear-arm`
(claimed from RESTFRONT, `parent:` this unit); with
`profile-validation-reads-its-pairs-through-the-plane-instance`, closes
`check-9-arm-5-is-a-second-copy-of-seg-planar-edge-pair-intersector`.
Changes RESTFRONT's `check-9-meeting-arms-silent-off-a-plane-and-on-ellipse-spiric-nurbs-edges`
and `check-9-does-not-check-a-ring-nested-inside-another-ring` (the
nesting test reads check 9's "disjoint"; the lane says whether it moves).

## Territory

`crates/topo/src/validate.rs`: BOXFINISH, MEASUREDOOR, ONEDOOR,
RESTFRONT. `crates/topo/src/shell.rs` (SHELL) and
`splitting/section_loops.rs` (ONEHOME and ten others) only if their
callers' types change, which this unit avoids. Announce on RESTFRONT's
and SHELL's logs.

## Review

**Dual**, difficulty **H**: the at-rest validator's acceptance moves,
arm 2's trim changes reading, and a wrong "disjoint" blesses a body.

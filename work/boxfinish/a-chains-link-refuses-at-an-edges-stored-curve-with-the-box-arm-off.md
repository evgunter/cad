---
id: a-chains-link-refuses-at-an-edges-stored-curve-with-the-box-arm-off
kind: issue
title: The tour's one-link chain refuses at plain Interval with the box arm off: the gather re-certifies its turned members' edges from values
status: parked
opened: 2026-11-24
priority: P1
cost: M
blocked_on: [the-gathers-finish-re-derives-certificates-its-moved-members-carry]
---


Found by M11 (the probe lane, 2026-11-24; `probe/M11-REPORT.md` on
`boxfinish/probe-switch-off` at `e37099a49a`), on one tree with the
switch's box arm off and on.

The tour command's chain study, one link, at `Interval`:

| | box arm on | box arm off |
|---|---|---|
| the row | "CERTIFIES 0.02 s, 0 refused" | "refuses 0.05 s, node 111e1e667832 ... an edge's stored curve does not certify against its faces" |
| the command | runs to its end, 109 s | panics at `chaintol.rs:297` (710 s at M10) |

So the exit M10 could not attribute is the box arm's: with the leaf
read at the box scalar, the link's finish refuses at an edge's stored
curve.

**M12 (2026-11-24, main with PRs 7370 and 7480): it is the scalar, not
an edge.** The chain study's row runs at PLAIN `Interval`. With the box
arm off the link's body (18 faces, 30 edges) is finished at `Interval`
and refuses on 24 of its 30 edges at the edge certificate's
`EndpointStart` (`carrier_endpoint_start`, margins `[0, 4.8e-5]` to
`[0, 7.7e-4]`), with 24 planar-face and 24 planar-boundary incidence
escalations beside them. Each margin is a distance whose lower end is 0
by construction and whose upper end is the joint's box (the link is
turned by ±0.03 rad): the identity "the carrier's start is the vertex"
is what a symbolic scalar's tier answers, and plain `Interval` has no
tier to ask. PR 7370 changed nothing (the payload is M11's to the
digit). At `Sym<Interval>` the same body passes these and refuses
later, at the volume (`[-1.08e-7, 2.53e-7]`).

**So this is a question about the charter, not a defect in a reader:**
what a leaf at plain `Interval` is once the switch goes. Over any box
that moves a body, an at-rest identity read at plain `Interval` can
only escalate. Designers weigh it before anything is built.

It holds the switch's deletion: the tour does not run to its end
without it. (BOXFINISH orchestrator)

## Weighed (2026-11-24)

Two designers, independently; they agreed at first report, and both
corrected this row's premise, which M12's trace then confirmed: **the
body that refuses is not a link, it is the product's gather** (the bar
and three pins: 18 faces, 30 edges, 20 vertices), finished after each
member's own finish had passed. So this is not a question about the
charter or about what plain `Interval` is: it is the gather
re-deriving, on moved geometry, facts each member's finish had decided
and its rigid move had carried. The sentence above ("over any box that
moves a body, an at-rest identity read at plain `Interval` can only
escalate") holds only because the gather re-reads it; a move alone
asks nothing.

The unit is
`the-gathers-finish-re-derives-certificates-its-moved-members-carry`;
this row is parked on it and closes when the tour's chain certifies at
plain `Interval` with the box arm off. Not put to Ev: no ratified text
changes (DESIGN `[one-way.places]` already states the carry) and the
designers agree.

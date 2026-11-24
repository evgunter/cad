---
id: a-chains-link-refuses-at-an-edges-stored-curve-with-the-box-arm-off
kind: issue
title: The tour's one-link chain refuses at Interval with the box arm off: an edge's stored curve does not certify against its faces
status: open
opened: 2026-11-24
priority: P1
cost: M
design: true
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

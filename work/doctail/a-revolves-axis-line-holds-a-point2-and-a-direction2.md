---
id: a-revolves-axis-line-holds-a-point2-and-a-direction2
kind: unit
title: A revolve's axis line holds a Point2 and a Direction2 in its profile's coordinates, so it can turn about a piece of its own profile
status: open
opened: 2026-11-02
priority: P1
cost: M
blocked_on: [a-profile-director-holds-one-direction2-and-reads-a-straight-pieces-heading]
parent: a-profile-cannot-refer-to-its-own-earlier-parts
---


U7 of `work/vseam/a-profile-cannot-refer-to-its-own-earlier-parts`. It
was added by the designers on the residue fork (Q11) and ruled on
`[ev]` PR 6346.

`[variables.poses]` says a revolve's axis line is a `Point2`/`Direction2`
in its profile's coordinates. `Node::Revolve` still holds
`axis_through: [S; 2]` (two `Length`s) and `axis_heading` (an `Angle`)
(`crates/editor-core/src/node.rs`, the `Revolve` variant; it is read in
`eval/wire.rs` `axis_heading`).

The build moves both onto the variable kinds U1 and U4 build, so that
a revolve can turn about a piece of its own profile (`Vertex` /
`Heading`) exactly. The 3-D lift waits for a reader. Cut by the VSEAM
orchestrator.

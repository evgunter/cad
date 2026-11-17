---
id: an-offset-edge-on-a-closed-section-loop-is-derived-on-its-base-opening-so-it-keeps-a-start
kind: issue
title: the offset door derives a closed section branch's edge on its base opening, a clamped net, so an offset edge on a loop keeps a start
status: open
opened: 2026-11-17
priority: P3
cost: M
---


Found at PR 7052's review.

`topo::offset_derive::branch_carrier` (`crates/topo/src/offset_derive.rs`,
`fn branch_carrier`, line 1004 at `ce33e72ce3`) takes a traced
`SsiBranch`'s carrier as a clamped net through `SsiBranch::net`. For a
closed ℝ³ branch that is the loop's base opening: a clamped cubic that
starts and ends at the loop's base knot. `running_as` and the door then
state the offset edge on that net, so an edge the offset door derives on
a closed section loop has a start, where C11 (PR 7042) says a closed
fitted section loop is a periodic cubic with none. It is no kink: the
base opening is C² across its ends by its control points, but a join or
a seat across that point reads a clamped end.

Not measured: no offset pose whose derived section closes was run.

A fix carries the branch's `Curve3::Periodic` through `running_as` (its
reversal is `Curve3::reversed`) to the edge's spec.

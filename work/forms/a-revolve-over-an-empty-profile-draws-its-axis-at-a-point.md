---
id: a-revolve-over-an-empty-profile-draws-its-axis-at-a-point
kind: issue
title: The revolve tool's axis preview over a profile with no loops draws a zero-length axis
status: closed
opened: 2026-11-02
closed: 2026-11-02
pr: 6401
---


(Found by `an-empty-profile-is-a-document-state`'s sweep of the readers
of a profile's loops.)

## What

A profile with no loops is now a document state, and the revolve tool
can hold one as its profile. `pane::viewport::push_revolve_axis`
(`crates/viewer/src/pane/viewport.rs`) sizes the axis it draws off the
loops' points: with none, `low` and `high` stay `0.0`, the margin is
zero, and the tool draws a zero-length axis leg with an arrowhead and
ticks stacked on the `through` point. The sweep-arc guard (`farthest`
is `None`) already returns, so nothing panics; the picture is just a
mark with no axis.

## Repair shape

Decide what the axis preview of the empty region is: no axis drawn, or
an axis of a fixed screen length. The revolve's default axis "off its
profile's extent" (`crates/viewer/README.md`, "A create gesture commits
the node") meets the same question when the extrude-and-revolve unit
builds it, so settle it there.

## Closed — 2026-11-02, PR 6401

The revolve tool and its axis preview are deleted: a revolve is
inserted from the selected profile and selected, and its body is its
own preview. A revolve of an empty profile takes the profile's y axis
as its default and refuses `EmptyProfile` on its badge.

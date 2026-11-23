---
id: a-revolve-refuses-a-profile-on-one-side-of-its-axis-as-across-it
kind: issue
title: A revolve refuses a profile that lies wholly on one side of its axis as lying across it, with a recourse that cannot help; reversing the axis heading builds
status: closed
opened: 2026-11-21
priority: P1
cost: M
closed: 2026-11-23
pr: 7425
branch: lathe/one-side-of-the-axis
---


Found by PROFILE's elbow probe (2026-11-21), run on hosted CI as the
throwaway PR 7280 (runs 44883617049 and 44889924284, `test (app)`,
`editor-core::zz_probe_elbow`), for Ev's question whether a hollow
cylinder's end loops can be revolved into an elbow
(`work/faceloop/a-faces-loop-or-closed-trace-cannot-become-a-profile-loop.md`).

## Measured

A pipe: `circle((0,0), 1.0)` and `circle((0,0), 0.6)` extruded 2. The
same two circles drawn on its end cap (`editor_core::draw_on_face`,
`OnFace::nearest()`), then `Node::Revolve` a quarter turn about a line
in the cap's plane, `axis_through = (x, 0)`, `axis_heading = h`. The
axis is 2 clear of the outer circle in every case. At `Tol::witness()`,
and the same at the 1e-6 and 1e-12 rows, on either cap and for either
sign of the angle:

| `axis_through` | `axis_heading` | revolve |
|---|---|---|
| (3, 0) | +π/2 | refuses |
| (3, 0) | −π/2 | builds |
| (−3, 0) | +π/2 | builds |
| (−3, 0) | −π/2 | refuses |

The refusal: "the revolve op refused: profile loop 0 vertex 0 lies
definitely across the revolve axis, and a revolve never carries
material through its axis. Recourse: move the profile to one side of
the axis; if the vertex should sit exactly on the axis, move the
geometry" (`crates/sweep/src/revolve/mod.rs`, near "carries material
through its axis").

## The defect

The profile is wholly on one side of the axis, so the sentence states a
fact the input does not have, and its recourse ("move the profile to
one side of the axis") is already satisfied. What decides the refusal
is which side of the DIRECTED axis the profile is on: reversing the
heading builds.

Two things to settle, in order:

- **Is the side requirement needed?** The angle is signed and the
  heading is authored, so a profile on either side of an axis line
  describes a well-defined revolve. If the requirement exists only to
  fix an orientation convention, the door can take either side and
  normalize.
- **If it is needed**, the refusal names the side and its recourse is
  "reverse the axis heading", not "move the profile".

Banded P1: revolving about an axis beside a profile is an everyday act,
and half of the ways to write it refuse with a sentence that sends the
reader the wrong way.

Not checked: whether the same reading refuses a profile that is not
drawn on a face (a free profile revolved about `(3, 0)` heading +π/2).

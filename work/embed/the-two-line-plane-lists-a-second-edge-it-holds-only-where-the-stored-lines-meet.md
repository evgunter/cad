---
id: the-two-line-plane-lists-a-second-edge-it-holds-only-where-the-stored-lines-meet
kind: issue
title: planes_through's plane through two shared line edges at a vertex lists both, and holds the second only where the two stored lines meet exactly
status: open
opened: 2026-11-22
---

## Finding

From `embed/straight-corner` (PR 7338, not landed) and its review,
2026-11-22. Not measured on the arm it names: stated from the code and
from the review's fixture on a sibling arm.

`topo::injective`'s `planes_through` (`crates/topo/src/injective.rs`),
its arm "through two line edges with a common end, at a shared vertex":
`n` is the two stored directions' cross product and `q` a point of the
FIRST line. Its `On` lists both edges and their ends as cells the plane
vanishes on. The plane holds the first line by construction. It holds
the second only if the second stored line meets the first exactly;
where the two stored lines miss each other by `δ` (each certified
against the shared vertex within the edge tolerance), `ψ` on the second
edge is `δ·|n|`, not zero, and a side on that edge is read as zero by
membership.

The stopgap run of PR 7338 built a plane the same way (two runs'
directions crossed, through a point of the first) and its review
measured the consequence there: `review_claim3_the_two_run_plane_lists_a_run_it_does_not_hold`
on `origin/embed/straight-corner-review` (`2fa9e6617`), two stored
lines `δ` apart with the faces crossing over `δ`, read `Certified` at
`δ = 2⁻²⁰` (about 1000 ε), at unit level. No body reached it. That arm
does not land.

## Reach

Two faces that share two line edges meeting at a vertex. Two plane
faces doing so are coplanar, so the arm's ordinary reach is a plane
against a ruled or free-form face, or two such faces. With
`embed/corner-pieces` a plane face's two-line corner is two pieces,
each holding one of its edges, but `planes_through` is handed the
FACES' near, not a piece pair's, so the arm is still built for every
piece pair of a pair near two shared lines. It cannot certify from a
plane piece there: a plane through two lines of a plane face is that
face's own plane, on which its pieces are zero everywhere and not
strictly signed. So the arm's reach is unchanged: a pair neither of
whose parts is a plane.

## What is needed

A fixture on the existing arm (two convex presentations sharing two
line edges, the second line stored `δ` off the first) to say whether it
certifies a crossing of `δ`. Then either the plane is built to hold
both lines for every member, or the second edge is read over its side
(`SELF_INTERSECTION_ON_SHARED`) instead of by membership.


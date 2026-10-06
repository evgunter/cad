---
id: cut-in-refusals-no-probe-reaches
kind: issue
title: Four of the cut-in's refusals have no row: a pole inside the circle, a hole inside it, ends on two loops, uncertified roots
status: open
opened: 2026-10-06
---

Found by the dual review of PR 4044 (`analysis/reach-dual/4044-r1`,
NOTE 2).

## What

`boolean::ops`'s `apply_cut_ins` refuses typed in four places no row
or reviewer probe reached (r1 ran ~360 ops per ε, r2 342):

- **the section circle holds a pole** (ops.rs ~3731-3735, "the section
  circle holds a pole of the face's chart"). On a revolve-charted sphere
  a pole is a vertex with seams through it, so a circle wholly inside
  one face cannot hold one; a face whose chart has a pole inside it
  (a sphere minted another way, or a re-charted one) might. Unknown
  whether any constructor reaches it.
- **a hole inside the circle** (ops.rs ~3948-3952, "the sphere face's
  boundary meets the cut's meridian inside the section circle"): the
  certificate's R-loop places the circle inside the face, so a ring
  lying wholly inside the circle's disc on the sphere is the case.
- **ends on two loops** (ops.rs ~4019-4023, "the cut's meridian joins
  two loops of the sphere face"): the nearest hit below on a ring and
  the nearest above on the outer loop, or the reverse.
- **uncertified roots** (ops.rs ~3866-3872): a boundary arc tangent to
  the meridian plane, or lying in it, near the cut.

## Asked

For each, a probe that reaches it (and a row pinning the refusal or,
better, a build), or a statement of why it is unreachable that names
the constructor-level invariant. A refusal no input reaches is dead
code to delete; one that is reachable and refuses a buildable body is
a liveness gap.

---
id: two-curve-curve-meeting-finders
kind: issue
title: The kernel has two curve-curve meeting finders: carrier_cross's closed-form line, circle and ellipse crossings, and the census's bisection search
status: open
opened: 2026-10-13
priority: P3
cost: M
---

Found by FUSE's review of PR 4946 (`census-curved-record-lanes-read-circles-only`, branch `fuse/census-reads-every-curved-edge`).

A design row on one intersector and its home.

- `boolean::carrier_cross` crosses a line, circle or ellipse with
  another in closed form (the boolean's crossing layer).
- `census::curved::reads_meet` searches any two carriers by
  `crate::bisect`'s walk with a second-order clearing bound, tangency
  floor and Gauss–Newton polish.
- `work/restread/the-census-edge-face-cut-crosses-chords-and-conics-only.md`
  proposes the second as a third consumer (a line against a spiric or
  spline boundary arc).

Two finders answering one question can disagree at the band. Which one
the kernel keeps, what the other becomes (an oracle, a fast path), and
where the survivor lives is the question.

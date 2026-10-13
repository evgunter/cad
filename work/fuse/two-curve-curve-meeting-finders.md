---
id: two-curve-curve-meeting-finders
kind: issue
title: The kernel has two curve-curve meeting finders: carrier_cross's closed-form line, circle and ellipse crossings, and the census's bisection search
status: open
opened: 2026-10-13
priority: P3
cost: H
design: true
needs_ev: true
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

## Weighed by a designer pair (2026-10-13); with Ev as the `[ev]` PR

After three rounds the two designers agree on the final state. One component answers whether, and where, two carrier spans meet:
- **One point-on-span reading and one span type.** These replace the four readings that sit in `carrier_cross::strictly_inside`, `census::curved::clear_of_end`, the census line lane and `LoopEdge::contact`.
- **One total `(CurveKind, CurveKind)` table, one method per pair, fixed at write time.**
  - Line, circle and ellipse pairs are closed forms in one conic's angle, through the half-angle ladder.
  - Spiric and spline pairs use the certified walk. The walk also serves as the oracle for the closed forms.
  - There is no runtime exit to the walk and no torus pre-clear.
- **Two contract clauses:**
  - a generator's in-band decision escalates and never prunes;
  - a `Zero` candidate is placed by the exactly degenerate formula.
- **Moves:** `bisect` goes to geom-core, and the rate bounds and the ladder move down. C5 gains a sentence saying it governs curve pairs too.

The near-concentric arcs are decided by the circle×circle arm. That closes `near-concentric-arcs-run-the-census-meeting-search-past-its-budget` with this design.

**The fork put to Ev is the home:** `geom-brep`, with the v0 Layering row amended, or `geom`, as that row stands.

Measured on the way, and filed: `cleave/a-near-tangent-coplanar-circle-pair-places-its-touch-off-both-circles` (P1). `carrier_cross` misplaces circle-pair candidates near the band, so the boolean reads `Clear` where the edges cross.

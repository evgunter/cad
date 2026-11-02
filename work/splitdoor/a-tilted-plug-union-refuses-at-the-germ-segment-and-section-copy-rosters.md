---
id: a-tilted-plug-union-refuses-at-the-germ-segment-and-section-copy-rosters
kind: issue
title: The tilted plug's union refuses at sectors::one_segment and chord_join's section copy, which read lines and circles only
status: closed
opened: 2026-10-12
priority: P1
cost: M
closed: 2026-10-28
pr: 5940
branch: splitdoor/tilted-plug-rosters
---


Found by the two designers on `a-tilted-plug-in-its-hole-refuses-at-the-ellipse-rims` (2026-10-12), each measured on a prototype that reads ellipses in `carrier_cross::boundary_crossing`.

With the interior question reading ellipses, the tilted plug's ∩ and both ∖ build at their closed forms. ∪ still refuses, at two lines-and-circles rosters further along:

- `sectors::one_segment` refuses `GermEdgeCarrierUnsupported { operand: A, edge }`. It reads two edges leaving a vertex pair along a germ, with paired far ends, as one segment only for lines and circles. As a property of curve kind that is right: two ellipses tangent at a point with a shared far end can differ between. Here, though, the identity is structural and already decided. Each rim is the meet of a top face and a wall face, and the glue door decided both face pairs to be one carrier (walls `SameOpposite`, tops `SameOriented`). A suggested direction is to read the glue's one-carrier verdicts on the two edges' parent face pairs before the kind rule, i.e. D10's "same construction" identity. Not measured.
- Once that is relaxed, `chord_join`'s copy of a section segment ("only lines and circles are copied") refuses an ellipse arc. Measured by one designer, up to here and no further.

The issue's closed form (the union is the slab, 12) stays unreached until both land.

## Slate triage (2026-10-26, CLEAVE)

- Re-banded P2 → P1: two lines-and-circles rosters refuse the tilted plug's ellipse rims: verb breadth beyond the everyday shapes (a split or boolean that refuses, or ships a body its own gate fails, on a legal input), P1 by `work/README.md` Priority.
- Moved to SPLITDOOR (`work/splitdoor/`) by `git mv`, id and body unchanged: the P1 track opened for split and boolean shapes that refuse, or ship a side their own gate fails, where a uniform reading would build: the split door's tier-3′ bar, tangent and pinch poses, and the curved-carrier lanes that read lines and circles only. CLEAVE measured 270.5 budget points against 30 and split on its priority seam (Ev, in chat, 2026-10-26).

## Closed (2026-10-28, PR 5940)

The tilted plug's ∪ is the slab (12) at every pose, both tilts, three ε. `sectors::one_segment`
reads the glue door's one-carrier verdict for plane×(cylinder|cone) sections before the kind rule;
`chord_join` copies any segment edge, run back through one `run_back`, restating a description no
chord states in the chart of the face it divides. FULL single review (mergeable, two MINORs fixed
in the fix pass). Rows filed are listed in the PR body.

---
id: a-union-seam-whose-pieces-lie-on-one-open-carrier-ranks-its-crossings-along-it
kind: issue
title: Crossings of a union seam whose pieces lie on one open carrier rank along that carrier's parameter; a closed carrier ties
status: open
opened: 2026-10-15
priority: P3
cost: M
---


## What

N2 ranks crossings of one line by one face, and vertices with one run,
"along a line they share … or, where the run holds no edge, a straight
seam in it". `emit_union::rank_along_seam` reads only a seam whose pieces
all lie on one straight line. That limit dates from before the names code
had a curved carrier reading (`Track`, PR 5073 and PR 5107); it was not a
decision against curved seams.

Ev, on PR 5107 (2026-10-14, answering why the clause is straight-only):
"makes sense; that can be a separate pr". That approved the recommendation
to rank along a seam whose pieces lie on one **open** carrier, by that
carrier's own parameter, straight or curved, and to keep a **closed**
carrier tied, since it has no start but where the kernel put the seam's
vertex and N2 refuses to read a name off that.

## Owed

- `rank_along_seam` reads a seam whose pieces lie on one open carrier,
  through `EdgeCarrier` / `Track`. A closed carrier and pieces on
  different carriers tie.
- N2's clause re-worded to "a seam in it whose pieces lie on one open
  carrier" (the wording Ev approved on PR 5107).
- A fixture: a wall crossing an arc seam twice, in every member order.
- `crossings-of-a-union-seam-of-several-curves-tie` (closed on PR 5107)
  stays closed: pieces on several curves still tie.

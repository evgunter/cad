---
id: a-two-carrier-curve-touch-has-no-bounded-locus
kind: issue
title: A face-face record on two carriers is confirmed at one point of its tangent line: the touch's endpoints are not bounded by vertex records or the locus's closure (C3)
status: parked
opened: 2026-10-11
blocked_on: [d10-one-way-to-say-intent-is-unbuilt]
cost: M
---


Found by CONTACTHOLD's fix pass on PR 4604 (dual review, R1 and R2).

`census::confirm_face_pair` (`crates/topo/src/census.rs`) reads a
face–face record on two carriers through `boolean::tangent_pair_at_rest`.
That door derives the closed-form tangent line, meets the two faces'
extents along it (disjoint extents are stale), and runs the C4 `Tangent`
table over the overlap, material sides included. The census then places
the line in both faces at the overlap's midpoint through each face's
containment door. In both faces, the record is confirmed. Off either,
it is stale.

What C3 asks and this does not build: the touch's **endpoints**, the
places where the line leaves either face, "are bounded by vertex records
or the locus's closure; an unbacked bound is `UndeclaredContact`". The
midpoint read also judges a record stale whose line misses one face at
the middle but enters it elsewhere along the overlap. That's a false
refusal, never a false confirm. The fix is the curve-touch analogue of
`ef_overlap_cells`: cut the overlap where each face's boundary crosses
the line, read each cell's midpoint, and back each cell's two bounds the
way `ef_bound_backed` backs an edge-on-face overlap's.

## 2026-10-17 — parked on the D10 hold, moved from RESTREAD

RESTREAD read this row against the D10 hold (`docs/DESIGN.md`, State of
the build) and the open [ev] PR 4666 (the census records what it
decides): it stands on contact records, declared pairs or the shape of
at-rest findings, which INTENT stages 4 and 5 rebuild. Per RESTREAD's
plan (a row needing declared pairs or contact parks and moves to
CONTACTHOLD), it moved here by `git mv` with its id unchanged. Priced
`M` where it carried no cost. (RESTREAD orchestrator)

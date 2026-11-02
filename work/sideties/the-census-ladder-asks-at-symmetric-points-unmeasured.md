---
id: the-census-ladder-asks-at-symmetric-points-unmeasured
kind: issue
title: The census's interior-point ladder asks at edge middles and centroids, and its ties are unmeasured
status: open
opened: 2026-10-27
priority: P2
cost: M
refs: [the-side-readers-aims-decide-exact-ties-at-a-documents-nominal]
---

`crates/topo/src/stands.rs` reads a selection's side through its own
ladder, the census's interior-point rungs:
- vertices;
- edge middles (`curve.mid_point()`);
- `planar_interior_point`, which tries vertex-triple centroids, chord
  midpoints and points across edges, each certified through the in-face
  walk.

Each of these lands on a symmetric point of a symmetric face. That is
the shape the side reader's aims had before `flush/aims-ties`
(`the-side-readers-aims-decide-exact-ties-at-a-documents-nominal`).

Unmeasured: the ladder's decisions are not under
`topo::SIDE_READER_FAMILY`, so `revalue_corpus::reader_ties_at_the_nominal`
cannot see them. Measuring them needs the ladder tagged under a family
of its own, or a probe that brackets it. Then the same census, over the
30 corpus documents, says whether any tie.

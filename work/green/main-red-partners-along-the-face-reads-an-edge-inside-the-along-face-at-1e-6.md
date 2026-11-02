---
id: main-red-partners-along-the-face-reads-an-edge-inside-the-along-face-at-1e-6
kind: issue
title: "Main red: a_vertex_read_again_classes_every_edge::partners_along_the_face_build_sound_against_the_germ fails at ε 1e-6 on an edge inside the along face"
status: open
opened: 2026-11-02
priority: P0
cost: M
---

Found by SPLITDOOR on `splitdoor/section-line-components` (PR 6310),
reported by the review of that PR. Re-run on a clean `origin/main`
worktree at `0b4711875` with `CAD_TOLERANCE_EPS=1e-6`: red there too,
identically, so it is main's and not 6310's. Not bisected.

## The row

topo `a_vertex_read_again_classes_every_edge::partners_along_the_face_build_sound_against_the_germ`
(`crates/topo/tests/a_vertex_read_again_classes_every_edge.rs`, its
`probe.bad` assertion), 176 s:

```
cells 3012, built 1755, empty 358, germ rows 11000; refused {"Escalated": 843,
"LineWedges::Overlap": 26, "PartnerAlongFace": 30, "partner: Escalated": 4,
"x: a corner within the band of the apex": 2}
```

Two bad cells, the half-turn pose `φ 250`, `h 0`, scale `0.03`, either
side of `δ`:

- `δ −2 | an edge inside the along face, turned, x − y`: A's edge
  `EdgeKey(4v1)` at MEET reads `[]`, the germ `Out`;
- `δ +2 | …`: the same edge reads `[]`, the germ `In`.

The germ row reads no class for an edge of A lying inside the along
face at the meeting vertex, where the germ itself reads a definite
side.

## Done when

The row is green at ε 1e-6, or the two cells refuse typed and the row
admits them by name. Its other ε rows were not measured here.

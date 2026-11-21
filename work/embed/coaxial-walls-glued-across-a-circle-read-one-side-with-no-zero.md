---
id: coaxial-walls-glued-across-a-circle-read-one-side-with-no-zero
kind: issue
title: Two walls of one cylinder carrier glued across a circle (the corpus bushing) refuse: the carrier arm has no departure, and the circle's plane signs one wall while the other is presented with no zero side
status: open
opened: 2026-11-21
priority: P0
cost: M
---


## Finding

Found by EMBED's lane `embed/trace-cylinder` (2026-11-21), tracing the
cylinder buckets of the per-solid census (`measure/gate/census-c2a0c41f40.json`
on `embed/gate-measure`). **20 tests** of the cylinder/cylinder edge
bucket, and likely most of the 9 slow corpus rows of that bucket not
traced (they time out under the trace): every corpus row that
evaluates the document `coincidence_door::a_bushing_stacked_on_its_own_axis_glues_its_walls`
builds (`m4_pr8_corpus` 3, `display_budget` 3, `names_verbatim_edge_evaluator`
2, `m10_p_fence` 2, and 10 more corpus rows of `editor-core::all` and
`viewer::all`). They all refuse at the same cell pair.

- **The pair.** Cylinder `1v19` against cylinder `3v1` across one shared
  edge, the circle where the two coaxial walls of equal radius meet; no
  vertex of the shared cell in reach. Both faces lie on one carrier.
- **The cells, at depth 40.** `1v19`: `u ∈ (0.288388, 0.294524)`,
  `v ∈ (1.0, 1.000977)`, presented `Of(Side(YLo))`, the circle at
  `v = 1`. `3v1`: `u ∈ (0.294524, 0.300660)`, `v ∈ (0.999023, 1.0)`,
  presented `Of(Zero::None)`: no side of its rectangle is listed as the
  zero, though its top side lies on the circle.
- **The arms.** The carrier arm cannot certify (each wall lies in the
  other's carrier, so neither departs it). The circle's own plane
  (`planes_through`) signs `1v19` (`across = 9.77e-4`, `Positive`) and
  gives `None` on `3v1`: with `Zero::None`, `sign_on` reads `ψ` by
  `centred` over the whole rectangle, which reaches `ψ = 0` on the
  side. The graph separator reads the same. The pair splits to
  `Budget(Depth)`, reported `Uncertified { arm: Departure }`.
- **One level down: not traced.** Why `3v1` lists no zero side is the
  cause, and the trace does not print its clips on this path. `zero_of`
  lists a clip as zero only when every edge on it is shared with this
  partner, so a line `v = 1` holding a second edge would read so; but no
  vertex is in reach of the pair (`near.vertices` empty), which argues
  against a split of the circle at the cells' common side
  `u = 0.294524`. Unchecked either way.

## Valid or in band

Valid: a bushing unioned onto its own axis, two walls on one cylinder
meeting along a circle (an edge the result keeps), a finished solid
its test builds as an ordinary result.

## Evidence

- Cheapest reproducer: `editor-core::all coincidence_door::a_bushing_stacked_on_its_own_axis_glues_its_walls`
  (0.31 s).
- Run on `embed/trace-cylinder-measure` with `CV_TRACE=1` (`CVT`,
  `CVG` rows): every one of the 20 traced tests prints the same two
  cells and the same `(Some(Positive), None)` under both the plane and
  the graph separator.

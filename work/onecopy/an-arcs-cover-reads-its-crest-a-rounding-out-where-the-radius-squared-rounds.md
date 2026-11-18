---
id: an-arcs-cover-reads-its-crest-a-rounding-out-where-the-radius-squared-rounds
kind: issue
title: An arc's cover reads its crest as sqrt(p² + q²), one or two ulps past the evaluation's r·cos where the radius's square rounds, so a window read off it halves a rounding off the arc
status: open
opened: 2026-11-18
priority: P3
cost: E
---

## Finding

Filed by EMBED's lane `embed/rational-window` (2026-11-18, PR 7120),
which reads a face's clearance window off its edges' covers
(`topo::clearance::boundary_of` through `cover::arc_cover` and
`Cover::box_in`).

`ArcWindow::extent` (`crates/geom/src/curves/boxes.rs`) puts an arc's
crest at `c ± √(p² + q²)`. At `Interval`, `p²` rounds wherever the
radius's square is not a float, so the crest reads one or two ulps out:
a circle of radius 0.9 about the origin in `z = 0` reads `y` up to
`0.9000000000000002` over `[0, π]`, where its interval evaluation reads
`0.9` (radius 0.7: `0.7000000000000001`; 1.3: `1.3000000000000003`).
Radii whose square is exact (½, 1) read bit for bit as the evaluation.
Pinned in `crates/geom/tests/curves/cover_box_against_the_whole_span_eval.rs`
(`at_a_crest_of_an_inexact_square_the_cover_reads_at_most_two_ulps_out`).

**What it costs.** It is sound (wider, never narrower). But a window read
off it moves every dyadic halving off the arc's own bound, and EMBED
measured a window an ulp wider turning a holding body into a refusal
(`work/embed/a-cell-whose-side-lies-a-rounding-off-a-curved-edge-reads-local-at-every-depth.md`:
the drum family, and the tour's `budfillet` under a padded read). On the
tour walk at f64, 89 window ends read 1–4 ulps wider than main's
evaluation (vase, sheave, the teapot and others), the size this
rounding gives, not traced edge by edge; no verdict moved.

**A fix to weigh.** An amplitude read exactly where one of `p`, `q` is
zero (an axis-aligned direction, the case every world-box row is), or a
hypot that rounds once. Neither is measured here.

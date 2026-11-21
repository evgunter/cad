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
rounding gives, not traced edge by edge; no verdict moved. That is a
one-time figure (2026-11-18, `embed/rational-window` against main
`c5bc653ae`): main's `demos/tour/examples/embed_walk.rs` prints verdicts
and receipts, not windows, and the count came from an uncommitted print
of each window in `topo::clearance::window_of`, diffed run against run.

**A fix to weigh.** An amplitude read exactly where one of `p`, `q` is
zero (an axis-aligned direction, the case every world-box row is), or a
hypot that rounds once.
- **Measured once, by PR 7120's review** (its mutant M6, on
  `embed/rational-window-review`): `crest = √(p² + q²).min(|p| + |q|)`,
  the meet of two sound bounds, which is exact on the world axes. The
  `geom` cover rows stay green except the pin of this rounding
  (`cover_box_against_the_whole_span_eval`'s
  `at_a_crest_of_an_inexact_square_the_cover_reads_at_most_two_ulps_out`,
  whose last assertion goes red, as it says it will), and no verdict
  moves in the drum family: radii 1 and 2 hold, and 0.3, 0.7, 0.9, 1.1
  and 1.3 still refuse (234 / 282 / 140 / 210 / 140 cells).
- **Both readers compute the amplitude.** `ArcWindow::extent` (the
  crest's value) and `ArcWindow::crest_outside` (the selector deciding
  whether the crest is inside the span) each compute `√(p² + q²)`. A
  fix to one alone desynchronises the selector from the value; they
  change together.

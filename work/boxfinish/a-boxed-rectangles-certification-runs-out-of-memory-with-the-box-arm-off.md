---
id: a-boxed-rectangles-certification-runs-out-of-memory-with-the-box-arm-off
kind: issue
title: The symbolic tier's shape report renders a blocking form without bound: a boxed rectangle's replay runs out of memory with the switch's box arm off (0.18 s with it on)
status: closed
opened: 2026-11-24
priority: P1
cost: E
branch: boxfinish/report-bound
closed: 2026-11-24
pr: 7704
---


Found by M10 and measured by M11 (the probe lane, 2026-11-24;
`probe/M11-REPORT.md` on `boxfinish/probe-switch-off` at
`e37099a49a`), on main after PR 7394 with the switch's box arm off.

`editor-core`
`a_boxed_rectangle_certifies_over_its_box::a_pen_drawn_and_a_cornered_rectangle_certify_over_a_boxed_width`
is killed for memory with the box arm off (245 s inside the suite;
under a 5 GB limit it aborts at 85 to 90 s) and passes in 0.18 s with
it on.

**It is not the drive.** The three drives before the stall finish (1
leaf; 3 leaves, 2 certified after one split; 1 leaf), and the
session's tables and the drive memo stand still while memory grows at
about 45 MB/s. What grows is one call of the test's `replay` helper
(`m10_8_arc_family_interval::replay`), which installs the shape report
(`geom_core::sym::start_shape_report`): on the first undecided ray
decision of the extrude's finish (check 7, `side_at_infinity_edge`,
for `PenRadians` at half 0.05) the report renders the blocking form as
text. Two stack samples thirty seconds apart are one frame chain:
`report::render_poly <- render_form <- render_indet <- render_poly <-
... <- report::render_node <- Sym::decide_sign <-
solid_contain::edge_meets`. The renderer recurses through each atom's
argument and builds one `String`; `explain_depth` limits the report's
explanation, not `render_node`'s forms.

**Why only with the box arm off:** with it on, that replay runs no
finish at the box scalar, so no ray is cast and the report has nothing
to render (measured for the tour and the a0 plate; inferred here).

**Not determined:** whether the rendering ends at all above 5 GB; the
row's other two forms (`Corners`) are never reached.

**The fix is on SYM's ground** (`crates/geom-core/src/sym/report.rs`):
the renderer bounds what it writes (a form past a size is named by its
size and its head, not spelled), with a row that a report over a deep
form ends in bounded memory. Then the rectangle row is re-run with the
box arm off to see what it reaches.

It holds the switch's deletion: a row that cannot run cannot be
re-baselined. (BOXFINISH orchestrator)

## Closed

PR 7704 (2026-11-24). One decision's three texts take at most
`RENDER_BUDGET` (768 KiB, a third each) in `geom-core`'s
`sym/report.rs`: a form past its share is written at the deepest
nesting that fits with each unspelled atom named by head, atom id and
argument size, and a text that is cut counts what it leaves. Rows in
`crates/geom-core/tests/shape_report_render_budget.rs`, each red under
a named mutant.

**The acceptance:** on the probe tree (the box arm off) with the head's
renderer, under a 5 GB limit,
`a_boxed_rectangle_certifies_over_its_box::a_pen_drawn_and_a_cornered_rectangle_certify_over_a_boxed_width`
is green in 10 s at 291 MB peak, where it was killed at 85 to 90 s.

**Left, each in its own file on SYM's slate:** a replay's report is
bounded per decision and not as a whole
(`work/sym/a-replays-shape-report-is-bounded-per-decision-and-not-as-a-whole.md`);
two depth-limited atom walks of the ladder have the same shape and
cost time, not memory
(`work/sym/two-depth-limited-atom-walks-have-no-memo-and-cost-the-product-of-the-fan-out.md`).
The experiment row is `docs/model-compare-log/rows/PR-7704.md`.

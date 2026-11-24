---
id: a-replays-shape-report-is-bounded-per-decision-and-not-as-a-whole
kind: issue
title: A replay's shape report is bounded per decision and not as a whole: a document with thousands of blocked decisions on deep forms holds gigabytes
status: open
opened: 2026-11-24
---


Left by BOXFINISH's report-bound unit (PR 7704, 2026-11-24), which
bounded what the shape report writes for ONE decision
(`crates/geom-core/src/sym/report.rs`, `RENDER_BUDGET`: 768 KiB, a
third each for the plain form, the early form and the explanation).
The report itself (`SHAPES`, handed out whole by `take_shape_report`)
keeps every decision's texts until it is taken, so a replay holds up to
its blocked decisions times that budget.

**How close a real document comes.** On the boxed rectangle with the
switch's box arm off
(`a_boxed_rectangle_certifies_over_its_box::a_pen_drawn_and_a_cornered_rectangle_certify_over_a_boxed_width`,
the row that unit was opened for) 23 of the 26 decisions rendered
reach a text's share, and the row peaks at 291 MB resident. A document
whose replay blocks a few thousand decisions on forms of that shape
(a handful of terms over atoms nested with a fan-out) would hold
gigabytes, which is the failure that unit was opened for, reached by
count and not by one text. On the documents the readers replay today
the report is far from it: 1,692 decisions, 53.7 MB in all on main's
unbounded renderer, the largest text 1.76 MB.

**Who holds the texts beyond the report** (every reader is a test;
no `src` calls `take_shape_report`):
- `editor-core/tests/m10_8_arc_family_interval.rs`, `replay` /
  `replay_retried`: returns the `Vec<DecisionShape>` whole. One copy,
  the report's own.
- `editor-core/tests/parallel_node_map_interval.rs`, `row`: formats
  each decision's plain form into a second string and keeps the rows
  of the serial run beside each threaded run's. Two to three copies of
  the plain forms.
- `editor-core/tests/m10_10_evidence_interval.rs`, the JSON dump:
  writes one decision a line to a file; holds no second copy.
- `sweep/tests/continuation_is_thread_count_invariant.rs` and
  `mass_props_are_thread_count_invariant.rs`: read the report's length
  only.

**What would settle it:** a budget on the report (the texts of one
`start_shape_report` .. `take_shape_report`), past which a decision
records its outcome and sizes without its texts and the report says
how many it left bare; or texts rendered on demand from a kept node
id, which needs the session alive at the read. Which, is SYM's call:
the report is its instrument.
(BOXFINISH report-bound lane)

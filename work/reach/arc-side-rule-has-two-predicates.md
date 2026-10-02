---
id: arc-side-rule-has-two-predicates
kind: issue
title: The chord's arc-side rule is the azimuth window on a monotone section and the run side on a tilted sphere section, where the chart-free one could serve every conic
status: open
opened: 2026-10-02
priority: P1
cost: M
design: true
---


## What

Found by the `reach/tilted-sphere-pair` lane, which added the second.
`chord_join` selects a chord's arc of the section conic two ways:

- `select_arc` — azimuth-window containment (M5 S9), where the conic's
  chart azimuth is monotone (`SectionConic::azimuth_monotone`: every
  cylinder conic the table admits, and a polar sphere section);
- `select_arc_by_run_side` — the arc leaves each run end on the run's
  left under the face's outward normal, for a sphere section tilted
  against the chart, whose azimuth doubles back.

The second reads no chart, so it would select the arc on every conic
the wall lane mints. Two predicates for one rule is the P1 shape. The
boolean's planar side (`JoinLane::BoolPlanar`) reads the mate wall's
window by value and has no run-side form yet
(`planar-side-of-a-tilted-plane-sphere-cut-has-no-arc-cue`).

## The question

Fold the window rule into the run-side one (every cylinder cut and
polar sphere cut then re-selects by the run; bits should not move, but
the `split_arc_window` and `bool_between_arc_window` rungs and the
`BoolPlanar` window plumbing lose their consumers), or keep the window
rule where its premise holds and say what it buys. The planar side's
cue is the same question one lane over. The run-side rule's
own open premise is `run-side-arc-rule-reads-only-the-run-at-each-end`.

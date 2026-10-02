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

## A third reading, on the split lane (CLEAVE, PR 3718)

CLEAVE's conic pairing (`splitting::join`'s `conic_pairs`) walks a
curved face's section conic in the direction `n_plane × n_out` and
pairs each entering crossing with the next one. The arc from an entry
to its exit in walk order is the arc inside the face. So on the split
lane the pairing already knows which arc each chord should take, and
`chord_spec` then derives it a second time, by azimuth window or by
run side. The three readings agree wherever each one reads: each names
the arc that leaves its ends into the face, and the run-side rule
refuses rather than choose at a corner where it cannot see that.

Today they never meet on one face. The split lane refuses sphere faces
at its reduce, and only tilted sphere sections take the run-side rule
(pinned by `crates/sweep/tests/tilted_sphere_pair.rs`,
`a_tilted_split_of_a_sphere_body_refuses_before_either_arc_rule`).
When the split lane admits sphere faces, the unification this item asks
for could hand the walk's arc to `chord_spec` instead of re-deriving it.

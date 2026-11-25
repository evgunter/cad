---
id: a-nan-distance-reaches-a-band-read-on-a-tilted-axis-sphere
kind: issue
title: a tilted-axis sphere's import reads a distance of NaN in the band: a NaN reaches a band decision
status: open
opened: 2026-11-25
---


Found by the dual review of `lathe/import-wrap-tip` (PR 7774, R1's
N1). It predates that branch (the reviewer's reading). Not reproduced
in that lane: the reviewer's witness file is not committed.

## What

Importing a sphere whose axis is tilted off the coordinate axes, the
review saw a refusal text of the shape "the distance NaN m (…) lies in
the band the file is read at". A NaN distance reached one of import's
band reads (`interpret::FileBand::read` or a sibling) and was reported
as undecided, where a NaN is no distance at all. A band read must
refuse a non-finite input as the defect it is (fail loud, naming what
produced it), or never be handed one.

## What would close it

The producer of the NaN found (likely a normalization of a vector that
vanishes, or an `acos` of a value a rounding past ±1, on the tilted
chart). Then a row pinning the tilted sphere's import, and the band
reads refusing a non-finite input distinctly.

- 2026-11-25 — witness committed: `crates/step-import/tests/fixtures/wrap-tip-review/hole_on_a_tilted_axis_sphere.step`
  (R = 10 mm, sphere axis (1,1,1)/√3 and reference (1,−1,0)/√2, a cap of
  angular radius 0.5 about +x cut away by a plane; closed form
  4.143631343316944e-6 m³). Both the one-band stack and `lathe/import-wrap-tip`
  refuse it with "entity #133: the distance NaN m (step_built_sense) lies
  in the band …": the NaN reaches the `step_built_sense` read. Pinned red
  at every ε by `crates/step-import/tests/wrap_tip_review_witnesses.rs`,
  `a_hole_on_a_tilted_axis_sphere_imports_at_its_closed_form`.

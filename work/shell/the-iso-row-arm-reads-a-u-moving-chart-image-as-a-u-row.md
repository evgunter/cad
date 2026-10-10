---
id: the-iso-row-arm-reads-a-u-moving-chart-image-as-a-u-row
kind: issue
title: replace_face's iso-row arm reads any IsoLine chart image as the u-row at p0.x, but chart_image mints images that move u, so a v-row is planned as the wrong row
status: open
opened: 2026-10-09
priority: P2
refs: [a-moved-fitted-faces-corners-have-no-root-on-a-derived-spline-section]
---

Found by SHELL's fitted-corners unit, measured on a scratch fixture not
kept in the tree: the unit box with its cap swapped for a bilinear NURBS
patch over `[0, 2]²` at `z = 1`, each cap edge re-described as
`EdgeDescriptionSpec::chart_image(cap, Pcurve::IsoLine { p0, pl })` on the
patch's own key (the shape `crates/sweep/tests/common/approx.rs:234`'s
`box_with_approx_cap` writes), `mint_pcurves`, valid at rest. Moving the
cap by ±0.05 through `topo::replace_face_offset`:

- `offset_edge_plans_for_tests` plans all four edges `Ok(None)` through
  the iso-row arm in `plan_edge` (`crates/topo/src/replace_face.rs:2095-2171`,
  "The one description that gets an EXACT carrier"; the premise at
  :2104-2107).
- The door refuses `ReanchorPastCarrierEnd { gap: 2.0 }` on a vertical
  side edge: a corner landed two metres off.

The arm's premise reads "An iso image on a DESCRIPTION is u-const by
construction — `EdgeDescriptionSpec::iso` is the only door that mints
one". `EdgeDescriptionSpec::chart_image`
(`crates/geom-brep/src/description.rs:283`) is public and mints any
`IsoLine`, so the two cap edges along `x` carry `pl = (0.5, 0)`. The arm
reads them as the `u`-row at `u = p0.x` with `v0 = v1`, extracts that
row from the new fit, and plans its ends as the corners. A later gate
caught it here, with a refusal that names the re-anchor, not the misread.

The same fixture over `[-1, 3]²` (rows interior to the fit) refuses
`IsoRow { error: Interior { u: 0.25 } }` at every edge, where the same
edges with stale descriptions derive as sections with the held planes
and the move builds (`crates/sweep/tests/encl_curved_loft_shell.rs:719`,
`a_moved_fitted_cap_stands_its_corners_on_the_held_sides`).

Owed: the arm takes a `u = const` image only, and an image that moves
`u` — or an interior row beside a plane — goes to the section route or
refuses by its own name.

Why it was not fixed where it was found: deciding `pl.x` zero at a
generic scalar is a decision on a chart slope with no stated margin. The
unit's review answered that this reason is thin: an exact guard, `pl.x
!= 0` refusing by name, can only refuse more than the arm does today —
it never takes a row the arm would not — so it needs no margin to be
sound. The guard is owed here either way; the margin question is only
whether a slope in band should take the section route rather than
refuse.

---
id: a-ruling-read-in-band-meets-consumers-that-read-no-projected-image
kind: issue
title: A ruling read in band now images projected, and three consumers downstream refuse it (the run walk, the planar trim inventory, props' cylinder inventory)
status: open
opened: 2026-11-03
---


Filed by PCERT unit E2 (`pcert/cylinder-line-lane`), from its sweep of
the readers of a cylinder line's image.

## What

Since E2, a line on a cylinder whose ruling reading
(`pcurve_chart_radial_moving`) is in band, or definite on a line the
incidence read does not show off the wall, images on the projected lane
(`projected::FramedCarrier::Line`) rather than as the meridian. Before
E2 the in-band reading folded onto the meridian form. Two consumers
downstream of the mint read a cylinder wall's boundary images and have
no arm for a projected one, and a third reads the line itself:

- `topo::chord_join`'s run walk (`chart_azimuth_range`, the
  `Pcurve::Projected(_) | Pcurve::General(_)` arm): a run bounded by a
  projected image ends `SplitJoinError::RunImageNotYet`. That is
  "not yet", the right ending for a lane that is not written, and the
  same one a sphere circle's in-band class reading meets today.
- `topo::chart_region`'s planar trim inventory (`pcurve_entry`, the
  `Pcurve::Projected(_)` arm, "Projected image is not a straight
  segment"). A projected line's chart image is `atan2` of an affine
  map, so it is not a straight chart segment, and the refusal is
  honest. A face bounded by an in-band ruling still loses the region
  query the meridian's straight segment answered.
- `geom_brep::props::curved`'s cylinder inventory (`cylinder_boundary`,
  `props_meridian_axial`), reached through tier 3's volume. A line on a
  cylinder face must read as a ruling there, or the face is not an
  iso-rectangle (`Premise::Inventory`). The E2 review measured it: an
  imported cylinder seam leaning `δ ∈ [1e-9, 5.6e-5]` now passes
  adoption, its row taking the projected lane, and refuses at tier 3's
  volume, where main refused it one door earlier, at the mint. It still
  refuses, so this is not a regression, but the face is lost one door
  later. Props also reads the ruling question in other units than the
  mint (`work/pcert/the-ruling-question-is-read-in-two-units.md`).

No row the unit ran reached the first two arms through a cylinder line:
`m10_bulge_interval`'s `pcurve_chart_radial_moving` pins did not move,
and the per-PR suite stayed green on E2's PR.

## Fix

The run walk's lane for a projected image: its azimuth box is
`ProjectedImage::chart_box`'s `u` range, placed by the deck map. The
trim inventory's arm for a projected line: a chord bound on the image
(the line's azimuth sweep is `O(tilt·span/R)`), or a polygon edge with
its deviation charged. Props' inventory: its ruling reading's lane
choice, once the ruling question has one home (the two-units row).
Each fix needs a row with an in-band ruling on
the face's boundary.

## Circles and cone lines reach them too (E4)

Since PCERT E4 (`pcert/circle-general-lane`), a circle on a cylinder,
cone or torus that the class readings do not show one of the chart's
own circles (a cone circle that is no rim, a torus crest circle, any
in-band class reading), and a cone ruling whose nappe reads in band,
also image on the projected lane, where before they refused "not yet" at
the mint. They meet the same three consumers:

- the run walk ends `RunImageNotYet`, the ending their mint refusal
  already took (`chord_join::run_image_refusal`);
- the trim inventory refuses "Projected image is not a straight
  segment", where before the face refused one door earlier;
- props' curved inventory reads a face's boundary carriers themselves
  (`props::curved`'s `cone_boundary` through `require_cone_circle` and
  `require_cone_generator`, `torus_classify`), so a face bounded by a
  circle that is none of its chart's, or by a cone line not shown a
  generator, refuses there (`Premise::Inventory`, or its escalation in
  band), again one door later than before.

`topo::replace_face`'s offset arm answers `None` for a projected image
(re-mint), which is right for a circle too. E4's per-PR runs (geom-brep,
topo, sweep at ε 1e-9) reached none of the three through a circle: no
row moved there.

## The cylinder volume is read (2026-11-04)

PR 6478 (spiricarm, the rotational-chart lane) gave tier 3's volume a
lane on the cylinder chart that reads a projected chord
(`geom_brep::props::quad::rotational`, `ProjectedEdgeQ`). That closes
the third consumer above on the cylinder. Its witness,
`sweep bisector_nodes::the_near_node_poses_at_the_witness_band`
(`work/pcert/main-red-a-near-node-cap-loses-its-volume-to-a-projected-ruling-image.md`),
went green at that merge. Props on the sphere, cone and torus charts
is still open; spiricarm's stages 2 and 3 cover it.

The PCERT main-reds lane re-swept the consumers for `Pcurve::Projected`
over `crates/*/src` at `143d99ec29`, plus a second pass for `_ =>` arms
beside a `Pcurve::Harmonic` arm, which can reach a projected image
without naming it:

- `topo::chart_window`'s run walk (`chart_azimuth_range`, now in
  `chart_window.rs`): still `RunImageNotYet`, as above.
- `topo::chart_region::pcurve_entry`: still "Projected image is not a
  straight segment", as above.
- `topo::props::quad_lane`: the NURBS-face arm refuses a projected
  image as corrupt, which is right because no spline chart mints one.
  Its wall-image read (`WallImage::Other`, from the `_` arm) carries a
  projected image as no closed form.
- Not named above: `topo::clearance`'s `monotone_line` and `iso_of`
  read a ruling's meridian `Harmonic` form as an iso line, and a
  projected ruling as none (the `_ => false` arm). That is
  conservative, because the clearance takes no iso shortcut for it, but
  a face bounded by an in-band ruling loses that shortcut.
- The mesher (`mesh::trimmed`, `mesh::chords`): it admits a projected
  image on an analytic chart (sampled at the chord pass's parameters),
  and refuses it on a spline chart as corrupt.
- `topo::replace_face`: re-mints, as above.

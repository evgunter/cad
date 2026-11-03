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

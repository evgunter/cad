---
id: a-ruling-read-in-band-meets-consumers-that-read-no-projected-image
kind: issue
title: A ruling read in band now images projected, and the run walk and the planar trim inventory read no projected image
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
no arm for a projected one:

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

No row the unit ran reached either arm through a cylinder line:
`m10_bulge_interval`'s `pcurve_chart_radial_moving` pins did not move,
and the per-PR suite stayed green on E2's PR.

## Fix

The run walk's lane for a projected image: its azimuth box is
`ProjectedImage::chart_box`'s `u` range, placed by the deck map. The
trim inventory's arm for a projected line: a chord bound on the image
(the line's azimuth sweep is `O(tilt·span/R)`), or a polygon edge with
its deviation charged. Each fix needs a row with an in-band ruling on
the face's boundary.

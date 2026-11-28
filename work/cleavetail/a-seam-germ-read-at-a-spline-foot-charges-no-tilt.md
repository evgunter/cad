---
id: a-seam-germ-read-at-a-spline-foot-charges-no-tilt
kind: issue
title: The seam germ read against a spline holding face charges no tilt for its normal, read at a cell-centre foot up to a band from the chord's point
status: open
opened: 2026-11-27
priority: P3
cost: M
---

Filed by the LATHE teapot lane on PR 7824's review (R1, 2026-11-27).

## What

`boolean::join::seam_germ_side` reads a section chord's side from
`into_region · n_f`, levered by the chord's extent (`germ_side`). Where
the holding face is a spline face, `n_f` comes from
`join::spline_held_normal`, at the in-trim foot
`contain::face_placement_charted` returns.

The tilt is not charged:

- That foot is the centre of the first Bézier cell the placement
  decides within the band of the chord's point `q`
  (`spline_region::read`). It can lie up to about one band from `q`'s
  own foot.
- So `n_f` can be tilted by about `κ · band` from the normal at `q`.
- `germ_side` charges no such tilt: its margin is the dot alone.
- `face_normal::spline_outward_normal_at` also drops the curvature arm
  `geom_brep::chart_point_normal` returns (`.map(|(normal, _)| normal)`),
  so no caller can charge it.

On the teapot every germ is decided far from zero. R1 checked all 48
readings against `point_in_solid` at three step sizes, and they agree.
But a germ within `κ · band` of zero would read a side the tilt could
flip.

## What it needs

`spline_outward_normal_at`, or the shared door
(`onedoor/three-readers-of-a-spline-foots-normal-share-no-door`),
returns the arm `chart_point_normal` already computes. `germ_side`
widens its margin by the normal's tilt over the foot's distance from
`q`, so a germ inside that tilt reads nothing rather than a side.

---
id: projected-turn-reads-short-at-a-window-wide-azimuth-inf
kind: issue
title: Projected check 2 reads Short at a window-wide azimuth inf, so a sweep short of a turn escalates where an inf over the defect would decide it
status: open
opened: 2026-10-25
priority: P3
cost: E
---


Split from `turn-rule-reads-short-at-the-global-speed-floor` (its "More
evidence" section, the projected site), which branch
`pred/turn-rule-follow-ups` closed for the carriers: certification's
check 2, `ConicArc::of` and the cone's ellipse door now read `Short` and
`Over` at `geom::carrier_speed::turn_defect_speed_lo`, the inf over the
turn's defect.

`geom-brep/src/pcurve_cache/projected.rs` check 2 is not a carrier's
turn: its extent is the hull of a chart image's azimuth sweep
(`ProjectedImage::sweep_box`), and its inf is
`pcurve_cache::azimuth_lever_inf` over the whole box's `v` window (the
sphere's `r·cos v` at the window's largest `|v|`, the torus's `R − r`,
the cone's `v_inf·sin α`). A sweep short of a turn whose headroom that
inf reads in the band escalates where the sup-only gate admitted it
(`projected/turn_rows.rs`, the `±3`/`±5` of the inf's and `±20`/`±30` of
the sup's cases). The site asks only "not over", and admits a sup that
reads the headroom definitely positive
(`TurnUndecided::not_over_at_sup`), so what escalates is the straddle
the inf cannot settle.

The local twin here is the azimuth lever's inf over the `v` the image
takes where its azimuth is within the headroom of its two extremes (the
stretch the defect is in chart terms), not over the whole box. That
needs the image's pieces near the sweep's ends, which `sweep_box`'s
cover has, so it is a projected-lane change rather than a
`carrier_speed` one. P3: the escalation is sound.

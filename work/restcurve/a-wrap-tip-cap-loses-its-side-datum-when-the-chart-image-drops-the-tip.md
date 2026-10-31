---
id: a-wrap-tip-cap-loses-its-side-datum-when-the-chart-image-drops-the-tip
kind: issue
title: A wrap-tip cap's PR 4549 side cross-check goes silent: the chart image drops the tip, taking the pole vertex off the loop, so the side reader answers Unencoded and a flipped small cap passes tier 3
status: dispatched
opened: 2026-10-30
priority: P1
cost: M
branch: restcurve/tip-side-datum
---

Found by designer A of the pole-form fork (fork-log row 222, on `[ev]` PR 6085, 2026-10-30).

PR 4549's side rule says a one-loop sphere face lies on the side of its loop that holds no pole (`geom_brep::props::curved` `pole_free_side`). The rule needs the pole off the face to decide the side. Under the wrap tip, `chart_image` (PR 6088) drops the tip as a spur. That takes the pole vertex off the loop, and the rim is left with a pole on each side. `agreed_side` then answers `Unencoded`, and check 6 falls back to reading the seam region. So a flipped small cap would measure its complement and pass tier 3.

**Fix:** the side reader keeps each dropped tip's singular vertex as the datum of the face's side. The rule becomes: the face lies on the side that holds its own pole vertices and not the others. That is the DESIGN sentence proposed on PR 6085 ("keeping the singular vertex as the datum of the face's side").

Nothing is live today, because no door mints a tip. This is a condition on the wrap-tip unit (`tier-3-admits-a-wrap-tip-only-on-its-faces-chart-singularity`), and it lands with or before the first door that mints a tip. The row it needs is a flipped small wrap-tip cap that refuses at tier 3. (RESTCURVE orchestrator)

## Released (2026-10-31)

Ev ruled on `[ev]` PR 6085, decision 3 (fork-log row 222): keep the wrap tip. A tip is any valence-1 end of a wrap edge, and the wrap edge is the face's seam kept to its end. A chart reader reads a tip as the seam, through the lift's reset at the singular vertex. A chart-free reader drops it, keeping the singular vertex as the side's datum. The hold is lifted. (RESTCURVE orchestrator)

## Evidence from PR 6097's fix pass (2026-10-31)

The side datum itself is not built there. A flipped small wrap-tip cap
(the unit sphere above y = 0.8, its two pole meridians made one wrap
edge) does refuse at tier 3, but not on the side reader. The
chart-singularity arm finds the far (south) pole inside the flipped face
(`ChartSingularityNotAVertex`, beside `PinchCornerCrossed`), and
`CurvedSenseInverted` does not fire. That row is
`a_flipped_small_tip_cap_refuses_at_tier_3` in
`crates/sweep/tests/a_wrap_tip_sits_on_its_faces_chart_singularity.rs`.
The silent side reader this row names is still silent. (RESTCURVE
implementer)

---
id: a-minted-refusals-provenance-is-guarded-only-by-its-doc
kind: issue
title: The lane relabel's mint provenance is guarded only by minted_refusal's doc: a caller that hands it a stored row ends a wrong row 'not yet'
status: open
opened: 2026-11-04
priority: P2
cost: M
---


Filed by PCERT unit E4 (`pcert/circle-general-lane`, PR 6485), from
the delta verifier's fix list X (X2, Q7).

## What

The projected lane relabels its refusals ("not yet", `LaneGap::Pose`
or `LaneGap::Envelope`) only when the image was just derived for the
edge. A stored row's envelope refusal says the row is wrong, and has
to keep its own term (`a_stored_rows_envelope_refusal_keeps_its_term`,
`a_foreign_stored_row_never_ends_not_yet`). One home does the
relabel: `general_lane_refusal`
(`crates/geom-brep/src/pcurve_cache.rs`). Two places reach it:
`chart_image`, for the projection's own refusals, and the `pub`
`geom_brep::minted_refusal`, for the certificate's refusal of the
mint's fresh image.

Nothing but `minted_refusal`'s doc says the image must be fresh. Its
callers are topo's mint walk (`certify_walked`,
`crates/topo/src/pcurves.rs:3219`; `read_loop`'s mint arm,
`pcurves.rs:5085`) and `tests/pcurve_villarceau.rs`. The next caller
that hands it a stored row would end a wrong row "not yet", a lever
offer, rather than as the contradiction it is. That repeats E4's M1
from the type's side.

## Why not fixed in E4

A provenance type (`chart_pcurve_over` returning a `Fresh`/`Minted`
wrapper that `minted_refusal` takes) does not stay local. The walk
moves the minted image before certifying it (`shift_v_branch`, the
sphere's twin), and `Walked::pcurve` also holds images the walk builds
itself (an `IsoLine` at the seam). So the wrapper would have to thread
through every `Walked` construction in `topo::pcurves`, and through
the branch shifts that keep it fresh, which is wider than E4.

## Fix direction

Carry provenance on the walked row (`Walked::fresh`, or a
`Walked<Minted>` / `Walked<Stored>` split) and have `minted_refusal`
take that row, not a bare `Pcurve`. That way a stored row cannot reach
the relabel without being constructed as minted.

## The widened relabel's residual risk

Recorded by the orchestrator from E4's delta verifier, on `a4f292287c`.
`general_lane_refusal` relabels every `SectorRefused` channel of the
mint's fresh image as `LaneGap::Pose`: Branch, Azimuth and Tube, as well
as Lever. On a truly fresh image, a check-4 `SectorRefused { Azimuth |
Branch }` means the mint's partition and its own certificate disagree.
That is either an in-band split, which correctly ends "not yet", or a
mint defect, which this relabel would now hide behind "not yet". The
row `a_lane_choices_azimuth_refusal_ends_not_yet` reaches the azimuth arm
only through a hand-edited image the mint never produces. A fix for this
row should also tell those two cases apart.

## 2026-11-10 — a stated General image reached the relabel (PCERT, PR 6532 review FF1)

PR 6532 widened `minted_refusal` to a bending carrier's `General` image,
and its review executed this row's hazard. A construction's stated
`General` row, its control 15 bumped `1e-4` in `v` so that every
check-3 sample still holds, reached the mint's own-chart arm
(`nurbs_iso_derive` returns a stated `c.pcurve` verbatim).
`certify_walked` then mapped its `FittedLimb { HullValueChart, 6.3e-5 }`
through `minted_refusal` and ended it "not yet".

The fix carries provenance on the walk's image, as data: a new
`geom_brep::ChartImage::Stated` holds an image a description or a stored
row states, which the derivation read and did not produce. The mint's
two relabel sites, `certify_walked` and `read_loop`'s mint arm, relabel
only a `ChartImage::Image` (derived in this walk); a `Stated` image keeps
its term. Rows:
`a_bending_carrier_on_a_spline_wall_takes_the_general_lane::a_stated_wrong_general_row_keeps_its_term_{f64,interval}`.
The mutant that relabels a `Stated` image reds them.

**This narrows the row but does not close it**, for `General` or for
`Projected`. `minted_refusal` is still `pub` and still takes a bare
`Pcurve`, so a caller outside the walk can still hand it a stored row.
The type-level fix this row names (`minted_refusal` taking the walked
row, or a `ChartImage`) remains.

## 2026-11-11 — the witnesses moved, and only the nightly runs them

PR 6815 made the edge door certify a stated image on a spline chart
through the mint's own door, so a wrong stated `General` row is now
refused at the build (`ChartImageUnavailable`), before the mint can
read it. The rows above were red on main for that reason, and only the
nightly showed it. They now reach the mint's relabel sites at rest: the
true row is stated, then its wall is re-parametrized under it. M5 reds
them again.

They cost 0.8–5.7 s, so they stay in the slow set. **This guard runs
only in the nightly**: a PR that moves the relabel sites in
`topo::pcurves`, or the edge door's stated route in
`geom_brep::certify`, without naming these rows, passes its gate with
the guard unrun.


---
id: iso-derivation-arms-assume-an-edge-spans-the-charts-whole-domain
kind: issue
title: nurbs_iso_derive's rim arms map an edge's whole interval onto the chart's whole u domain, so mint_pcurves refuses on any body whose spline-chart wall edge has been split
status: closed
opened: 2026-09-14
priority: P1
cost: H
branch: iso/arc-rim-sub-arc
pr: 5630
closed: 2026-10-23
---

Found by PR 2531's fix pass (TOPO, `split-edge-children-lack-pcurve-rows-on-curved-charts`)
and by both of that unit's reviewers, independently, by execution.

**The premise.** `crates/topo/src/pcurves.rs`'s `nurbs_iso_derive`
derives a described-NURBS wall's chart image from the edge's carrier
and the chart's own domain. Both rim arms read the domain END POINTS
and nothing about how much of the chart the edge covers:

- the **arc-rim arm** builds `Pcurve::IsoArc { p0: cu0-or-cu1,
  pd: cu1 − cu0, t0, angle: span, breaks }` — `p0 → p0 + pd` is the
  chart's WHOLE `u` domain, and `angle` is the edge's whole carrier
  span, so the map is "this edge's interval covers this chart's `u`
  domain";
- the **seam LINE arm** takes the neighbour's own moving channel
  verbatim and positions it at a domain end (`side_pick(&column,
  &[cu0, cu1])`), so the image it mints for a sub-edge spans the
  column the parent spanned.

For an edge the loft BUILT that premise holds. For a SUB-edge — one
child of a `Body::split_edge` — it does not, and the arms do not
notice: the arc arm's two candidate images are the whole domain in
each direction, a half-rim matches neither, and it refuses
`IsoUnsupported { what: "an arc rim whose chart image runs in neither
u direction …" }`; the line arm mints a full-column image, which the
loop walk then reports as `LoopDiscontinuity`.

**Measured** (fix pass, on `sweep::loft_body` prisms — the fixtures are
now committed as `crates/sweep/tests/split_edge_loft_charts.rs`):

| body | rows | `mint_pcurves` unsplit | after one wall-edge split |
|---|---|---|---|
| square-profile prism | 16 `IsoLine` | `Ok(())` | `Err(LoopDiscontinuity { half_edge: 25v1 })` |
| bulged-profile prism | 14 `IsoLine`, 2 `IsoArc` | `Ok(())` | `Err(Certify { half_edge: 3v1, error: IsoUnsupported { … } })` |

The unsplit control is the point: the pass has nothing against these
charts, and what it cannot re-derive is a row for an edge that covers
part of one.

**Why it matters now.** `split_edge` carries its children's rows since
PR 2531, so a split of a spline-chart wall is tier-3 valid without the
pass — the refusal costs nothing where the rows are already right.
What it still costs is every OTHER route to a sub-edge on such a
chart: any producer that runs the whole-body mint after surgery
(`topo::pcurves`'s module docs list them — the boolean pipeline, the
splitting lane, `shell`, `sweep`'s revolve/tube/loft/fillet,
`step_import`) refuses on a body it could have minted, and every
half-edge-minting Euler op's recovery step
(`work/topo/half-edge-minting-euler-ops-leave-a-minted-curved-face-incomplete.md`)
is that same pass.

**What a fix has to decide** is what a sub-edge's chart image IS: the
arms derive `p0`/`pd` from the domain because the edge's own extent is
not in evidence anywhere they read. The carrier's start and end chart
feet (`derive_chart_foot`, already used by the line arm's interior-seam
case) are the candidate measurement, and the arc arm additionally needs
`breaks` re-cut for the sub-arc rather than `uniform_breaks(spans)`.

## The line half moved (2026-10-13)

`pcert/off-iso-line-takes-its-general-row` moved the IsoLine half. In
the measured case the square prism's split sub-edge reached the mint's
LINE-RIM arm (a `Line` carrier in chart-image or scaffold form), not the
wall-seam arm named above. That arm now takes the boundary-iso
schedule: each closed-form candidate is judged a quarter span in, so a
whole-domain row no longer passes off the start corner, and the row
the end feet measure holds. `mint_pcurves` re-mints the split square
prism with every wall row an exact `IsoLine`, and
`sweep`'s `split_edge_loft_charts` is re-pinned to that.

**The IsoArc half is unchanged.** The arc-rim arm still offers only the
whole-domain images and refuses a half-rim `IsoUnsupported`; the
`breaks` re-cut this item names is still owed. The wall-seam arm's
column positioning is unchanged too (filed separately as
`work/pcert/the-wall-seam-arm-positions-its-column-off-the-start-foot.md`).

`pcert/arc-rim-row-pick` changed how the arc-rim arm picks its row,
not its map. Before, the arm picked the row by the carrier's START at
the chart's `u₀` corner, so the second half of a split rim (which
starts inside the `u` domain) refused for lying on no boundary
(`"the carrier's start point lies on neither chart boundary …"`). Now
the row and the direction are one pick at the interior probe, so both
halves reach it and are refused there, for the map this item names:
`IsoUnsupported { what: "no boundary row of this chart, in either u
direction, holds the circle at the interior probe — not an arc rim
spanning the chart's u domain" }`. `sweep`'s
`an_arc_rim_picks_the_row_it_lies_on::a_half_rim_reaches_the_pick_and_is_refused_there`
pins it; a fix here reds that row.

The arc rim's candidates (each boundary row × each `u` direction, the
whole domain) are a second spelling of `iso_schedule`'s closed-form
step, and the arc rim has no measured-map step after it: a rim that
spans part of the chart's `u` has nowhere to go but the refusal. That
fall-back is this item's IsoArc half.

## The arc half moved (2026-10-23)

`iso/arc-rim-sub-arc`. What a sub-edge's chart image IS was already
settled by `split_edge`'s carry: a child keeps the parent's `IsoArc`, the
row over the parent's arc `[t0, t0 + angle]`, and evaluates it over its
own span inside that arc; tier 3 certified those children. So the fix
is a derivation of that image, not a new map, and `breaks` keeps the
row's uniform sub-arcs (they locate `g` on the image's own arc, which is
still the whole row). `geom_brep::pick_arc_rim` now has the boundary-iso
schedule's shape: the closed form (`arc_rim_candidates`, the edge's
interval onto the whole row) and then a measured step
(`arc_rim_measured`), each judged by the same probe. The measured image
reads its arc off the row and the circle: `angle = m·2·acos(w)` from the
row's interior weight, `at0` the carrier's angle at the image's start
corner (an `Azimuth`), moved onto the branch nearest the middle of the
arcs that could hold the edge. The certificate checks it: it rebuilds
the circle's form from `at0` and `angle` against the row's control net,
and now decides the edge's span lies in the arc
(`pcurve_iso_arc_span`, closing
`work/select/an-iso-arc-certificate-never-checks-the-edge-span-lies-in-its-arc.md`).

A whole rim still takes the closed form, bit for bit. `mint_pcurves`
re-derives a split bulged prism (both halves, an off-centre split, a
double split, a rim against the chart's `u`), and tier 3 passes:
`sweep`'s `a_rim_covering_part_of_its_row_re_mints_the_rows_arc`. The
mesher's chord schedule still refuses such a child, filed as
`work/tess/a-split-arc-rim-meshes-no-chord-point-on-its-sub-arc-joints.md`.

## Closed (2026-10-23, PR 5630)

Both halves of the premise are gone.
- **The arc-rim arm** now has a measured step after its closed-form candidates. A rim covering part of its row takes the row's own `IsoArc` (the parent's arc), evaluated over the edge's span. The certificate decides that the span lies inside the arc (`pcurve_iso_arc_span`).
- **One judge for every iso arm:** each arm, line and arc, now judges its candidates by `geom_brep::probe_pick` at both ends of the span and a quarter in. A closed-form image whose ends sit on the domain's ends therefore cannot win on an edge covering part of it by passing through the carrier at one point.

Pinned by `sweep`'s `a_rim_covering_part_of_its_row_re_mints_the_rows_arc` suite, including the quarter-point families for both arms. Residues filed:
- `work/tess/a-split-arc-rim-meshes-no-chord-point-on-its-sub-arc-joints.md`;
- `work/restate/the-describing-door-refuses-a-wall-re-charted-reversed-in-u.md`.

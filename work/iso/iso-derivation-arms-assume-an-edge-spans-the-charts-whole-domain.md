---
id: iso-derivation-arms-assume-an-edge-spans-the-charts-whole-domain
kind: issue
title: nurbs_iso_derive's rim arms map an edge's whole interval onto the chart's whole u domain, so mint_pcurves refuses on any body whose spline-chart wall edge has been split
status: open
opened: 2026-09-14
priority: P1
cost: H
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
the row and the direction are one pick at the interior probe, and both
halves refuse for the map this item names: `IsoUnsupported { what: "an
arc rim whose chart image is neither boundary row in either u direction
— the rim does not span this chart's u domain along one" }`. `sweep`'s
`an_arc_rim_picks_the_row_it_lies_on::a_half_rim_is_refused_for_its_map_not_its_row`
pins it; a fix here reds that row.

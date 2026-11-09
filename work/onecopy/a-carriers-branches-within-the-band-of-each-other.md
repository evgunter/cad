---
id: a-carriers-branches-within-the-band-of-each-other
kind: issue
title: A carrier whose branches lie within the band of each other near a point certifies On on either branch: no door excludes an edge that comes within the band of itself
status: closed
opened: 2026-11-04
priority: P2
cost: M
closed: 2026-11-09
pr: 6618
---



Found by the FULL review of PR 6483 (`onecopy/foot-certificate`), its MINOR-1.

`Foot::on_carrier`'s `On { t }` certifies the point up to the band's zero: the carrier passes within the zero of it at `t`. It certifies the branch only where the carrier's branches are decided apart. Take a cubic hairpin with branches 3e-10 apart, inside the 1e-9 zero, and an `Interval` box of ±1e-10 about a point on the near branch, seeded on the far one. `on_carrier` reads `On { t = 0.92 }`: the seed's branch, whose tangent is reversed, with nothing logged. At `f64` the same probe reads `On { t = 0.08 }`. It is pinned as `geom::curves::on_carrier::rows::a_hairpin_closer_than_the_band_certifies_on_at_the_seeds_branch_at_interval`.

A reader of the branch's own geometry at `On`'s `t` (a tangent, a curvature, a side) can read the wrong branch's. `topo::census`'s `half_edge_tangent` is one: through `curved::param_on_interior` and `Span::read` it reads `deriv(t)`. `Span::read` refuses the far branch only when that branch lies outside the span.

Where I looked for a door that excludes such an edge, and found none:
- `geom_brep::certify`'s `SpanMeterCollapsed`: it refuses this cubic, at `h = 3e-10` and at `h = 2e-8` alike. Its tip speed is `1.5·h`, so its `speed_lower_bound` over the unit knot domain meters `1.5e-18` and `6.8e-15` m. The meter excludes only a fold whose speed sinks below it. A fold that stays fast still evades it: a semicircle of radius below the band, traversed over a short knot span between legs on long ones, keeps the meter well above the band while its branches lie within it;
- the census's curved edge×edge pass (5′): it reads pairs of edges, and does not read an edge against its own far parameters;
- `topo::injective` / `clearance`: their arms read face cells that touch, as the clearance assertion asks, not an edge's distance from itself.

Decide which applies:
- an edge that comes within the band of itself away from its parameter neighbourhood is a defect tier 1 (or the certificate) refuses;
- or the readers of a branch's geometry read it only where a certificate separates the branches (for example a curvature-bounded neighbourhood of `t` clear of the rest of the span).

The first is a gate on bodies, and the second is a reading. Either way, `Foot`'s doc already states what `On` holds.

## Measured, and what remains

The span reader is pinned, on the cubic as a bare carrier, as `geom_brep::curve_pair::span_rows::a_span_over_a_hairpin_closer_than_the_band_reads_on_at_the_seeds_branch`. The whole span of the 3e-10 hairpin is read as an edge is read, about a point on the near branch at `t = 0.08`:
- at `f64`, it reads the near branch from either seed;
- at `Interval` with a ±1e-10 box, seeded at the span's middle (the census's seed), it escalates;
- at `Interval` with a ±1e-10 box, seeded on the far branch, it reads `On` at `t ≈ 0.92`, the tangent reversed, with nothing logged;
- at a ±1e-12 box or a point box, it reads the near branch from either seed.

Readers that can hand `Span::read` or `Foot::on_carrier` a seed other than the span's middle, and then read geometry or cut at `t`:
- `census` shared-run ends (`run_ends`);
- `ring_path`'s plane roots;
- `solid_contain::curved_edges`;
- `pair_section` (seeded at the span's start);
- `slit` and `carrier_cross`, which cut at `Hit::On`'s `t`.

No door refuses such an edge where its fold stays fast (the semicircle above); the cubic is refused only by its meter. Tier 3 names global self-intersection and minimum clearance as deferred (`topo::validate`'s tier-3 docs), and the clearance engine runs at no gate at rest.


## Closed (PR 6618)

The fork is decided, and this row closes into the P0 unit
`the-at-rest-gate-certifies-embedding`.

- **The home is the ratified at-rest embedding gate** (`DESIGN.md`
  `[topology.tiers]`, ruled at PR 6301). An edge or face that comes
  within the band of itself away from its own neighbourhood is a body
  whose boundary is not embedded. That gate refuses it, and every
  reader of a branch rests on it. There is no `certify` check, no `On`
  arm, and no ratified text change.
- **This row's "tier 1 refuses" option was a mis-statement.** Tier 1 is
  structural and reads no geometry. The option meant the embedding
  gate.
- **K's floor**, the one point the designers did not settle, goes to Ev
  at PR 6625.

The defects the fork found in the engine that gate runs on, the residual
and the pins for when it lands are now on the unit's own row.

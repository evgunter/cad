---
id: an-interval-foot-seeded-far-from-the-point-escalates-on-a-clear-carrier
kind: issue
title: An Interval foot seeded far from its point escalates on a carrier whose branches are well apart
status: open
opened: 2026-11-09
priority: P3
cost: M
---

Found by the lane on `a-carriers-branches-within-the-band-of-each-other`, measuring the hairpin.

`Foot::on_carrier` (`geom::curves::on_carrier`) at `Interval`, on the cubic hairpin of its rows (`hairpin(0.2)`: branches 0.2 apart, two hundred million zero bands), for a box of `±r` about the point at `t = 0.08`, by seed:

| r | seed 0.08 | seed 0.5 (the tip) | seed 0.92 (far branch) |
|---|---|---|---|
| 1e-10 | `On`, t = 0.08 | `Escalated(InBand)` | `Escalated(InBand)` |
| 1e-11 | `On` | `Escalated(InBand)` | `On`, t = 0.08 ± 1e-10 |
| 1e-12 | `On` | `Escalated(InBand)` | `On`, t = 0.08 ± 1e-11 |
| 0 | `On` | `On`, t = 0.08 ± 1.4e-12 | `On` |

Each escalation logs `curve_carrier_gap`, `curve_carrier_piece` and `curve_carrier_piece_clear`, and its gap margin is `[0, 6.6e-9]` at ±1e-10, and `[0, 7.3e-8]` at ±1e-11 seeded at the tip. That is several zeros wide, on a box ten to a thousand times narrower than the zero. The polish lands at the point's own branch from every seed (the `0` row), so the width looks like the foot's `t` enclosure widening with the seed's distance from the point, not the box; not yet traced. The piece walk then does not clear the piece about the point either.

Readers seed at the span's middle (`census::curved::on_interior_at`), which on a symmetric hairpin is its tip: an edge whose middle lies far from the point escalates where a seed near the point reads `On`. Conservative, never wrong; it costs readings. Reproduce through `Span::read` with the box helper in `geom_brep::curve_pair::span_rows` on `cubic_hairpin(0.2)`.

Decide whether the gap's enclosure should be read at a foot re-polished from the point's own enclosure (narrow) rather than carried from the seed, and whether the walk's piece about the point should clear the same way.

---
id: the-corner-fan-volume-of-an-oblique-needle-cancels-the-bodys-extent-cubed
kind: issue
title: The volume re-derivation fans every face from one corner, so an oblique needle's enclosure carries an absolute floor of ulp(L)·L² and its f64 volume can be orders wrong
status: closed
opened: 2026-10-14
closed: 2026-10-15
pr: 5119
priority: P2
cost: M
refs: [a-fan-over-carrier-ends-reads-an-ulp-gap-at-the-faces-length-times-its-lever]
---

Found by FLUSH's needle-class design fork (fork-log row 153, PR 5119),
designer probes on `flush/needle-design-y` at `bfec39bf9`
(`crates/topo/tests/needle_design_y_probes.rs`, a hidden
`volume_enclosure_probe` in `props.rs`).

## What

`topo::props::rederive` takes each face's flux about one corner of the
body. On a needle of length L and width w turned off the axes, a far
face's fan has three nearly parallel vectors of length about L, whose
O(L³) products cancel to O(w²L): an absolute error near ulp(L)·L²,
about 1e-16 at unit extent, whatever w. An axis-aligned needle escapes
it (its products are exact in binary); a closed-form face escapes it.

## Measured (ε 1e-9 unless noted)

- Oblique needle (0,1)×(0,w)×(0,w) rotated off the axes: the
  enclosure's width is 1.1–1.7e-16 at every w. At w 10.5ε,
  V 1.09e-16 with enclosure [4.0e-17, 1.8e-16] (width 1.28·V); at
  12ε 1.19·V, 21ε 0.36·V, 30ε 0.13·V. At ε 1e-6, width ≈ 1e-6·V.
- `brick ∩ brick` leaving a 30ε needle at ε 1e-12: two poses build
  (an aimed ray decided) while the f64 volume reads 5.9e-17 against a
  true 9e-22, and the enclosure straddles zero ([−6.3e-17, 6.3e-17]).
  `mass_properties` reports that volume.

## Owed

The designer's proposed re-posing: each planar face's flux as its
plane's signed distance from the corner times the face's own area,
whose error scales with the face's own flux (ulp(L)·L·A_face). If Ev
rules the volume's sign bandless on PR 5119 (row 153), this is the
conditioning that makes that sign hold on oblique needles, and the
needle unit takes it; otherwise it stands on its own as a wrong
`mass_properties` value.

## Resolution

FLUSH's volume half, PR 5119. Two commits:
- **`735bee07c`, the planar arm.** The re-derivation reads each planar face's flux as its plane's distance from the reference point along the stored normal, times its own signed area: `((a − c)·N)(A⃗·N)/(N·N)`. It does so on the fan route and the polygon route alike (`props::quad_lane`'s `plane_flux`). The polygon rows' exact oracle reads the same value.
- **`8a921c14a`, the reference point.** The point is now a vertex of the body, no longer the least corner of the loop points. On a body turned off the axes, that corner stands off by up to the body's length. That distance multiplied each long face's flux, so the terms cancelled in pairs: `brick ∩ brick` leaving a 2Kε needle at ε 1e-12 still straddled zero in three of four poses. A vertex has the body's own coordinates, so moving a face's geometry by it rounds no more than the body does.

Width/|V| of the re-derivation's enclosure for the needle `1 × w × w`, turned about (37, −21, 13) by 0.7 rad off (0.3, −0.2, 0.5), before → after both commits:

| ε | w = 10.5ε | 12ε | 15ε | 21ε | 30ε |
|---|---|---|---|---|---|
| 1e-9 | 0.15 → 1.7e-8 | 0.095 → 1.3e-8 | 0.069 → 1.3e-8 | 0.037 → 8.8e-9 | 0.017 → 6.5e-9 |
| 1e-6 | 1.6e-7 → 2.0e-11 | 1.2e-7 → 1.7e-11 | 7.2e-8 → 1.2e-11 | 3.1e-8 → 8.6e-12 | 1.5e-8 → 6.5e-12 |

Along the axes it is unchanged, at about 4e-16.

The f64 sums that `mass_properties` reports still fan from one corner, so their value on an oblique needle stays as measured above. The volume's sign, which the certified enclosure now reads alone, no longer rests on them.

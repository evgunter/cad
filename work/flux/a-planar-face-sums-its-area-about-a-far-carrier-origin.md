---
id: a-planar-face-sums-its-area-about-a-far-carrier-origin
kind: issue
title: A planar face's closed form sums its vector area about the carrier plane's stored origin, so a small face far from that origin loses its area to cancellation
status: open
opened: 2026-10-03
priority: P3
cost: M
refs: [volume-door-reads-a-tiny-valid-boolean-result-wrong]
---


Found by REACH while fixing tier 3's check 7 on
`work/contact/volume-door-reads-a-tiny-valid-boolean-result-wrong`.
This is the cause of that item's measurement error. The sign reading is
fixed separately; this measurement error is not.

**What the closed form does.** `geom_brep::props::planar_face`
(`crates/geom-brep/src/props/mod.rs`) computes
`A⃗ = Σ ½∮(p − origin) × dp` and `flux = origin · A⃗`, where `origin` is
the carrier plane's STORED origin (the face walk passes it through
`topo::props::closed_form_of`). The doc calls `origin` the translation
reference ("Mäntylä's far-from-origin conditioning remedy"). A boolean
result's face keeps its operand's carrier, so that origin can be metres
from the face. The cross products then cancel at
`|p − origin|² · 2⁻⁵²`, whatever the face's own size.

**Witness (instrumented, per face).** The ∩ of the corner pose in
`crates/topo/tests/contact9_side_codes.rs`
(`a_vertex_pair_reads_a_dipping_chord_at_its_far_vertex`, the
`control, 1 m edges` row) at ε 1e-12 is a 5.83e-19 m³ tetrahedron
(closed form `det·dip²/(6·b_z·c_z)`). It has four planar faces, on
carriers anchored 1.5–4.5 m away:

| face | f64 area | interval area (exact geometry) |
|---|---|---|
| far-end triangle, carrier origin (6.5, 6.5, 1) | 0.0 | [0, 1.79e-15] |
| side, carrier origin (3.5, 3, 0.5) | 3.0310883e-9 | [3.0310804e-9, 3.0310974e-9] |
| side, carrier origin (3, 3.5, 0.5) | 3.0310898e-9 | [3.0310825e-9, 3.0310994e-9] |
| top, carrier origin (10, 10, 0) | 3.4999950e-9 | [3.4999701e-9, 3.5000154e-9] |

The far-end face's true area is ~1e-19 m² and reads exactly 0. The sum
is −2.96e-16 m³, its interval re-derivation is
[−1.85e-14, +1.76e-14] m³, and the exact volume is +5.8e-19 m³.

A built witness at every ε is `topo`'s
`tier3_tests::far_anchored_slab`: a 1 mm × 1 mm × 100 nm brick with its
six planes re-anchored about 5 km along themselves. Its exact volume is
+1e-13 m³, and the walk reads −1.2e-12 m³.

**The cure.** Anchor each loop's vector area at a point of the face, e.g.
its first vertex `v₀`, so the cross products are of the face's own size.
Take the flux as `v₀ · A⃗` (`v₀` is on the plane within the band) or as
`origin · A⃗` with `A⃗` taken about `v₀`, since `A⃗` does not depend on
the anchor for a closed loop. That bounds the area error by the face's
size. The remaining `origin · A⃗` term about the world origin is then a
well-conditioned product.

Rows to update when it lands:
- `contact9_side_codes::RESOLVED_VOLUME`, which trusts the door only
  above 1e-12 m³;
- `far_anchored_slab`'s premise, which asserts the walk reads the slab
  negative and goes red once the measurement is cured, so it needs a
  new witness of a sum whose rounding is larger than the volume.

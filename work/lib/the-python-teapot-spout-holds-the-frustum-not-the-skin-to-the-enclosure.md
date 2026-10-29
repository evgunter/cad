---
id: the-python-teapot-spout-holds-the-frustum-not-the-skin-to-the-enclosure
kind: issue
title: The bindings' teapot spout holds the straightened frustum, not the skin's own volume, to the certified enclosure
status: open
opened: 2026-10-24
---


## What

`crates/pncad-py/tests/test_north_star.py` (the teapot class, the
spout's block after `v_spout = (1 - bore * bore) * self.frustum_volume(...)`)
asserts `abs(props.volume - v_spout) <= props.volume_pad`, and the same
for the area. `v_spout` is the straightened frustum: the closed form of
the swept tube the cubic skin INTERPOLATES, not of the skin itself. The
two differ by the loft fit, `1.156e-10` m³ (2.1e-6 of the volume). The
skin's own volume is `5.505264881074549e-5`: a 24-point Gauss–Legendre
divergence-theorem quadrature over the built walls, which agrees with
the 16-point rule to 3e-27. The frustum's is `5.5052533158981754e-5`.
For the area, the frustum misses the skin by 2.2e-5 of it.

At the default ε the binding's assert passes only because the pad is
wider than the fit's miss. At ε = 1e-12 the Rust scene's same assert
went red once PR 5629 let the measurement meet its target: a pad of
`8.09e-12` against a miss of `1.156e-10`. The block's comment also says
the Rust scene's 1e-12 reading does not open. It does now.

`demos/tour/src/teapot.rs` (`patch_quadrature` and the spout block in
`stops`) is the shape of the fix:
- Hold the enclosure to the skin's own integrated volume and area,
  with the caps' authored annuli added in closed form.
- Hold the frustum to that reference at the cubic fit's scale,
  `2·(5/384)·Δ⁴`.

The binding may need a door to a face's NURBS jet to say this
naturally. If it has none, that is a binding-coverage finding of its
own.

## The binding has no door to the reference (ISO, 2026-10-24)

The Rust scenes' reference is `demos/tour/tests/common/patch_quadrature.rs`.
It reads each face's surface, and a NURBS patch's knots, weights and
control net. `crates/pncad-py/src` exposes none of these to Python:
there is no surface accessor on a face, and no control net, knot
vector or evaluator for a patch. Fixing this test therefore first
needs a binding door to a face's NURBS data, which is that
binding-coverage finding. ISO left the test as it was.

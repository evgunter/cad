---
id: idealised-sweep-closed-forms-held-inside-certified-enclosures
kind: issue
title: Projectbox's wall 1 no longer refuses at the default eps (main red); it and lily's Pappus hold an idealised sweep's closed form inside a certified enclosure without showing the built body IS that sweep
status: open
opened: 2026-10-24
---


## What

These two sites hold a closed form inside the kernel's certified
volume enclosure. The teapot spout's frustum oracle had the same shape
and went red at ε = 1e-12 once its enclosure narrowed.
- `demos/tour/src/lily.rs` (`BRACKET_CEILING`, the `lily_leaf_b` /
  `lily_leaf_c` Pappus loop) asserts that the certified volume
  CONTAINS Pappus for a rigid section carried in the path's normal
  frame.
- `demos/tour/src/projectbox.rs` (`standing_spring`, wall 1) asserts
  `(props.volume - want).abs() <= props.volume_pad`, with
  `want = coil_volume(π WIRE_H²)`.

Each closed form is the volume of the IDEAL sweep. A containment
assert is only right if the built body is exactly that sweep. If the
sweep door fits or approximates the path or frame, the closed form is
an approximation's value, and the assert goes red the day its
enclosure narrows below the fit error. That happened to the spout:
`work/iso/the-teapot-spouts-frustum-oracle-misses-its-measured-volume-at-1e-12.md`.

**Projectbox is red on main at the default ε**, and not because of
its oracle. Since PR 5629, the walk at the default ε panics in
`walls::wall`: *"wall 1 (measure the round-wire spring's volume) NO
LONGER REFUSES"*. At 1e-12 it still refuses `QuadratureBudget` (width
2.74e-8 against a 1.024e-9 target). At 1e-6 it measures
`0.010359820631111011 ± 4.9e-5` against `A·L = 0.010405853509495785`.
That is a gap of 4.6e-5 inside a 4.9e-5 pad, which is 4.4e-3 of the
volume: the sweep's own deviation from the ideal coil, and the
`COIL_GAP` band admits it. The last nightly predates 5629, so no
nightly has shown this yet. The teapot's 1e-12 panic masked it on
the PR gate.

Retiring wall 1 at the default ε, as the probe says, puts the same
containment there under a pad narrower than the 4.4e-3 gap. So the
retirement has to settle the oracle first: hold the enclosure to the
built coil's own integrated volume, and keep `A·L` as the fit check
`COIL_GAP` already is.

Lily is not red: its bracket at 1e-12 and at 1e-6 is wider than its
ceiling.

Owed, per site: show that the built body is the ideal sweep
(an exact rational path and frame), or integrate the built surfaces
as `teapot.rs`'s `patch_quadrature` does and hold the enclosure to
that, keeping the closed form as a fit check.

---
id: focal-section-image-on-a-flat-cone-misses-by-a-cos-squared-factor
kind: issue
title: a cone ellipse's chart image on a flat cone (α near π/2) misses its carrier by about u·A/cos²α, so a frustum cut far from any degenerate pose refuses at 1e-12
status: closed
opened: 2026-10-12
priority: P3
cost: M
refs: [conic-section-just-past-its-band-fails-certification-downstream]
closed: 2026-10-14
---


Found by GERM's measurement for
`conic-section-just-past-its-band-fails-certification-downstream`
(probe: `crates/sweep/examples/conic_past_band_probe.rs`).

## What happens

On a flat cone the `Pcurve::FocalSection` image `chart_pcurve` mints
misses its carrier by about `u·A/cos²α`, independent of how close the
section is to a parabola. At `α = 1.55` (`1/cos²α ≈ 2300`) that puts
a well-conditioned ellipse (`k = 0.87`, half the parabolic tilt) past
the band at ε = 1e-12.

## Measured

Frustum of half-angle 1.55, height `S`, top radius `S/2`, revolved
about `y`, split by the plane through `(0, S/2, 0)` tilted
`φ = f·(π/2 − α)` off the axis, axis-aligned:

| S | f | A | carrier residual | whole-turn map residual | split at 1e-12 |
|---|---|---|---|---|---|
| 0.01 | 0.5 | 0.33 | 2.2e-15 | 7.2e-14 | splits |
| 0.01 | 0.9 | 1.3 | 3.5e-15 | 2.4e-13 | `Pcurves(Certify)` |
| 1 | 0.5 | 33 | 2.2e-13 | 7.2e-12 | `Join(Euler(Certification { Escalated }))` |
| 1 | 0.8 | 68 | 2.0e-13 | 7.8e-12 | `Join(Euler(Certification { Escalated }))` |

At ε = 1e-9 the `S = 100` frustum refuses the same way at `f = 0.5`
(map residual 7.2e-10). Reading the vertex azimuth off `ra`
(`focal-section-vertex-azimuth-is-read-off-a-cancelling-sum`) moves
it by at most 3× at this half-angle (generic pose: 2.4e-11 → 2.3e-11
at `f = 0.9`, 7.1e-8 → 3.6e-8 at `f = 0.999`), so this is a second
mechanism; unmeasured beyond the
scaling, which points at the slant `v = h/cos α` the image's second
channel is stored in.

## What closes it

Measure which of the image's numbers carries the `1/cos²α`, and
re-derive it from the carrier where it does not have to divide by
`cos α`.

**Not touched by the vertex-azimuth fix** (`focal-section-vertex-azimuth-is-read-off-a-cancelling-sum`, closed 2026-10-13): at `α = 1.55` it moves the whole-turn miss at most 3× (generic pose, `f = 0.9999`: 3.6e-6 → 8.8e-7), so the `1/cos²α` mechanism stands.

**Moved 28× by the `β` fix** (`cone-section-interval-mint-escalates-on-a-beta-enclosure-of-u-over-k-levered-near-a-parabola`, closed 2026-10-13): at `α = 1.55`, `f = 0.9999`, scale 1, the generic-pose whole-turn miss goes 8.8e-7 → 3.1e-8, and `f = 0.999` 3.6e-8 → 2.4e-9; both still refuse or escalate, about 4000× the `α = 1` miss at the same `k`, so a flat-cone factor stands.

## Closed: fixed by GERM's PR #5043 (PCERT, 2026-10-14)

The `1/cos²α` was not in the image. It was in the carrier the image is
minted from: `plane_cone_section`'s `TiltedEllipse` arm read
`K = c² − sin²α` and a centre that cancels to `K` along the normal.
#5043 reads `K = −D·M` and the centre off the apex's foot.

The item's probe (`sweep/examples/conic_past_band_probe.rs`, cone
family, `α = 1.55`), re-run on main after #5043:

| ε | S | f | whole-turn map residual, before → after | after, in `u·A` | split |
|---|---|---|---|---|---|
| 1e-12 | 0.01 | 0.5 | 7.2e-14 → 4.2e-16 | 5 | splits |
| 1e-12 | 0.01 | 0.9 | 2.4e-13 → 1.6e-15 | 5 | splits (was `Pcurves(Certify)`) |
| 1e-12 | 1 | 0.5 | 7.2e-12 → 3.6e-14 | 5 | splits (was `Join(Euler(Escalated))`) |
| 1e-12 | 1 | 0.8 | 7.8e-12 → 7.4e-14 | 5 | splits (was `Join(Euler(Escalated))`) |
| 1e-9 | 100 | 0.5 | 7.2e-10 → 3.6e-12 | 5 | splits (was refused) |
| 1e-9 | 100 | 0.8, 0.9 | → 8.1e-12, 1.5e-11 | 5 | splits |

Every row's residual is now about `5·u·A`, where `A` is the larger
semi-axis and `u = f64::EPSILON/2`. It was about `1000·u·A` before.

**What still refuses, and why it has no item here.**
- **`S = 1`, `f = 0.999`** (`k = 0.045`) escalates at the edge's
  `EndpointStart`, margin `1.28e-12`, at the band's own scale. That is
  the near-parabola mechanism, uniform in `α`, and not this item's flat
  factor. It is GERM's open
  `conic-section-just-past-its-band-fails-certification-downstream` and
  `a-steep-frustum-cut-at-a-shallow-tilt-refuses-at-the-endpoint-gate-at-1e-12`.
- **`S = 100` at ε = 1e-12.** The probe cannot build the operand: its
  extent is about 9.7e3, so `u·extent ≈ 2.2e-12 ≥ ε`, and the band sits
  below f64's resolution of the operand itself. The section's image
  escalates there too, `MapResidual` at `f` 0.5, 0.8 and 0.9 (whole-turn
  residual 3.6e-12 to 1.5e-11, about `5·u·A` again). That is the same
  resolution floor, not the flat factor, so it has no item of its own.

**Row.**
`pcurve_cone_section::a_flat_cones_section_images_and_certifies_at_its_own_resolution`:
- The item's own frustum pose (`skew_section`): `α = 1.55`, height 0.1,
  `f` of 0.5, 0.8 and 0.9.
- Each image certifies over a whole turn on its own band
  `(1e-12, 1e-11)`, with an envelope within 48 `u` of the section's
  size, three times the carrier's own allowance in
  `flat_cone_tilted_ellipse.rs`.
- The ceiling binds below the band's zero at every pose. Today's
  envelopes read 27, 8 and 31 `u`.
- These red every pose:
  - the arm's old `K`;
  - the arm's old centre;
  - the centre moved 30 `u` back along the major axis (64, 86 and
    70 `u`).

---
id: a-rigid-turn-reads-a-valid-ellipse-sliver-inside-out
kind: issue
title: A rigid turn of a tier-3-valid section-ellipse sliver reads it inside-out, and mass_properties returns Ok with a wrong-signed volume, at ε = 1e-9
status: closed
opened: 2026-10-14
priority: P1
cost: M
branch: cleave/rigid-turn-sliver-volume
closed: 2026-10-19
---


Found by PR 5100's fix pass (`cleave/seam-split-invalid-ok`), asked whether
`topo::transform_rigid` can flip tier 3's verdict on a large-axis
section-ellipse body. It can, to a definite wrong verdict.

**Probe:** `crates/sweep/examples/turned_sliver_probe.rs`. The unit
cylinder is split by a plane through `(cos φ, sin φ, 0) + L·t` (`t` the
rim's tangent) with normal `cos ψ·w + sin ψ·ẑ` (`w` the wall's normal
there): an ellipse of semi-major axis `1/sin ψ`. Each side that passes
`AtRestBody::validate` is turned by `transform_rigid` about four axes
through `(0.3, −0.2, 0.5)` (`ẑ`, `x̂`, `(1, 2, 3)/|·|`, `(−0.3, 0.7, 0.2)/|·|`)
at seven angles (0.1, 0.7, 1.3, 2.2, 3.0, −1.1, 5.9), then
`validate_geometric` again.

**Measured (release):**

| ε | turned valid sides | refused after the turn |
|---|---|---|
| 1e-9 | 784 | 40, all `NegativeVolume` |
| 1e-12 | 336 | 0 |
| 1e-6 | 336 | 0 |

Every flip at 1e-9 is the thin `above` side (the sliver between the
plane and the wall), and none is a turn about `ẑ`, the cylinder's own axis:

- ψ = 1e-5: φ 0 L 0 (6 turns), φ 0.4 L 0.3 (7), φ 2.2 L 0.1 (4);
- ψ = 3e-6: φ 0.4 L 0.3 (11), φ 2.2 L 0.1 (12).

ψ = 1e-2, 1e-3 and 1e-4 never flip. The sliver is about `tan ψ` deep
(1e-5 to 3e-6 m), far outside the band at 1e-9, and a rigid motion
preserves volume, so check 7's definite negative is a wrong answer about a
valid body, not an in-band one.

**Unmeasured:** whether `transform_rigid` moves the geometry wrongly or
check 7 misreads the turned body (the `NegativeVolume` payload carries no
margin to read), and why 1e-12 does not flip; there the split refuses more
of these poses first, so fewer sides reach the turn.

## The certified volume door is silently wrong (PR 5100's re-review)

`mass_properties` on a turned body returns `Ok` with a wrong-signed volume
that depends on the turn's angle:

| ψ | φ, L | original V | turned V |
|---|---|---|---|
| 1e-5 | 0, 0 | +2.385e-8 | −1.85e-8 at 0.7; −1.22e-8 and −2.92e-8 at other angles |
| 1e-5 | 2.2, 0.1 | +2.39e-8 | −1.13e-7 |
| 3e-6 | 0.4, 0.3 | +3.92e-9 | −3.20e-7 |
| 3e-6 | 2.2, 0.1 | +3.92e-9 | −2.18e-7 |

Each turned body has the same topology as the original, and turning it back
validates. So this is a wrong answer from a certifying door, not only a
refusal, and the row is P1. Whether the transform or the volume integration
is at fault is still unmeasured.


## Measured: the arc's vector area, not the transform

(cleave/rigid-turn-sliver-volume) Per face, against a 40-digit oracle,
on ψ = 1e-5, φ 0, L 0, turned 0.7 about x̂: both cylinder walls and the
top cap are right to 5e-12 after the turn; the section plane's flux is
−5.9629668e-3 against −5.9628397e-3, off by 1.27e-7 with pad 0. The
exact vector area of the turned STORED loop (ellipse centre
(0, −64421.49, 76484.47), semi-axes 1e5 and 1) is right to 1e-14, so
`transform_rigid` placed the geometry correctly.

The fault is `geom_brep::props::loop_vector_area`'s conic arm,
`(C − ref)×(p1 − p0) + a·b·Δt·axis`: the two terms are about 900 and
cancel to the face's 6e-3, and the chord's rounding at `|C|` = 1e5
(~1e-11) reaches the sum times `|C − ref|` = 1e5. Unturned, the
ellipse's centre and its rounding both lie along ẑ, so the cross product
cancels the rounding; a turn about ẑ keeps that, which is why no ẑ turn
flipped. The planar closed form carries pad 0, so check 7 read the
wrong sum as a definite negative.

Coverage on main, every turned side of the probe against the oracle
(volume ± pad, ε·area and rounding): 238 of 812 enclosures miss at 1e-9
(worst 2750× the slack, at ψ 1e-4 as well as 1e-5 and 3e-6), 14 of 348
at 1e-12 (ψ 1e-3), 0 of 348 at 1e-6. With the arc read as the triangle
`ref, p0, p1` plus its segment `a·b·(Δt − sin Δt)`: 0 misses at all
three, worst 0.043 of the slack, and the probe's 40 flips are 0.

Sibling filed, not fixed here: a cone wall sums about its apex, the
same shape with the apex as the lever
(`work/nearpoint/a-cone-wall-far-from-its-apex-measures-its-rounding-at-the-apex-lever`,
P1: a drafted revolve turned reads 2.6 % wrong with pad 0). Its cure
has to decide what a loop's gap means, which the arc's does not: the
arc's new form equals the old one edge by edge in ℝ.

Sibling, already filed and not fixed here:
`work/nearpoint/a-planar-face-sums-its-area-about-a-far-carrier-origin`
(the same rounding-times-lever shape, with the stored plane origin as
the lever); this branch adds a public-door witness to it.

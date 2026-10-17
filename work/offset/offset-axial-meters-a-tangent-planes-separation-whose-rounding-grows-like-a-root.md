---
id: offset-axial-meters-a-tangent-planes-separation-whose-rounding-grows-like-a-root
kind: issue
title: solve_corner decides a plane's cut of a corner circle on the separation 2ρ·sinΔ, which a rounding u·amp in the plane's offset moves by about 2ρ·√(2u) near tangency, far past ε
status: open
opened: 2026-10-13
priority: P3
cost: E
---

Found by the class sweep of `pcert/cone-section-interval-beta`, which
fixed a `√(1 − x²)` read near `x → 1` in `chart_pcurve`'s cone arm. Not
measured here; this is the arithmetic.

## What happens

`solve_corner` (`topo/src/offset_axial.rs`, the one-meridian arm)
places a corner where a plane meets the corner's circle:

- `amp` is the plane's reach across the circle and `rhs` its offset, so
  the azimuths are `φ₀ ± Δ` with `cos Δ = rhs/amp`
  (`offset_axial.rs:1501`, through `clamp_unit`).
- `sin Δ = √(1 − cos²Δ)` (`:1502`), and the decision
  `offset_axial_azimuth` meters `separation = 2ρ·sin Δ` against the band
  (`:1503-1504`).

Near tangency `1 − cos²Δ ≈ 2(amp − rhs)/amp`, so the separation is a
square root of the plane's distance from tangency. `rhs` carries a
rounding of at least `u·amp` (more where the positions it subtracts
are larger), which moves `sin Δ` by about `√(2u)` there, and the
separation by about `2ρ·√(2u)`: about 4e-8 at `ρ = 1`, forty times
ε = 1e-9. So within about
`ρ·√u` of tangency the f64 verdict is the rounding's, and at `Interval`
the enclosure is that wide and escalates. Rewriting the root (for
example `√((amp − rhs)(amp + rhs))/amp`) does not help: the loss is in
the quantity, not the formula.

## What closes it

Decide transversality on the plane's distance from tangency
(`amp − rhs`, a length whose rounding is that of `rhs`) rather than on the
separation, or say why the separation is the right meter and add a row
at a plane within `ρ·√u` of tangency that shows what the verdict is
there.

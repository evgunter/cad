---
id: torus-residual-rho-is-not-reached-by-the-normal-form
kind: issue
title: The torus implicit residual's `ρ = |w|` is not reached by the symbolic tier's normal form
status: open
opened: 2026-10-26
---


The torus residual (`crates/geom-brep/src/implicit.rs:31-38`) is
`((ρ − R)² + h² − r²)/2r` with `ρ = |w|`, a square root of the radial
component's norm. Fork row 204 decides limb 2 at a parameter-family
scalar on the free-t residual `f_S(C(t))` as the margin's symbolic
identity. The designers' census measured Zero on line/plane,
circle/cylinder/sphere/tilted plane, ellipse/cylinder and spiric/plane,
but not on a torus operand. The symbolic tier's normal form does not
reach `ρ = |w|` (a root, not a polynomial in the carrier's values), so
a carrier on a torus would not be proved by its identity even where it
lies on the torus exactly. That needs either a root-free form (the torus's quartic,
`(|q|² + R² − r²)² − 4R²|w|²`, which eliminates `ρ` using `ρ² = |w|²`)
or a stated reason the torus stays on the numeric channel. Either one
is measured in the symbolic-tier census.

---
id: the-fillet-reads-two-signs-off-numbers-into-what-it-stores
kind: issue
title: The fillet reads two signs off numbers into what it stores: tangent_point's copysign(rho) and the line×line setback's half_tan.abs()
status: open
opened: 2026-11-24
---


Found by the signed-radius unit's sweep of `.abs()`, `copysign` and
`signum` in the constructors
(`work/boxfinish/a-verb-stores-a-radius-as-a-magnitude-and-the-symbolic-tier-cannot-use-the-sign-its-gate-decided.md`),
and left by it: neither site stores a radius. Both are in
`crates/profile/src/sugar.rs`.

**1. `Leg::tangent_point`, the circular-leg arm.** The tangent point is
`arc.center + spoke · (arc.radius / spoke.norm().copysign(rho))`: the
measured spoke length is given the sign of the offset radius
`ρ = R − σ·τ·r`, a sign read off a number into a stored point. The
enclosing-class gate (`fillet_offset_*`) refuses only a definite
Negative of ρ, so a ρ in band of zero reaches the `copysign` at either
sign, and the stored point then jumps across the leg's centre with the
rounding of ρ. Over a box the `copysign` is an atom the symbolic tier
holds no sign for.

**2. `line_line_fillet_trims`, the setback.** `setback = radius ·
half_tan.abs()`, where `half_tan = tan(φ/2)` of the corner's turn. The
function already holds the turn's side as a literal (`sgn`, from the
decided `fillet_corner_turn`), and `σ·tan(φ/2)` is the same positive
length with no magnitude taken; as written the setback is an `abs`
atom over a box, and the fit margins (`fillet_leg_fit`) and the trimmed
ends built from it carry it.

`work/profile/the-fillets-registered-tangency-says-it-holds-where-the-doors-decisions-hold.md`
is about what the fillet registers and names neither site; whoever
takes that row reads the same function and can take the second site
with it.

**The tests that say it is closed.**
- A circular-leg fillet whose offset radius is in band of zero stores
  the same tangent point at either sign of ρ, or refuses typed.
- A line×line fillet over a box on its corner stores trimmed ends whose
  forms hold no `abs` of the half-turn tangent.

---
id: the-lane-free-rounding-bound-charges-a-thin-face-its-spread-squared
kind: issue
title: The lane-free rounding bound charges a thin face its spread squared
status: open
opened: 2026-10-19
priority: P3
cost: M
refs: [5119]
---

On a body with no quadrature lane, the volume's sign is read from `[V − b, V + b]`, where `b = props::rounding_bound`. That bound charges each planar term u·R·D², with D the face's spread. A long, thin face's cross products actually round at about u·w·D, where w is its width (`crates/topo/src/props.rs:1397-1459`, `rounding_bound`; the spread is squared at `:1454-1456`).

**Measured** on PR 5119's frozen head by its second reviewer (`flush/shell-role-review-r2`, `review-probes/pr5119-r2-delta2/`), on the unit needle bar 1×w×w across 7 poses and both orientations:
- **At ε 1e-6:** all 70 cases read correctly.
- **At ε 1e-9:** 67 of 70 refuse as `SignLimit::Scalar`. Here b ≈ 5e-14 against V = w² ≈ 1e-16 to 9e-16, so these doors read only bars wider than about √b, roughly 230ε. The retired V/A gate cut off near 40ε.

The refusal is honest: it is typed as a capability limit ("no way through yet"), not as a thinness verdict, and no reading came out wrong. It is a reach this door can win back.

**The work:**
- Bound each planar face term by the rounding of its own cross products, not by its spread squared.
- Guard the case of a loopless face: a closed sphere or torus adds nothing to b (`props.rs:1454`, the loop-box guard). Its vector area is zero, but nothing asserts that.
- Round the widened ends outward. Today they round to nearest, one ulp against a factor-16 allowance.
- Pin the needle grid at 1e-9 so that it reads.

The other route is to give these doors a quadrature lane, which would make the bound moot. Which one fits is part of the work.


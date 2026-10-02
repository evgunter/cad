---
id: line-roots-carry-no-root-slack-meter
kind: issue
title: Line x sphere and line x wall roots carry no root-position meter, where the circle doors refuse an unplaceable root
status: open
opened: 2026-10-02
priority: P3
cost: M
design: true
refs: [circle-sphere-root-slack-refuses-near-tangent-pairs-at-1e-12]
---


## What

`line_sphere_roots` (`crates/topo/src/boolean/solid_contain.rs`)
decides the discriminant (`bool_ray_sphere_disc`, the squared
half-chord over `2r`) and returns `quadratic_roots` with no meter on
where the roots are. The circle doors refuse a root whose position
the rounding moves by more than the band (`bool_circle_*_root_slack`,
`circle_roots::first_harmonic_roots` and `half_angle_roots`).

Near a tangency the line's roots are as ill-conditioned as the
circle's: with half-chord `h` the root moves by about
`δc / (2h·|d|)` under an error `δc ~ u·|w|²` in the constant term, and
the smallest definite `h` is `√(2r·Kε)`. At ε 1e-12 and unit scale
that is `h ≈ 4.5e-6` and a root error of `~2e-11` m — twenty times the
band — answered as two certified roots. (Estimated from the formula,
not executed.)

## What has to be decided

Whether the line doors owe the circle doors' meter (refuse an
unplaceable root), or the circle doors' meter is stricter than the
consumers need — a pierce vertex at the computed root is within ε of
both carriers either way (the residual there is the rounding, not the
slack), so what the slack protects is the span and trim decisions
made on the root's parameter. Either way the two lanes should agree,
and the posture is a design question (`design: true`).

Same posture to check on `line_wall_roots` (line × cylinder) and the
line × torus quartic in the same file. Found by the sweep in the
circle × sphere root-slack unit; HONE's ground (`boolean/*`, latent
unsoundness).

The split lane's conic × plane crossing
(`crates/topo/src/splitting/classify.rs`, `conic_crossing_roots`) is a
third instance of the shape: roots `φ ± acos(−D/R)` with no position
meter, and `acos` near `±1` (a graze just past the band) amplifies the
ratio's rounding by `1/√(1 − (D/R)²)`. There `split_edge`'s child
certification re-verifies every insertion against ε, so a misplaced
root is refused downstream rather than built; the question is whether
it should be refused at the root, as the circle doors do, or placed
better (the circle doors now measure the half-chord from the extreme
nearer zero, `circle_roots::first_harmonic_roots`).

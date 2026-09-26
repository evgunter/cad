---
id: generic-pose-window-is-tighter-than-the-biquadratic-arm-bound
kind: issue
title: the ray-torus generic-pose sweep holds roots to an absolute 1e-6 while the biquadratic arm documents a K*eps error, 1e-5 at eps 1e-6
status: open
opened: 2026-09-25
priority: P4
cost: E
---


## The finding

`Tally::compare` (`crates/topo/src/boolean/r1_probes.rs`) flags a ROOT
disagreement when a certified root is more than an absolute `1e-6`
from the geometric oracle, at every ε. `line_torus_roots`'s own ladder
doc (rung 2, `bool_ray_torus_odd` Zero) says the biquadratic arm solves
the quartic with `q̂` SET to zero whenever `|q̂| ≤ K·ε·ext²`. That costs
a root error of first order in the dropped term, bounded at about
`K·ε`, which is `1e-5` at ε = 1e-6: ten times the row's window.

Measured with a throwaway biased probe (400 rays per ε, drawn with a
tiny axial component or passing just above the midplane, against
`oracle_roots`): worst root error **8.3e-7 at ε = 1e-6** (7.6e-7 on a
second draw aimed at the arm's edge), 4.7e-10 at 1e-9, and 6e-14 at
1e-12. It is under the window, but by 20%, and the arm's documented
bound allows ten times more. The generic sweep reaches this arm with
probability of roughly 1e-5 per ray (`|e·n| ≲ 4.5e-6`), so a red here
would be a within-contract pose, not a defect: a false alarm on
somebody's branch.

The window should be the contract's: `max(1e-6, band.escalate)` or the
arm's own stated bound, with the reason written beside it. The root
accuracy outside the arm is ~1e-13 since the resolvent fix, so the
window could also TIGHTEN off the arm. That needs the arm's `Zero` to
be visible to the tally.

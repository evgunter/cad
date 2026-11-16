---
id: the-square-wall-arm-reads-a-tangent-past-the-reach-its-row-allows-at-1e-12
kind: issue
title: section_cert's square-wall search at ε 1e-12: a pipe along the core's tangent reads Tangent(ct_critical_gap) 8.4e-10 from the degeneracy, five times past the reach the row allows a tangent, on one seed
status: open
opened: 2026-11-15
priority: P2
cost: M
---


Filed by GREEN's `green/nightly-1115` lane, from nightly 43680833019's
ε 1e-12 row.

`topo boolean::section_cert::square_search::the_square_wall_arm_agrees_with_the_traced_section`
reports one mismatch on
`CAD_TOLERANCE_EPS=1e-12 CAD_FUZZ_SEED=0x96ec838c8a0d073e CAD_FUZZ_EFFORT=1`
(reproduced on main `3e7a8c93b`, `opt-level = 2`):

`a pipe along the core's tangent: R 4.637347047154025, r 2.3596463425345844, (e, z0, rc) (4.637347046918878, 0.0000000002458877987512386, 2.3596463413519726), tilt 0: R-tan at Degeneracy { tangent: 8.423828044659914e-10, top: 9.367231434964651e-10 }: Tangent("ct_critical_gap")`

The arm answers `Tangent`, and the row allows a `Tangent` only within
16 escalation bands of the tangent degeneracy (1.6e-10 at this ε); this
pose stands 8.4e-10 from it, about five times that. So the arm refuses
a pair the row says it should resolve: a refusal wider than its stated
reach, not a wrong section. Whether `ct_critical_gap` reads too wide at
this ε or the row's `near` bound is too narrow is not settled here.

**The pose, so the case does not depend on the row's draw order**
(`square_search::dump_line`'s line for the draw, by replaying the seed;
the form `scripts/oracles/torus_square_wall_mpmath.py` reads):

`{"eps": 1e-12, "torus": {"c": [3.932433754465009e0, 9.939592900913814e-1, -3.8589947577850414e0], "a": [-2.4440949427088535e-1, -8.357531592149658e-1, 4.9171196443899345e-1], "b1": [8.954784135822432e-1, 0e0, 4.451049435899685e-1], "R": 4.637347047154025e0, "r": 2.3596463425345844e0}, "wall": {"o": [-4.386048683549949e0, 3.1920067891015e-1, -9.140639123497888e0], "d": [-9.692440759207577e-1, 1.954963843454262e-1, -1.4948941434222646e-1], "e1": [-1.9771807791516469e-1, -9.80258925828035e-1, 0e0], "rc": 2.3596463413519726e0}, "answer": {"kind": "Tangent('ct_critical_gap')"}}`

Not pinned as a deterministic row here.

**Owed.** Say which side is wrong, and pin the pose as a deterministic
row beside the fix.

Reproduce:
`CAD_TOLERANCE_EPS=1e-12 CAD_FUZZ_SEED=0x96ec838c8a0d073e CAD_FUZZ_EFFORT=1 cargo nextest run -p topo -E 'test(the_square_wall_arm_agrees_with_the_traced_section)'`.

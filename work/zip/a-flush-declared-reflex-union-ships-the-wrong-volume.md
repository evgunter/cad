---
id: a-flush-declared-reflex-union-ships-the-wrong-volume
kind: issue
title: A flush-declared union on the reflex-corner probe returns volume 16 against the closed form 15.979, sound at every tier
status: open
opened: 2026-10-02
priority: P0
cost: M
---


(JOIN-1 dual review, PR 3790: R1's reflex battery. Pre-existing: the
same on main at 0abf909cb and on JOIN-1's fixed head.)

## What

`a` is the 315° reflex prism `prism_z` over `(0,0) (2,2) (-2,2)
(-2,-2) (2,-2) (2,0)`, z ∈ [0, 1] (material everywhere but the 45° wedge
`0 ≤ y ≤ x`). `b` is `prism_ops` over the unit square `(0,0) (1,0) (1,1)
(0,1)`, z ∈ (1, 3), described with `describe_as_intersections` and
sheared `z' = z + sx·x + sy·y` with `(sx, sy) = (−0.5, 0.25)`, so its
bottom cap passes through `a`'s reflex corner `(0, 0, 1)`. The fixture is
`crates/sweep/tests/join1_r1_probes.rs` `join1_r1_reflex_battery`
(profile `sqQ1`).

`union_with(&a, &b, &flush_declarations(&a, &b))` returns a body that
passes tiers 2 and 3′ and the at-rest certificate, of volume **16.0**.
The closed form is `14 + 2 − v∩ = 15.979166…`: the overlap is the part
of `b`'s sheared floor below `z = 1` over `a`'s material, `y > x` and
`−0.5x + 0.25y < 0` in the unit square, `v∩ = ∫ (0.5x − 0.25y) dA =
1/192 + 1/64 = 1/48` (checked by hand: `x ∈ [0, ½]`, `y ∈ [x, 2x]` gives
`1/192`; `x ∈ [½, 1]`, `y ∈ [x, 1]` gives `1/64`). The union kept the
overlap twice — a wrong body that every gate passes.

The op is flush-declared, so the declared-REST zip takes over when the
join refuses; whether this body is the zip's or the join's is not
measured. On main the same pose's ∩ and ∖ refuse
`Join(UnpairedLooseEnds)`; on JOIN-1's fixed head they build sound at the
closed form (∩ = 1/48 exactly, ∖ = 14 − 1/48), which corroborates the
overlap. JOIN-1's fix pass leaves this union unchanged (still 16.0) and
turns four other poses of
the battery that were wrong on main (`sqQ1` and `dRight` unions at
`sx ∈ {−0.5, −0.25}`, volume 15 or 16) into `Euler(FanStartMismatch)`
refusals.

## Next

Trace which lane builds it; pin the pose as a row asserting the
closed form; the backstop that refuses an implausible volume does not
see a 0.13 % excess.

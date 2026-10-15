---
id: a-glued-rod-in-its-bore-escalates-in-one-operand-order
kind: issue
title: A rod turned 1e-8° in a bore of its radius builds with the rod as operand A and escalates a carrier crossing's span with the bore as operand A
status: open
opened: 2026-10-15
priority: P3
cost: M
---


Found by PR 5177's review (off-target, pre-existing: the reviewer's digests are identical on its base).

## What happens

The fixture is `crates/sweep/tests/bisector_nodes.rs::a_rod_against_a_bore_builds_every_operation`'s, at ε 1e-9: the brick `[−2, 2]³` bored to radius 1 about `z`, and the unit rod `rod_y(4.0)` turned about `x` by `π/2 − θ`. At θ = 1e-8° the axes are 1.7e-10 rad apart and the rod's wall lies within the zero band of the bore's along its whole length.

- **Rod as A:** ∩, ∪ and ∖ build (∩ empty, ∪ one solid, rod ∖ bore one solid).
- **Bore as A:** all three escalate `Escalated { decision: Crossing(OnEdge) }`, predicate `bool_carrier_cross_in_span` (`crates/topo/src/boolean/carrier_cross.rs:189`), with margin `1.286e-9` against the band `[1e-9, 1e-8]`.
- **θ = 1e-6° and 1e-4°:** both orders refuse `CurvedPierceUnsupported` alike (operand B, then A: the same face and edge).

## Why it is a finding

Escalating is never a wrong answer. But the two orders read one pose differently: the bore-as-A order finds a carrier crossing's span in band, and the rod-as-A order either decides that crossing or never meets it. A margin that decides the pose in one order should decide it in the other. Otherwise the order that builds rests on a decision the other order calls undecided.

## Reproducer

```rust
let bore = bored([(-2.0, 2.0); 3], spin(&cyl(1.0, 3.0), Vec3::new(0.0, 0.0, 1.0), PHI));
let rod = spin(&rod_y(4.0), Vec3::new(1.0, 0.0, 0.0), PI / 2.0 - 1e-8_f64.to_radians());
// finished(...) both, then topo::intersect(&bore, &rod, tol) escalates while
// topo::intersect(&rod, &bore, tol) builds.
```

The first step is to find which reading the rod-as-A order takes for the carrier crossing the bore-as-A order meets: whether it reaches `strictly_inside` at all, and on which carrier.


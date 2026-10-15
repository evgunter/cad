---
id: a-glued-rod-in-its-bore-escalates-in-one-operand-order
kind: issue
title: A rod turned 1e-8° in a bore of its radius builds with the rod as operand A and escalates a carrier crossing's span with the bore as operand A
status: closed
branch: tang/glued-rod-order
opened: 2026-10-15
closed: 2026-10-15
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

## Closed

Fixed on `tang/glued-rod-order`: both orders now escalate at 1e-8°, on the same pair.

**What the probe found.** Both orders ask the same predicate of the same cluster of candidates on the rim at z = −2. The candidates are several closed forms' views of where the glued seam ruling meets the rim, spread over `[2.341592652946771, 2.341592654232815]`, `1.286e-9` wide. The sweep had already split the rim at one member of the cluster, and which member follows the operand order:
- **Rod as A** splits at `…3911304`. The other members lie `9.6e-10`, `6.4e-10` and `−3.2e-10` from it, all Zero, so it builds.
- **Bore as A** splits at `…4232815`, and the farthest member lies `1.286e-9` away, in band.

Each order read its own margins honestly. The rod-as-A order's "one crossing" rests on Zero being transitive, which a banded comparison is not.

**The fix** (`carrier_cross::one_point_at_end`). The candidates that read Zero at a span end, together with the end itself, must read as one point pairwise. This is how `vtxfac::ring_groups` reads the pierces at one point. A pair in band escalates, whichever member the split took.

**Outcomes at the default band.** At 1e-9° both orders build alike. At 1e-8° both escalate `Crossing(OnEdge)` on `1.286e-9`. At 1e-7° both escalate on `1.359e-9`, read at two sites (a containment door and a vertex-on-face coincidence). Pinned by `crates/sweep/tests/bisector_nodes.rs::a_rod_glued_in_its_bore_answers_alike_in_either_order`, whose orders agree at every band, and by `carrier_cross::crossing_rows::candidates_at_an_end_escalate_where_two_of_them_lie_in_band`.


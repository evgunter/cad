---
id: a-via-arcs-carrier-is-never-checked-against-its-authored-through-point
kind: issue
title: paths: a Via arc lands on its end vertex by construction, so nothing reads its stored carrier against the through point it was authored by; a quarter-tangent that loses digits bends the arc off q silently
status: open
opened: 2026-10-20
priority: P2
---


Found by the sweep of
`a-stored-arcs-end-is-never-checked-against-its-authored-vertex`: the
blind spot of its landing cross-check.

**The gap.** A `Via` arc is lowered from its chord
(`path::via_arc`, `crate::bulge_leg`; the fused incoming in
`path/family.rs` reads `crate::bulge_from_via` too), so it lands on
both of its vertices whatever its quarter-tangent X is: the lowering
puts the centre on the chord's bisector. An X that loses digits
therefore does not miss the end vertex. It moves the carrier, and the
arc no longer passes through the authored `q`. Nothing reads that:
the facts the lowering registers are its two endpoints, and
`seg::constructed_landing` reads the landing only. The same holds for
every lowered arc whose X is computed from other authored data; a
`Bulge` arc's X is the authored data itself, so it has nothing else to
miss.

**Measured** (#5403's `Via` probe, `tests/tangent_arc_near_a_full_turn.rs`'s
`the_through_point_arc_near_a_full_turn_keeps_its_end_vertex`, with
`sugar::split_half_tan` mutated back to its first arm alone, the
spelling before #5403; default ε, f64):

| R | landing miss | `q` off the stored carrier | `ConstructedProfile::validate` |
|---|---|---|---|
| 1.36e4 | 4.7e-12 | 1.4e-5 | Ok, silent |
| 1.37e5 | 2.6e-11 | 0.32 | Ok, silent |
| 1.37e6 | 4.0e-10 | 382 | Ok, silent |

#5403's select fixed the live instance; this is the guard for the next.

**Shape.** The landing check's shape, at the `Via` sites, on the
through point: on the witness lane, `|rim(q) − radius|` against the
band (`ON_CARRIER_ULPS` for the resolution), unlogged, a resolvable
Positive refused as a kernel defect. The sites are the leg
(`path::via_arc`'s caller in `path.rs`), the fused incoming
(`family.rs`, `bulge_from_via`) and the arrival (`ViaArrival`).


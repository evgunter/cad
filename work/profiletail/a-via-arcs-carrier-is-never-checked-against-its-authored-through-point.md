---
id: a-via-arcs-carrier-is-never-checked-against-its-authored-through-point
kind: issue
title: paths: a Via arrival or fused incoming whose fillet fits exactly stores no arc on its Via carrier, so nothing reads that carrier against the through-point q it was authored by
status: open
opened: 2026-10-20
priority: P3
cost: E
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
the facts the lowering registers are its two endpoints, and the
door's landing read (now part of `seg::read_back_arc`) reads the
landing only. The same holds for
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


## What #5449 did, and what is left

Ev ruled on #5449 (2026-10-20) for the tripwire. The path door now
reads every stored `Via` arc's carrier against its `q` at a point
scalar (`seg::read_back_arc`'s `path_arc_through_point`):
the leg (`path::via_arc`), the fused incoming's run
(`family.rs`, `through_point` onto `PendingArc::through`) and the
arrival's and close's run (`resolve_arc_arrival`, `resolve_arc_close`).
`q` rides `BuiltArc::through` to `Core::build` and is never stored. The
cancelling mutant's 1.4e-5 m miss refuses `ArcMissesAuthoredPoint` at
every ε row (`a_cancelling_via_arc_refuses_off_its_through_point_at_f64`).
On main, `split_half_tan` holds `q` to within an ulp or two of the
carrier's magnitude at the row's probes (3.6e-12, 2.9e-11 and 0 m at
R = 1.36e4, 1.37e5, 1.37e6; 1.9e-9 and 3.0e-8 at 1.37e7 and 1.37e8,
where the scene cannot resolve the read and the door now refuses).

**Left.** Where the fillet fits exactly (`fit_out` Zero on an arrival,
`fit_in` Zero on a fused incoming), the fillet arc takes the whole
`Via` side and no arc on the `Via` carrier is stored. The carrier still
shapes the fillet (the resolution is tangent to it), but nothing stored
is on it to read against `q`. A read of the derived carrier itself at
`verbs::via_carrier` and `PointIncoming for Via` would cover it; it is
a read of a value the stored form does not hold, so it was left out of
#5449's door-read shape. P3: no construction is known to miss there.

## Re-homed at PATHS's close (2026-10-21)

Moved from `work/paths/` to PROFILETAIL (`work/profiletail/`) when PATHS closed. PATHS's live rows went to four programs opened at this close and cut along the priority seam (PROFILE, PROFILEREST, PROFILETAIL, PROFILEPROSE; `docs/doc-ledger/paths-leaves-the-tracker.md`). The item's id, content and history are unchanged.

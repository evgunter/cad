---
id: a-sketch-origin-far-from-its-profile-spends-the-placements-rounding
kind: issue
title: A sketch origin far from its profile costs the carriers the placement's rounding of the sketch coordinates: at 1e3 a vertex misses its plane by 3.4e-13, a third of ε = 1e-12, and past the session box a cell that built refuses
status: open
opened: 2026-11-15
priority: P2
cost: M
---

Found by the review of BOXFINISH's `boxfinish/sweep-frames` (PR 6972,
reviewer r1), on the ground PR 6953 (`onehome/rim-span`) settled: a
line rim's span is the sketch segment's own length and its direction
the sketch chord's, both turned by the station's frame.

A line rim now ends at `q_from + ‖b − a‖·place(d)`, and its far vertex
is `place(b)`. Both are the placement of sketch coordinates; when the
sketch origin is far from the profile, those coordinates are large and
the two spellings part by the placement's rounding of them, about
`u·|sketch coordinate|`. The same holds for a vertex against its cap or
wall plane, which is the frame. Measured by the reviewer on a turned
frame with the body near the world origin (the profile drawn about the
sketch point the frame places there): sketch origin 1.4e3 away, edge-end
residual 0 → 1.6e-13 and plane/vertex residual 3e-15 → 3.4e-13; 1.4e4,
1.5e-12; 1.4e6, 1.4e-10. At ε = 1e-12 a cell at 4.2e3 that built on the
base refuses on the head
(`AnalyticRung3(KernelLimit { operand: Plane, bound: 1.8e-12 })`). Inside
the 1e3 m session box nothing flips, and a third of ε is spent where
none was.

Pinned inside the box by
`a_sweep_keeps_the_frame_it_is_handed::a_slab_far_from_its_sketch_origin_holds_its_vertices_to_the_placements_rounding`:
at a sketch origin 1e3 away every vertex lies on its plane to eight ulps
of 1e3, and under ε/2 at every ε row.

**Why it is not cured on that branch.** The far vertex cannot simply be
the carrier's end, nor the carrier's end the placed vertex
(`[simple.conditioned]`): a vertex is shared by two rims, a strut and
up to three planes, and each of them reaches it by its own construction
from the sketch. Re-posing one edge's end onto the next edge's start
chains the rounding round the loop and moves it to the closing vertex;
re-posing a vertex onto its plane moves it off the others. The
digits are lost in the sketch coordinates themselves, which a placement
far from its profile makes large, so the cure is upstream of the doors:
a profile re-anchored near its own points (a sketch frame whose origin
is the profile's, the placement carrying the offset as one rigid motion
applied once), or a residual charged to that offset by name. Which one
is this program's call.

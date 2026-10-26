---
id: a-split-leaves-an-in-band-sliver-wall-face-that-refuses-only-at-rest
kind: issue
title: A split nearly along a cylinder's axis through a ruling's vertex leaves a curved wall sliver of in-band width beside a face of another surface, which refuses only at the at-rest gate, as VolumeUncomputable
status: open
opened: 2026-10-14
priority: P2
cost: M
refs: [4861, 4335]
---



## What

Found by PR 4861's at-rest pins (`crates/sweep/tests/section_ellipse_seam_poses.rs`,
`COIN_FLIP_1E6`). It reproduces on main `98353b4835` (CLEAVE's census
probe, the same 18 poses).

At ε = 1e-6, the unit cylinder is split by planes through a seam vertex
`(1, 0, 0)` or `(−1, 0, 1)` with normal `(0, 1, 3e-5)`, the vertex
`c·ε` off along the normal for `|c| ≤ 0.99`. The split returns `Ok`.
One side's wall face is a sliver between the seam ruling and the
plane, at most `θ·h = 3e-5` wide, and tier 3 refuses that side:
`VolumeUncomputable { source: Face { source: Escalated { predicate:
"props_quad_face_extent", margin ≈ 7.5e-6, check: Extent } } }`. The
face's mean width is in band (between ε and Kε = 1e-5).

The same family at ε = 1e-9 and `θ = 1e-4` refuses earlier, at reduce
(`SliverSector`). At `θ = 3e-5` and ε = 1e-6, nothing in the split
reads the sliver, and the reading surfaces only at rest, as the volume
quadrature's escalation.

## Owed

Under Ev's PR 4335 rule, an in-band reading of the input is `Escalated`
with a lever. Decide where the split reads a face of in-band width,
which is likely reduce's sliver reading, so that the refusal is typed
there with its lever, not left for the at-rest gate to end. Then
re-baseline `COIN_FLIP_1E6`.

## Note from `a-split-along-a-seam-at-a-vanishing-tilt-returns-an-invalid-body` (PR 5100, 2026-10-14)

The split now gates its sides at rest. These 18 poses (ε = 1e-6, θ = 3e-5,
coin-flip planes) refuse at the split door as
`SplitFinishError::ResultNotAtRest` on the side named in
`section_ellipse_seam_poses.rs`'s table, so they no longer refuse "only at
rest". The finding is `VolumeUncomputable`, not an in-band `ShellRole`, so
the shared typing (`topo::boolean::at_rest_refusal`) types it as the
kernel's. If the face's in-band width is the input's reading, the lever this
row asks for still has to come from a reading upstream of the gate.

## Note from the seam poses' main red (2026-10-19)

The 18 pinned poses build since `968cd6c904` (PR 5334, "split: halves end
with maximal faces"), and `COIN_FLIP_1E6` is empty. Each half now ends
with `Body::merge_coplanar_faces`, which merges the sliver into the wall
face across the seam ruling, so neither side keeps an in-band face.
First-parent bisect at ε = 1e-6: `1bd5147199` passes the old table,
`968cd6c904` fails it. PR 5185 (`7b9ec71c57`) still passes it.

The sides are right, checked at ε = 1e-6 over all 22 `θ = 3e-5` poses.
- Every side passes tier 3 (`AtRestBody::validate`) and tier 3′
  (`validate_pseudomanifold`, no contacts). Each has 4 faces: two caps,
  one wall, one section. Before PR 5334, at `c = ±50`, one side had 6.
- `mass_properties` matches the closed form
  `∫₀¹ seg(D − θz) dz`, where `seg(s) = acos s − s√(1−s²)` and
  `D = θ·v_z + c·ε·|n|`. The error is `|c|·ε/3` at `(1, 0)` and
  `2|c|·ε/3` at `(−1, 1)`, at most 6.6e-7, against a slack of 8.28e-6
  (`volume_pad + area·ε`). At `c = 0` and `c = ±50` the error is
  below 6e-15. Examples:
  - `(1, 0) c 0`: above 1.570826326795, below 1.570766326795.
  - `(−1, 1) c 0.99`: above 1.570765006795 (want 1.570764346795),
    below 1.570827646795.
- The two sides sum to π within 9e-16 in every pose.

**What remains.** The merge only absorbs a sliver whose neighbour across
the ruling is on the same surface. Change the ruling's neighbour to a flat
and the refusal is back:
- Take the prism `[((0, 1), tan(3π/8)), ((1, 0), 0)]`, height 1: a
  270° wall from `(0, 1)` to `(1, 0)`, closed by the flat `x + y = 1`.
- Cut it with the plane through `(1, 0, 0) + c·ε·n̂`, `n = (0, 1, 3e-5)`.
- At ε = 1e-6, `c ∈ {0, ±0.5}` refuses `at-rest gate above
  VolumeUncomputable`. `c = ±0.9` refuses at reduce (`SliverSector`), and
  `θ ≥ 1e-4` builds.

With the wall and the flat swapped (`[((1, 0), tan(3π/8)), ((0, −1), 0)]`),
the sliver is planar and validates. The title now names what remains, and
**Owed** above still applies to it: no pinned test exercises this pose yet.

## Slate triage (2026-10-26, CLEAVE)

- Re-banded P3 → P2: an in-band face width read only by the at-rest gate: interval and error propagation, P2 by `work/README.md` Priority.
- Moved to HONEPOSE (`work/honepose/`) by `git mv`, id and body unchanged: the P2 track opened for in-band refusals at measured poses: a reading that lands in band where the pose is not near a decision. CLEAVE measured 270.5 budget points against 30 and split on its priority seam (Ev, in chat, 2026-10-26).

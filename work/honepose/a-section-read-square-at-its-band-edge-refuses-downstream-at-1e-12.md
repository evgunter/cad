---
id: a-section-read-square-at-its-band-edge-refuses-downstream-at-1e-12
kind: issue
title: A section read square to a wall's axis at its band's edge certifies at the join and refuses downstream at 1e-12, with no lever
status: open
opened: 2026-10-12
priority: P2
cost: M
---


## What

Found by the lever check of
`a-section-curves-end-read-in-band-of-its-vertex-refuses-as-a-kernel-defect-at-the-join`
(probe `sweep/examples/section_end_census_probe.rs`, family `bool`).
The join now types a section read square to a wall's axis in its zero
band, whose stood-in circle misses a chord's vertex one rounding past ε,
as `SplitJoinError::SectionEndEscalated { SquareToAxis }`
(`chord_join.rs`'s `SectionEnds::reading`). At ε = 1e-9 and 1e-6 every
such pose refuses there. At ε = 1e-12 the same reading lands on the
other side of the rounding: the join's endpoint pins certify, and the
result refuses later, at two sites, neither with a lever:

- **The at-rest gate's volume** (`props.rs`, flux's ground): the unit
  cylinder (`0 ≤ z ≤ 1`) against the box `[−3, 3]² × [−1, 0.5]` turned
  `1e-12` rad about `y` through `(0, 0, 0.5)`, the probe's `squarebox
  t1e-12`. ∪ in both orders and box ∖ cylinder refuse
  `ResultInvalid { VolumeUncomputable { Face { Escalated { cause:
  props_loop_closed, Enclosure [9.99644811372491e-13,
  1.0000908725573501e-12] } } } }`, "There is no way through yet"; ∩ and
  cylinder ∖ box build.
- **The pcurve mint pass** (`pcurves.rs`): the cylinder against the
  slab `[1 + dd, 3] × [−2, 2] × [0.25, 0.75]` turned `1e-12` about `y`
  through `(1 + dd, 0, 0.5)`, `dd ∈ {−5e-11, −1e-9}` (`walltilt
  t1e-12`): all six runs refuse `Pcurves { Certify { ResidualExceeded {
  MapResidual, sample: 0 } } }`, a definite miss whose sentence names no
  recourse.

Both are the input's reading (the plane's tilt off square, `t·r = ε`)
told downstream as a kernel limit or nothing. The split's square poses
at 1e-12 (`section_end_poses`) refuse at the join, so this is the
Boolean's.

## Owed

Measure where the stood-in circle's loop closure and its pcurve's first
sample read the tilt, and tell the join's story there (the plane read
square, its lever), or show that a definite miss at the pcurve mint is
the kernel's. The props landing is flux's ground; the root reading and
the pcurve mint are this slate's.

## Re-measured on main `eb6f9c0f80` (PR 4810's rework, 2026-10-14)

The census probe's `bool` family at ε = 1e-12, after main's merge:

- **The square box now builds** in all six runs at every tilt the
  probe tries (`0`, `ε`, `±1e-2`), so the props landing above no
  longer reproduces and flux owes nothing here.
- **The slab still refuses** at `t = 1e-12`, `dd ∈ {−5e-11, −1e-9}`:
  all twelve runs are `Pcurves { Certify { ResidualExceeded {
  MapResidual, sample: 0 } } }`, a definite miss whose sentence names
  no recourse. The slab's levers (`t = 0`, `±1e-2`) build.

What is owed narrows to the pcurve mint: say whether the stood-in
circle's first map sample reads the tilt, and tell the join's story
there, or show the miss is the kernel's.


## Slate triage (2026-10-26, CLEAVE)

- Re-banded P3 → P2: a band-edge reading refuses downstream with no lever at 1e-12: interval and error propagation, P2 by `work/README.md` Priority.
- Moved to HONEPOSE (`work/honepose/`) by `git mv`, id and body unchanged: the P2 track opened for in-band refusals at measured poses: a reading that lands in band where the pose is not near a decision. CLEAVE measured 270.5 budget points against 30 and split on its priority seam (Ev, in chat, 2026-10-26).

---
id: tessellation-ring-and-step-refusals-name-the-kernel
kind: issue
title: mesh: tessellate calls a ring on a curved face and a non-positive sizing step kernel bugs; the first is a case not yet built, and the second may be a stored cone's
status: open
opened: 2026-10-21
parent: a-typed-kernel-defect-ending-may-launder-a-bug-d2-forbids
priority: P3
cost: E
---


A child of the census in `work/encl/a-typed-kernel-defect-ending-may-launder-a-bug-d2-forbids.md` (`## Census, second pass`, as of main 69ab780194). Why a typed kernel-only ending is wrong in either D2 class is in that row's "What".

## Sites

- `crates/mesh/src/types.rs` `TessellateError` `Display`, `RingOnCurvedFace` (~548; raised in `curved.rs` `tessellate_curved` and `trimmed.rs`). The door is `mesh::tessellate` over any body at rest. **Class 5, likely**: the text says "no construction produces one, so this is a kernel bug", which is a stale premise. But `topo::props`' `MassPropsError::RingOnCurvedFace` treats the same state as reachable and not yet measured ("move the cut so it crosses the face's edge"), and the Euler doors and a file both reach a curved face with an interior ring.
- `types.rs` `TessellateError` `Display`, `StepNotPositive` (~574). The door is `mesh::tessellate`. **Class 2, unsure**: every caller of `sizing.rs` `ceil_count` derives its step (`sqrt` of a non-negative ratio, or `cap_angular`), but representability (`half_angle > 0`) is checked only at tier 3, not at `set_face_surface`, so a negative stored half-angle may reach it. If one does, this site is class 1 and takes the face's lever. Probe it before any panic.

## Repair shape

- **Class 5** (`RingOnCurvedFace`): `NOT_YET_ENDING`, naming the face, as props does. A tessellation arm for the ring retires it.
- **Class 2** (`StepNotPositive`), once the probe above clears it: panic per `[failure]`, with its tests (`sizing.rs` ~1026) moved to `#[should_panic]`.

## Moved to TESSWORD (tessellation refusal words, mesh docs and contract hygiene) (2026-10-28)

Cut out of TESS by its semantic-seam split (`work/README.md`, Track size) when TESS measured 146 budget points against 30. Moved by `git mv`; the id and body are unchanged.

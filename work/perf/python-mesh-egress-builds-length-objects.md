---
id: python-mesh-egress-builds-length-objects
kind: issue
title: pncad-py mesh positions and triangles materialise per-coordinate Length objects - a fifth of the tessellation again
status: open
opened: 2026-09-10
---

## The finding

Measured by the PERF kernel lane (`perf/explore-kernel`; release wheel,
4 vCPU). A torus tessellated at 1 mm from Python (333 k triangles) takes
425 ms in the kernel; reading it back costs `mesh.triangles` 51 ms and
`mesh.positions` 41 ms, because both materialise per-vertex Python
tuples and per-coordinate `Length` objects
(`crates/pncad-py/src/py/mesh.rs:325,339`). That is ~22 % of the
tessellation again for a caller that wants arrays (numpy, matplotlib —
`demos/` renders are exactly that caller).

## What a fix is

"Do this faster", additive: a raw-array door beside the typed one (a
buffer-protocol export, or plain float/int lists) for callers that
declare their unit once. The typed `Length` surface is deliberate and
stays. LIB territory (`crates/pncad-py/*`), announced. Low priority:
it is one call's fifth, not a seat's wait.

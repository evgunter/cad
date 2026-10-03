---
id: tessellation-chart-frame-is-a-hand-rolled-undecided-triple
kind: issue
title: planar.rs chart_frame builds the whole tessellation triple by hand, with no length decided
status: open
opened: 2026-09-15
priority: P1
cost: E
---


## What

Found by FRAME-WITNESS's §4 sweep (`docs/FRAME-WITNESS-SPEC.md`, the
frame-witness unit of SCALAR).

`crates/mesh/src/planar.rs` `chart_frame` builds an origin and a
right-handed orthonormal triple, and every step of it is written out
here with no length decided anywhere:

- `normal = normal_sum.normalize()` — the Newell cross-sum, normalized;
- `u_ref = far.reject_from(normal).normalize()` — the farthest point's
  offset, shed of its off-plane part and normalized;
- `v_ref = normal.cross(u_ref)`.

That is the shape `geom_core::OrthoFrame` now records as a type: a
decided normalize, a residual perpendicular to it, and the cross
product they determine. Two of the three lengths here can be zero for a
real input — a degenerate loop, and a walk whose farthest point lies on
the normal — and what happens then is stated as propagation ("a
degenerate loop propagates non-finite components to the CDT, which
refuses typed"), one lane away from the thing that was actually
unusable.

## Why it is filed rather than fixed

The function takes no `Tol`, so it holds no band, and the mint's
refusals want one; giving it a band is a signature change on MESH's own
tessellation path with its own refusal vocabulary to choose. That is a
mesh decision, not a scalar-lane one, and the sweep that found it does
not get to make it.

The function's own doc already argues why its in-plane axis is NOT
`newell_plane`'s ambient `orthonormal_basis` pick, which is a separate
and still-good reason for this frame to be its own: the row is about
the decisions, not about the choice of `u`.

## Pointer

`crates/geom-core/src/linalg/ortho_frame.rs` — `OrthoFrame::gram_schmidt`
is this ladder written once, with both lengths decided and a typed
refusal naming which axis. `work/props/a-widened-derived-placement-normalises-a-straddling-newell-sum.md`
is the same undecided `normalize` one crate over.

## Re-homed at S-MESH's exit (2026-09-16)

Moved from `work/mesh/` to TESS (opened at this exit as S-MESH's successor for the tessellation kernel) when S-MESH closed (`docs/S-MESH-EXIT-WALK.md`); the item's content, id and history are unchanged.

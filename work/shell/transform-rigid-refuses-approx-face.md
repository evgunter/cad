---
id: transform-rigid-refuses-approx-face
kind: issue
title: transform - map an Approx face; the composition law holds, the mapped certificate needs a re-derivation lane (OFF-D)
status: closed
opened: 2026-08-26
github: 1020
refs: [1012]
closed: 2026-09-04
---

## From GitHub issue 1020

Opened 2026-08-26; 0 comments.

`topo::transform_rigid` refuses a `Surface::Approx` face typed (`TransformError::ApproxSurface`). The composition law itself holds and is pinned numerically in `geom-brep`'s suite (`approx_surface::a_rigid_map_of_an_offset_is_the_offset_of_the_rigid_map`): a rigid map carries unit normals to unit normals, so `M(S + d·n) = M(S) + d·n_M`, and the mapped fit certifies against the mapped base at the same `d` and the same tolerance, the two sup bounds agreeing to 1e-9.

**What blocks the door, precisely.** `map_surface` is generic in `T` and carries neither band nor tolerance; re-deriving the mapped surface's two-limb certificate is `f64`-only fit-door work. Carrying the *existing* certificate across is not an option — that is a geometry change, and the never-trust posture forbids it (`EdgeCurve::with_remapped_surfaces` is narrow on purpose: keys, never geometry).

**Two shapes for the fix**, either acceptable:
1. Inject a certifier at the transform door, the way `EdgeCurve::certify_nurbs_lane` takes its plane×NURBS lane — a caller that can re-derive hands one in, and a caller that cannot gets today's refusal.
2. A `PropsQuadLane`-shaped per-scalar lane, mirroring `recertify_approx`.

**Scheduled at OFF-D** (the face-replacement/shell unit), which is the first consumer that will want to move an `Approx`-faced body. Recorded in `work/verbs/log.md`'s OFF-C entry and cited from PR #1012.

Cost of leaving it: nil today. A body with an `Approx` face is a body with spline geometry, and `Surface::Nurbs` has refused this pass since it existed (`TransformError::NurbsPlaceholder` — whose message is itself stale, predating the NURBS evaluators; worth fixing alongside).

Filed from VERBS-OFF-C (#1012), MINOR-3.

## Home

Scheduled at OFF-D, the face-replacement/shell unit, which is VERBS' own; `crates/geom-brep/src/offset*.rs` and `crates/topo/src/replace_face.rs` are in its `paths:` territory.

## Closed (SHELL-2, PR #1758, 2026-09-04)

Shape 2, on the lane `transform_rigid` already binds: the mapped base
and fit nets, the certificate re-derived through the scalar's fit
lane, `ApproxLaneUnsupported`/`ApproxRecertify` typed where it cannot.
The `NurbsPlaceholder` message now says what issue record 1346
established; that arm stays open there.

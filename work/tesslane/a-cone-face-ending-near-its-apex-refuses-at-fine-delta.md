---
id: a-cone-face-ending-near-its-apex-refuses-at-fine-delta
kind: issue
title: A cone face ending near its apex refuses CertificateExceeded at fine δ: the development triangulates, the chart certifies
status: parked
opened: 2026-11-14
priority: P1
blocked_on: [the-cone-arm-certifies-the-triangle-its-development-makes]
---

## Finding

The unit funnel, `revolved[(0,0), (1,1), (0,1)]` (apex at the origin, rim
of radius 1 at `y = 1`), split by the plane through `(0, h, 0)` with normal
`(sin 0.3, cos 0.3, 0)`. Measured on `tesslane/chart-table` (PR 6950, its
third pass), the half above the cut, whose section ellipse passes within
`h` of the apex:

| h | refuses `CertificateExceeded` at δ | bound | meshes at δ |
|---|---|---|---|
| `1e-2` | `1e-3, 2e-4, 5e-5` | `1.25δ … 1.49δ` | `2e-2, 5e-3` |
| `3e-3` | `1e-3, 2e-4, 5e-5` | `1.67δ … 4.0δ` | `2e-2, 5e-3` |
| `1e-3` | `5e-3, 1e-3, 2e-4, 5e-5` | `1.04δ … 6.1δ` | `2e-2` |
| `1e-4` | `5e-3, 1e-3, 2e-4, 5e-5` | `1.03δ … 7.8δ` | `2e-2` |
| `1e-6` | `5e-3, 1e-3, 2e-4, 5e-5` | `1.03δ … 8.0δ` | `2e-2` |

The half below the cut meshes at every δ, and cuts at `h = 0.02, 0.03, 0.1,
0.3` mesh both halves at `2e-2 … 5e-5`. The row
`reach_probes::a_cone_face_ending_near_its_apex_refuses_at_fine_delta`
(`crates/mesh/tests/reach_probes.rs`) pins exactly this table.

**On main the face refuses at every δ**, `UnsupportedCurve` from
`trimmed::trim_frontier`: main has no trimmed cone arm (read from main's
source, `crates/mesh/src/trimmed.rs`, `trim_frontier`; not rebuilt).

## Mechanism

Verified: the cone arm triangulates in the cone's development
(`crates/mesh/src/trimmed.rs`, `developed`, `cdt_point`) and certifies each
triangle in the chart, `cert::cert_cone` (`crates/mesh/src/cert.rs`,
`cos α·sin α·v_max·(1 − cos(Δu/2))`), which reads a triangle's azimuth span
`Δu` at its widest `v`. With `AnalyticRow::cert`'s cone arm zeroed, every
refusing case above meshes, and `reach_probes::geometry`'s sampled true
deviation is within δ in all but two: `h = 3e-3, δ = 5e-5` at `1.63δ` and
`h = 1e-3, δ = 5e-5` at `1.23δ` (the `h = 1e-6` cases stop first at the old
shape bound, so they are unmeasured). So the certificate is conservative for
most and some faces truly need more points.

Likely, not re-verified here (a reviewer's reading of the refused triangles):
they join small-end ellipse points (`v ≈ 0.014`) to first-lattice-row points
(`v ≈ 0.085`) across `Δu ≈ 0.44` rad, near in the development and wide in the
chart. The lattice is uniform in `v` and has no row between the small end
and its first row.

## Candidate cures (not chosen)

1. A cone certificate that reads the developed triangle (its true deviation
   from the cone, which the development makes a planar question).
2. Certificate-driven refinement on the cone arm (today the spline arm's
   only), inserting the developed centroid (`TrimmedFace::inside_point`).
3. A lattice graded toward the face's small end, so no triangle spans from
   the section's small end to a far row.

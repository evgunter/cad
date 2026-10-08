---
id: conic-section-just-past-its-band-fails-certification-downstream
kind: issue
title: plane x cylinder and plane x cone mint a section whose construction error outgrows the band just past their degenerate trilean, and it refuses at certification instead of at the arm
status: open
opened: 2026-10-01
---

Found by the plane × cone split lane (`plane-cone-elliptic-section-split-refusal`),
measuring tilts toward the parabola.

## What happens

Both conic arms in `crates/geom-brep/src/intersect.rs` decide their
degenerate boundary with one trilean levered at the caller's `extent`
and mint the exact `Ellipse` on the definite side:

- `plane_cylinder_section`'s `pc_axis_plane_parallel`
  (`c = axis·n`): the semi-major axis is `r/|c|`;
- `plane_cone_section`'s `pn_conic_type`
  (`D = sin α·‖axis×n‖ − cos α·|axis·n|`): the semi-major axis is
  `|δ|·sin α·cos α / (c² − sin²α)`, with `c² − sin²α ∝ |D|`.

Just past the band the minted carrier is enormous, and its centre and
axes carry an absolute rounding error that scales with it. The arm
calls it zero-residual; certification downstream does not. Measured
through `topo::splitting::split` at `Tol::witness()` (band `1e-9` /
`1e-8`):

| fixture | tilt | outcome |
|---|---|---|
| cylinder r=1, h=1 (revolve), plane through `(0, 0.5, 0)` | `π/2 − 100ε` off the axis | `Join(Euler(Certification { Escalated { EndpointEnd, carrier_endpoint_end, 2.4e-9 } }))` |
| same | `π/2 − 1000ε` and beyond | splits |
| frustum `1 → 1/2`, `α = atan ½` (revolve), plane through `(0, 0.5, 0)` | parabola `− 3ε` / `+ 3ε` | `Escalated(pn_conic_type)` — the arm's own band |
| same | parabola `− 10ε` | `Join(Euler(Certification { Escalated { Surface1Residual, carrier_on_surface, 2.2e-9 } }))` |
| same | parabola `− 100ε` | `Pcurves(Certify { ResidualExceeded { Envelope } })` |
| same | parabola `− 1000ε` | `Pcurves(Certify { Escalated { Envelope, pcurve_envelope, 5.4e-9 } })` |
| same | parabola `− 10⁴ε` and beyond | splits, valid at every tier |

(`ε = 1e-9` here.) Every outcome is typed and no invalid body is
emitted, but between the arm's band and the point the carrier is
accurate enough, the refusal comes from a later door with the wrong
cause: a certification residual rather than "this section is too close
to degenerate to construct at this tolerance".

## What closes it

An arm-side admission criterion on the carrier's own conditioning, in
the shape `cone_cylinder_section`'s `coc_station_reach` already takes
for its circles (`BeyondOperandExtent`): refuse at the arm when the
constructed carrier's position error — growing like the semi-major
axis — cannot be held under the band. What the right lever is (the
semi-major axis against `extent`, or against the band's own reach) is
the design question; a naive `A ≤ extent` refuses legitimate long
sections (the frustum cut at tilt `1.0` has `A ≈ 3.5` against a face
extent near `2.2`, and certifies exactly).

## The region is wider than the table (dual review of #3688, 2026-10-01)

The table above starts at the arm's band. Measured by the review's
certificate sweep, which drives `chart_pcurve` + `PcurveCache::certify`
on the plane × cone ellipse directly (`ε = 1e-9`), the downstream
refusal reaches much farther from the parabola, and it grows with the
cone's half-angle and the scale:

| cone | tilt (fraction of the parabolic limit) | outcome |
|---|---|---|
| `α = 1`, unit scale | 0.999 | certifies (envelope 1.2e-10 on the `+` traversal) |
| `α = 1`, unit scale | 0.99999 | `ResidualExceeded { MapResidual }` (map residual **1.8e-6 m**) on one traversal, `Escalated(pcurve_map_residual)` on the other |
| `α = 1`, scale 1000 | 0.999 | `ResidualExceeded { MapResidual }` (1.5e-7 m) |
| `α = 1.55`, unit scale | 0.95 | `Escalated(pcurve_map_residual)`, margin 1.0e-9–1.4e-9 |
| `α = 1.55`, unit scale | 0.999 | `ResidualExceeded { MapResidual }` (1.8e-7 m) |

The carrier itself stays accurate across that range (its own residual
1.3e-10 at `α = 1`, 0.99999); it is the chart image's map residual that
grows, so an admission criterion has to bound the image's conditioning
as well as the carrier's.

**The refusal's text misnames the cause.** In the split, a frustum cut
`1e-8` short of the parabola (`φ = 1.1071`) refuses at the Euler gate:

> an Euler operation refused: geometry attachment gate: the
> start-endpoint residual at sample 0 escalated: margin 2.03e-9 lies
> inside the ambiguity band (1e-9, 1e-8). There is no way through: this
> is a kernel defect; report it

The input is legal and only ill-conditioned at this ε, so "no way
through" and "kernel defect" are both false: a looser tolerance or a
tilt farther from the parabola goes through. The ending is
`CertifyError::render(Reading::Build)`'s for every attachment-gate
residual (`geom-brep/src/certify.rs`), which is right for a carrier the
kernel built from well-conditioned data. The arm-side criterion above
is what makes it right here too: the section refuses at the arm, with a
tolerance recourse, before a carrier reaches the gate. Until then this
is the class's mislabelled door.

---
id: surface-frame-directions-are-unchecked-at-rest
kind: issue
title: No tier-3 check reads an analytic surface's frame directions - a zero axis or u_ref is named nowhere, and geom's crate docs say tier 3 certifies them
status: review
opened: 2026-09-24
priority: P3
cost: D
refs: [ATREST-6]
parent: ATREST-13
---

## What

ATREST-6 gave check 1 a poison read of every analytic surface's
stored datums (`poisoned_datums`, `crates/topo/src/validate.rs`): each
must be a finite number, and a plane's `normal` must not be the zero
vector. The zero-vector question is asked of the plane's normal ONLY,
because that is what the unit's spec scoped. Every other stored
direction — the `axis` of a cylinder, cone, sphere or torus, and the
`u_ref` of every analytic kind — is read for finiteness and nothing
else, so a zero axis or a zero `u_ref` passes check 1:

- a zero `axis` collapses a cylinder's `∂v` and a cone's generators,
  and puts a sphere's or torus's `v_ref = axis × u_ref` at zero;
- a zero `u_ref` collapses `radial(u)` for every axisymmetric kind and
  `v_ref` for the plane.

Neither describes the surface its variant names, which is the
plane-normal argument word for word.

The wider half: `geom`'s crate docs state that the frame fields are
"conventional data, unchecked here … **Tier-3 geometric validation
certifies the invariants at rest**" (`crates/geom/src/lib.rs`, the
unit-vector bullet). No tier-3 check reads a surface frame's
unit-ness or `u_ref ⊥ axis`. `validate_geometric`'s own not-yet list
(`crates/topo/src/validate.rs`, "Curve conventional-invariant
certification") names the CURVE half as not independently certified
and says nothing of the surface half, so the two documents disagree
and the crate docs are the one that overclaims.

## What must be decided

Whether a zero direction is the poison half (a datum that describes
no locus, named at check 1 beside the plane normal) — the likely
answer, and a two-line extension of `poisoned_datums` — and,
separately, whether unit-ness and orthogonality are tier 3's to
certify at all, or `geom`'s sentence is what should change.

## Fence

Track P. `crates/topo/src/validate.rs` (check 1); the `geom` sentence
is `props` ground and a seam to announce.

## Answered (ATREST-13)

**The poison half.** A zero or non-finite `normal`, `axis` or `u_ref`
of any analytic surface is `ValidationError::PoisonedSurfaceDatum`,
through the one reading ATREST-6's plane normal used
(`crates/topo/src/validate.rs`, `is_direction`: `is_finite_length`
per component, then `is_zero_length` on a finite norm). Pinned by the
six zero-direction rungs of
`check_1_names_the_analytic_datum_that_describes_no_locus`
(`crates/topo/src/tier3_tests.rs`).

**Unit-ness and `u_ref ⊥ axis`, measured first.** *Does a consumer
read an off-convention frame as a different locus?* Yes, for every
axisymmetric kind: the evaluators read the frame through
`geom::azimuth::frame` (`radial(u) = u_ref·cos u + (axis × u_ref)·sin
u`) while the implicit forms and the section arms read `axis` and the
radius as the geometric axis and radius, so a `u_ref` of length
`1 + δ` evaluates a cylinder of radius `r(1 + δ)` and an `axis` of
length `1 + δ` evaluates an elliptic section — two loci off one datum.
For a plane it does not (a non-unit `normal` or `u_ref` spans the
same plane), and a plane's tilt and a cone's frame move the locus by
an amount that grows with the face's extent, which no datum levers.
*Does the corpus mint frames off unit beyond the band?* No. The
corpus instrument (CI run 36154432046; every `validate_geometric`
call of the nextest suite at all three ε rows, the tour's tests and
binary, the wild STEP corpus) logged every frame deviation above
`1e-15`. The largest locus movement it implies (deviation × the
kind's radius) is `1.7e-14 m` at `f64` (a wild STEP cylinder's
`u_ref`, `4.2e-13` off unit at `r = 0.04`; `step-import` adopts a
near-unit `DIRECTION` verbatim within the file's ε_in) and
`1.5e-13 m` at the interval scalar (an enclosure's width, a
`fillet_h5` circle `axis` at `r = 0.05`) — against the tightest row's
ε of `1e-12 m`. The stop clause did not fire.

**So the frame is a representability convention**, refused through
ATREST-6's door with the band: `geom::Surface::representability_margins`
takes the run's `Band` and returns, after the scalar conventions, the
frame's six ε-slack margins at the kind's radius (cylinder and sphere
`r`, torus `R + r`; `geom::surfaces::frame_margins`), read by check 1's
one `Bounds::lo` compare and refused as
`UnrepresentableSurfaceDatum { datum: Axis | URef, end }`. Pinned by
the frame rungs of the same test (a `u_ref` one part in a million long
at `r = 1`, a half-length sphere axis, a tilted torus `u_ref`) and the
inside rows of `datums_inside_their_conventions_draw_no_datum_verdict`
(a `u_ref` one part in 1e12 long, a plane frame of length 3).

**`geom`'s crate docs made true**: the conventions paragraph now says
what check 1 certifies and what it does not. The uncertified half is
filed: `unlevered-frame-conventions-are-uncertified-at-rest`.

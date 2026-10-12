---
id: a-section-curves-end-read-in-band-of-its-vertex-refuses-as-a-kernel-defect-at-the-join
kind: issue
title: A section curve's end read in band of the vertex it meets refuses at the join as a kernel defect, at the split and at the Boolean
status: dispatched
opened: 2026-10-12
priority: P2
cost: M
branch: cleave/section-end-inband-join
---


## What

Found by the sweep of `cleave/minted-edge-inband-cert` (the near-tangent
census over the split door and over curved seam poses; probes
`sweep/examples/split_near_tangent_census_probe.rs` and
`seam_near_tangent_census_probe.rs`). The join mints each section chord
through `chord_join.rs`'s `mef`/`mekr` with the section curve's spec
(`chord_spec`), and its certification refusal reaches the user whole
through `SplitJoinError::Euler`. Its endpoint pins (`EndpointStart`,
`EndpointEnd`) are typed exact constructions in certify's ending table, so
an in-band reading ends "There is no way through: this is a kernel
defect; report it". On these poses the miss is the input's, at a band's
edge:

- **A vertex read on the plane at the zero band's edge.** Split of the
  unit cylinder (`z ∈ [0, 1]`) by the plane through `(1, 0, δ)` with
  normal `(1e-3, 0, 1)`: the seam vertex `(1, 0, 0)` reads on the plane,
  and the section ellipse ends `|δ|` from it along the seam.
  `δ = ±1e-9` at ε = 1e-9: `EndpointEnd` 1.0000000001241954e-9;
  `δ = ±1e-12` at ε = 1e-12: `EndpointEnd` 1.0000000259993925e-12.
- **The plane read square to the axis, or along it, in the conic arm's
  zero band.** The same cylinder by the plane through `(1, 0, δ)` with
  normal `(t, 0, 1)`, `t·r = ε`: at ε = 1e-9, `t = 1e-9`,
  `δ ∈ {5e-8, 1e-6, 1e-4}`: `EndpointStart` 1.0000000000000098e-9 to
  1.0000000000022002e-9; at ε = 1e-6, `t = 1e-6`, `δ ∈ {5e-5, 0.1}`:
  `EndpointEnd` 1.000000000000004e-6, `EndpointStart` 1.000000000001e-6.
  The Boolean reaches the same join: the cylinder against a slab whose
  face through `(1 + δ, 0, 0.5)` is tilted `1e-6` about `y`, at
  ε = 1e-6, `δ ∈ {−5e-5, −1e-3}`, refuses all six runs
  `Join(Euler(Certification { Escalated { EndpointStart | EndpointEnd } }))`,
  margin 1.0000000000287557e-6.

On these poses the margin is the input's offset read one rounding past ε,
the PR 4732 story (the certificate bounds the carrier). But that is not
the door's typical in-band reading. The review of PR 4773 ran a dense
cylinder census of planes nearly along the axis, cutting the split's own
elongated ellipses (semi-major axis about `r/θ` at tilt `θ`). At
ε = 1e-12, 774 runs refuse `Escalated { EndpointStart | EndpointEnd }` at
about 1.1–1.4e-12, at every offset, with no vertex near the band. That is
the ellipse's own rounding amplified into the band, a Q1 defect, for
which "kernel defect" is the honest ending. An endpoint residual alone
cannot tell the two populations apart.

Where the miss is the input's, each story wants its own decision and
lever:
- for the first, "move the split plane or the geometry so each vertex
  that meets the plane lies clearly on it or clearly off it";
- for the second, "tilt the plane clearly square to (or along) the
  curved face's axis, or clearly off it".

The finish's describer already types the line-chord case
(`SplitFinishError::EdgeOnFacesEscalated`, through
`merge_faces::edge_on_faces_reading`, which deliberately leaves circles
and ellipses to the gate). The join runs upstream of it.

The definite neighbour, at ε = 1e-12: the plane along the ruling at
azimuth 0 or 0.5, tilted 1e-6 off the axis, refuses
`ResidualExceeded { EndpointStart }` (1.0e-11 to 1.16e-10) in 80 runs over
`δ ∈ [−1e-7, 1e-7]`: the ellipse's semi-major axis is 1e6 m, and its
rounding outgrows the band. That is GERM's
`conic-section-just-past-its-band-fails-certification-downstream`, where
the measurement is added.

## Owed

First separate the two populations at the join: an in-band endpoint pin
whose margin is an input offset (a vertex at the zero band's edge, or the
conic arm's trilean read in its zero band) from one that carries the
section conic's amplified rounding. Only the first may become an
`Escalated` refusal of the input; the second stays the kernel's (Q1), or
becomes an arm-side refusal on the carrier's conditioning, as GERM's row
asks. Then give each input reading its lever, at both doors, and check
that moving the parts as told builds.

## Designed (2026-10-12; designer pair converged after one reconciliation round, no ratified text changes)

PR 4810's first build re-read the escalated pin with the vertex moved
onto the plane. Its review measured that the re-read reads a sum, so
at ε = 1e-12 a near-axial ellipse's own rounding was typed `OnPlane`
or left as the defect depending on the vertex offset. No reading at
the join separates the two populations. The join's miss has three
terms with three owners:
- the vertex's offset δ from the plane, grown by 1/sin ψ (ψ the
  dihedral between plane and wall at the vertex): the input's;
- a stood-in circle's Zero'd margin: the input's;
- the curve's own resolution: the kernel's.

The kernel's term is two limits of storing an ellipse as a centre plus
axes with θ = 0 at a major vertex. Both were measured, and matched to
the digit in two independent replications:
- **(a)** The parameter near ±π/2 resolves only to ulp(π/2)·A, with
  A ≈ r/θ. A clockwise span taken as 2π minus a small span adds
  ulp(2π)·speed.
- **(b)** A far centre (≈ w/θ off-axis) holds only about
  5·u·|centre − face|.

Q1's "re-posed or certified, never covered by K", in four units:

1. `a-section-ellipse-is-re-posed-at-its-nearest-principal-vertex`
   re-poses the curve and removes (a). Prototype on 4810's head at
   1e-12: the coin-flip family builds 66/66 and through-axis planes
   96/96.
2. `the-section-table-admits-a-curve-on-its-resolution` admits on
   ρ ≤ ε/K, refuses the rest (b) as a kernel limit with no lever, and
   makes `Curve3::resolution` the one rule.
3. This row (PR 4810, reworked on top of 1 and 2): every escalated
   attachment pin on an admitted section curve is the input's. That
   covers the endpoint pins and a stood-in circle's surface residual
   (review m3). Each is typed by provenance (`OnPlane` /
   `SquareToAxis`), with no re-read. `SectionEnds::reading` goes; the
   types, the stories, their levers and `edge_on_plane_refusal` stay.
   A definite miss stays the gate's.
4. `the-split-reads-a-vertex-against-the-section-it-ends-not-the-plane`
   is separable. Once it lands, a definite join miss really is a defect
   (review m2).

Why ε/K and not ε: with ρ ≤ ε, an in-band pin typed as the input's
could be up to 90% the kernel's. That would offer the kernel's error
as the user's lever, which PR 4335 forbids. Why a kernel limit and not
a tilt lever: the same rule; D4 ¶1(i)'s last-resort ending applies.
Labels and reports are on `analysis/design-fork/cleave-section-end-carrier-bound`
and in the orchestrator's log.

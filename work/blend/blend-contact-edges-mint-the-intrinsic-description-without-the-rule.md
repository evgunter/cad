---
id: blend-contact-edges-mint-the-intrinsic-description-without-the-rule
kind: unit
title: sweep: a blend's contact edge mints TangentIntersection without the must-carry rule
status: dispatched
opened: 2026-09-13
branch: blend/14-contact-edge-must-carry
---


## Finding

`geom_brep::must_carry_over_edge` is the one home of the rule that decides
what description a definitely-smooth join carries: lane gate, the
certification schedule's interior stations, three-way typed answer. The two
sweep verbs route through it.

A blend's CONTACT edge does not. `attach_contact`
(`crates/sweep/src/blend/surgery.rs`, the `EdgeDescriptionSpec` selection
below `let is_seam = …`, ~`:3893`–`:3918`) picks the description from two
STRUCTURAL flags and nothing else:

- `is_seam` (the carrier is a `ContactCarrier::SeamArc`) → `seam`;
- `transverse` (`ContactCarrier::Chord` or `TransverseArc`) → `Intersection`;
- otherwise → `EdgeDescriptionSpec::TangentIntersection { s1, s2, witness }`
  (~`:3918`).

That last arm is an intrinsic MINT with no lane gate, no station walk and no
in-band escalation — the same structural shape as the revolve fold BLEND-9
corrected, one level over: if the contact edge's second-order determinacy is
ever in-band, the body is built with a description tier 3 then refuses under
`tangent_second_order` (`crates/topo/src/validate.rs`, the must-carry arm's
`decide("tangent_second_order", …)`, ~`:4029`).

## The reading (measured, default ε, band `(1e-9, 1e-8)`)

A throwaway row walked every interior station of every contact edge that
stores `TangentIntersection`, through `geom_brep::tangent_second_order` with
`geom_brep::edge_extent` for the extent:

| fixture | contact edges | lane | stations | min margin |
|---|---|---|---|---|
| `cube(1.0)` filleted at `r = 0.15`, all edges | 48 (24 band–support plane/cylinder, 24 band–corner cylinder/sphere; the delta's count, corrected from 40) | all in lane | all `Positive` | `7.5e-2` |
| `rod_with_flat` filleted at `ROD_FILLET = 0.1` on `rod_creases` | 4 | all in lane | all `Positive` | `5e-2` (plane support), `4e-2` (cylinder support) |

The numbers are exactly the closed form: the band is tangent to its support
along the contact locus, so the relative transverse normal curvature is
`|1/r_band ∓ κ_support|` and the folded lever arm is `r_band`, giving
`margin = |1/r_band ∓ κ_support| · r_band² / 2`. For a plane support that is
`r_band/2` (0.075 and 0.05 above); for the rod's `R = 0.5` cylinder support it
is `|1/0.1 − 1/0.5|·0.01/2 = 4e-2`. Seven or more orders above `K·ε`.

## The decision this wants

The class is real and is NOT definite by construction. The closed form
collapses exactly when the support's transverse normal curvature matches the
band's — a band OSCULATING a support of its OWN convexity (the difference
branch, `κ_support = 1/r_band`; the rod's convex band on its convex cylinder
support is that branch, at `R = 0.5` against `r_band = 0.1`) — and
it is in-band whenever `|1/r_band ∓ κ_support| · r_band² / 2 ∈ (ε, K·ε)`. On a
plane support that needs a blend radius of a few `ε`, which the blend arm's own
radius gates refuse long before; on a curved support it needs only a near-match
of two curvatures, which is ordinary geometry at ordinary scale.

So the honest disposition is: route the arm through
`geom_brep::must_carry_over_edge` (it would also give the contact edge the lane
gate it has never had — a `Nurbs`-supported band is outside
`tangent_certificate_lane` and cannot store an intrinsic tangency at all), and
add the row that builds a near-osculating concave blend and asserts the typed
refusal. Deliberately NOT done on BLEND-9: that unit's spec scopes the change
to the extrude strut arm and the revolve latitude join, and constrains
`dihedral.rs` to the wrapper.

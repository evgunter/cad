---
id: the-near-node-poses-row-is-red-on-main-at-a-single-caps-intersection
kind: issue
title: bisector_nodes::the_near_node_poses_at_the_witness_band is red on main: A's cap at δ = 1e-8, (B, A), ∩ refuses at its result gate, whose volume quadrature cannot read a cut face's image
status: open
opened: 2026-11-03
---



Found by ONEDOOR's one-surface-record lane, 2026-11-03, at the default ε
(1e-9), run locally.

`crates/sweep/tests/bisector_nodes.rs`
`the_near_node_poses_at_the_witness_band` fails on the cell "A's cap
δ = 1e-8, direct (B, A), ∩". The row admits a build or a `Containment`
escalation there; the boolean returns its result gate's refusal
instead:

    Err(ResultInvalid { errors: [VolumeUncomputable { solid: SolidKey(1v1),
      source: Face { face: FaceKey(4v1), source: QuadratureUnsupported {
      what: "curved-cut face half-edge carries a pcurve with no harmonic
      form on an analytic chart (a FITTED or PROJECTED image, …)" } } }] })

Red at main `55f1a93ff1` and at `cc23ffe8fb`, run on clean checkouts of
each. Two perpendicular rods meet in no one-carrier pair, so no
`Body::unify_surfaces` call runs during the row (checked with a print in
it on the ONEDOOR branch). The red is not that door's.

The row's other cells and `bisector_nodes`' other rows pass. Which edge
of face 4 carries the projected or fitted image, and which stage minted
it, is unread.

Wanted: find the stage that mints that image on the ∩'s curved cut face
and mint it in closed form, or refuse the cell typed at the decision
that is in band. Then the row is green again.

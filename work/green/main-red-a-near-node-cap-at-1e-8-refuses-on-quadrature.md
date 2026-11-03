---
id: main-red-a-near-node-cap-at-1e-8-refuses-on-quadrature
kind: issue
title: main red: bisector_nodes::the_near_node_poses_at_the_witness_band, A's cap at δ = 1e-8 (B, A) ∩, refuses on an unwired quadrature where the rim's placement escalation is pinned
status: open
opened: 2026-11-03
priority: P2
cost: E
---



Found by `onecopy/foot-certificate` running sweep's full suite. It reproduces with the same message on `origin/main` at `c02eb40cf`.

`sweep::all bisector_nodes::the_near_node_poses_at_the_witness_band` panics at `crates/sweep/tests/bisector_nodes.rs` (~592):

> A's cap δ = 1e-8, direct (B, A), ∩: the rim's placement in band, got Err(ResultInvalid { errors: [VolumeUncomputable { solid: SolidKey(1v1), source: Face { face: FaceKey(4v1), source: QuadratureUnsupported { what: "curved-cut face half-edge carries a pcurve with no harmonic form on an analytic chart (a FITTED or PROJECTED image, or a cone section's or Villarceau circle's focal section) — its Green-form boundary integral is not wired" } } }] })

The row pins an escalation on the rim's placement for this cell. The operation now builds and fails the volume gate on a pcurve with no harmonic form, so whichever PR moved the operation past the placement also moved it onto an unwired quadrature lane. `touchrec/a-single-cap-near-a-rods-axis-fails-tier-3-prime-on-its-own-records` reads neighbouring cells of the same test, but not this failure.

Reproduce: `cargo nextest run -p sweep -E 'test(=bisector_nodes::the_near_node_poses_at_the_witness_band)'`.

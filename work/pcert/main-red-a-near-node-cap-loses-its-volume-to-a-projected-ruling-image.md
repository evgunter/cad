---
id: main-red-a-near-node-cap-loses-its-volume-to-a-projected-ruling-image
kind: issue
title: main red: since E2 (PR 6470) a cap 1e-8 off a node loses its volume to a projected ruling image (bisector_nodes::the_near_node_poses_at_the_witness_band)
status: open
opened: 2026-11-04
priority: P1
cost: M
refs: [a-ruling-read-in-band-meets-consumers-that-read-no-projected-image]
---


Filed by the SECTARMS cylinder 3b-ii lane (PR 6466), which met it on its
slow-set run after merging main.

## What

`crates/sweep/tests/bisector_nodes.rs`
`the_near_node_poses_at_the_witness_band` (in the slow set, so the
per-PR gate does not run it) is red on main. Its cell

    A's cap δ = 1e-8, direct (B, A), ∩

expects a body or the rim's placement escalating in band
(`BooleanDecision::Containment`). It now gets

    ResultInvalid { errors: [VolumeUncomputable { solid: SolidKey(1v1),
      source: Face { face: FaceKey(4v1), source: QuadratureUnsupported {
      what: "curved-cut face half-edge carries a pcurve with no harmonic
      form on an analytic chart (a FITTED or PROJECTED image, …" } } }] }

## The commit

Bisected on main, the row alone, ε 1e-9:
- `27205d771f` (first parent of the merge below): passes (1 of 1).
- `7a8852d21d`, "Merge pull request #6470 from
  evgunter/pcert/cylinder-line-lane" (PCERT E2): fails at
  `bisector_nodes.rs:592` with the refusal above.
- `a2b8e4633c`: fails identically.

E2 moved an in-band ruling on a cylinder from the meridian form to the
projected lane (`projected::FramedCarrier::Line`). The cap `1e-8` off the
station leaves a short ruling where its disc meets the partner wall,
read in band. Its projected image reaches tier 3's volume, which has no
harmonic form for it. This is the third consumer named in
`a-ruling-read-in-band-meets-consumers-that-read-no-projected-image`
(props' cylinder inventory), with a boolean row as its witness. That
row recorded no regression because nothing it ran reached it; this one
reaches it and went red.

The same cell on SECTARMS's branch (which retires the bisector lane)
fails identically, so it is not the lane: PR 6466 leaves it red and
says so.

## Done when

The row is green on main again: the cell builds at its closed form, or
refuses with the placement escalation it pins, or it is re-pinned on a
ruling of the volume's lane for a projected ruling image.

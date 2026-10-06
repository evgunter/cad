---
id: blend-material-is-never-checked-against-faces-that-are-not-its-supports
kind: issue
title: blend: the band's material is never checked against faces that are not its supports, so a concave fillet grows into an island and builds an overlapping body every tier admits
status: open
opened: 2026-10-06
priority: P1
cost: H
---

Found by the lane that made the blend carve each chain inside its own
shell (`a-blend-refuses-a-solid-of-several-shells`).

The battery and the surgery read a chain's supports, their rings, its
corners and its caps (`blend_surgery` in `crates/sweep/src/blend/surgery.rs`,
`consumption_sweep` and `face_clearance` in `battery.rs`), and nothing
else. A CONCAVE band adds material; whether that material reaches a face
that is not one of the chain's supports is never asked. When it does, the
blend builds an overlapping body and no tier refuses it.

Two witnesses, both a `[0,4]³` block with the `[1,3]³` cavity whose
twelve edges are filleted at `r = 0.25` (the band's material reaches
`r(1 − 1/√2) ≈ 0.073` off each cavity wall at an edge), with an island
`0.05` off the cavity's walls:

- **One shell, on main (930c880a).** `vented_cavity()`
  (`crates/sweep/tests/common/cavity.rs`) with the island
  `[1.05,2.95]² × [1.05,2.4]` standing on a round stem (`rod` at
  `(2,2)`, radius 0.1, `z ∈ [0.9,1.1]`), unioned in: one solid, one
  shell. `fillet_edges` builds; `AtRestBody::validate` and
  `validate_geometric` return `Ok` (and so does
  `validate_pseudomanifold`, read on the per-shell branch, whose
  one-shell path is main's);
  `mass_properties` reads `V = 60.39064985215749`, the island's corners
  counted twice.
- **Two solids, now that the blend reads per shell.** The SEALED cavity
  with the island `[1.05,2.95]³` unioned in as a second solid:
  `fillet_edges` over the void shell's twelve edges builds;
  `AtRestBody::validate` and `validate_geometric` return `Ok`
  (`tier-3-admits-two-solids-of-one-body-whose-material-overlaps`), and
  only `validate_pseudomanifold` refuses it (`InstanceInterference`).
  `V = 63.159977219228125`, exactly the disjoint sum
  `56 + (8 − rounded_box_volume(1.5, 0.25)) + 1.9³`, so the overlap is
  counted twice.

A convex band removes material, and the same question stands for it
whenever a face that is not a support lies within the band's reach (a
thin wall over a void, a slot behind an edge).

The fix is the blend's: a clearance check between each band (its
trimlines, its surface, its corner patches) and every face of the body
within its reach that is not a support of that chain, in any shell. It
refuses typed when interference is certain or cannot be ruled out. The
per-shell door makes the second witness reachable through
`Node::Fillet`, which is why this is P1: an ordinary document
(an island in a hollow, unioned) ships a wrong body silently.


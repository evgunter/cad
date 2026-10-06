---
id: blend-material-is-never-checked-against-faces-that-are-not-its-supports
kind: issue
title: blend: the band's material is never checked against faces that are not its supports, so a concave fillet grows into an island and builds an overlapping body every tier admits
status: open
opened: 2026-10-06
priority: P0
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


## A third witness: a CONVEX band through a thin wall (review of PR 4092)

The full review of PR #4092 (sided radius headroom), on its head
`e1e1ef43`, built the convex case. `thin_flare` is a revolved wall
0.05 thick with a bore of radius 0.225 that turns 30° into a cone (the
tour's Klein neck→flare in miniature). The convex inner corner's band
reaches `0.0353·r` along the bisector, and the outer corner sits at
`0.0518`, so the band leaves the material once `r > 1.466`. At
`r = 1.5` and `1.6`, `fillet_edges` builds and `validate_geometric` is
`Ok` at all three eps rows; the band's bisector point lies past the outer
wall. On main the unsided radius headroom refuses these by accident (it
limits `r` by the bore's curvature, which the ball does not roll
against). The reviewer's probes are on branch `review/4092-probes`
(`crates/sweep/tests/review_4092_probes.rs`). PR 4092 waits on this row.

## Priority

P0, raised by the BAND orchestrator: a normal verb (fillet) silently
returns a wrong body that every tier admits, on ordinary geometry (an
island in a hollow; a fillet in a thin-walled revolve). This is "a live
wrong answer" in `work/README.md`'s bands.

## Scope note (orchestrator)

This extends predicate 2 (face clearance, C8 in `crates/geom-brep/README.md`)
from "the support faces' own boundary features" to "every face of the
body within the band's reach that is not a support of the chain, in any
shell". It is the predicate's ratified meaning, not a new predicate. The
check must certify clearance or refuse typed (`FaceClearance` /
`FaceClearanceUncertified`), before construction, and replay at
`Interval`.

## Findings (the lane that built it)

Measured against the tree at `cadf2ed1`:

- **The one-shell witness reproduces as stated**: with the meter taken
  out, `fillet_edges` builds the island-in-a-vented-cavity body; with it,
  the request refuses `FaceClearance { bounded: false }` on the island's
  own edge.
- **The two-solid witness cannot reach the meter through `fillet_edges`
  on main**: the surgery's body door refuses a body of two solids first
  (`UnsupportedBody`), and PR 4113's per-shell door is not merged. The
  row pins it at the meter itself (`test_support::band_reach`).
- **The convex flare is caught by the meter at `r = 1.5` and `1.6`** and
  certified clear at `r = 0.5` and `1.0`; on main predicate 1 refuses
  these radii first, so PR 4092 (sided headroom) is what will carry them
  to the meter through `fillet_edges`.
- **Where the meter runs.** It runs in the surgery's pre-mutation phase,
  after the ring carry-through pass, not inside the battery: at the
  battery it pre-empted the surgery's exact ring and boundary meters and
  the one-solid body door on 26 rows of the existing corpus, each of
  which those meters already refuse for the same physical fact (a ring
  or edge in the band's material, or a body the surgery does not
  carve). It is still before any surface is minted.

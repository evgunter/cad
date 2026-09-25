---
id: full-revolve-emits-split-planar-walls
kind: issue
title: A full revolve of an axis-touching profile emits each planar wall as two same-key halves, so F7 refuses it as a boolean operand through every door above the kernel
status: open
opened: 2026-09-25
priority: P0
cost: D
refs: [swept-continuation-walls-reach-the-boolean-unmerged, torus-operand-gate-admission]
needs_ev: true
---


## What

Measured by GERM's torus-gate measurement lane on main (2026-09-25), with
throwaway kernel examples.

- `Revolution::Full` of a profile that touches the axis runs
  `revolve/full.rs::build_wire`, the two-band sweep (0..π, then π..2π).
  Every band-2 wall is minted `FaceSurface::Shared(carrier)` from its
  band-1 twin. A CURVED twin pair is `gate_maximal_faces`' canonical
  maximal form. A PLANAR one is not.
- So a plain rectangle touching the axis, full-revolved into a cylinder,
  yields 6 faces. Each end disc is two half-discs on one key. The body
  passes `validate`, `validate_closed` and `validate_geometric`. A union
  with a DISJOINT box refuses `NonMaximalFaces`. After
  `Body::merge_coplanar_faces` (2 groups) the union succeeds.
- The same holds for a dumbbell half with single-segment caps: its end
  disc, shoulder annulus and joint disc are three split pairs.
- The profile's authoring does not matter: no `continue_to`, no collinear
  pair. So Ev's 2026-09-25 F7 ruling on declared continuations
  (`work/band/swept-continuation-walls-reach-the-boolean-unmerged.md`)
  does not cover it.
- The kernel repair has no door above the kernel. Kernel tests route
  around it (`sweep/tests/blend1_r1_probes.rs` says so;
  `bool2_cone_doors.rs` uses a quarter cone). Through `Node::Revolve` →
  `Node::Boolean` the union refuses. This is the wall Ev hit as a user on
  the revolved dumbbell (`work/germ/torus-operand-gate-admission.md`),
  ahead of every torus door.

**The π-revolve sibling.** `Revolution::Partial(π)` of an axis-touching
profile gives Start and End caps that are coplanar, co-oriented and
adjacent across the axis edge, on DIFFERENT keys. `merge_coplanar_faces`
finds no group, and the boolean refuses `UndeclaredCoincidence`, so such
a body is never an operand. Its coplanarity rests on θ = π, decided
numerically, which F7 does not merge on.

## The question

What the revolve should produce, and where maximality is owed, changes
F7's text or N3/N4's. So this is a design fork for Ev. GERM is running
the designer pair on it and will open the `[ev]` PR. The implementation
is CARVE's (`revolve/*`), with the naming half in the emitter.

## Put to Ev (2026-09-25)

Ev, on the `[ev]` PR: "it sounds like the fix should be to [have it] stop
doing that and just emit the right thing to begin with; is that
impossible/blocked?" It is not blocked. The two-band split exists only so a
curved wall's pole or apex keeps valence 2. The PR now states the answer
that way: the full revolve CONSTRUCTS each planar wall as one face, so F7
is unchanged (no revolve mints a same-key planar adjacency, and no merge
stage is needed). The merged-wall naming both designers recommended
follows: plain `Band(s)`, with no `BandPi(s)` and no `Pole` for a planar
segment. The π revolve stays a separate, parked question.

---
id: a-rim-whose-vertex-lies-on-a-wrap-edge-refuses-shared
kind: issue
title: A rim lying in a curved face whose vertex lies on the face's wrap edge refuses Shared: its end joins a vertex the face's loop passes twice, which the parting reads as two ends
status: open
opened: 2026-11-25
priority: P2
cost: M
---


Found by `heldrecords/parting-division` (2026-11-25), on
`heldrecords/split-on` at `80f11deb86` with that branch's narrowed
`CurvedFace` gate, at the default ε. Before it these refused
`CurvedFace` ahead of the surgery; now they reach it and refuse
`EdgeInFace { gap: Shared }`, pinned by
`crates/sweep/tests/a_closed_rider_lying_in_a_face.rs`
`a_rim_whose_vertex_lies_on_a_wrap_edge_refuses_shared`:

- the tube of radii 1 and 2 about `y` less the full revolve of
  `(1, .5), (1.2, .7), (1.2, .3)`: a closed circle in the bore;
- the solid torus `R = 3, r = 1` about `y` less the full revolve of
  `(3, 1), (2.8, .6), (3.2, .6)`, and a quarter turn of it;
- SECT's torus `R = 2, r = 1` less the puck `ρ ≤ 2.5`,
  `y ∈ [0, √3/2]` (its top rim the parallel at tube angle `π/3`), as
  built and turned 1 rad about `y`
  (`a-curved-slit-fails-tier-3-prime-on-its-two-coincident-arcs`,
  2026-11-20 note).

## What

The rim's vertex lies on the face's wrap edge, an edge both of whose
halves bound the face, so a vertex-vertex record joins the rim's end
to a vertex the face's loop passes twice. `boolean/slit.rs`
`part_riders` lists a closed edge's one vertex as both its ends, so
`joined` holds the pair twice; `part`'s second joint finds the end
already fused (`loop_leaving` of it is empty) and refuses `Shared`. An
open arc ending on the wrap edge refuses the same way at its joint.

The words are true (the end lies on the face's loop more than once),
but the configuration is ordinary. The rim runs once round the face's
chart from one side of the wrap edge to the other, so the face divides
there. The two copies of the end belong one to each side of the wrap
edge, read by which of the two passes of the loop each copy's
half-edges leave along.

## Done when

A rim whose vertex lies on a wrap edge divides the face, each end
parted or joined at the pass of the loop on its own side. The rows
above build at their closed forms with tiers 3 and 3′ at three ε, or
meet `PartsTouch` where they part the result into pieces: the torus
less the puck does (`∩ = 13.381483883`).

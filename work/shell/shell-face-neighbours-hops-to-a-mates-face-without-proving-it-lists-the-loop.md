---
id: shell-face-neighbours-hops-to-a-mates-face-without-proving-it-lists-the-loop
kind: issue
title: shell's face_neighbours reads a mate loop's face without proving the face lists the loop, so the open-face component count can join through a loop no face owns
status: open
opened: 2026-10-01
priority: P3
---


## What

Found by the receipt (§5) of PR 3669, which closed the same hop in
`Body::movefac` (`crates/topo/src/movefac.rs`, the labelling).

`shell::face_neighbours` (`crates/topo/src/shell.rs`) walks each of a
face's loops, takes each member's mate through `Body::mate`, reads the
mate's `parent_loop`, and returns that loop's `face` as a neighbour.
Nothing proves that the face it names lists the loop (its `outer` or one
of its `rings`), nor that the mate's own `edge` is the member's
(`Body::proven_mate`'s `NotSameEdge`), nor that a walked loop is the
whole of its claimants.

Its readers:

- `shell::count_components`, under `check_designation`: the number of
  edge-adjacency components the shell's remaining faces fall into, which
  decides `ShellError::OpenFacesDisconnect`. A mate loop torn to name
  another face joins two components through a loop that face does not
  own, so the count is low and a designation that disconnects the shell
  passes.
- `shell`'s antiparallel-walls check (`face_neighbours(body, a.face)?
  .contains(&b.face)`), which skips a pair it reads as adjacent.

`shell` and `shell_open` refuse a lookup that fails with
`ShellError::Corrupt`; the sweep that found this did not find a tier-1
check of the input before `check_designation`, and validates only the
assembled result (`validate_geometric`).

## Measured

Not probed. The shape is the one PR 3669 witnessed in `movefac` on
`fixtures::detached_digons(1)`: the digon's second face's outer loop
torn to name the seed face, and the shell torn to stop listing the
second face. At that PR's base `movefac` returned `Ok` with face
counts `[2, 1]`, joining the seed face's component to the digon's
through a loop the seed face does not list
(`movefac::tests::movefac_refuses_a_mates_loop_its_face_does_not_list`).

## Fix shape

Hop through `Body::proven_mate` and prove that the face the mate's loop
names lists the loop, refusing `ShellError::Corrupt` naming the loop, as
`movefac`'s labelling refuses `NotOwned { child: Loop, owner: Face }`.

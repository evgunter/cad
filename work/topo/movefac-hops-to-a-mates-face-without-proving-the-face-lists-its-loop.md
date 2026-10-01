---
id: movefac-hops-to-a-mates-face-without-proving-the-face-lists-its-loop
kind: issue
title: movefac's mate hop reads the mate loop's face without proving that face lists the loop, and drops a hop into a labelled component
status: open
opened: 2026-10-01
priority: P3
---


## What

Found by the review of PR 3592 (its finding N2). `Body::movefac`'s
labelling (`crates/topo/src/movefac.rs`, the precondition phase of
`movefac`) hops from a cycle member to its mate (`Body::proven_mate`),
then from the mate's `parent_loop` to that loop's `face`, and labels
the face as a neighbour. PR 3592 proves the downward direction: when a
face is popped, every loop its `outer`/`rings` list names it back
(`NotOwned { Loop, Face }`). The upward hop has no converse proof:
nothing checks that the face the mate's loop names lists that loop.

So the walk can reach a face through a loop the face does not own.
When the loop's true owner is still listed by the shell, the owner is
popped later and its own loop check refuses (`NotOwned`). When the
owner is not listed, nothing ever pops it, and the tear is carried.

A second, related read: when a hop lands on a face that is already
labelled (`component.contains_key(neighbor)`), the hop is dropped
rather than merging the two labels. On a valid body that cannot
happen across labels, since the walk from a seed reaches its whole
component before the next seed. On torn input it can, and the
partition then depends on the order of the shell's face list.

## Measured

Executed by the PR 3592 review at head `bfab1dc0f8`, in a scratch
probe that was not committed, on `fixtures::detached_digons(1)`, whose
shell lists four faces: the seed face, its partner, and the digon's
`d1` and `d2`.

- Untouched: `Ok`, 2 shells, face counts `[2, 2]`.
- **Two tears**: the shell stops listing `d2`, and `d2`'s outer loop
  (the loop of the mate of `d1`'s first half) has its `face` set to
  the seed face. `movefac` returns **`Ok`, 2 shells, face counts
  `[2, 1]`**: `d2` is carried unlisted, and the seed face's component
  is joined to the digon's through a loop the seed face does not list.
- One tear (only the loop's `face`, `d2` still listed): refuses
  `NotOwned { child: Loop(ld2), owner: Face(d2) }` when `d2` pops.

## Fix shape

At the hop, prove that the mate loop's `face` lists the loop (its
`outer` or one of its `rings`), refusing `NotOwned { child: Loop,
owner: Face }` as the downward check does. A hop into a labelled face
of another label is then a fault the labelling can refuse rather than
drop; whether it is reachable once the converse holds is the first
question to answer.

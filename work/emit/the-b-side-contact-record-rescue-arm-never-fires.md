---
id: the-b-side-contact-record-rescue-arm-never-fires
kind: issue
title: The seam-vertex pass's B-side contact-record rescue arm fires at no vertex in the tree
status: closed
opened: 2026-09-15
priority: P0
cost: D
branch: emit/b-side-rescue
closed: 2026-09-23
pr: 3103
---


## What

`crates/editor-core/src/names/emit_topo.rs`, the seam-vertex arm of
`name_boolean_edges`'s vertex pass, has two contact-record rescue arms:

- `([ae], [], _, Some(pb))` — an A-descended edge, the B parent supplied
  by a `reduction_contacts` vertex-vertex row;
- `([], [be], Some(pa), _)` — its mirror.

**The second never fires.** Censused 2026-09-15 over every seam vertex
the whole workspace test suite produces plus fifteen declared-union
fixture families swept over every member order — 17,381 vertices —
`partner_a` was `Some` at **zero** of them and `partner_b` at 1,441.

The cause is upstream of the arm, in how the pair is read:

```rust
let (va_key, vb_key) = match (naming.a_keys, naming.b_keys) {
    (Direct, Grafted) => match inv_vertices.get(&v) { Some(&vb) => (None, Some(vb)), None => (Some(v), None) },
    (Direct, Absent)  => (Some(v), None),
    (Absent, Direct)  => (None, Some(v)),
    _ => (None, None),
};
```

`partner_a` is reached only through `vb_key`, and `vb_key` is `Some`
only under `(Direct, Grafted)` with a graft entry or under
`(Absent, Direct)`. Whether those key regimes occur at all for a
seam-vertex pass is the question; the census says the resulting arm does
not.

## Why it matters

A dead arm is either a rule with no subject or a rule whose subject is
being routed elsewhere. Either way the B side of that pass has no
rescue, which is what makes the mirror of
`SeamVertexParentage`'s treated shape the more exposed of the two:
`a=0, b=1` is the COMMONEST seam-vertex shape in the tree (10,865 of
17,381, against 5,305 for `a=1, b=0`), so the population that would need
the rescue is the larger one.

Nothing currently reaches the catch-all — the residue arm fired zero
times in the same census — so this is not a live refusal. It is a rule
that cannot fire, sitting in front of a population that is bigger than
the one its twin serves.

## Found by

`two-emitter-refusals-a-legal-declared-union-reaches` (lane `wire-e2`),
review round 2, which used the census to replace a symmetry claim it
could not support. Out of that unit's fence: the unit re-classifies
refusals and does not change which arm of the pass fires.

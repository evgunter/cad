---
id: valid-ringed-torus-faces-build-but-cannot-be-meshed
kind: issue
title: A valid body whose torus wall holds a spiric ring builds and measures, but tessellation refuses it RingOnCurvedFace as a kernel bug
status: open
opened: 2026-10-21
priority: P2
cost: M
refs: [5567]
---


Found by PR 5567's review (FLUX unit E, `flux/torus-spiric-ring`, NOTE A).

Since unit E, a boolean of a plane-bounded body with a torus whose plane
section is an oval inside a torus wall builds a valid body: the wall
keeps the oval as a ring, the spiric lane measures it (every loop read,
each ring wound against the face), and tier 3 passes it. The review's
corpus (`review/5567-01Lm5UxC`, `crates/sweep/tests/review_5567_probes.rs`,
`review_5567_pads_orders_and_mesh`) has 42 such bodies across the donut
and the one-face-pair tori.

None of them tessellates. Both curved lanes refuse a ringed face before
they read it:

- `crates/mesh/src/curved.rs:183`: `if !face.rings.is_empty()` →
  `TessellateError::RingOnCurvedFace`;
- `crates/mesh/src/trimmed.rs:220`: the same.

The refusal's Display (`crates/mesh/src/types.rs`, `RingOnCurvedFace`)
still says "no construction produces one, so this is a kernel bug". That
was true before unit E and is now false: the body is valid and a public
boolean made it. So a valid body that builds and measures can be neither
meshed nor exported through a mesh, and the refusal blames the kernel.
PR 5567 corrected the variant's doc comment; the Display and its ending
are this row's, since reclassifying the refusal from defect to
capability limit moves its ending.

What to decide: mesh a ringed torus wall (the trimmed lane's
inside/outside classification over every loop), or, until then, refuse
it as a capability limit (`NOT_YET_ENDING`) and not as a kernel bug.

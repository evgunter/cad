---
id: pick-wide-candidate-needs-a-bound-over-mesh-coordinate-error
kind: issue
title: closing the ring's own wide candidate needs a bound over the mesh's coordinate error, which crossing's does not cover
status: open
opened: 2026-09-16
---


Filed by EDIT-PICK3's fix pass as the scheduled residue of
`pick-a-wide-but-informative-barycentric-wins-over-the-transversal-neighbour`,
which closes at EDIT-PICK3's merge with this much of it left.

## The finding

The `t` interval resolves the parked row's class wherever two crossings
are closer together than the near-coplanar triangle is LARGE. The
gallery ring's own ray is not that case, and the reason is the scope of
the bound, not a defect in the order.

On the ring after its bump, the `−y` ray through
`(0.24519632010080758, 0, 0.04877258050403218)`: the winner is a flat
face at `det = 1.66e-19`, conditioning `7.19e-16`,
`|det| / bound_det = 5.72`, answering `t = 1.4487652724897624`. Two
transversal neighbours answer the vertex `0.031` further on, exactly.
The winner's certified interval is `[1.43395, 1.46358]` — `0.030`
across, because the ring's triangles are `0.016` on a side — and it
lies WHOLLY BELOW the neighbours' intervals. It precedes them. The
tie-break never runs, and the door answers it for an arithmetic reason
rather than by noise.

Nothing in the derivation says that is wrong. `crossing` bounds the
ROUNDING of an evaluation over the corners it is given; it says so, at
"What the bounds bound". What it does not bound, and does not claim to,
is how far those corners are from the surface they approximate. The
ring's flat face is a tessellation of a curved one at the display
tolerance, so its corners carry an error of their own that is orders of
magnitude above `1.66e-19` — and a candidate whose interval is `0.030`
long is exactly the candidate for which that second error matters most.

## What would close it

A bound that composes the two: the arithmetic's rounding (which
`crossing` has) and the mesh's own deviation from the surface it
stands for (which nothing in `resolve/pick.rs` can see — it is a
property of the tessellation, `mesh::tessellate`'s chord tolerance and
the face's curvature). With it, the ring's winner's interval would
reach past the vertex, the two would be a certified tie, and the
tie-break would take the better-certified claim, which is what the
parked row asked for.

**It is not a factor.** Review lane pick2-r2's "doubled bound" shape
separates exactly these two candidates and is a chosen constant; the
ruling `what-t-the-pick-door-answers-and-with-what-width` names that
as the tuning the whole derivation exists to avoid, unless the doubling
is itself derived, "for instance, from the mesh's own coordinate error,
which the current bound explicitly does not cover". This row is that
sentence, given its own file.

## Where it touches

- `crates/editor-core/src/resolve/pick.rs` — `crossing`'s "What the
  bounds bound", `t_span`'s two terms.
- `crates/viewer/tests/index_memo.rs` —
  `a_wide_but_informative_candidate_answers_before_the_rings_aimed_vertex`
  and `RING_WIDE_CANDIDATE_T` record the answer this row would move.
- The pick door would need a tessellation-derived quantity it does not
  take today, which is the design question rather than the work.

---
id: pick-wide-candidate-needs-a-bound-over-mesh-coordinate-error
kind: issue
title: closing the ring's own wide candidate needs a bound over the mesh's coordinate error, which crossing's does not cover
status: closed
closed: 2026-09-20
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

## Put to Ev (2026-09-19, EDIT orchestrator) — the sixth `[ev]` PR

**The question.** The pick door's certified `t` interval bounds the
ARITHMETIC (`crossing`'s barycentric bounds and `t_span`'s projection
rounding) and nothing else; the mesh's own deviation from the surface
it stands for is outside it, and `crossing`'s doc says so ("a triangle
whose corners are themselves approximations is a question for whoever
tessellated it"). The pick door already receives that number:
`NodePick::build` takes the chordal tolerance `delta` it tessellates
at. Should the certified interval compose the tessellation's
deviation, and if so per what?

**Recommendation: yes, per candidate by its patch's surface kind.** A
patch tessellated from a planar face has corners ON the surface (the
chords are exact) and contributes nothing; a patch from a curved face
has corners within `delta` of it, so its interval widens by the
ray-direction projection of that deviation — a bound derived from
`delta` and the ray/triangle geometry, no chosen factor (the doubling
your PICK3 ruling refused stays refused). Consequence under your tie
ruling: a cursor within the tessellation's deviation of a shared edge
between two curved faces becomes a certified tie between faces and is
REFUSED, where today it answers whichever arithmetic happened to win;
on the gallery ring the wide flat candidate and the two transversal
neighbours become the tie the parked row asked for. Kernel unit (the
EDIT-PICK lineage: v6 dual, block EDIT-B2 slot 2) once ruled.

**Alternatives.** (a) One `delta`-wide term for every candidate,
planar or not — simpler, refuses more near-edge picks on flat models
than the geometry warrants. (b) Leave the interval arithmetic-only
and say so at `TSpan::width` — zero code, and a certified interval
that is not one where the model is curved. (c) Compose it but keep the
answer: treat a `delta`-only tie as ONE answer by the nearer `t` — a
second key, which your item-2 ruling on `[ev]` #2795 removed for the
reason it would return here.

## RULED (2026-09-20, Ev on `[ev]` #2889) — alternative (b): the certificate stops at the tessellation, by design

Ev: "referring to the tesselation is probably correct? it's what the
user can see." Read as (b) and confirmed on the PR: the pick is a
question about the picture the user sees, and the tessellation IS
what is picked — so an interval that certifies the arithmetic on the
triangle as given is the certificate the question needs, and the
mesh's deviation from the surface it stands for is not a pick error.
The recommendation (compose each candidate's deviation) is not built;
no kernel unit.

## Closed (2026-09-20, EDIT orchestrator) — by design, E-class

One sentence at `crossing`'s "What the bounds bound" paragraph
(`crates/editor-core/src/resolve/pick.rs`) names the reason, citing
this row and the ruling. `TSpan::width`'s "a measurement, never a
key" and the item-2 tie ruling are untouched. The ring's wide flat
candidate stays an answer, not a tie: its rounding is the only
uncertainty the door is asked about.


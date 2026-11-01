---
id: a-vertex-near-a-tilted-faces-carrier-but-w-from-its-face-refuses-vertex-on-face
kind: issue
title: A cavity corner 0.894w from a tilted fill face's carrier, but w from the face itself, refuses Coincidence(VertexOnFace) at 10.5ε
status: open
opened: 2026-10-25
priority: P2
cost: M
---


Found by the review of FLUSH's PR 5694 (its probe `needle_cavity_review` on
`flush/needle-cavity-section-loop`'s review branch `flush/needle-cavity-review`
@ `956611da`, finding N4). Re-measured on PR 5694's branch, where the same
refusal comes first, before the join reaches any section loop.

**Repro.** The needle channel of
`shell_role_reader::channel` (PR 5694): the brick `(−1,2)³`, the slab
`(0,1)×(0.5,0.8)×(0.5,0.5+w)`, and the fill flush with the slab on five
faces, whose face toward the channel leans. It runs from `y = 0.5 + w`
at the floor `z = 0.5` to `y = 0.5 + w(1 + s)` at the ceiling, with
`y0(z) = 0.5 + w + s(z − 0.5)`. With s = 0.5 and w = 10.5ε, at ε 1e-9
and 1e-6, axis-aligned and turned about `(1,2,3)` by 1.1 rad,
`(brick ∖ slab) ∪ fill` (both orders), `slab ∖ fill`, and
`slab ∩ (brick ∖ fill)` (both orders) all refuse `Escalated {
Coincidence(VertexOnFace, Moot) }`. The margin is ±9.391e-9 (axis-aligned,
ε 1e-9), ±9.67e-9 turned, and 9.39e-6 at ε 1e-6, all in band. At
10.5ε, s = 3 refuses the same way. At s = −0.5 and −0.9 the fill-first
ops (`fill ∪ hollow`, `(big ∖ fill) ∩ slab`) refuse instead on a
`Containment` loop-boundary margin of 5.25e-9 and 1.05e-9. Those are
real in-band ceiling gaps (0.5w and 0.1w), not this carrier read (PR
5694's fix-pass check). At 30ε only s = 0.5 builds (pinned:
`shell_role_reader::a_needle_channel_whose_fill_face_leans_reads_its_side_off_its_seam`;
turned at ε 1e-9 it refuses on the sector question); s = 3 still
refuses `VertexOnFace` there and s = −0.5 refuses `Sectors`.

**Measured reading.** `9.39e-9 = 0.894w = w/√(1 + s²)`. That is the
distance from the cavity's floor corner on the wall `y = 0.5` to the
leaning face's carrier plane. The plane's foot of that corner lies
outside the face, and the corner's distance to the face itself (its
floor edge) is `w`, which is definite. So the decision holds the
vertex against the face's carrier rather than the face: a pair that is
not in band refuses as one.

**Where.** Not traced. The refusal type is `reduce.rs`
`vertex_refusal` (`VertexRefusal::Undecided` →
`Coincide::VertexOnFace`); its plane read is `bool_vertex_face_side`
(`decide_vertex_on_plane`) there.

**What is owed.** Find the read, and have it place the vertex against
the face (its trim), or refuse only on a pair the band forbids. Then
pin the leaning channel at 10.5ε.

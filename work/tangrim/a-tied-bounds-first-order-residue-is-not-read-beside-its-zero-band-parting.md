---
id: a-tied-bounds-first-order-residue-is-not-read-beside-its-zero-band-parting
kind: issue
title: A tie whose parting reads within the zero band rides with its first-order residue unread beside it
status: open
opened: 2026-11-02
---


Found by `a-leaves-verdict-carries-no-witness-point`'s lane
(`tangrim/leaves-witness`, PR 6385), on ONEREAD's note in
`work/oneread/the-line-cone-reads-a-flanks-side-beside-the-one-bound-side-reading.md`
`## Decided` (item 2: one `link::metric::Reading` for both orders; the
first-order residue beside a tie is not subtracted from the parting).

`bound_side::Tie::descend` reads the parting `(κ_b·n̂ − II_F(d̂, d̂))·l²/2`
alone; the tie's first-order residue `d̂·n̂·l` (decided Zero, so at most
the zero band at the reach) is not in it. Since PR 6385 a parting off the
zero band stands on its witness (`Tie::witnessed`), the bound's own point
at the arm, whose offset holds every order, the residue included; so the
residue is read wherever the parting is. It is not read where the parting
falls within the zero band: `descend` returns `Descent::Rides` there
without a witness, and a residue up to `zero` beside a parting up to
`zero` lets a bound stand up to twice the zero band off the face at its
arm while it reads as riding it.

Owed: one rule for tie and residue at the zero-band rung, read on the
same `Reading` ONEREAD's lane 1 threads through `bound_side::read` and
`Tie::descend` (`the-cone-takes-a-bounds-side-from-the-one-bound-side-reading`):
either the witness is read at the `Rides` rung too (its zero band then
certifies the ride), or the residue is charged into the parting before
the ladder.

**Measured (PR 6385's per-tie log, default ε, every tie of `-p sweep -p
topo`)**: the zero-band rung is not only the residue's. In the suite's
bodies one test reads a `Rides` whose bound's own point at the arm stands
far off the face:
`a_pierce_held_in_a_curved_face::a_plane_face_holding_the_pierce_over_a_cylinder_builds_in_every_op_and_order`,
a bisector of a cylinder sector read against the plane face, its parting
±1e-19 (so `Rides`), its witness 3.33e-6 and 1.75e-5 off the plane
(8 reads each). The bisector runs obliquely across the cylinder, whose
normal section bends square to the plane's normal at the vertex, so both
leading terms vanish; the third-order term over the arm does not. The
row builds. Which side that bisector truly reads, and whether its `On`
moves the result, is unmeasured.

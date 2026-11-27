---
id: the-villarceau-identity-certificate-becomes-the-poses-conic-torus-lies-rung
kind: issue
title: The Villarceau identity certificate becomes the pose's conic × torus Lies rung
status: open
opened: 2026-11-26
priority: P2
refs: [a-villarceau-circle-lying-on-a-torus-is-unsettled]
---


Filed by GERM U4c (PR 7818), the one D3 residue it leaves in `slit`.

## What

`slit::conic_or_line_on` reads whether an edge's carrier lies on a face's
surface by the curve × face pose (`geom_brep::curve_face::pose`) for every
built analytic pair, except one arm: a conic against a torus that the pose
refuses `NotLies` in band (`curve_face_pose_not_lies`). There `slit` keeps
the sampled identity certificate it read every conic × curved pair with
before U4c: nine points over the conic's turn through
`geom_brep::variety`, then the Lebesgue bound between them
(`slit::villarceau_certificate`, `crates/topo/src/boolean/slit.rs`). It is
a second reading of `Lies` (D3, "one reading per question"), and it
stands because the pose has no rung that reads a Villarceau circle, or any
conic of a torus's own but a parallel or a meridian, lying on the torus.

U4c's rows show the residue carries real answers. Under the pose alone:

- the Villarceau circle of the ring torus `(3, 1)` and of a fat torus read
  **on** today and would escalate
  (`torus_rows::a_torus_conic_reads_on_it_and_a_near_miss_does_not`,
  `a_villarceau_circle_on_a_fat_torus_reads_on_it`);
- a near miss on a fat torus reads **unread** today and would escalate
  (`a_near_miss_on_a_fat_torus_does_not_read_on_it`).

The mutant that drops the certificate arm reds all three.

## Owed

The certificate moves into the pose as the conic × torus arm's `Lies`
rung: the nine samples and the bound decided in the pose, with a bound
beyond the band answered as a `Meter` refusal, so the near miss stays
unread. `slit`'s certificate arm then goes, and `slit` reads the pose
alone. This changes what `cross` answers for a conic lying on a torus
for every reader (the curved lane reads a Villarceau edge `LiesOn` where
it now refuses), so the curved lane's rows for a conic against a torus
are re-read with it. It closes or subsumes
`a-villarceau-circle-lying-on-a-torus-is-unsettled`.

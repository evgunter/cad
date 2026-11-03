---
id: sect
kind: program
title: SECT — the section-first join: the torus pairs it is built for, the one-site and on-carrier sections on curved faces, its P2/P3 remainder, and the C5 dispatch and rung-3 residue behind the section object
status: ready
opened: 2026-10-10
group: kernel/boolean/join
prefix: sect/
tag: (SECT orchestrator)
paths: [crates/topo/src/boolean/join.rs, crates/topo/src/splitting/join.rs, crates/geom-brep/src/intersect.rs, crates/geom-brep/src/pair_section.rs, crates/topo/src/boolean/vtxfac.rs]
keep_out: [opened 2026-10-10 by JOIN's close (Ev approved the plan in chat 2026-10-10) - the rows moved by git mv with ids and bodies unchanged and JOIN's directory left the tracker (docs/doc-ledger/join-leaves-the-tracker.md), FLUSH holds JOIN's planar and declared-flush residue on the same join.rs - SECT owns the curved section-frame and lane rows outright, so a per-pair section frame or join lane is built here as an instance of the section-first join and nowhere else as a stopgap, coincident and tangent sections are decided at the section door and wait on INTENT stage 4 PR E (booleans-glue-on-zero), SECTTAIL (cut 2026-10-25) and SECTRUNG (cut 2026-10-28) on its priority seam, and SECTSITE (cut 2026-10-28 through SECTARMS), merged back 2026-11-02 when the budget came to count only P0 P1 and unbanded rows (docs/doc-ledger/secttail-sectrung-and-sectsite-merge-back.md) - several of their rows may close by retirement when a pair moves onto the section value so a row is read against SECT's and SECTARMS' progress before it is built, SECTARMS builds the section object's remaining transverse arms, a one-site or on-carrier reading here is a reading of geom_brep::pair_section's components and never a per-pair frame, the torus on-carrier sector (U-H1) was held by D10 until INTENT closed on 2026-10-30 (#6103) and is dispatchable, crates/geom-brep/src/ssi* is SSIARITH SSIEDGE SSIMARCH ground, crates/topo/src/boolean/* is shared ground with FLUSH ZIP FUSE GERM CONTACTHOLD TANG CLEAVE BOXES PIN by design and RESTFRONT owns validate.rs, so run scripts/work.py territory on your branch and announce the seam in the PR where the changes are coupled]
---
**The section-first join.** Ev ratified it in `[ev]` PR 4313
(`docs/DESIGN-FORK-LOG.md` row 100; "The shape to give" in
`work/sect/cylinder-sphere-germ-pair-has-no-join-lane.md`): the join
builds on one certified section per germ face pair, at the rung C5
gives that kind pair, computed once and read by both operands and the
matcher. Per-pair section frames and per-kind-pair lanes retire into it
rather than growing.

SECT owns the curved section-frame and lane rows outright: the cylinder
× sphere lane, the skew-cylinder and torus frames, the one-site and
along-edge rings, and the spellings of order and tangency the section
object replaces. No per-pair stopgap lane is built elsewhere (Ev,
2026-10-10, left the choice open; JOIN's orchestrator chose ownership,
because a stopgap rebuilds what the design retires). The build was
gated on INTENT stage 4's PR A (PR 4364), which merged
2026-10-09T21:06Z, so it is startable.

**The torus pairs are here, and they are P0** (refold approved in
chat, 2026-10-25). Teapot wall 2 (sphere × torus) refuses at the
torus germ frame, and lily wall 1 (torus × torus) at the section
certificate's interior-loop guard. Both are built on the section
object, and the torus rows refuse at C5 until C5 has their arms, so
the C5 arms (`c5-plane-torus-cone-cylinder-arms`, from GERM) and the
torus × torus certificate arm (from ORBIT) moved here beside the
torus frame. Step 1, the cylinder × sphere lane, is P0 because every
P0 here is built on the object it introduces. SPLICE's demo joins
that pierce a curved wall (klein wall 3, teapot wall 3) also need
the section object's rung-3 arm for a NURBS face against a curved
one.

**Split 2026-10-28** (Ev, in chat): the section object's other arms
went to SECTARMS, and its one-site and on-carrier rows to SECTSITE
(merged back 2026-11-02).

**One-site and on-carrier sections on curved faces** (the two P1 rows
SECTSITE held): a closed edge lying in a cone, cylinder or torus face,
where the one-site join has no arm to side its conic; and the C5
table's plane × torus and cone × cylinder residue, its U-H1 touching
poses (a torus sector on the plane, the tangent circle, the Villarceau
pair), whose D10 hold lifted on 2026-10-30. Both are read on the section
object. Lily wall 8 stops at the second's general plane × torus rung,
P0 since 2026-11-03 (`work/splice/curved-demo-joins-build.md`).

**Beside the torus thread, the remainder** (merged back 2026-11-02 from
SECTTAIL and SECTRUNG, the P2/P3 rows cut out on the priority seam):
- the section-first join's own P2/P3 rows: the bar through a ball, the
  plane tangent at a wrap edge's vertex, the along-edge ring, the
  closed edge's germ on a planar face, the cone against a parallel-axis
  torus, the travel slack, tilted tori's ball separation;
- the C5 dispatch and rung-3 residue SECT's units filed: the double
  spelling of the C5 kind dispatch and its unguarded radii, the cone ×
  torus and tilted plane × torus rung-3 arms, R-tan's wording for an
  arm's proof gap, the march's cost near the cylinder × sphere node and
  on torus germ rows (and the ball × cylinder P1 cost row), the torus
  doors' tube and gap charges, the operand order a section is carried
  in, and the recording scalar's missing SSI lane.

Several may close by retirement when SECT or SECTARMS moves a pair onto
the section value; read each against `work/sect/log.md` before building
it.

Charter and order: `work/sect/plan.md`; narrative in `work/sect/log.md`.

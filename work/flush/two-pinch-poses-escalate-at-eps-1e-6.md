---
id: two-pinch-poses-escalate-at-eps-1e-6
kind: issue
title: Two of pinch_faces_tessellate's poses escalate at eps 1e-6 (the join's partner order, a pierce sector's curvature side)
status: open
opened: 2026-10-06
priority: P3
cost: E
---

Found by CLEAVE's ray-walk PR 4083. Its diff seeds `topo`, so it ran
`sweep`'s rows at ε 1e-6 and 1e-12 too. Two poses of
`crates/sweep/tests/pinch_faces_tessellate.rs`
(`a_face_through_two_vertices_on_one_point_tessellates`) refuse at
ε 1e-6 and build at 1e-9 and 1e-12.

1. `notch307 fib117 edge psi=1.9 cp S`:

   ```
   Err(Escalated { decision: Coincidence(Join, Moot), diag: Indeterminate {
     margin: Value(-5.196042020649827e-6), band: (1e-6, 1e-5),
     predicate: Some("bool_join_nearest") } })
   ```

   Measured on `main` at `cadf2ed188` with nothing of PR 4083 applied: the
   same refusal. Two chords from one germ to two candidate partners
   (`join::nearer`) differ by 5.2 µm, inside the 1e-6 band.
2. `Lbot cyl fib4 psi=0.9 seam cp S`:

   ```
   Err(Escalated { decision: PierceCurvature, diag: Indeterminate {
     margin: Value(4.202625733034012e-6), band: (1e-6, 1e-5),
     predicate: Some("bool_pierce_sector_side_curved") } })
   ```

   Not measured on `main`. The row never reached this pose there: pose 1
   panics first. The predicate is `boolean::sectors`' sagitta charge,
   which no code PR 4083 changed reaches. With PR 4083's one change that
   reaches a boolean's boundary reading reverted (`ConicArc::hit`
   reading the distance to the arc), it refuses the same way.

Both are margins in band at that ε, so each refusal is an honest
escalation unless the decision can be read another way on these poses.
PR 4083 pins both in the row (`REFUSES_AT_1E6`, each with its predicate)
so the row is not red on the extra ε rows. To be decided, per pose:
whether it should build at 1e-6, and if so, remove its pin.

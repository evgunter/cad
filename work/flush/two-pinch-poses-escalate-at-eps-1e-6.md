---
id: two-pinch-poses-escalate-at-eps-1e-6
kind: issue
title: Two of pinch_faces_tessellate's poses escalate at eps 1e-6 (the join's partner order, a pierce sector's curvature side)
status: open
opened: 2026-10-06
priority: P3
cost: M
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

## Measured (branch `flush/e-tail`, 2026-10-11)

Both poses still escalate at ε 1e-6 on `main` at `121741a2b`, through
the plain booleans, on the predicates and margins above.

**Pose 2 (`Lbot cyl fib4 psi=0.9 seam cp S`) is honest, and settled.**
`bool_pierce_sector_side_curved` (`boolean/sectors.rs`, `read_side`)
reads `slope·l − l²/R` at `l = slope·R/2`: a lower bound on how far the
sector's bound clears the curved wall within its reach. It reads
4.2e-6; the exact circle's departure (`l²/(R + √(R² − l²))`, about
`l²/2R`) puts the clearance's peak at about `slope²R/2`, 8.4e-6. Both
lie inside the band's escalation threshold of 1e-5 at ε 1e-6, so which
side the bound lies on is not certain at that ε: the geometry's sliver,
not the kernel's. Its pin in `pinch_faces_tessellate.rs`
(`REFUSES_AT_1E6`) stays, and its comment now says why.

**Pose 1 (`notch307 fib117 edge psi=1.9 cp S`) waits on a proof.**
A probe in `find_match` placed the escalation in its second stage, not
in one germ's partner choice (`nearer_along`): `keep_nearer` over the
germs' nearest pairs, between the pairs `((6,0),(1,0))` and
`((3,0),(0,1))`, chords 1.000045882 and 1.000051078. The two pairs
share no germ. That order picks which pair the round commits first,
and it can move the matching only through what the next round re-ranks
once the first pair's germs are consumed (a germ whose nearest partner
was consumed takes its next one). Whether an in-band tie between two
disjoint pairs can change the matching, and so whether it may take
scan order (D9) instead of escalating, is a property of the greedy
matcher to prove or refute, not a margin to re-read.

## Re-priced

`cost: M`, a proof task (the orchestrator, 2026-10-11): pose 1 waits
on that proof; pose 2 is done, its pin's comment written on
`flush/e-tail` (PR 4714).

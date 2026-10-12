---
id: two-pinch-poses-escalate-at-eps-1e-6
kind: issue
title: Two of pinch_faces_tessellate's poses escalate at eps 1e-6 (the join's partner order, a pierce sector's curvature side)
status: review
opened: 2026-10-06
priority: P3
branch: flush/two-pinch-pose-proof
pr: 4769
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

## Measured (branch `flush/two-pinch-pose-proof`, 2026-10-12)

On `main` at `300f7cefe` pose 1 still escalates on `bool_join_nearest`.
A probe in `join::find_match` dumped its first round: 14 germs, all
on straight germ lines, and every germ's nearest partner has it as
its own nearest. That is seven disjoint mutual pairs, and no germ's
nearest lies in a pair it does not belong to. The tie is between two
of those pairs (chords 1.000045882 and 1.000051078).

## Fix: the proof

**Where every germ's nearest partner keeps it back, the order among
pairs cannot change the matching.** Commit a pair `(g, h)`. No other
germ `q` has `g` or `h` as its nearest: if `q`'s nearest were `g`,
then `g`'s nearest would be `q`, and it is `h`. `nearer_along` compares
two of one germ's candidates and reads nothing else, so taking away
candidates other than a germ's nearest leaves that nearest unchanged.
The next round therefore has the same pairs, minus the one just
committed, and they are still mutual. By induction the rounds commit
exactly the first round's pairs, in whatever order. A germ with no
partner gains none, because candidates are only ever consumed.

Where some germ's nearest is not mutual, committing a pair can turn
that germ to its next partner. The round never ranked that pair, so
the order can matter. There the order is still read, and an in-band
reading still escalates. One mechanism makes the order matter on a
conic: a germ chooses along its line by travel, the round ranks by
chord, and on an ellipse of aspect √2 or more the chord to a farther
partner can be the shorter one. No pose was built to show it. On
straight lines an argument from the edge-ordered greedy matching
(an edge is matched iff no earlier adjacent edge is) suggests that
disjoint ties never matter there. The fix does not rely on that.

`boolean/join.rs`: `find_match`'s second stage is now `first_pair`,
whose doc comment carries the argument. When every germ's nearest is
mutual it reads the order through `nearer_or_scan`, where an in-band
chord difference keeps the incumbent (scan order, D9) as an exact tie
does. Otherwise it uses `nearer`, as before. Every definite reading is
the same as before, so a pose that built before builds the same way.
Only a pose that escalated here changes.

Pins:
- `first_pair_rows::an_in_band_order_is_free_only_among_mutual_nearests`
  (`join.rs`). The same in-band tie between two disjoint pairs keeps
  the round when all germs are mutual. It escalates on
  `bool_join_nearest` once a fifth germ's nearest lands in one of the
  pairs.
- Pose 1 is out of `REFUSES_AT_1E6` (`crates/sweep/tests/pinch_faces_tessellate.rs`).
  It builds and tessellates at ε 1e-6, 1e-9 and 1e-12, with the mesh's
  volume exact. If the premise stops holding on that pose, the tie
  escalates again and the row goes red.

Pose 2 is unchanged (above).

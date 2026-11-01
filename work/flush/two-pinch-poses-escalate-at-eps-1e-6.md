---
id: two-pinch-poses-escalate-at-eps-1e-6
kind: issue
title: Two of pinch_faces_tessellate's poses escalate at eps 1e-6 (the join's partner order, a pierce sector's curvature side)
status: closed
opened: 2026-10-06
priority: P3
cost: M
branch: flush/two-pinch-pose-proof
pr: 4769
closed: 2026-10-12
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

**The matching is free of the pair order where three premises hold.**
The premises are:

- every germ's nearest partner has it as its own nearest;
- every germ is straight;
- the band's escalation threshold exceeds twice its zero (`K > 2`).

The run then commits exactly the first round's pairs, or escalates.

- **No other germ points into a committed pair.** Commit `(g, h)`. If
  another germ's nearest were `g`, then `g`'s nearest would be that
  germ, and it is `h`.
- **A straight germ ranks by one number.** It ranks its candidates by
  chord, and each reading is the difference of two chords.
- **A shortened scan keeps its winner `w` (`K > 2`).** The scan keeps
  its incumbent unless a candidate reads at least `K·zero` nearer, and
  a tie is within `zero`.
  - Every candidate met before `w` lost to an incumbent at least
    `K·zero` farther than `w`, so it lies at least `(K − 1)·zero`
    beyond `w`.
  - Every candidate met after `w` lies no nearer than `w − zero`.
  - A scan that has lost candidates other than `w` therefore meets `w`
    against an incumbent at least `(K − 1)·zero` farther, which `K > 2`
    puts past a tie. Either `w` takes the incumbency or the reading
    escalates, and nothing after it reads `K·zero` nearer.
- **Each later round** holds the same pairs less the committed ones,
  still mutual. Its scans may escalate under one order and not under
  another, so the run commits these pairs or escalates.
- **The order still reaches the surgery.** It picks which pair is
  joined first, which scan order already decided for exact ties.

**Why each premise is needed.**

- **`K > 2`.** Two chained ties span `2·zero`, which `K ≤ 2` can read
  as definite. The review's witness: at `K = 2`, the scan `[3, 2, 1]`
  picks 1, and with 3 removed it picks 2. `Tolerance` admits any
  `K > 1` (`CAD_AMBIGUITY_K`).
- **Straight germs.** Along a conic, `nearer_along` reads travel from
  the incumbent's site, not one number per candidate, so the argument
  does not carry over as written. Filed as
  `work/flush/conic-germs-pair-order-is-unargued.md`.
- **Mutual nearests.** Without it, committing a pair can turn a germ to
  its next partner, a pair the round never ranked.

`boolean/join.rs`:

- `find_match`'s second stage is now `first_pair`, whose doc comment
  carries the argument.
- `first_pair` chooses its comparator once. Where the premises hold it
  takes `nearer_or_scan`: an in-band reading keeps the incumbent (scan
  order, D9), as an exact tie does. A poisoned margin or a straddling
  enclosure still escalates. Elsewhere it takes `nearer`, unchanged.
- The per-germ scan is `keep_own`, which `find_match` and the tests
  share.

Every definite reading is the same as before.

Pins (`first_pair_rows` in `join.rs`):

- `an_in_band_order_is_free_only_where_the_matching_cannot_move`. A
  tie of five zeros between two mutual straight pairs keeps the round
  at `K = 10`. It escalates on `bool_join_nearest` in each of these
  cases:
  - a fifth germ's nearest lands in a pair;
  - `K = 2`;
  - the germs are a circle's.

  A poisoned chord escalates even where the order is free.
- `at_k_at_most_2_first_pair_reads_the_order_among_mutual_pairs`, the
  `K` gate's guard. On the review's eight germs at `K = 1.1`, the
  rounds reach one that is all mutual and straight, with pairs in
  band. There `first_pair` escalates on `bool_join_nearest`; without
  the gate it would take scan order.
- Pose 1 is out of `REFUSES_AT_1E6`
  (`crates/sweep/tests/pinch_faces_tessellate.rs`). It builds and
  tessellates at ε 1e-6, 1e-9 and 1e-12, and the mesh volume is exact.

Two conic-germ rows still escalate, as before, and are unchanged:

- `band_apart_partners_on_a_steep_ellipse.rs` at `k = 10`;
- `germ_plane_torus_frame.rs`, the bar across the hole, at 1e-6.

Both are in the conic row.

## Batteries moved

Every `#[ignore]`d test of `sweep`'s `all` (77) ran at ε 1e-9 and 1e-6,
on `main` at `f6d326490` with this branch's `join.rs` and with main's.
Outcomes were diffed by pose. Every move is an escalation that now
builds. Each pose moved at 1e-6 reads, verbatim, as it read at 1e-9
with main's `join.rs`. The four `RCW` lines are the exception: they
escalated at 1e-9 too, and now build alike at both ε.

At 1e-9:

- `near_tangent_battery`: 126 → `OK SOUND` (`Ltop` 108, `w60` 12,
  `asym` 6).
- `join_rc_probes` `rc_wide_battery`: 180 → `OK SOUND`.

At 1e-6:

- `corner_pairs_battery`: 252 → `OK SOUND`.
- `pinch_runs_battery`: 18 → `OK SOUND`.
- `dbl_battery`: 12 → `OK SOUND`.
- `rc_wide_battery`: 4 → `OK SOUND`.
- `review_sixx_r2_probes` (`notch343 edge … m=2.0e-2`): 30 →
  `OK SOUND`.
- `four_pairs_battery`: 54 moved.
  - 50 → `OK SOUND`.
  - 4 → `OK BAD`: `four i=4 j=2 t=2 k=0` and `i=7 j=6 t=0 k=3`, U,
    both orders. They are `OK BAD` at 1e-9 on main too. That is the
    seams-row class
    `work/flush/a-pinch-the-seams-do-not-link-keeps-its-cones-on-separate-keys.md`
    (`flush/touch-key`).
- `curved_notch_pinch`: 6 (`curved i=11 j=6 k=0`, every op and order)
  → bodies. Two read `t3p=ok`, four `CensusUndecidable`, four mesh
  `Triangulation(Plane rings=0)`, exactly as at 1e-9.

Nothing else moved. The set of failing ignored probes is the same in
both runs; they are measurement probes that need env input or are
known-red.

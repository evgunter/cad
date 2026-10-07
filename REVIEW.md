# Review (second, full) — PR 4249 at frozen head `6f0f01e2` (main `875e049a`)

**Verdict: APPROVE-WITH-FIXES.** MAJOR 0 · MINOR 2 · NOTE 4. Every adjudicated finding is fixed; the guard catches every
3- and 4-pair BAD I built and refuses only two-key bodies; its class-extension step is dead and unpinned (MINOR-1).

**Method.** Release builds of `sweep`'s `all` binary in separate target dirs for main and head. The head binary
carries env-switched mutants (`REV_MUT`, default off = the head's code; patch `review-fix-mutants.patch`, reverted
from the tree). The topo unit tests ran in a debug build with the same patch. Probes ported unchanged: r1's
`review_r1_4249.rs` (plus a `t3err=` tier-3′ suffix on head only, stripped before diffing) and r2's block. New
probe: `join_pierce_runs_sweep/review_fix_4249.rs`, `rf_four_pairs_at_one_vertex`. It sets the notch's corner
against four cube corners touching only at `v`; 916 poses pass r2's overlap filter. Every line is read by
`outcome`, and every built body also by `cones_at` against `vertices_at(v)`, `faces_through_two_vertices_at`,
`pierce_point_finding`/`shared_point_finding` and `check_mesh` at δ 0.05.

## Claims
**A. The guard is right — holds** (executed and inspected), with one dead step (MINOR-1).
- *Legal body refused?* No. Under `REV_MUT=noguard`, every line the guard refuses becomes `BAD` or a `SOUND` body
  whose cones sit on two keys. `three`: 22 BAD, 11 two-key. `tripod`: 18 BAD, 37 two-key ("do not share one
  point"). `four`: 112 BAD, 55 two-key. Not one is a one-key body.
- *By inspection:* a point key is one position, and `vv` contacts and seam classes tie only coincident vertices; only
  a band-chain of contacts could reach a distinct point (not seen).
- *Missed hang?* None found. Every moved `SOUND` body is one-key and finding-free (below).
- *By inspection:* every vertex at the hung point is in the `vv` component. Mint copies keep their point (`mev_null` mints "no point", `null.rs:242`), and so do the cone
  splits (`zip.rs:36`), `carve` ("keys preserved", `finish.rs:286`) and `revert` (key-for-key, `revert.rs:145`).
- *`GraftMap::points`:* total over the source's points (`combine.rs:451-455`), chained in `then` (`:366`), built on
  both graft paths (`:298-327`, `:343`): a faithful twin of `vertices`, and wider.

**B. No wrong body ships — holds** (executed; main vs head, lines moved):

| probe | lines | → SOUND | → `SharedVertexCrossings` | → BAD |
|---|---|---|---|---|
| `pinch_runs_battery` | 3 023 | 217 | 0 | 0 |
| r1 `three` | 1 007 | 101 | 33 | 0 |
| r2 `tripod` | 3 985 | 321 | 55 (3 from `Euler`) | 0 |
| **`four`** (new) | 5 333 | 353 | 167 | 0 |
| r1 curved notch | 1 512 | 4 | 0 (4 → `JoinDesync`) | 98 (item 3) |
| r2 `both` (both operands pinched) | 2 015 | byte-identical | | |

- Every moved `SOUND` body is clean (`vertices_at(v)` = `cones_at`, one key, `f2=0`, meshes, finding `None`): pinch
  217/217 (r1's detail probe), three 101/101, tripod 321/321, four 353/353. Unmoved pre-existing BAD: three 8,
  tripod 10, **four 24**. The pinch battery's 468 two-key `SOUND` bodies are unmoved, none among the 217.

**C. Nothing else moved — holds** (executed, byte-identical main vs head):
`pierce_runs_battery` 4 535 lines, `corner_pairs_battery` 16 380, `join1_r1_reflex_battery` 1 152, `rc_wide_battery`
shards 5/19/48/77 of 84 (480 each, away from the earlier reviews' shards), r2 `both` 2 015.

**D. Plumbing — holds** (inspection).
- Built in slot-then-plan order, BFS'd over `vv` in order, collected into a `BTreeSet`, first refusal wins: D9
  holds. With no hang, `hung` is empty and both loops are no-ops (the byte-identical batteries agree).
- The only reader is `ops.rs:668-687` → `zip::refuse_split_hung_points`. `HungPoint` is `pub(crate)`, and
  `BooleanReduction` already had `pub(crate)` fields.
- `through_the_join`'s other callers (`mod.rs:4192`, `ops.rs:6305`) are test-support only. The recut re-entry
  (`ops.rs:880`) passes through the guard again.

**E. Mutants — partly falsified** (executed). The rows are `join_pierce_runs_sweep::` (24 non-ignored).
- Red: `noguard` (row `three_pairs_…`; probes 22/18/112 → BAD); `oneop` (only the hung vertex's operand's keys;
  same as `noguard`); `nohang` (4 rows); `cut1`/`cut2` (r1 M2/M2b, r2 `onecut`: `a_sibling_is_held_by_both_its_cuts`);
  `noheldheld` (r1 M4: `sibling_holders_place_…`); `isup` (`a_vertex_in_both_side_sets_names_no_sense`).
- **`noclass` (skip the class extension): survives.** The rows pass, and pinch, three, tripod and four are each
  byte-identical to the head (MINOR-1).

## Adjudicated findings
1. MAJOR (three pairs → BAD): **confirmed fixed.** `three` and `tripod` give 0 refusal → BAD, and so does `four`.
   The guard reads keys and classes only (`zip.rs:372-399`); no position is read.
2. MINOR (unpinned hold test and arms): **confirmed fixed.** r2's `onecut`/`isup` and r1's M2/M2b/M4 all go red,
   on unit tests. No battery line reaches the arms yet: disclosed.
3. MINOR (curved, 98 lines): **confirmed non-definite.**
   All 98 pass `t2` and `cert` with one key at `v`; the 45 `t3p=false` are all `CensusUndecidable` "a curved face
   of one is within reach" (`t3err`, every line); 96 have `operand=false`, 2 fail the mesh. No definite BAD (NOTE-2).
4. Docs and style: **confirmed fixed**, with one gap (MINOR-2): `Held` (`insert.rs:142`), the header (`:9-12`, `:53-65`), "Before hang_in_turned" (`:1171`), `by_strut`
     (`:1417`), the row split (`join_pierce_runs_sweep.rs:1431/1453`), `OtherCut::owner` (both `run_cuts` callers
     pass a plan) and `shared_at` (`:1113`).

## Findings
**MINOR-1 — the guard's class extension decides nothing on any line, and nothing pins it** (executed).
- `zip.rs:376-384` unions the hung keys with every seam class they meet. `REV_MUT=noclass` skips it, and every
  row, the pinch battery, `three`, `tripod` and `four` stay byte-identical.
- By inspection, `point_classes` ties only A and B vertex keys at a seam point, which the `vv` component already
  holds. So the step is likely dead by construction.
- The PR body and the doc (`zip.rs:358-366`) present it as part of the guard's logic. Either a row that needs it,
  or its removal.

**MINOR-2 — `SharedVertexCrossings`' doc names neither new emitter** (inspection).
- `mod.rs:2297-2319` lists two causes, the reconcile's unplaceable run and B's nested plan. The unit now also
  refuses it from `sibling_holders` (`insert.rs:1007`) and from the zip guard (`zip.rs:391`).
- The zip guard's cause is a different class: an operand pinch whose keys no seam links, after the zips. Its
  `vertex` and `partners` are reduction-clone keys reported from the result stage.

**NOTE-1** — `three_pairs_…` (`join_pierce_runs_sweep.rs:1689`) asserts `ab` only; the guarded `ba U`/`ba S` there
are unasserted (the guard is order-blind, so `noguard` still goes red).
**NOTE-2** — The PR body's curved counts are slightly off.
- It gives 44 census lines; I count 45 `t3p=false`.
- "Exact volume" fails on 4 lines (`curved i=0 j=4 k=4 ab U/S, ba U/S`), which are 1.98e-7 off the oracle. That
  is an oracle artifact: main's `ab I` at that pose is off by the same amount, and U + I = vA + vB holds to 9
  decimals.
**NOTE-3** — Four pairs reach the hang (353 → `SOUND`) and the guard (167); base's 24 unmoved `BAD` lines there are
the parked separate-keys class's 4-pair witness, absent from that row's `## Measured`.
**NOTE-4** — The guard refuses 103 bodies on my probes that pass every check but hold their cones on two keys
(claim A): per the ruling, and each refused on main, but it is the parked row's cost.

## Style
Exercised: Q1–Q7. Q8 was partial: I read `zip.rs` 1-60 and 230-400, `insert.rs`'s header, and `:960-1200`, not
the whole of `insert.rs` (3 100 lines).
- **Q1 (likely).** `hung_points` (`mod.rs:4577-4601`) hand-rolls a fixed-point closure over `vv`, while
  `zip::Roots` (`zip.rs:238`) is the union-find for the same "keys at one point" question. With `point_classes`
  and `share_points`, that is a third reading of "the keys of a pinch".
- **Q1 (unsure).** The guard's `live` set (`zip.rs:385-390`) re-derives `share_points`' `on_key` index over every
  vertex, per hang, three lines after `share_points` built it.
- **Q3 (sure).** MINOR-1: a mutant-proof step.
- **Q4 (sure).** MINOR-2: the variant doc was not updated with its two new emitters.
- **Q5 (likely).** `zip.rs`'s module header (`:33-41`) describes `split_cones` and `share_points`, but not the new
  refusal the module now owns.
- **Q7 (unsure).** One feature spans four homes: `insert::Hang`, `mod::HungPoint`/`hung_points`, `ops.rs`'s inline
  graft translation (which silently `filter_map`s unmapped B keys) and `zip::refuse_split_hung_points`.
- **Q7 (unsure).** Reusing `SharedVertexCrossings` for "a pinch left on two keys" means a caller cannot tell an
  insertion refusal from the parked separate-keys class.
- **Q6 (likely).** "No battery line reaches" `sibling_holders`' refusing arms (`insert.rs:1059-1069`) is
  re-measured by nothing: the unit tests pin the function, not reachability.

REVIEW COMPLETE

# Review — PR #4317 "insert: one position order round a vertex orbit, and one "holds whole" reading"

Frozen head 1deb0892, base main 047d10d5. Sequential arm, single review. Release builds in separate target dirs per side. Probes are in `review-probes/` on this branch: three model scripts and four mutant patches.

**Verdict: APPROVE-WITH-FIXES.** MAJOR 0 · MINOR 1 · NOTE 4. Nothing raised here needs a second review.

## Claims

1. **The case table: holds.** *Executed (model).* I exhaustively compared `held_cut`'s old reading (rotated to `lo.0`) with the new `on_arc` over a fixed origin, mapped to the five match arms. That covers every `(lo, hi, x)` over 1–5 entries with 3 within-entry ranks, including twins, ties at either end, and arcs wrapped past entry 0 (`review-probes/held_cut_case_table.py`).
   - Where `lo.0 ≠ hi.0`, no arm differs.
   - Arms differ in exactly one case: `lo.0 == hi.0` with `hi` before `lo`, an arc that wraps inside one entry. Old reads it as "not held"; new reads it as the near-whole orbit.
   - That case cannot be reached. `is_strut` is defined as `run_fan` being empty, and `run_fan` is empty for `from == to` (`insert.rs:2203`). So every run whose ends share an entry is a strut, and `segment` puts a strut's ends in walk order (`leads`, `insert.rs:1284`). "A fan's ends sit in different entries" therefore holds by definition for every producer: the plans, `reversed()` in `reconcile_pass`, and `b_runs`. It is a property of `is_strut`, not of whoever makes the fan.
   - The PR's table lists only the rows whose readings differ. The rows that agree are omitted rather than missed.
2. **The precondition: holds. The new refusal cannot be reached.** *Inspection.*
   - `build_sectors` pushes corner 0 first (`sectors.rs:213-247`). Unbisected, its `end_reach` is the corner's own chord. Bisected, the end-sharing half goes first and keeps `reach_end`. `orbit_corners` only ever makes `Chord` or `Extent` reaches (`sectors.rs:~300-324`, the curved edges included), so `end_edge()` holds for entry 0.
   - Only two places build the arrays that reach `walks_before`: `mod.rs:4473-4474` (through `plan_null_pairs`, `reconcile_shared` and `mint_plans`) and `vtxfac.rs:433` (through `strut_faces_first`, `vtxfac.rs:962`).
   - `reverse_when_asked` reverses the list of orbits, not their entries (`insert.rs:239`). `recl_edges` takes the arrays as `&[BoolSector]` (`recl.rs:516`). `orbit_of` hands out whole slices.
   - Nothing permutes or filters an array. Main builds no pose that this refusal would now reject, so it is not a regression.
3. **`arc_holds`' extra comparisons are implied: holds.** *Executed (model).*
   - `holds_whole`: old and new agree on all 4 290 ordered strut pairs inside one physical sector, with ties allowed at any end other than both (`holds_whole_old_vs_new.py`).
   - `arc_holders`: old and new agree on all 4 368 ordered arc pairs over every matching of 2, 4 and 6 positions (`arc_holders_old_vs_new.py`).
   - A tie between strut germs in one physical sector cannot break the agreement under an exact order. The new pairs `(ilo, hi)` and `(lo, ihi)` lie inside the outer arc, which `leads` already decided. No battery shows a new refusal.
4. **`walk_faces_first` behaves as before: holds.** *Inspection plus batteries.*
   - Across entries, old `precedes` read from the physical sector's first entry. Because entry 0 opens a physical sector (claim 2), that is a rotation that keeps the order inside every physical sector.
   - Within one entry it still reads `strut_order`.
   - Its callers are `insert.rs:1605`, `insert.rs:1946` and `vtxfac.rs:962`, all on `build_sectors` arrays.
5. **Nothing moved: holds.** *Executed.* Base against head, `--ignored --exact`, release, outputs diffed with the "finished in" lines dropped. Every run exited 0 on both sides.

   | battery | lines | moved |
   |---|---|---|
   | `pierce_runs_battery` | 4 537 | 0 |
   | `pinch_runs_battery` | 3 025 | 0 |
   | `corner_pairs_battery` | 16 381 | 0 |
   | `rc_wide_battery`, shards 0, 13, 29, 41, 57, 70, 83 of 84 | 7 × 481 | 0 |
   | **`four_pairs_battery`** (not run by the PR) | 5 334 | 0 |
   | **`eight_crossings_with_a_pinched_cube_battery`** (not run by the PR) | 577 | 0 |
   | **`near_tangent_battery`** (not run by the PR) | 7 202 | 0 |

   - `four_pairs` is the one that reaches four pairs at one shared vertex. 982 of its runs refuse `SharedVertexCrossings`, which means `reconcile_pass` → `held_cut` → `on_arc` ran with turned runs.
   - `eight_crossings` is the nested matching: 576 runs, all `OK SOUND`.
6. **The mutants.** *Executed.* Rows: topo `--lib boolean::insert`, topo `--test all`, and sweep `--test all` filtered to `join|pinch|corner|reflex|rc_`. The unmutated head is red on one row, `sphere_twin_rows_interval::a_turned_half_cap_at_a_tight_k_keeps_its_twin`. That row says "run it under nextest", so it is a harness artifact and I discount it below.
   - **M1, the tie rule in `arc_holds` flipped** (a tied end counts outside): **killed** by 2 topo rows, `union_flush_onto_edge_contact::a_dangling_null_edge_inside_another_along_one_end_builds_in_every_op` and `…::every_tied_strut_witness_holds_with_its_vertex_pairs_reversed`. No sweep row goes red, so the tie rule's only guards are those two topo rows.
   - **M2, `held_cut` reading from the run's first entry again: survives every row.** This is an equivalent mutant, not a coverage gap. Claim 1's model shows the only input that tells the two readings apart cannot be reached.
   - **M3, `on_arc`'s ends swapped:** **killed** by 5 unit, 26 topo and 19 sweep rows.
   - **M4, the precondition check removed:** **killed** by `walks_before_reads_one_order_from_the_orbits_first_entry` (its topo and sweep integration rows were not awaited). That is expected, since no production orbit reaches the check (claim 2).
7. **The sweep is complete: holds, with one blind spot named.** *Executed:* a shape-different grep over `boolean/{insert,vtxfac,recl,sectors,join}.rs` for `% n`, `.0 <`/`>`, `sort_by`, `cmp(`, `position(`, `min_by` and `max_by`.
   - New hits:
     - `vtxfac.rs:1306-1316` `out_runs`, a walk from a local origin (the first `In` entry). The PR's `+ n -` and `% n <` shapes cannot match its `(anchor + 1 + k) % n`. It extracts runs and compares no two positions. It is the copy of `splitting::insert::above_runs` that is already filed (`work/cleave/run-loops-of-split-and-pierce-are-twins.md`).
     - `insert.rs:556-572`, `run_order` and `walk_run`'s `p1 < p0` on walk ranks. Consistent with the fixed order.
     - `insert.rs:1380`, `tied_held`'s within-entry `walks_after`. No origin.
     - `join.rs:1271` and `sectors.rs:968`. Neither is a position.
   - I agree with the PR's "not folded" dispositions for `vtxfac.rs`, `recl.rs`, `rest.rs` and `zip.rs`, and with keeping `arc_holders`' `len`.

## Findings

- **MINOR, doc. `on_arc`'s wrap sentence is wrong for one region** (`insert.rs:1414-1415`, shown by the model). It says "a position before `lo` is past it and a position after `hi` short of it". A position strictly between `hi` and `lo` of a wrapped arc is after `hi`, yet the code returns `short of hi = Some(false)` for it (`insert.rs:1427-1431`). It is "short of `hi`" only when it is not before `lo`. The code is right; the sentence over-claims.
- **NOTE.** The case table rests on "a fan's ends sit in different entries". That holds only because of how `is_strut` is defined, and nothing at `on_arc` or `held_cut` asserts it (`insert.rs:1314-1323`). A future run with `from == to` that is not a strut, such as a whole-orbit fan, would be read silently as a near-whole wrapped arc rather than refused. *Inspection.*
- **NOTE.** M2 is an equivalent mutant. Its survival is what the PR's argument predicts and shows no gap. *Executed.*
- **NOTE.** M1 is killed only by topo rows, and none of the sweep batteries' non-ignored rows pin the tie rule. Two rows guard it. *Executed.*
- **NOTE.** The PR body says the measured head was 8b03369e. The frozen head 1deb0892 differs from it only under `work/`, and I re-measured on 1deb0892. *Inspection.*

## Style

Questions exercised: Q1, Q2, Q3, Q4, Q5, Q6, Q7. Q8 was partial: I read `insert.rs`'s module header and every function the diff touches, not all 3 200 lines.

- **Q1/Q7: `arc_holders` invents fallibility** (`insert.rs:633-634`, `insert.rs:639` and `insert.rs:652`). Its integer comparator is `Ok::<_, ArcRefusal>(…)`, so `holds(o, k)?` and `== Ok([true, true])` are error paths that can never run. They exist only so the ints can share `on_arc`'s `Result` signature, and the generic shape costs the simple reader. *Sure.*
- **Q1, fresh-instance check.** `on_arc` decides wrap itself with `before(hi, lo)` (`insert.rs:1424`). For walk positions, `walk_run`/`run_order` (`insert.rs:556-572`) have already decided which way a run goes, and `arc_holders`' `len` reads extent a third way (`insert.rs:632`). One concept, "this arc wraps the origin", is now spelled in three places, and the PR adds one of them. *Unsure.*
- **Q1: two within-entry orders remain.** `walk_faces_first` reads `strut_order` from the arrival edge, while `walks_before` reads `walks_after` (`insert.rs:1966-1985`). This is disclosed and pinned by `the_strut_order_agrees_with_the_walk_within_an_entry`. It is still a second spelling, kept in step by a test and a comment. *Likely* (it predates this PR).
- **Q2: the precondition is checked on every comparison.** `walks_before` re-tests `sectors[0].end_edge()` on every call (`insert.rs:1399-1403`). The invariant belongs to the array, so a type or a check at `build_sectors` would carry it once. The comment does now match an enforcement, which is better than before. *Unsure.*
- **Q4: the deleted `holds_whole` comment.** It explained why reading from `lo`'s entry would wrap an inner segment that straddles `lo`. Its hazard is now closed by the fixed origin, so deleting it is right. No other prose cites `precedes`, `edge_bound_entry` or the origin parameter (checked by grep over the whole repo). *Sure.*
- **Q5: the module header predates this PR and disagrees with the code.**
  - `insert.rs:37-39` says the forward-crossed bound is the sector's START.
  - `run_fan` and `mint_run` say entering entry `k` crosses `sectors[k].end` (`insert.rs:2194-2236`).
  - The header's "empty span (both germs in one sector)" also omits the twins case that `mint_run`'s doc names.

  *Unsure* which one is meant to hold.
- **Q3: the new unit test can fail.** M3 and M4 turn it red. Its refusal row, a rotated twin first, is genuine. *Sure.*
- **Q6:** both deviations (the added precondition, and moving `holds_whole`'s tie comment to `arc_holds`) are improvements, so nothing is owed. *Likely.*

REVIEW COMPLETE

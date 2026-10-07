# Review: PR #4240 at 3708477019e5dd56487d3770cbceb2d7ce1d7dd7

**Verdict: APPROVE-WITH-FIXES** · MAJOR 0 · MINOR 4 · NOTE 5

Line numbers are the frozen head's. The probe rows are this branch's `validate::tests::review_probes` (`crates/topo/src/validate.rs`). For the battery and suite runs, I instrumented a copy of the head so that every corner group and ring pair logs its verdict.

## Claims

1. **The wedge test: holds, with one gap (MINOR-1).** `wedge_holds_reads_every_class` (executed) covers convex, reflex, straight and cusp wedges, bounds, and the mirror through `-n`. The arriving side as the mate of `prev` (`validate.rs:6941`), interior-left (`entity.rs:78-97`) and ring orientation agree. "On a bound is not inside" passes no line crossing: two lines on one ray are an `EdgeEdgeOverlap`. Tangent arcs are the filed tie residue.
2. **No legal body refused: holds** (executed).
   - Batteries: pinch, pierce, corner_pairs, and `rc_wide` shards 0/17/41/66/83. 13,140 groups reached the arm, all planar, all `ok`.
   - Suites `sweep`, `editor-core`, `step-import`, `verbs`, `pncad`, `topo`: 32,424 groups (10,839 cylinder, 6,890 torus, 1,050 cone, 164 sphere, 13,436 plane). Not one legal group is non-`ok`. The non-`ok` groups are the PR's pin, which is meant to refuse, and the inside-out fixture in NOTE-2.
   - About 90k ring pairs; every suite green apart from nextest-only rows (NOTE-5).
   - The ruling's figure-eight hole with a closed arc passes (`a_figure_eight_hole_with_a_closed_arc_passes`).
   - Off a `Seam`, slits and bridges cannot rest: check 6 refuses wedge 0/2π, and tier 2 refuses strut tips. So the cusp silence (`validate.rs:7070`) is unreachable there (likely).
   - Not run: the Python suite.
   - Brief correction: the PR ran all 84 `rc_wide` shards, so none were left unpicked.
3. **The curved arm: holds** (likely). Normals come only on regular charts: cylinder, sphere and ring torus. Cone, NURBS and `Approx` read `Ok(None)` and stay silent. First order is exact for the infinitesimal wedge at any non-zero angle, and ties read `Zero` → silent, which is filed. The suite reaches it 18k times, and M1 turns a sphere row red. The silence leaves no mark (NOTE-3).
4. **`RingMeetsRing` vs producers: no producer found** (inspection).
   - `weld_pinches` joins two loops touching at a corner into one (`boolean/finish.rs:507`).
   - `profile` refuses touching segments.
   - The shell glue's gap is filed, and check 9 now refuses it untyped.
   - STEP import validates at its gate (`step-import/src/lib.rs:931`), so a file whose face has two touching inner bounds now refuses at import: right under the ruling, but unmentioned in the PR body.
5. **D9 and escalation: holds.** The sort key `(PointKey, index)` is total (`validate.rs:6906`). `wedge_holds_escalates_in_band_only` (executed): an in-band lever escalates; a pair the other bound decides does not; a turn in band escalates. The `RingContactEscalated` reuse is MINOR-4.
6. **The pins go red: partly falsified (MINOR-3).** Each mutant ran over the topo lib and the `topo/tests/all` rows matching pinch|ring|holes_meeting|cylinder|seam|torus|sphere. M3 also ran over the instrumented `sweep` and `step-import` suites.

   | mutant | rows outside this branch's probes that go red |
   |---|---|
   | M1 flip the lever sign | crossed-prism pin, 5 `holes_meeting_at_a_vertex`, `vtxfac` |
   | M2 reflex reads as convex | **none** (my probes only) |
   | M3 drop the edge skip | none in topo; `sweep` `mate7a_torus_rest` row + 10 false verdicts |
   | M4 drop the ring-pair loop | `check_9_refuses_two_rings_touching_at_a_vertex` |
   | M5 straight arm silent | **none** (my probes only) |
   | M6 arriving side not the mate | pin, 5 `holes_meeting`, 2 `pierce_strut_at_a_pinch` |

7. **Threading: holds.** These are all threaded:
   - tags and the tag inventory, the stub, the census;
   - `editor-core::attribute`, the samples roster, `payload-rung-sweep`.

   `validation_error_tag` is exhaustive, and the `_ => None` extractors are questions. Stale prose is S3.
8. **Sweep: partly.** My sweep (check-9 word filters, readers of a face's loops) found filters the PR left narrow (S1) and parallel corner readers (S2).

## Findings

**MINOR-1. The skip compares edges, so a closed arc at a pinch hides a crossing** (`validate.rs:6953`, executed).
- `edge == out_edge || edge == in_edge` drops every side on an edge the corner holds. A closed edge has two opposite sides at the point.
- `a_crossed_pinch_through_a_closed_arc_passes_check_9_at_head` builds a ring through P twice: a triangle, plus a radius-0.2 circle walked against it. Its corners (0°→270°) and (300°→180°) overlap, and the oracle confirms it. The host face draws no word from all of tier 3.
- The skip is load-bearing for legal bodies (M3: 10 false verdicts in `sweep`, mostly at closed `seam: true` circles). The fix is therefore not side identity alone; the `Seam` description may separate the two.
- No producer is known to reach the crossed shape.

**MINOR-2. The ring-pair loop is quadratic with no broad phase** (`validate.rs:6824-6845`, executed: `ring_pair_cost`, a grid of triangular holes, tier-3 local checks timed).

| rings | head | ring-pair loop off |
|---:|---:|---:|
| 400 | 174 ms | 3.7 ms |
| 1,600 | 2.45 s | 13.4 ms |

A 10k-hole plate projects to about 100 s per `validate_geometric`, which every verb runs at close. The PR body does not measure it.

**MINOR-3. M2 and M5 are pinned by no row outside this branch.** Both make the arm more lenient, so no legal body can catch them; only a crossed pin can. The PR ships one crossed pin, which is convex. The curved arm has no crossed pin.

**MINOR-4. `RingContactEscalated` reused for ring pairs drops `other`** (`validate.rs:6837`). It is indistinguishable from the outer-loop word (`validate.rs:6816`), so one ring can carry two look-alike words. The deviation is disclosed but unscheduled (no `work/` row).

**NOTE-1.** The PR's reason for the skip ("a seam vertex reads nothing") is incomplete. What it carries is closed seam circles, and the same mechanism silences MINOR-1.

**NOTE-2.** On an inside-out cylinder (`step-import rev_import_probe::three_rim_flipped_wall_layering`), `PinchCornerCrossed` echoes `CurvedSenseInverted`. The arm reads the stored sense and calls a damaged file a crossing.

**NOTE-3.** The arm's silences (cone, NURBS, `Approx`, ties) leave no mark on a passing body. This is consistent with check 9's other residue. It is filed.

**NOTE-4.** Unrelated pre-existing fuzz hit: `movefac::tests::tears::movefac_on_a_few_torn_bodies`, seed 3482798898476541996, `ops_ring_bridge`, panics at `surgery.rs:253`.

**NOTE-5.** Unrelated to this PR: `sphere_twin_rows_interval::a_turned_half_cap...` and `editor-core msolve8_levered_clash::c4_*` fail under `cargo test` because they commit a process-global tolerance. Each passes alone; they are nextest-only rows.

## Style

Exercised: Q1, Q2, Q3, Q4, Q5, Q6, Q7, and Q8 (the check-9 region, `validate.rs:6645-7090`, end to end; not the 14.9k-line file).

- **S1 (Q1/Q4, sure). The check-9 filters drifted.** This is a class; sweep for more copies.
  - `validate.rs:11440` was widened, but its doc still says "All four".
  - Not widened:
    - `sweep/tests/topo_ring_nesting.rs:63` ("four words");
    - `check_9_decides_a_key_shared_loop_pair` (`validate.rs:11205`), which asserts "check 9 must be silent";
    - `check_9_refuses_a_ring_that_lies_outside_its_outer_loop` (`validate.rs:11280`).
- **S2 (Q1, likely).** "A corner is a slice" has three homes:
  - `test_support_meeting.rs` `corners_disjoint` (Newell normal, angles, micron rounding, across loops);
  - the mesher's `PinchWedge`;
  - `wedge_holds`.

  There is no shared core. The oracle compares across loops, and the arm does not.
- **S3 (Q4/Q5, sure). Stale sentences:**
  - `pncad-py/src/validation.rs:56` ("its face's own outer loop").
  - `pncad/src/prelude.rs:505` ("`RingContact` is `RingMeetsOuter`'s").
  - `validate.rs:6706` ("the shell verb's own door refuses ahead of them"), false for ring pairs per the PR's filed row.
  - `docs/KERNEL-VERBS.md:433-452` (check 9 as ring vs outer only).
- **S4 (Q2, likely).** "`outer_*` fields name `other`'s" (`validate.rs:1486`): a comment reconciling a payload whose field names are now false for one of its two users.
- **S5 (Q6, likely).** The disclosed deviations "reuse `RingContactEscalated`" and "one refusal per point" owe a schedule. No `work/` row exists.
- **S6 (Q7, unsure).** `pinch_corners_at` (`validate.rs:6945-6985`) computes the normal lazily inside the pair loop through `Option<Option<_>>` and returns `Ok(())` from inside it. Hoisting it per group would read more plainly.
- **S7 (Q3, sure).** The closed-arc probe asserts the head's silence on purpose, so it goes red when MINOR-1 is fixed. Flip it to assert the refusal then.
- **S8 (Q2, likely).** The skip's comment and the PR body justify it by seam vertices. Its real load is closed seam circles (NOTE-1), so the reason written beside the code is not the one that holds it up.

REVIEW COMPLETE

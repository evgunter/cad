IN PROGRESS (awaiting one run: the seam-skip mutant over the instrumented sweep suite)

# Review: PR #4240 at 3708477019e5dd56487d3770cbceb2d7ce1d7dd7

**Verdict: APPROVE-WITH-FIXES** · MAJOR 0 · MINOR 4 · NOTE 5

Line numbers are the frozen head's. The probe rows are on this branch, in
`crates/topo/src/validate.rs` under `validate::tests::review_probes`.

## Claims

1. **The wedge test is right: holds, with one gap (MINOR-1).** Executed:
   `wedge_holds_reads_every_class` covers convex, reflex, straight and cusp
   wedges, sides on either bound, and the mirror through `-n`. The arriving
   side as the previous half-edge's mate (`validate.rs:6941`), the
   interior-left convention (`entity.rs:78-97`) and the ring orientation
   agree: no false refusal across the batteries and suites below. "A side
   along a bound is not inside" lets nothing through for lines: two lines on
   one ray are an `EdgeEdgeOverlap`. For tangent arcs it is the filed tie
   residue.
2. **No legal body is refused: holds** (executed). I instrumented the head
   so that every corner group and ring pair logs its verdict.
   - Battery subset: pinch, pierce, corner_pairs, and `rc_wide` shards
     0/17/41/66/83. That is 13,140 groups, all planar, every one `ok`.
   - Suites `sweep`, `editor-core`, `step-import`, `verbs`, `pncad`, `topo`:
     32,400 groups (10.8k cylinder, 6.9k torus, 1.0k cone, 0.2k sphere, the
     rest planar), 0 non-`ok` on a legal body. The two `crossed` verdicts in
     `topo` are the PR's pin. The one in `step-import` is an inside-out
     fixture (NOTE-2).
   - 90k ring pairs, with every suite green apart from the nextest-only rows
     (NOTE-5).
   - Seam vertices, torus four-corner groups and sphere rows read `ok`.
   - The ruling's figure-eight hole (one ring through two vertices on one
     key, with a closed arc) passes:
     `a_figure_eight_hole_with_a_closed_arc_passes`.
   - A slit or bridge cannot sit at rest off a `Seam`: check 6 refuses
     wedge 0/2π, and tier 2 refuses strut tips. So the cusp silence
     (`validate.rs:7070`) is unreachable there (inspection, likely).
   - Not run: the Python suite.
   - Brief correction: the PR measured all 84 `rc_wide` shards, so none
     were left unpicked.
3. **The curved arm is sound: holds** (likely). `face_outward_normal_at`
   has arms only for cylinder, sphere and ring torus, all regular
   everywhere on the surface. Cone, NURBS and `Approx` return `Ok(None)` →
   silent. The levers are first-order, which is exact for the
   infinitesimal wedge at any non-zero angle. Tangent ties read `Zero` →
   silent, and that is filed. The suite reaches the curved arm 18k times,
   and M1 turns a sphere row red, so it is exercised. The silence is filed
   (`restfront/the-corner-slice-arm-is-silent-...`) but leaves no mark on a
   passing body (NOTE-3).
4. **`RingMeetsRing` holds the ruling: no producer found** (inspection).
   - `boolean/finish.rs:507`: `weld_pinches` joins two loops touching at
     a corner into one.
   - `profile` refuses touching segments.
   - The shell glue checks the outer loop only (filed by the PR; check 9
     now refuses there, untyped).
   - `split_section_rings` passes.
   - STEP import validates at its gate (`step-import/src/lib.rs:931`), so a
     file whose face has two touching inner bounds now refuses at import.
     Correct under the ruling, but a behaviour change worth one line in
     the PR body.
5. **Determinism and escalation: holds.** `keyed.sort_unstable()` sorts
   `(PointKey, index)`, whose index part is unique, so the order is total
   (`validate.rs:6906`). Executed: `wedge_holds_escalates_in_band_only`.
   An in-band lever escalates. A pair decided outside by the other bound
   does not. A turn in band escalates. The reuse of `RingContactEscalated`
   is MINOR-4.
6. **The pins can go red: partly falsified (MINOR-3).** I ran each mutant
   over the topo lib and the `topo/tests/all` rows matching
   pinch|ring|holes_meeting|cylinder|seam|torus|sphere.

   | mutant | PR's own rows red | review probes red |
   |---|---|---|
   | M1 flip the lever sign | crossed-prism pin, 5 `holes_meeting_at_a_vertex`, `vtxfac` | yes |
   | M2 reflex arm reads as convex | **none** | `wedge_holds_reads_every_class`, closed-arc row |
   | M3 drop the seam skip | **none** | closed-arc row |
   | M4 drop the ring-pair loop | `check_9_refuses_two_rings_touching_at_a_vertex` | — |
   | M5 straight arm silent | **none** | `wedge_holds_reads_every_class` |
   | M6 arriving side not the mate | pin, 5 `holes_meeting`, 2 `pierce_strut_at_a_pinch` | yes |

7. **Threading is complete: holds.** Tags and the tag inventory, the stub,
   the census, `editor-core::attribute`, the samples roster and
   `payload-rung-sweep` are all threaded. `validation_error_tag` is
   exhaustive. The `_ => None` extractors are questions, so they are
   correct. Stale prose is listed under Style.
8. **The sweep is complete: partly.** My own sweep, of the readers of a
   face's loops and of check-9 word filters, found filters the PR did not
   widen (S1). It also found two parallel corner-slice readers (S2). No
   other reader assumes a repeated-point corner is uncrossed.

## Findings

**MINOR-1. The seam skip compares edges, not sides, so a closed arc at a pinch
hides a crossing.** `validate.rs:6953` (executed).
- `edge == out_edge || edge == in_edge` drops every side on an edge the
  corner holds. A closed edge (a whole circle) has two different sides at
  the point.
- `a_crossed_pinch_through_a_closed_arc_passes_check_9_at_head` builds a
  ring through P twice: a triangle, plus a radius-0.2 circle walked
  against it. The corners are (0°→270°) and (300°→180°). They overlap,
  and the oracle confirms it. The host face draws no word from all of
  tier 3.
- The skip is not load-bearing for the seam it is justified by (M3:
  `SEAM_RESULT`). A seam's two sides at the seam vertex point the same
  way, so "a side on a bound is not inside" already silences them.
- Comparing half-edges, not edges, would keep the seam case and close
  this. No producer is known to reach it.

**MINOR-2. The ring-pair loop is quadratic, with no broad phase, and
dominates tier 3 on hole-heavy faces.** `validate.rs:6824-6845` (executed:
`ring_pair_cost`, triangular holes in a grid, tier-3 local checks timed).

| rings | head | ring-pair loop off |
|---:|---:|---:|
| 400 | 174 ms | 3.7 ms |
| 1,600 | 2.45 s | 13.4 ms |

A 10k-hole perforated plate projects to about 100 s per `validate_geometric`,
which every verb runs at close. Not in the PR's measurements.

**MINOR-3. Three arms are pinned by no row the PR ships** (M2, M3, M5 in
claim 6). A reflex corner reading as convex, or a straight corner going
silent, ships green: the batteries are differential and build no crossing.
The curved arm also has no crossed pin. It is reached only by uncrossed seam
groups and one sphere row.

**MINOR-4. Ring-vs-ring escalation reuses `RingContactEscalated { face,
ring, source }`.** `validate.rs:6837`. It drops `other`, and it is
indistinguishable from the ring-vs-outer word at `validate.rs:6816`. One ring
can carry two identical-looking words, one against the outer loop and one
against a ring, with no way to tell which pair. The PR discloses it, and no
followup is scheduled.

**NOTE-1. The PR's "Sides on edges the corner itself holds are skipped, so a
seam vertex reads nothing" rests on the wrong reason** (MINOR-1, M3).

**NOTE-2. On an inside-out cylinder, the corner arm echoes
`CurvedSenseInverted` with `PinchCornerCrossed`.** The face is
`step-import rev_import_probe::three_rim_flipped_wall_layering`, read under
its stored sense; the second word calls a damaged file a crossing.

**NOTE-3. The corner arm's silences leave no mark on a passing body**
(cone, NURBS, `Approx`, ties). This matches check 9's other residue
(ellipse arms), so it is consistent, but a pass still claims more than
was read. The silences are filed.

**NOTE-4. A pre-existing fuzz counterexample, unrelated to this PR.**
`movefac::tests::tears::movefac_on_a_few_torn_bodies`, seed
3482798898476541996, `ops_ring_bridge`, panics at `surgery.rs:253`.

**NOTE-5. Two kinds of nextest-only failure appeared under `cargo test`;
neither is this PR's.** `sphere_twin_rows_interval::a_turned_half_cap...`
and `editor-core msolve8_levered_clash::c4_*` commit a process-global
tolerance. Each passes when run alone.

## Style

Questions exercised: Q1, Q2, Q3, Q4, Q5, Q6, Q8 (the check-9 region,
`validate.rs:6645-7090`, read end to end; not the whole 14.9k-line file).

- **S1 (Q1/Q4, sure). The "check 9's words" filters drifted.** Each is a
  class instance; there may be more copies than these.
  - `validate.rs:11440` was widened, but its doc still says "All four".
  - Not widened:
    - `crates/sweep/tests/topo_ring_nesting.rs:63` ("four words");
    - `check_9_decides_a_key_shared_loop_pair` (`validate.rs:11205`),
      which asserts "check 9 must be silent";
    - `check_9_refuses_a_ring_that_lies_outside_its_outer_loop`
      (`validate.rs:11280`).

    These would not see a `RingMeetsRing` or `PinchCorner*` word.
- **S2 (Q1, likely).** "A corner is a slice of its face" now has three
  homes, with three vocabularies:
  - `test_support_meeting.rs` `corners_disjoint` (Newell normal, angles,
    micron rounding, cross-loop);
  - the mesher's `PinchWedge`;
  - `wedge_holds`.

  No shared core, and the test oracle checks across loops while the arm
  does not.
- **S3 (Q4, sure). Stale sentences:**
  - `pncad-py/src/validation.rs:56`: "How a ring meets its face's own outer
    loop". The stub was updated.
  - `pncad/src/prelude.rs:505`: "`RingContact` is `RingMeetsOuter`'s".
  - `validate.rs:6706`: "the shell verb's own door refuses ahead of them".
    That is false for ring pairs, as the PR's own filed row says.
  - `docs/KERNEL-VERBS.md:433-452` describes check 9 as ring vs outer only.
- **S4 (Q2, likely).** The `RingMeetsRing` doc and the field comment carry
  "`outer_*` fields name `other`'s" (`validate.rs:1486`). The payload
  type's field names now lie for one of its two users. That comment
  reconciles two spellings of one rule.
- **S5 (Q6, likely).** The deviation "reuses `RingContactEscalated`" is
  disclosed but owes a schedule: no `work/` row.
- **S6 (Q7, unsure).** `pinch_corners_at` computes the normal lazily
  inside the pair loop, through `Option<Option<_>>`, and returns
  `Ok(())` from inside it. Hoisting it per group would read more plainly.
- **S7 (Q3, sure).** The closed-arc probe asserts the head's silence on
  purpose. Turn it red to green when MINOR-1 is fixed.

REVIEW COMPLETE

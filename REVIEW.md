# Review r2 — PR #4139 at frozen head 7863f27b

**Verdict: APPROVE-WITH-FIXES.** MAJOR 0 · MINOR 3 · NOTE 4.
Main baseline `f9bf3bca` (the PR's), release, separate target dirs. Lane isolation kept: no
`join/pinch-one-vertex-per-cone-review-*` branch but this one was fetched or read. The cylinder
and near-tangent poses come from PR 4038's review branch `join/pierce-pinch-families-review-r2`
(`examples/r2_pinch_probes.rs`, run locally, with operand-error and mesh print-outs added; not committed).
Probe rows: `crates/sweep/tests/join_pinch_cones_r2_probes.rs` (ignored batteries).

## Claims

1. **No wrong body; the ruled shape — holds (sure for the poses run).** Executed:
   - *Tri-cone family* (`r2_pinch_cones_battery`, 768 lines). Cube corner ∖/∪/∩ a parallelepiped
     cone poking through 0–3 of its faces. The poses give 3 cones at one point, and one cone holding
     three runs of each operand (∪, ∩). Gaps run 0.3 down to 1e-6 (near-tangent), with 4 rigid
     turns, both orders. Result: 768/768 `SOUND`, all mesh. The vertices on the point equal the cone
     count read without the kernel (sphere sampling in Python, components(in)+components(out)−1).
     **But main ≡ head ≡ mutant B on every line:** this lane (vertex–vertex) keeps per-run copies,
     and `split_cones` never splits there.
   - *Pinched operand* (`r2_pinched_operand_battery`, 720 lines). The 3-vertex result above is used
     as an operand against a box whose face passes through the point. Main ≡ head on class and
     facts. No loop passes a vertex at the point twice and no vertex's orbit misses its own
     half-edges (`twice=0 gaps=0` everywhere). No wrong volume. The 160 tier-3′-only BADs are the
     same lines on main. The split is not reached here either.
   - *Cavity touching the outer shell at the pinch* (L=0.5, gaps negative): 1 solid, 2 shells, `SOUND`.
   - *Near-tangent r2 set* (40 320 lines, main vs head): the PR's numbers reproduce exactly.
     87 refusal→`SOUND`, all meshing (`check_mesh`; all 34 222 built head bodies in the set mesh);
     7 `JoinDesync`→BAD, each tier-3′ `CensusEscalated` only (margins in the 1e-9…1e-8 band) at
     the closed-form volume, so not definite; 19 BAD→refusal; 3 `SOUND`→`ResultInvalid
     {ShellRoleUndecided: Escalated}` (`shallow200`/`vee300`/`notch307`, d=1e-8).
   - Not built: two pinches on one face, island-meets-notch, curved pinches beyond the cylinder
     set. A plane through `cube ∪ tri-cone`'s star link gives ≤2 pieces (200 directions): no 3-cone split.
2. **Cone computation.**
   - **Holds at ≤2 runs per vertex; unsure at ≥3 (see MINOR-1).** With 2 section corners σ is its
     own inverse, so σ_B∘σ_A cannot tell orbit direction or which B run an A run continues into.
     No row I or the PR ran puts ≥3 section corners on a split vertex.
   - Re-pairing by edge identity is sound by inspection (zip.rs:383-407): partners read the live
     `parent_loop` after `kef`/`mfkrh`; the one-to-one check runs both ways.
   - The self-fusion refusal (zip.rs:417) and the interleave refusal (zip.rs:342) were reached by
     no line I ran. For interleaving I argue unreachable: two cones' arcs on one simple link
     circle cannot interleave without their B chords crossing. The self-fusion refusal needs a cone
     holding ≥2 runs of each operand at a split vertex; not constructed.
3. **Shell split — holds (likely).** `movefac` only re-labels edge-disconnected components, which
   tier 2 needs anyway. Mutant A (no shell split) turns 4 rows red. Lumps that meet only at a pinch
   leave as their own solids: the tri-cone B∖A gives 3 solids and the PR's rows pass check 10.
   No case found where two pieces had to stay one shell.
4. **Operand-only BADs — holds.** Re-ran r2's cylinder set (17 280 lines): 68 refusal→BAD
   (38 `JoinDesync` + 30 `PinchUncrossed`).
   All 68 pass t2, t3′ and the certificate at the closed-form volume and fail the far-brick union
   as `Containment(VolumeUncertified)`, the same error as all 11 958 of main's operand-only BADs:
   right bodies, a known class. Also 218 operand-only BADs gain tier-3′ `CensusUndecidable`
   (curved, cross-part) and 6 go BAD→`VolumeUnmeasured`, both disclosed. r1/island sets not re-run.
5. **Kept `weld_pinches` — holds (likely).**
   `union_pinch_member_order` pins one vertex at each pinch, unchanged by the PR; consistent, as the
   touching blocks' coincident edges carry the orbit through (one cone). No probe body holds a
   crossing (`twice`/`gaps` = 0). Filed as P1 `the-pre-zip-pinch-weld-retires-…`. I found no weld
   whose vertex keeps two cones after `split_cones`, but did not build one specifically.
6. **Nothing else moved — holds (sure).** main vs head byte-identical (class lines) on:
   `pinch_runs_battery` 3 027, `pierce_runs_battery` 4 539, `corner_pairs_battery` 16 383,
   `join1_r1_reflex_battery` 1 155, `j3r2_r1_reflex_battery` 1 155, `rc_wide_battery` ×84 shards 40 236.
   I did not re-take the mesh column on these.
7. **Mutants — real (sure).**
   A (no shell split): 4 red, as the PR says. B (no split, `if true` at zip.rs:335): 10 red (PR: 9).
   C, mine: `cone_b[k] = cone[k]` at zip.rs:312 (a B run keyed by its own pair, not the A run it
   continues) **survives** all 23 `join_pierce*` rows, the pinch and pierce batteries
   (byte-identical) and the tri-cone family: MINOR-1.

## Findings

- **MINOR-1 (test-gap, executed).** The cone cycles are untested past two section corners per
  vertex.
  zip.rs:298-313: σ's direction and the `cone_b[σ_A(k)]` indexing are the substance of the route;
  mutant C rewrites the latter and survives every row and battery I ran (claim 7), as the PR's own
  "equivalent at two cones" survivor says. Nothing scheduled covers a ≥3-cone split, and no row
  reaches the self-fusion and interleave refusals (zip.rs:342, 417).
- **MINOR-2 (doc, inspection).** `split_cones`'s whole doc comment, including `# Errors`, is
  attached to `type RunsAt` (zip.rs:200-234), not to `split_cones` (zip.rs:236). The function is
  undocumented in rustdoc, and the alias documents a function.
- **MINOR-3 (test-gap / stale doc, inspection).** `join_pierce_strut_facing.rs`:
  the module doc (:27-28) promises "one vertex per cone at `v`" for every op, and `assert_pose`'s
  doc (:159-164) still says "no face runs through two of them"; the body (:198-214) deleted that
  guard and asserts only a shared point key plus meshing. A pose whose cones merged onto one
  vertex, or split onto too many, passes if it meshes.
- **NOTE-1.** Both of my independent families never reach the split: main ≡ head ≡ mutant B. The
  split path's coverage is the PR's own rows. This is a gap in what independent probing could
  reach, not a defect.
- **NOTE-2.** The 3 `SOUND`→escalated refusals (claim 1) follow from the shell split making
  near-tangent sliver shells whose role read escalates. They are disclosed and typed, never silent.
- **NOTE-3.** `Descendants::absorb_fusions` lost its doc line; now only `absorb_zip`'s helper (ops.rs:2557).
- **NOTE-4.** Measurement claims with no guard: "PinchWedge fires on 0 lines" and "no battery line
  reaches it" (the self-fusion arm). The PR body states them; nothing goes red if they stop being
  true.

## Style

Exercised: Q1, Q2, Q3, Q4, Q5, Q6, Q7, Q8. Q8: I read zip.rs (583 lines) end to end.

- **Q1 (likely).** Three spellings of "the vertices at a point" in the touched tests:
  `faces_through_two_vertices_at` (join_pierce_runs_sweep.rs:734), the staircase's inline filter
  (:842-846), `a_pinch_keeps_one_vertex_per_cone`'s loop (strut_facing:277-306); `pierce_point_finding`
  (:334) is a fourth near-relative. Look in `boolean_pinch_copies.rs` too.
- **Q1 (likely).** The kept `weld_pinches`/`pinch_site`/`Joint::Hole` fuse vertices on one point
  key, and `split_cones` then splits them on the same key. Two near-parallel mechanisms on one
  concept, the second not fully replacing the first; it is disclosed and filed.
- **Q2/Q5 (sure).** `zip.rs:16-18` and `:52` ("a welded pinch, with one correspondent per meeting")
  still hold only because the weld is kept. They should go with the filed issue.
- **Q3 (sure).** No row can go red on a wrong orbit direction (MINOR-1), and none reaches the three
  `ZipCorrespondence` arms of `split_cones`.
- **Q4 (sure).** The deleted guard's prose survives at strut_facing:159-164 (MINOR-3). That is the
  code drifting from a meant invariant, not just rot: the count assertion is the invariant.
- **Q6 (likely).** The deviations (weld kept, checks 9 and corner-slice) are filed as issues:
  scheduled. The measurement claims (NOTE-4) carry no guard and no reason at the claim site.
- **Q7 (sure, taste).** `split_cones` is a ~190-line function that loops a 4-tuple over the two
  sides with `&dyn Fn` closures and `&mut` section lists, guards a loop with `.filter(|_| moved)`
  (zip.rs:384), and computes `rebuilt` even when discarded. I would split σ/cones, per-side split
  and re-pair.
- **Q7 (unsure).** `split_cones` mutates the body before its one-to-one check can refuse
  (zip.rs:350-407); harmless if the body is a discarded working copy on `Err` (not verified).

REVIEW COMPLETE

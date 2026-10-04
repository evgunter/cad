# Verify: PR #3984 (reach/planar-lane-curved-carrier), last fix pass

Frozen head **075ed8f3a645f48a3fe3eb2f8392578d41bdcb26**. The branch head equals it; it has not moved.
Review freeze 8abb6e7931. Verifier session 2026-10-04. No code changed on the PR branch, nothing merged, no comments posted.

## Mutants
Each mutant is the smallest edit to the head's source, run against `-p topo --lib -E 'test(/planar_lane_carrier_rows|splitting::classify::|offer_rows|lying_on_rows/)'` (28 rows) and then reverted. The tree was clean after each.

| mutant | edit | red rows | result |
|---|---|---|---|
| **M5** (brief): only the spiric arm routed to `frontier()` | `reduce.rs` curved arm: `(None, Curve3::Spiric{..}) => return Err(frontier()),` before `(None, _)` | `a_spiric_crossing_a_cylinder_wall_refuses` | **red**: killed, as claimed |
| **M6** (brief): spiric answered as `Line` | `classify.rs` `plane_crossing_lane`: `(None, Line \| Spiric) => Line` | `a_spiric_dipping_through_a_plane_face_refuses`, `each_carrier_kind_lands_on_its_own_lane` | **red**: killed, as claimed |
| M2 (r1/r2): the whole curved arm to `frontier()` | `(None, _) => return Err(frontier())` | `a_spiric_crossing_a_cylinder_wall_refuses`, `an_arc_crossing_a_cylinder_wall_refuses` | red |
| M3 (r1/r2): spiric/spline answered as `Line` | `(None, _) => Line` | `each_carrier_kind…`, `an_arc_crossing_a_plane_face_once…`, `a_spiric_dipping…`, `an_arc_dipping…`, `lying_on_rows::a_nurbs_boundary_edge_does_not_certify` | red |
| M1 (r1/r2): planar arm reads `Unlaned` as a line | `PlaneCrossingLane::Unlaned => None` | `an_arc_crossing_a_plane_face_once…`, `a_spiric_dipping…`, `an_arc_dipping…` | red |

Unmutated, the same 28 rows: 28/28 at ε 1e-9, 1e-6 and 1e-12.

## ε runs (`CAD_TOLERANCE_EPS`)
- The PR's new and changed rows (5 `planar_lane_carrier_rows`, the `splitting::classify::tests` including `each_carrier_kind_lands_on_its_own_lane`, `offer_rows`, `lying_on_rows`; 28 in all): **28/28 at 1e-9, 1e-6 and 1e-12.**
- `cargo nextest run -p topo` (all targets, slow set included) at 1e-9: **2233/2233** (114 skipped by nextest).
- Battery, `-p topo -p sweep -p editor-core -p pncad-py -p pncad`, all at the head:

  | ε | rows | result |
  |---|---|---|
  | 1e-9 | 7177 | 7177 passed, 300 skipped |
  | 1e-6 | 7177 | 7177 passed, 300 skipped |
  | 1e-12 | 7177 | 7177 passed, 300 skipped |

  No red, so no comparison with `origin/main` was needed.
- Hosted CI on the head: run 37164039056, `success` (test, python suite, lint, gates; `head_sha` checked). Its ε 1e-6/1e-12 step covers only the seeded crates.

## Claim checks
1. **The gate stays: TRUE.**
   - c0339802c is an exact revert of c4f3840bd..5501a3f03. `git diff c4f3840bd^ c0339802c` over the 33 files it touches is byte-identical to the main merge 9ad6652f1's own contribution to those files (only index hashes and hunk offsets differ). The attempt leaves no residue.
   - `--remerge-diff`: 9ad6652f1 and 075ed8f3a merged clean. a78488fa3 had one conflict in `insert_crossings`, resolved to main's `edge_clears(..., &BoxFrame::aimed(..))` under the fix pass's `Unlaned` arm. Neutral.
   - `gate_operand_edges` is present and called from `gate_operand` for both operands.
   - Non-main, non-doc code changes since 8abb6e7931 are all in 1e570445d, and the fix brief asked for each:
     - `reduce.rs`: gate docs and comment (F1); curved arm matches `Conic::of` first, `(Circle|Ellipse, None)` arm removed (F10/F11).
     - `classify.rs`: `plane_crossing_lane` via `Conic::of`, `crossing_lane` wrapper removed, its doc moved onto `plane_crossing_lane`, tests use a local `on_split_plane` (F12); row doc (F14).
     - `mod.rs`: variant docs, `CONIC_EDGES_RECOURSE`, "whose box meets" (F2/F5).
     - `boxes.rs`, `contain.rs`: comments only (F15).
     - `planar_lane_carrier_rows.rs`: `spiric_cap` fixture and two rows (F6).
     - `test_north_star.py`: comment only (F1).
   - No other code change.
2. **F6: TRUE.** Both mutants run, both red (table above).
3. **F2/F5: TRUE.**
   - `CrossingCarrierUnsupported` is raised only at `reduce.rs:1237` (planar arm) and `:2200` (curved arm), both inside `sweep_direction` / `sweep_and_settle`.
   - Every non-test caller (`sweep_traces_with_pad` mod.rs:3518, `sweep_records` :3573, the boolean driver :3686) runs `gate_operand_pairs`, and so `gate_operand_edges` on both operands, first. The sweep only splits edges (it keeps the carrier), so it mints no spiric or NURBS edge.
   - `offer_rows`, `coplanar_conic_rows`, `planar_lane_carrier_rows` and reduce's other `curved_face_arm` callers are `#[cfg(test)]`.
   - The gate's public refusal is pinned by `s16_box_soundness`, `spiric_rim`, `offc_r1_probes` and `review_cleave_nurbs_lane`.
   - One recourse constant (`mod.rs` `CONIC_EDGES_RECOURSE`), used by both Display arms.
4. **F10/F11/F12: behaviour-neutral, TRUE.**
   - `Conic::of` (`geom-brep/src/implicit.rs:662`) builds exactly the frame the hand-built code did: a circle as major = minor = radius, an ellipse field for field. It returns `None` only for Line, Spiric and Nurbs.
   - The removed `(Circle|Ellipse, None)` arm was therefore unreachable.
   - `crossing_lane` was a pure forwarder.
   - Rows green at all three ε (above).
5. **Filed items: TRUE.** All five exist and `work.py lint` is ok:
   - `reach/delete-the-boolean-operand-edge-gate` (unit, **P1**, refs 3984);
   - `reach/join-and-continuation-sites-blame-the-edge-gate-for-a-spline-edge` (P2);
   - `reach/split-insert-crossings-second-edge-clears-arm-is-unpinned` (P4);
   - `reach/rod-minus-brick-minus-slab-refuses-solids-do-not-cross` (P2);
   - `lib/cancellation-edit-race-row-assumes-a-slow-evaluation` (P4).

   Also filed, beyond the brief's list: `reach/nurbs-edge-crossing-rung-is-the-ring-composite` (parked) and `reach/spiric-operand-edges-reopen-with-their-first-producer` (deferred).
6. **Battery:** **TRUE, and stronger at this head.** The lane counted 7161/7161 at a78488fa3. At 075ed8f3a the same five crates run **7177/7177 at 1e-9, 1e-6 and 1e-12**; the 16 extra rows came with the main merge. The binding census and the Python suite are not re-run locally here. Hosted CI's "python suite (wheel + guide + north-star)" job is `success` on the head (run 37164039056).

## Non-blocking notes
- **N1: PR body F13 row, false in one clause.** It says `restfront/an-ellipse…` cites no `conic_plane_crossing_roots` at this head. It does: `work/restfront/an-ellipse-stored-minor-over-major-passes-tier-3.md:79`, an open item, whose line was already there at 8abb6e7931. The F13 citation sweep missed it. `docs/PERF-SCAN-2026-08.md:824` also keeps the old name, but that is a dated scan.
- **N2: line drift.** The residue item `join-and-continuation-…` cites `join.rs:1036`, `:1902`, which are 1093 and 1959 at this head. It names each function too, so the drift is minor.
- **N3: weak oracle bound.** In `a_spiric_dipping_through_a_plane_face_refuses`, the oracle asserts the crossing's `z < 1.2`, but the brick face spans `|z| ≤ 1`. The value, ≈0.835, is inside the face, so the row is right; the stated bound is just looser than the face.
- The PR body's "7161/7161" is scoped to a78488fa3. The frozen head has 7177 rows in the same five crates; the main merge brought the rest. See the battery result above.

## Verdict
**VERIFIED.** Every lane claim holds at 075ed8f3a:
- both brief mutants (M5, M6) are killed by the named rows;
- the gate-deletion attempt is exactly reverted;
- each remaining code change was asked for by the fix brief and is behaviour-neutral where it claims to be;
- the variant is unreachable from every public path;
- all filed items exist;
- the battery is green at all three ε.

No blocking points. N1–N3 are documentation nits.

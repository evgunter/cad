# Verify: PR #3982 (`reach/split-gate-oriented-box`)

Frozen head: `4739b3305342041c3d713541728151ac98958b24`. The branch head had not moved at push time.
Fix pass read: `72be7a835` and `f7f33b7b7` (the commits after the review head `7ca2662950`), against dual reviews r1 and r2.
Local runs used their own target dir, `CARGO_INCREMENTAL=0`. Each mutant was applied alone, then reverted with `git checkout`.

## Mutants

The rows run were every `split_gate` row in topo and sweep, plus the unit rows for `torus_rect_extent`, `zone_extent`, the aimed spiric reach, the per-coordinate extents and the frame rounding. That is 16 rows, all at ε 1e-9.

| mutant | edit | rows red | lane claim |
|---|---|---|---|
| M3 | `most_cos`: `crest - w.lo` → `crest - w.hi` | sweep `reach_split_gate_window::a_cut_into_the_faces_window_refuses_at_the_gate`, `…::a_cut_clear_of_the_faces_window_splits`; topo `the_torus_rect_extent_is_the_rectangles_support_along_every_direction` | killed ✔ (e2e now) |
| M4 | `torus_rect_extent`: `most_cos(a.atan2(m), v)` → `…, u)` | the same three rows | killed ✔ (e2e now) |
| M4b | `zone_extent` result narrowed by `1e-6·r` at each end | sweep `a_cut_into_the_faces_window_refuses_at_the_gate`: the capped cylinder at 1e-7 in, upright, gives `Ok(whole)`; topo `the_zone_extent_is_the_zones_support_along_every_direction` | killed ✔ (e2e now) |
| M6 | `census.rs` spiric arm: `let axis = frame.vector(*axis)` → `*axis` | topo `boxes::tests::the_spiric_edge_reach_holds_the_curve_in_an_aimed_frame` (only) | killed ✔ |
| pad×0 | `sweep_pad(band) * 0.0` | topo::all `split_gate_per_face::the_box_is_read_with_its_pad` (only) | killed ✔ |
| pad dropped | `- pad` removed from `gap` | the same row (only) | killed ✔ |
| zone √ revert | `at = h·a + s·√(r² − h²)` instead of the stored ρ | **none: 16/16 green** | asked to report; not a blocker |
| extra X1 | `ball_extent`: `x: center.y.widen(r)` | `every_extent_reads_each_coordinate_from_that_coordinate_alone` | (my check) |
| extra X2 | `ball_extent`: `y: center.x.widen(r)` | **none** | (my check, see claims) |

## ε

| ε | the 16 new/changed rows | full `nextest -p topo -p sweep` |
|---|---|---|
| 1e-9 | 16/16 | **4246/4246** passed (176 skipped, the default profile's ignored rows) |
| 1e-6 | 16/16 | not run (not asked) |
| 1e-12 | 16/16 | not run (not asked) |

No red, so there was no comparison with `origin/main` to make.

## Claims

1. **`reach_split_gate_window.rs`: true.** Cuts go 1e-4·s and 1e-7·s past the least and greatest support. There are 6 fixtures: the complement band in full and at a 1.0 rad partial, the rounded cylinder at 2.0 rad, the capped cylinder, the truncated ball and the two-rim zone. Each takes 13 directions and 2 poses. Every cut refuses at the gate with the face's kind. Clear cuts (1e-4·s + 20ε) split, and each half is checked against a hand-written `(ρ, y)` grid. The closed-form support is cross-checked against a dense sample.
   - Caveat: the volume tolerance is `2e-3·V`. That cannot see a sliver, so soundness rests on the refusal row, not the volumes. The refusal row does carry it: it kills M3, M4 and M4b.
   - Caveat: the zone fixtures are full turns only. The partial-azimuth sphere case is the P1 item filed below.
2. **Mutant kills: true for all six.**
   - M3, M4 and M4b are now killed end-to-end, not only by the unit rows. This closes r1 MINOR-1 and r2 NOTE-1.
   - M6 and the pad mutants are killed by one row each.
3. **`zone_extent` reads each end from the stored `(h, ρ)` pair: true in the code, unsupported by any row.** Reverting to `√(r² − h²)` leaves every row green, so the `√(2rδ)` cancellation argument in the doc is unmeasured. This is not a blocker, as the brief says.
4. **"coordinate-i-from-i invariant enforced by a row over all seven extents": overstated.**
   - The row does cover all seven extents in `boxes.rs`: slab, cone_frustum, ball, torus, torus_window, conic and arc.
   - It only holds coordinate **0** fixed while 1 and 2 vary. A mutant that mixes `x` into `y` (X2) stays green.
   - The `BoxFrame` doc now says the row "holds each of them to it, bit for bit", which claims every coordinate.
   - Soundness is unaffected: the split gate reads only coordinate 0 of an aimed box (`reach_clears` reads `lo.x`/`hi.x`).
   - The fix is either to rotate the row's varying inputs through all three coordinates, or to word the doc as coordinate 0.
   - The gate's own `zone_extent` and `torus_rect_extent` are not in the row. They take scalars per coordinate, so they hold the invariant by signature.
5. **Items filed: true.** `work/reach/split-gate-zone-ignores-the-azimuth-window.md` is P1 and `split-gate-torus-ring-fallback-has-no-door.md` is P3. `work.py lint` is ok. The spiric item now cites `edge_reach_in`, which closes r1 NOTE-3 and r2's Q4.
6. **Review style points:**
   - Addressed, by reading the diff:
     - `plane_offset` replaces the hand-spelled offset.
     - There is now one window composition (`census::torus_chart_window`).
     - The gate's frame is passed to `edge_clears`.
     - The rule box is computed only when no patch reads.
     - The cylinder-clip comment states the any-frame argument.
     - The pose row's promised flank cuts (±z at 1.1) were added.
     - The frame rounding is measured (`the_aimed_frames_rounding_is_a_few_ulps_of_the_point`, out to 1e8 m).
   - Still open, both NOTEs: `insert_crossings` still builds `BoxFrame::aimed` per edge, and the extents' fold into `boxes` stays scheduled.

## Verdict: **VERIFIED**

Every listed mutant is red on the rows the lane names, and none that it claimed killed stays green. The ε rows and the 1e-9 suites are green.

Two non-blocking points:
- (a) The per-coordinate row and the `BoxFrame` doc overstate their reach: only coordinate 0 is pinned.
- (b) No row sees the rim-pair reading in `zone_extent`.

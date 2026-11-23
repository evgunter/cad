Report part 2 of 2.

**Runs.**
- mesh whole, `--ignore-default-filter` (slow set and split_rulings included): 510/510 at 1e-9, 1e-6 and 1e-12, on 5b9d62fbc7.
- The one commit after that adds a route_rows row; route_rows ran at all three ε on 650e2ce14e.
- Green on 650e2ce14e: `run-gates.sh`, `work.py lint`, `doc-gate`, and clippy `-D warnings` on mesh `--all-targets`, geom-brep `--all-targets` and viewer `--features app --all-targets`.

**Mutants**, each on mesh whole with the slow set:

| mutant | red | caught by (named rows) |
|---|---|---|
| a rectangle sent to the chart lane | 39 | D9 ×2, split_rulings `a_routed_face_is_chorded_for_the_chart_lane` and `a_ruled_face_with_a_split_ruling_meshes`, the split-seam donut, the slit rows, … |
| the branch door skipped | 6 | the half cap ×2 rows, the hemisphere pair, the K row, D9 ×2 |
| the image rule removed | 5 | the tilted cells, the K row, the oblique lens, D9 ×2 |
| main's "route removed" (fact 4 off) | 8 | T1 `reach_probes::a_cone_face_met_on_its_axis_meshes_at_every_delta`, split_rulings ×4, `curved::tests` ×2, the K row |
| main's "door confirmation dropped" (split read before the doors) | — | see below |

- In the first mutant, one row (`sizing_theorem_rows::a_worst_triangle_at_delta_to_the_last_bit…`) hung, and I killed it after 13 minutes. The mesh suite has no timeout.
- The "door confirmation dropped" mutant first reddened only the K row, and only by record order. Its real effect was unpinned: a split face whose door cannot decide would be meshed on the chart lane. New row: `route_rows::a_split_side_is_not_read_before_the_doors_answer`. It is the in-band washer wall with its seam ruling split, and it must refuse `Escalated props_rim_fit`. It is green at all three ε and red under the mutant (`DomainUnread` instead).

**Main moved again.** origin/main b8136c1649 includes PR 7052 (sectarms/periodic-loop-carrier), which conflicts with this branch in crates/geom-brep/src/props/curved.rs. It also touches mesh chords.rs, memo.rs, sizing.rs and reach_probes. Not merged; say go and I will merge it.

**Changed PR-body paragraphs.**
1. "The route": replace "by the first of three facts" with "by the first of four facts", and add after fact 3:
   "4. A rectangle the doors certify on a ruled chart (cylinder or cone) whose straight side holds a point — a ruling split into several edges, or a cone side through its apex (`walk::splits_a_straight_side`, PR 7049's test, read here once and only of a certified rectangle) → the trimmed lane, its domain read at the face's turn. The doors' `Rectangle` goes unspent, so the curved lane still takes nothing the doors did not certify, and is handed only ruled rectangles with bare sides; its guard (`SIDE_CHORD_POINT`) refuses one that reaches it otherwise. PR 7049's `lane_of(.., band)` and the `SizingTols` it threaded through the chord pass are subsumed by `Routes`."
2. "Byte and K-stream evidence", the last bullet:
   "Both rows' bytes were read on `9c47bfbe57` at ε 1e-9, 1e-6 and 1e-12 and agree with this head at all three. The K stream is `326c40c3f0`'s plus the split-side fact's one read per ruled face per run (16, 8 and 6 chart reads on the rounded prism, washer and cone), the memo hit's unchanged; main after PR 7049 reads the same fact at each dispatch call, about nine a face."
3. "No face that meshes on main changes lane", a new bullet:
   "**Split-side faces (PR 7049):** every `tessellate` answer of `split_rulings`, the T1 row and sweep's inflection-ruling module digests to main `9c47bfbe57`'s at the three ε (≈300 mesh and 366 sweep answers), except the notched wall, which is no rectangle and meshes here where main refuses it at the shape door. Armed against `9c47`'s `lane_of` over the mesh suite: no panic on a face main meshes."
4. Mutants table: add the two rows above (fact 4 off: 8; split read before the doors: `route_rows::a_split_side_is_not_read_before_the_doors_answer`).

**Next, detached:** the viewer's slow-set rows (`display_budget`, `review_pick_r2`), sweep's meshing modules, then the armed sweep and the tour. Results to follow. I am pushing nothing more without your go.

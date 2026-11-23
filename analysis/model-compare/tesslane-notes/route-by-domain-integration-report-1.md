PR 7049 merge: pushed 650e2ce14e2c8c1619b0311b104c35c92881442c (merge 5b9d62fbc7 of origin/main 9c47bfbe57, plus one row commit). `git ls-remote` confirms it. Report part 1 of 2.

**Integration, by file.** Your hypothesis holds: the split-side test is the route's fourth fact, read after the doors. Main's dispatch plumbing is removed as subsumed by `Routes`.
- tessellate.rs:
  - Mine: `Routes`, `route_of` and the fold.
  - Main's `lane_of(.., band)` is gone, and so is its minting of `SizingTols` before the chord pass so that the pass could re-ask the dispatch. `Routes::of` is still the one reading per face.
  - New fact 4, in `route_of`'s `Shape::Rectangle` arm: if `walk::splits_a_straight_side(body, fk, surface)`, the route is `Route::Trimmed(None)`. The domain is then read at the face's turn, as for a ringed face, which is main's lane answer.
  - Docs updated: `Lane::Trimmed` and `route_of` (four facts).
- What the token means for such a face: `Rectangle` still means "the doors certified a rectangle", and the curved lane is still reachable only through it. For a split-side face the route drops the token unspent. So the lane is handed only ruled rectangles with bare sides, and main's lane guard (`curved::inside_a_side` → `UnsupportedCurve { SIDE_CHORD_POINT }`) stays as the defect check for one that reaches it otherwise.
- walk.rs: main's `splits_a_straight_side`, verbatim except two removals:
  - its `band` parameter;
  - its `require_iso_rectangle_face(..).is_ok()` confirmation. The route asks the split test only of a face the doors certified (`iso_rectangle_shape`, asked once), so the confirmation is structural; that is your "asked once".
- curved.rs:
  - Main's side kept whole: the fallback lattice and `bare_sides` deleted; `grid_counts`' cone arm `(nu, 1)`; the `inside_a_side` guard and `SIDE_CHORD_POINT`; the re-pins (`TOTAL_MESHED` 250, `TOTAL_REFUSED` 0, the wedge's 40).
  - Not taken: main making `require_iso_rectangle_face` `pub(crate)`. It is `iso_rectangle_shape` here.
  - Main's `a_ruled_face_with_a_point_inside_a_side_refuses_here` is adapted: it gets its chords from `Routes::witness` and spends a token from `iso_rectangle_shape` by hand. It still asserts 2 refusals.
- chords.rs: mine whole. Main's diff there was only the `SizingTols`/`lane_of` plumbing.
- trimmed.rs:
  - Kept from main: `singular_row_shape(.., developed)`, so no `SeamTwice` in the development.
  - Mine: the `face_target`/`arm_steps` signatures.
  - Main's `lane_answers` test helper reads `Routes`.
- sizing.rs: my band doc; main's `tests::witness_tols` kept (main's curved row uses it).
- nextest.toml: main's line, with my notched rows' names.
- split_rulings.rs: every row kept, one re-pinned. `a_notched_wall_with_a_split_ruling_refuses_at_the_shape_door` becomes `…_answers_as_the_unsplit_wall`:
  - want `Meshed`, because the unsplit notched wall meshes on this branch (unit 4);
  - the split body is now read with `got_enclosing` against the unsplit solid, because a split body is not finished;
  - the row's invariant (every split cell answers as the unsplit wall does) is unchanged.

**What moved.**
1. Bytes. route_rows' 8 tilted cells and its 9-body corpus are equal on 9c47bfbe57, 326c40c3f0 and this head, at 1e-9, 1e-6 and 1e-12. PR 7049 moved no byte.
2. Split-side faces mesh to main's bytes. With a temporary, uncommitted hook printing a digest of every `tessellate` answer, I ran split_rulings (7 rows), the T1 row and sweep's `a_box_along_a_fillets_line_arc_junction` (4 rows, the inflection ruling included) on 9c47 and on this head, at all three ε:
   - 297, 296 and 300 mesh answers and 366 sweep answers. Every one is identical, refusals included, except one row: the notched wall row.
   - That wall is no rectangle. Main refuses it `NotIsoRectangle props_rim_level`; here it meshes on the chart lane, which is unit 4's change and not the split-side class.
   - No split-side face moves a byte, changes lane or refuses against main.
3. K stream (route_rows' K row). PR 7049 moved main's own stream on the three ruled bodies (rounded prism, washer, cone), on a run and on a memo hit.
   - Cause: main's `lane_of` runs the split test (`Chart::v_of` reads `chart_point_on_axis`, `pcurve_chart_cut_side` and, on the cone, `chart_point_cone_nappe`) at every dispatch call, about nine per face (prism +144 reads = 16 × 9).
   - This head = 326c40c3f0's stream plus exactly one split read per ruled face per run (+16 prism, +8 washer, +6 cone; every other decision identical). The memo hit is 326c's unchanged, because the route's reads are spliced only where the lane runs.
   - I re-pinned the three run digests, with that reason in the row's doc. The row now equals main minus main's repeated reading, not main's 9c47 stream.
4. Armed differential: main 9c47's `lane_of` (rings, trim carriers, a tilted sphere circle, a split side the door admits) against `route_of`, over the mesh suite, 510 rows:
   - No face main sends to the trimmed lane is routed elsewhere, except two faces where both refuse `DomainUnread` (`Gap`): main in its lane, here in the route, the same error. The armed assert reds those two rows only because it fires; unarmed they pass.
   - No face that main's doors pass is routed off the curved lane.
   - 151 faces moved from main's curved lane to the trimmed lane, every one door-refused on main (16 of them split-and-notched cylinders).

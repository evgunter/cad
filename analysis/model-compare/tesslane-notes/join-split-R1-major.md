REVIEWER lane, PR 7573 @ 24ee9a28fb — MAJOR (test lane, claim 10), DEMONSTRATED. The code is right by my reading; what is false is the PR table's row for it.

**No row holds that `join::split` reads its pieces.** With the call to `held` removed from `join::split` (so no split piece is read on its certificate or for a fold, on any lane), the WHOLE `mesh` suite with the slow set passes at the default ε: 569 of 569.

The PR's table: "The pieces are held as the lane holds a triangle | `join::tests::a_split_piece_refuses_the_face_as_its_lane_would` | red when the verdict is skipped". That row calls `held` directly with hand-built pieces; it cannot see whether `split` calls `held`, with which pieces, with which chart corners, or with `folds` set. The unit's item 2 ("each half is held to that face's certificate ... one that fails refuses the face, typed") and the brief's claim 3 are the mechanism, and nothing in the suite goes red when it is absent. `[tolerance.chordal]` for every split piece therefore rests on a call no test observes.

Source. Tree: the frozen head, with this one edit in `crates/mesh/src/join.rs` at the end of `split` (an env switch so one build serves every mutant; `rv` is `std::env::var("RV_MUT").is_ok_and(|v| v == name)`):

    if rv("no_held") || (rv("no_held_rect") && !folds) || (rv("no_held_folds") && folds) {
        return Ok(());
    }
    held(face, made, &row, &outward, run)

Command: `RV_MUT=no_held cargo nextest run -p mesh --profile ci --ignore-default-filter --no-fail-fast`
Raw: `Summary [ 720.052s] 569 tests run: 569 passed (12 slow), 4 skipped`
(569 = the suite's 564 + 5 probe rows of mine, which assert nothing about this.)

Same switch per lane, on the 73 rows nearest the unit (`join::tests`, `memo::tests::a_patch_split…`, `trimmed::tests::a_side_two_faces…`, `tessellate::tests`, `split_rulings`, `d9_mesh_goldens`, `faces_that_bound_no_region`, `patch_memo`, `route_rows`, `collapsed_rows`, the two fold rows of `reach_probes`): `no_held_rect` (rectangle and spline lanes' pieces unread) 73/73 pass; `no_held_folds` (trimmed analytic and walked lanes' pieces unread) 73/73 pass; and a third switch, `no_fold` (the pieces' `outward` answers `None`, so no piece is read for a fold, certificate still read) 73/73 pass. Its whole-suite run is in progress.

For contrast, the other mutants on the same 73 rows, same build: census splits nothing: 7 red. Floor off: 1 red (the shell row). Split at the straight midpoint: 2 red (shell row's volume, −3.9e-16; the ε digest row). Split in all but the first patch (storage order): 4 red. Holders not read: 1 red, the unit census row only. Development off, apex rule off: 1 red each, the unit `midpoint` row only. `unstore` not called: 1 red (`memo::tests`). Fold budget 0: 2 red (`reach_probes`). So the table's other "red when" cells hold; this one does not.

Why no body-level row can see it today: the PR's own statement that no piece refuses on any body measured. A row that would hold it needs a split whose piece fails, which the suite has no natural body for; I am running the suite under a forced census (every inside side split, whether or not a second patch makes it) to get such sides and will report what `split` does with them.

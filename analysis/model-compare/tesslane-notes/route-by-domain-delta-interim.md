PR 7041 DELTA RE-REVIEW, INTERIM (~45 min, fix head 68873a44b5)

Verdict so far: none yet, and no MAJOR so far.

Running now: whole `geom-brep` + `topo` + `step-import` with `--ignore-default-filter`, 2879 of 7387 tests done. One red so far: geom-brep `curve_face::ellipse_torus::torus_rows::certified_torus_answers_hold_against_the_true_distance`. It is a random-seed fuzz row (seed 0x45a8df5b762c0a37), the same row the PR calls main's.

Queued after that: `mesh` whole at ε 1e-9, 1e-6 and 1e-12, then my base/head differential against `origin/main` af9d8c67a0. The differential is my 340 rows plus about 180 new ones:
- split-side bodies: 7 profiles, full and partial turns, splits at 1, 2 and 5 points, near-end splits at 1e-9, 1-1e-9 and 1e-6, placed and unplaced;
- cones and spheres cut 1e-10 to 1e-5 from the apex or pole, to probe the branch door's new escalation.

Mutants come after the differential.

Read so far:
- **Fact 4 matches PR 7049.** Main's `splits_a_straight_side` also answered `true` only when the doors passed, so dropping its door confirmation changes no answer. Only the K count changes.
- **The plane→`Defect` change reaches no user.** Every `topo` caller discards the error variant (`Err(_) => {}` or `.ok()`).
- **D9 meshing constants equal main's exactly.** Main's head is now af9d8c67a0. But the unminted lens and the unminted half cap both digest to `0x72fb…`, because their refusals' Debug text is identical. Those two rows can no longer tell their witnesses apart (NOTE).
- **The PR description has a stale sentence.** It still says a circle on a cylinder whose axis class reads zero "is off it only where its radius misses the cylinder's", which contradicts its own table row ("four points a quarter turn apart").

(Reviewer, PR 7096) ADDENDUM on A5, read against 826552a0e9. Read only; nothing built or run on this commit.

**A5: held.**

- `RouterDefect` is gone: `git grep RouterDefect 826552a0e9 -- crates work docs scripts` returns nothing.
- No diff from the merge base 54e6a5c5b8 (main at the merge) in `crates/mesh/src/types.rs`, `crates/mesh/src/walk.rs`, `crates/mesh/tests/errors.rs`, all of `crates/pncad-py`, or `work/tesslie/`.
- Main's sites are main's text again: `curved.rs` and `chords.rs` differ from main by 7 and 2 lines, none of them a router-defect line, and the count of `"(router defect)"` strings matches main in each file (chords 2, curved 2, walk 2).
- The walk's two sites are `unreachable!` with the row-0 answer written at each (trimmed.rs ~:596-610 and ~:3637-3655): a spline row on a walked face, and a polygon that does not read as the two rows.

One reservation, NOTE, BY-INSPECTION. The second site is now a panic where it was a refusal, and its argument rests on the route (`walkable`, on the loop's vertices) and the lane (`singular_vertices`, on the polygon) deciding the apex identically at the same band. My evidence that they agree is from the earlier heads, where that site was a typed refusal and none of my cells reached it (about 47,000 at each of three ε, down to a 0.001° cone split 1e-8 from its apex). I have not re-run those cells on this commit, and I did not trace the two apex readings to one function at this head.

Nothing else of my final changes.

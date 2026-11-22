ADDENDUM (PR 7096, claim A5 at 826552a0e9): HELD. I checked it by reading only; I ran no rows on 826552a0e9.
- `git grep RouterDefect|router_defect 826552a0e9` finds nothing.
- Against main 54e6a5c5b8, these have zero diff: types.rs, walk.rs, tests/errors.rs, all of crates/pncad-py, work/tesslie.
- The router-defect sites in chords.rs (551, 612), curved.rs (181, 646) and walk.rs (268, 271) carry main's "(router defect)" text. chords.rs's only diff from main is the `Lane::Walked` arm.
- trimmed.rs: the walk's two sites are `unreachable!`, each with its row-0 answer and a row-4 message that gives the face and the polygon's counts. I accept both row-0 answers: the spline row is a run-time match shared by every arm, and the polygon does not exist when the route runs.
- No change to my final's verdict (APPROVE-WITH-FIXES).

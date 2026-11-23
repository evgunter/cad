INTERIM — PR 7043 delta at 6f41f1b0ef. Still running.

Verdict so far: **APPROVE-WITH-FIXES**, with one MAJOR (demonstrated):
- The two viewer slow-set rows are **red on 6f41f1b0ef and green on origin/main (cdee746161)**. I built both trees and ran just those two tests, `--ignore-default-filter`.
  - `review_pick_r2::the_certified_determinant…`: (451512, 147856, 16596, 8142) against the pinned (451800, 147952, 16824, 8244).
  - `display_budget::the_budget_commits_the_delta_it_always_has`: only `cut_cylinder` moves (probe_triangles 7848→7816 at 1e-4, 84640→84504 at 5e-6). That's an analytic trimmed cylinder, NOT a spline face, so the both-bands rule is not the candidate; the clearance itself is.
- Hosted CI can't see it: both rows are in `.config/nextest.toml`'s slow-set filter, which PR CI skips unless the diff spells the test name. They would go red on the first nightly after merge.
- The frozen head b6993486b8 is building now, to say whether the first pass already moved them (likely) or the fix pass did.

Also so far:
- `cargo nextest run -p mesh` whole at 6f41f1b0ef: 497/497.
- Correction to my first report: I called the nine cells' refusing cone face a trimmed-lane reach of `UnresolvedWinding`. The body's rectangle-lane account fits my own evidence: treating unresolved as a fold in `emit` changed nothing.

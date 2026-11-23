tesslane/collapsed-side — STAGE 2 REPORT (part 1 of 2)

HEAD a87d047b5aaf3c3b93da1e2b907d07136dd0fa28 (confirmed with git ls-remote). It is a merge of main, which has no change in crates/mesh; `cargo check -p mesh --tests` is green after the merge.

WHAT CHANGED, ALL IN ONE COMMIT faf84d1564 (no trailers, blinded):
- crates/mesh/src/trimmed.rs
  - `face_polygon`: the face's singular vertices and its UV polygon, built in one place for the lane and the rows.
  - `CollapsedSides`: the spline chart's sides, read from `singular_points`; `holds` is exact.
  - `SideWalk`: walks each collapsed side between its two copies at ⌈span/h⌉ pieces.
  - `singular_vertices` now takes the decision as a closure (analytic: banded, as before; spline: exact).
  - `trim_polygon` takes `(singular, walk)`.
  - The emit and census comments now name the spline copies.
- crates/mesh/src/chart_table.rs: module docs (a spline chart's singular points are its collapsed sides, read by the lane from the home).
- crates/mesh/src/trimmed_collapsed_rows.rs (new): the rows.
- .config/nextest.toml: two slow rows (6.03 s and 0.57 s locally), with the timing comment. `check-slow-set-names` reports no dead entry. The budget-only row is left out: it exists only under a feature, and its CI row runs plain `cargo test`.
- work/tesslane/a-spline-vertex-on-two-collapsed-sides-keeps-its-polygons-copies-unwalked.md: filed (a vertex on two collapsed sides keeps its copies with no point walked between them; no door builds one). `work.py lint` passes.

RAN LOCALLY:
- the whole mesh suite at the default ε: 538/538 pass, all golden and digest rows included;
- the new rows at ε 1e-6 and 1e-12: 5/5 pass at each;
- the budget falsifier row with `--features budget`: passes;
- clippy -D warnings with and without `budget`, and fmt;
- the seven mutants below;
- base-against-head per-face digests (scratch probe, not committed).

LEFT TO CI: the whole mesh suite at ε 1e-6 and 1e-12, every other crate, and the nightly.

NOTHING SLOWED ME DOWN beyond one stray foreground job: `check-slow-set-names.py` reads stdin and hung without a listing. I stopped it and reran it with a listing.

The PR body follows in part 2.

# Lane `heldrecords/nary-lineage` — P1

Row: `work/heldrecords/the-nary-door-panics-on-an-accumulation-face-with-no-member-lineage.md` (read it in full on `origin/main`). Branch `heldrecords/nary-lineage` off `origin/main`; PR base `main`. Review tier: single FULL review.

The n-ary Boolean door (`crates/topo/src/boolean/nary.rs`) panics on an `unreachable!` ("accumulation face … has no member-face lineage") when the corpus document `die_composed_tour` is evaluated at f64 after one `SetVarValue` edit (eight edits listed in the row, all +30%). Input-reachable panics are fail-loud violations (DESIGN `[failure.taxonomy]`).

1. **Measure first** (`memories/refusal-text-is-not-cause.md`): reproduce each of the eight edits (the reviewer's probe is on `origin/boxfinish/review-split-reads` @ `ceba9d94e`, `crates/editor-core/tests/`, record `probe/review-split-reads/head/differential.txt`). Trace which accumulation face lacks lineage, which stage minted it, and why the invariant the `unreachable!` states does not hold. Write the trace as a dated section on the row before fixing.
2. **Decide whether the edited die is geometry a user would build** (is each edited value a valid part?); if yes, the row is P0 — say so in your report.
3. Fix where it starts: the lineage invariant held by construction (the stage that mints the face records its member lineage), or, if a face can legitimately have none, the door types that case. Never a panic, never a silent drop. Sweep `nary.rs` and its neighbours for other `unreachable!`/`expect` on the same invariant, and for the same shape of lineage gap in other doors; hit list in the PR body.
4. Pins: each of the eight edits as a row (body at a checked volume, or a typed refusal stated as what it is); a pin that goes red if the lineage gap returns (a mutant dropping the record).

Standing rules: `briefs/common.md` on this branch.

You are a REVIEWER lane for the BOXFINISH program of this repo (a B-rep CAD kernel in Rust). You are a delegated lane, NOT an orchestrator: `memories/orchestration-model.md` is not about you. You were dispatched by the BOXFINISH orchestrator (session `session_013QoYWYDafoXFXBkoDoWM5w`). You review ONE pull request, PR 7704 (`boxfinish/report-bound` → `main`), at the FROZEN head `830a04b5915b18159090f3b38b386f62505ec462` (after `git fetch origin boxfinish/report-bound`; if it does not resolve, tell me and stop). Check that commit out detached and review exactly it. You fix nothing on that branch, you do not merge, approve or comment on the PR, and you push only to your own branch `boxfinish/review-report-bound` (probe rows, if you write any), which this brief and the program's branch-prefix convention authorize.

This is a STYLE review: the change bounds what a diagnostic renderer writes and decides nothing in the kernel, so its correctness can largely be read. The style lane is the whole of the job, with the few things below held to the tree. A finding that rises to a correctness MAJOR is still reported as one.

## Read first, in full
- `CLAUDE.md`.
- `docs/prompts/reviewer-style-lane.md`: binding; your report names which of its questions you exercised and carries `sure` / `likely` / `unsure` on every finding.
- `memories/review-and-dependency-policy.md`, `memories/output-stability-as-justification.md`.
- `docs/prompts/implementer-discipline.md`: what the author was bound by.
- The unit: `work/boxfinish/a-boxed-rectangles-certification-runs-out-of-memory-with-the-box-arm-off.md`, whole file.
- The PR: its body and its diff against the merge base with `origin/main` (four files: `crates/geom-core/src/sym/report.rs`, a new test file, its line in `tests/all.rs`, one row filed on another program's slate). The body is the author's account and everything below is the dispatcher's: hypotheses to check against the tree. Correcting either is a finding in its own right.

## What the change is
The symbolic tier's shape report renders the form that blocked a decision as text. Its renderer limited nesting depth and nothing else, so a text was the product of the fan-out over the spelled levels, and one decision of a boxed rectangle's replay did not finish rendering within 5 GB. The change gives one decision's texts a stated budget (`RENDER_BUDGET`, split in thirds between the plain form, the early form and the explanation), writes through a writer that takes no byte past its limit (`Ink`, `Pen`), spells a form at the deepest depth that fits, and names what it leaves out.

## Hold these to the tree (briefly; then the style questions)
1. **The budget has one home and cannot be exceeded by construction**: no path builds a text and cuts it afterwards; no second constant restates the bound.
2. **Within the budget the text is main's, byte for byte** (the author: full depth is unchanged; 46 of 46 reader rows before and after; no pin moved). Is that shown by a row, or only by the readers happening not to move?
3. **The rendering is deterministic and a function of the form and the registered names alone** (an atom named by its node id: is a node id stable across runs, schedules and sessions, or does a report's text now vary where it did not?).
4. **The rows can fail**: two are red on main by a size; the third is "green on main by construction". What does the third pin, and what would make it red?
5. **The fixed thirds** (the author's deviation: a large plain form cannot starve the early form) against one pool: is it the better final state, and is it said at the constant?
6. **The sweep**: two unmemoized depth-limited walks in the ladder were filed on another program's slate and not fixed. Is the row accurate, on the right slate, and is the blind spot the body names the real one? Run your own differently shaped sweep over `crates/geom-core/src/sym/` for a walk or a `Display`/`Debug` that spells a form without a bound.
7. **What is still unbounded**: the author says a replay's report is bounded by decisions × the budget, not by a constant. Does that leave the unit's row (a row that could not run) closed, or does something owe a schedule?

Hosted CI: the runner queue is long, so say what the run on this head showed when you looked, at step level, and run yourself what a queued lint job would have told you (`cargo fmt --check`, clippy as CI spells it, `scripts/run-gates.sh`, the doc gate, `python3 scripts/work.py lint`) and `geom-core`'s tests. `geom-core` carries two inherited reds, both tree-wide source censuses (`bounds_census::every_sole_bracket_bound_door_is_in_the_roster`, `sym_rule_f_rows::the_copysign_mint_sites_the_tree_holds_are_these`): confirm they are not this diff's.

## How to work
- Your own target directory outside the checkout (`CARGO_TARGET_DIR=/home/user/review-reportbound-target`, `CARGO_INCREMENTAL=0`; CI's profile is `CARGO_PROFILE_DEV_OPT_LEVEL=1 CARGO_PROFILE_TEST_OPT_LEVEL=1` with `line-tables-only` debug) and your own scratch directory (`/home/user/review-reportbound-scratch/`), never the session scratchpad. One heavy build at a time; a job that would outlive a 600 s foreground call is launched `setsid`-detached to a log and polled in the foreground; never end your turn with a job running; do not poll hosted CI beyond one look.
- NO Co-Authored-By trailer in anything you commit; do not name yourself, your model or any model, or any lane, anywhere; no email addresses or personal identifiers.

## The report
ONE message to the orchestrator with the `send_message` tool of the claude-code-remote MCP server (`session_id: session_013QoYWYDafoXFXBkoDoWM5w`), ≤150 lines: the verdict first (APPROVE · APPROVE-WITH-FIXES · NOT-MERGEABLE-AS-IS · REJECT); MAJOR / MINOR / NOTE, each marked DEMONSTRATED or BY-INSPECTION with its confidence; the seven items above, each held or broken with what you ran or read; a `## Style` section (which questions you exercised; each finding with its place, two to five sentences and a confidence; no proposed fixes; a long tail is expected); the rubric (idiom and structure, test quality, doc and comment honesty, each 1 to 5 with one line of evidence); spec deviations, reported and SILENT; what you executed and what you only read.

# BUDGET — the plan

the tessellation budget instrument: what tess-lint reads, what it compares, and what its figures are worth

Opened 2026-09-20 by INSTR's priority-seam cut
(`work/README.md`, Track size). Nothing dispatched.

## The slate

**23 budget points** of dispatchable work against a ceiling of 30.

| pri | item | cost | title |
|---|---|---|---|
| P3 | `cut-line-commit-names-no-baseline-change` | D | The tess-budget cut line stamps the sweeping tree's HEAD, so the commit it names need never have touched the baseline |
| P3 | `gate-findings-name-no-columns-and-recoverable-has-two-aggregations` | D | the gate's finding lines render figures through a helper and name no columns; Row::recoverable and SceneTotals::recoverable are one name over two aggregations |
| P3 | `no-guard-reds-on-an-unlabelled-figure-in-the-budget-doc` | E | Nothing reds when an unlabelled current figure arrives in docs/TESS-BUDGET.md, and the fix that disclosed the hole walked into it |
| P3 | `pin-table-has-no-whitespace-tail-row` | E | cut_line_pin's TABLE has no row whose date carries a non-space whitespace tail |
| P3 | `tess-budget-doc-identity-column-list` | D | docs/TESS-BUDGET.md's two identity-column enumerations still name the sizing-block entry tess-lint no longer carries |
| P3 | `tess-budget-doc-note-finding-rule` | E | the note-vs-finding rule has a fourth home in docs/TESS-BUDGET.md, carrying only its short form |
| P3 | `tess-lint-growth-margin-unprotected-from-ceil-quantisation` | D | The gate's growth margin is unprotected from the split scan's ceil quantisation — D206 closed only the continuous half |
| P3 | `tess-lint-re-cut-folds-uncompared-columns` | D | A re-cut folds every column the gate reads but compares against nothing, and the fold has no diff |
| P3 | `tess-lint-recourse-quote-half-pinned` | E | the recourse quote is pinned from the binary's side only, and docs/TESS-BUDGET.md is the unpinned half |
| P3 | `tess-lint-zero-certificate-two-meanings` | E | tess-lint admits worst_cert = 0 as a reading, and the kernel says it has two meanings |
| P3 | `tess-meter-sampled-retune-figure-unreproducible` | E | tess-meter's 21-sample retune figure names a draw nothing records, and its reference moved with SPLIT_SCAN_SAMPLES |
| P3 | `tools-assert-sweep-blind-spots-unscheduled` | D | Two blind spots in the tools-wide cannot-fail assert sweep, and nothing schedules them |
| P4 | `report-constraint-activity-line-names-no-columns` | E | tess-lint's constraint-activity line prints four indicator columns under prose names |
| P4 | `tools-doc-prose-cites-thirteen-dead-work-meter-paths` | E | tools/ doc prose cites thirteen work/meter/ paths that resolve nowhere, two of them in the list whose own sentence claims its pointers followed |

## Order

`tess-lint-zero-certificate-two-meanings` first: `worst_cert = 0`
read as a certified zero where the kernel means "no certificate" is
the one row here that makes the instrument report a FACT that is not
true, rather than failing to report one.

Then `tess-lint-re-cut-folds-uncompared-columns`, which is the same
shape at the gate level — a column that can move with nothing saying
so. The doc rows are class `E` and are drive-bys for whoever edits
`docs/TESS-BUDGET.md` next.

## Review posture

OPEN, for this program's first dispatch. INSTR inherits protocol v7
(`docs/MODEL-AB-LOG.md`, Ev 2026-09-19): the dual on triaged-in units
only, opus/opus outside it. Nobody has re-asked the triage question for
this slate, so the first orchestrator answers it here rather than
inheriting an answer.

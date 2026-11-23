# BUDGET — the plan

the tessellation budget instrument: what tess-lint reads, what it compares, and what its figures are worth

Opened 2026-09-20 by INSTR's priority-seam cut
(`work/README.md`, Track size). Nothing dispatched.

## The slate

**26 budget points** of dispatchable work against a ceiling of 30
(re-counted 2026-10-26 in the tracker sweep). Live rows only.

| pri | item | cost | status | title |
|---|---|---|---|---|
| P3 | `cut-line-commit-names-no-baseline-change` | D | open | The tess-budget cut line stamps the sweeping tree's HEAD, so the commit it names need never have touched the baseline |
| P3 | `face-measure-carries-no-lattice-metric` | E | open | FaceMeasure and the budget CSV carry no lattice metric, so no band's residual aspect over it is recomputable or guarded |
| P3 | `gate-findings-name-no-columns-and-recoverable-has-two-aggregations` | D | open | the gate's finding lines render figures through a helper and name no columns; Row::recoverable and SceneTotals::recoverable are one name over two aggregations |
| P3 | `no-guard-reds-on-an-unlabelled-figure-in-the-budget-doc` | E | open | Nothing reds when an unlabelled current figure arrives in docs/TESS-BUDGET.md, and the fix that disclosed the hole walked into it |
| P3 | `pin-table-has-no-whitespace-tail-row` | E | open | cut_line_pin's TABLE has no row whose date carries a non-space whitespace tail |
| P3 | `row-raised-bands-cost-grows-with-the-band-spread` | E | open | Rows raised for narrow-column bands cost a face in proportion to its band-aspect spread: up to 14x triangles on an uneven-station cylinder at e = 1e-4 |
| P3 | `snap-bands-column-reads-zero-since-the-snap-retired` | E | open | The budget CSV's snap_bands column reads 0 since the column snap retired; drop it at the next baseline re-cut |
| P3 | `tess-budget-doc-note-finding-rule` | E | open | the note-vs-finding rule has a fourth home in docs/TESS-BUDGET.md, carrying only its short form |
| P3 | `tess-lint-gate-red-on-main-23-findings` | M | open | the nightly k-lint (release-budget) tessellation-budget gate is red on main with 23 findings against the 2026-10-02 baseline, and nothing has read them |
| P3 | `tess-lint-growth-margin-unprotected-from-ceil-quantisation` | D | open | The gate's growth margin is unprotected from the split scan's ceil quantisation — D206 closed only the continuous half |
| P3 | `tess-lint-re-cut-folds-uncompared-columns` | D | open | A re-cut folds every column the gate reads but compares against nothing, and the fold has no diff |
| P3 | `tess-lint-recourse-quote-half-pinned` | E | open | the recourse quote is pinned from the binary's side only, and docs/TESS-BUDGET.md is the unpinned half |
| P3 | `tess-lint-zero-certificate-two-meanings` | E | open | tess-lint admits worst_cert = 0 as a reading, and the kernel says it has two meanings |
| P3 | `tess-meter-sampled-retune-figure-unreproducible` | E | open | tess-meter's 21-sample retune figure names a draw nothing records, and its reference moved with SPLIT_SCAN_SAMPLES |
| P3 | `tools-assert-sweep-blind-spots-unscheduled` | D | open | Two blind spots in the tools-wide cannot-fail assert sweep, and nothing schedules them |
| P3 | `uneven-station-lofts-cost-triangles-without-the-column-snap` | E | open | Uneven-station lofts cost up to 14% triangles at degree 2, and 62% on a degree-3 loft, without the retired column snap; a cost lever if a real body needs it |
| P4 | `report-constraint-activity-line-names-no-columns` | E | open | tess-lint's constraint-activity line prints four indicator columns under prose names |

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

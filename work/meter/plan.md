# METER — the budget and K instruments (plan)

**STATUS: OPEN (2026-09-06).** Opened in the tracker-wide cut of
2026-09-06 (`docs/WORK-TRACKS-2026-09.md`, addendum 2), claiming
code-quality Track K's `tools/*` half. Live state is
`work/meter/log.md`'s tail and the item files beside this plan, never
this file.

Branch prefix (the #396 convention): **`meter/`** — unit branches
`meter/<unit>-<slug>`, orchestrator branch `meter/orchestrator`.
Away-channel tag `(METER orchestrator)`. A/B ordinal band
**METER = 3200–3299**, claimed in `docs/MODEL-AB-LOG.md`'s banding
entry in the opening commit; infra-only units record no row, so the
band is claimed for bookkeeping.

## Charter

The instruments MEASURE and REPORT; they never gate on a millisecond
and never justify a mesh by its own size
(`memories/tessellation-budget.md`, `memories/perf-measurement-lane.md`).
What this program fixes is where an instrument's claim and its
arithmetic have parted: a join on an allocation ordinal that a face
reorder silently breaks, a scan whose resolution is coarser than the
tolerance it reports against, a doc block transcribed from a sweep it
no longer describes, and rosters copied from the kernel's vocabulary
with no pin. The standing rule from `baseline_census.rs`: **a census
has one executable home and every other site points at it.**

## Review posture

The S-TCOST posture, confirmed and sharpened by Ev in chat
(2026-09-07): **no A/B protocol on this program** — no draws, no arms,
no `docs/MODEL-AB-LOG.md` rows. The band stays claimed for bookkeeping
only, which is what `program.md` already says it is for.

**One style review per unit is the default**
(`docs/prompts/reviewer-style-lane.md`, dispatched per
`docs/REVIEW-STYLE-DISPATCH.md`). A full falsification review is
reserved for the units where a wrong answer is expensive rather than
merely wrong, and on this slate that means the two that move committed
measurements other lanes read:

- `D206` — the constant change is one line and its consequence is
  every committed budget number. The claim to falsify is that
  `SPLIT_SCAN_SAMPLES = 379` actually puts the one-sided envelope under
  the gate's margin on the `ceil`'d column the gate READS, rather than
  on the continuous objective `D105` bounded.
- `k-report-baseline-fold-cert1-roster` — the re-derivation is
  mechanical and its READING is not. `props_meridian_pole`'s in-band
  population is benign by construction, and a baseline that records it
  as a landing corrupts K attribution.

Everything else takes the style lane alone. A re-cut of committed
budget data is a PROPS coordination, not a lane's call.

## Unit order

0. `tess-lint-face-ordinal-join` — **the cheap tripwire has landed and
   the item does not know it.** `Kind::Rekeyed` with `Rekey::Absent` /
   `Rekey::Column`, under `compare`'s rule-4 precondition, closes both
   branches the item names: the wrong-face compare and the silent drop.
   The item's body predates the fix (it is #746's text, 2026-08-20) and
   `C15` and `D201` both already say the mis-join is closed. What the
   unit owes is the verification and the one live question the closure
   raises — an ordinal permutation on a scene with NO Hessian-sized
   face lands in `notes` rather than `findings` (`compare`'s `gated`
   split), which is exactly #738's `diefillet` case, so the ordinals
   the item found permuted are today reported in the quietest voice the
   lint has. Decide whether that is the right voice and record it.
1. `D213` + `D214` — one lane in `tess-lint`: the `Row::nurbs` doc and
   `IDENTITY_COLUMNS`' reason corrected; the missing sizing block
   refused at parse rather than read as `Absent`.
2. `tess-budget-doc-finding-block-stale` — the finding block re-derived
   from the committed baseline (or `tess-lint`'s report header cited),
   with which sweep and when.
3. `D203` — the cross-column invariant rule written once and cited from
   both instrument sites.
4. `cut-prefix-three-unpinned-spellings` — the cut-line prefix pinned
   in one direction (the `CHART_TAGS` precedent); the CIW seam drawn
   in the PR.
5. `k-lint-predicate-roster-unpinned` — the `EPS_COUPLED_PREDICATES`
   roster pinned to the kernel's minted names in one direction; the
   PROPS seam drawn in the PR.
6. **`D206` + `D201` — ONE re-cut, not two.** `D206` raises
   `SPLIT_SCAN_SAMPLES`; `D201` (ruled arm A by Ev, 2026-09-07) adds
   the per-face `StableName` column. Both force a re-cut of
   `docs/tess-budget-data/tess-budget-baseline.csv`, each a full
   release sweep over every tour scene and each its own PROPS
   coordination — so they land together, separable in the diff, with
   the sweep taken once after both are in. Note the column is NOT
   additive: `tools/tess-lint` pins column positions
   (`IDENTITY_FIRST`, `SIZING_FIRST`, `DEV_SAMPLES`,
   `INDICATOR_FIRST`) against `EXPECTED_HEADER`, so inserting one
   moves the blocks the parser polices, and both census tests
   re-derive. `D206` carries the program's full falsification review;
   `D201`'s half takes the style lane, and Ev's *"it's a demo so it
   doesn't matter much"* is a scoping instruction — an honestly ABSENT
   name on a scene that cannot hand over an evaluation is an
   acceptable outcome, not a reason to widen the lane into `demos/`.
7. `k-report-baseline-fold-cert1-roster` — the next K-REPORT baseline
   re-derivation, folding `props_meridian_pole` and the re-shaped
   sphere rim margins; a runbook pass.
8. `C15` — **NOT discharged by reading the column, and the re-plan is
   the unit** (log, 2026-09-08). Unit 6 landed `D201`'s column and
   **0 of the 64 sized rows carry a name**, the 14 rows of the seven
   indistinguishable pairs among them: coverage and defect are
   disjoint. What the unit owes instead is what an undetected swap
   COSTS, given an executable home. The reading, taken here and
   checked against the rules rather than the module docs: rule 1 is
   per-SCENE triangle totals, so a within-scene swap cannot move it —
   a theorem; rule 2 compares `recoverable()` = `grid_cells /
   span_opt_cells` and both are bit-identical within all seven pairs
   on the committed baseline — a READING, which is why it needs a
   home; and `worst_dev` is reported, not gated (the gate emits
   `Vanished`/`Triangles`/`Uncovered`/`Slack`/`Rekeyed` and none reads
   it), so the swap moves no finding at all. **The report-side half of
   this row was FALSE and is corrected** (log, 2026-09-08): a swap
   moves nothing the report PRINTS either — verified by swapping each
   pair in the committed CSV and diffing `tess-lint --top 100`, all
   seven identical. `main.rs` prints `SceneTotals::total_slack`
   (triangle-weighted), not `Row::total_slack`'s `delta / worst_dev`,
   and `worst_cert` is printed by nothing. What an undetected swap
   costs is a wrong-face attribution in a column nothing reads. The
   gate-invisibility goes in `baseline_census.rs` beside the pair
   census, so a re-cut making a swap GATE-visible fails and names what
   moved, and a scene becoming document-built fires and says `C15`
   became dischargeable. `C15` stays OPEN: the
   defect is live, what closes it is corpus-side, and Ev's second
   clause on `D201` scopes that as not urgent rather than as done.
   Unit 0 took the tripwire half that used to sit under this step.
9. `report-header-column-phrases-unqualified` — the report header
   prints the column name beside each cell figure (and prints
   `opt_cells`), decided together with `SceneTotals`' field docs. Needs
   `tools/tess-lint/src/lib.rs` free; disclosed by unit 2 and rowed
   here so the next step has an owner.
10. `fold-the-two-baseline-census-files` — `baseline_census.rs` and
   `baseline_sizing_census.rs` folded into one `tests/baseline.rs`.
   Lands only after unit 2's and unit 0's PRs are both merged, since a
   fold touches both files or neither.
11. `k-report-era-witnesses-have-no-guard` — the era
   `threshold_provenance.rs` re-derives four shipped constants against
   gets a row of its own, over the committed `m7-eps-*.csv.gz`. Needs
   `tools/k-lint/*` free; disclosed by the CERT-1 roster fold and rowed
   here so the era stops being the one load-bearing claim in
   `docs/K-REPORT.md` that nothing computes with.

12. `tess-lint-twinned-csv-fixture` — promoted to a unit by unit 6's
   review: `FIXTURE_NAME` is a third hand-twinned literal, and
   blanking the token while keeping the width reddened NOTHING on
   either side. Needs `tools/tess-lint/*` free.

## Exit shape

The twelve land (with `D201`'s answer either built or ratified as
not-now), Track K's `tools/*` half is empty; the walk convention
applies.

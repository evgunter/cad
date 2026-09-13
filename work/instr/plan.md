# INSTR — the instrument crates and the documents they feed (plan)

**STATUS: OPEN (2026-09-08).** Opened by METER's exit walk
(`docs/METER-EXIT-WALK.md` §4), which Ev ratified on 2026-09-08 at
PR #2212 — *"1. open"*, then *"the new 3, instr, and your plan all sound
good!"*. Live state is `work/instr/log.md`'s tail and the item files
beside this plan, never this file.

Branch prefix (the #396 convention): **`instr/`** — unit branches
`instr/<unit>-<slug>`, orchestrator branch `instr/orchestrator`.
Away-channel tag `(INSTR orchestrator)`. A/B ordinal band
**INSTR = 3300–3399**, the next free band, claimed in
`docs/MODEL-AB-LOG.md`'s banding entry in this program's opening commit.

## Charter

The instruments MEASURE and REPORT; they never gate on a millisecond and
never justify a mesh by its own size
(`memories/tessellation-budget.md`, `memories/perf-measurement-lane.md`).
METER fixed where an instrument's claim and its **arithmetic** had
parted. What it left is the layer beneath: where an instrument's claim
and its own **execution** have parted. Every row on this slate is a check
that does not check, a figure with no home that can red, or a sentence
that describes a tree that has moved — save one, `C15`, which is here
because the census that would detect it is here, and which waits on a
corpus change no lane here makes. The standing rule from
`baseline_census.rs` carries over: **a census has one executable home and
every other site points at it.**

## Review posture

Inherited from METER, and open for Ev to reset. METER ran **no A/B
protocol** (Ev's posture ruling, 2026-09-07) and claimed its band for
bookkeeping only; `instr` inherits the band and the question — the exit
walk's §8 says in as many words that whether this program runs any A/B
protocol at all is not settled by the ratification. Until Ev says
otherwise: no draws, no arms, no `docs/MODEL-AB-LOG.md` rows beyond the
band claim itself.

**One style review per unit is the default**
(`docs/prompts/reviewer-style-lane.md`, dispatched per
`docs/REVIEW-STYLE-DISPATCH.md`). A full falsification review is reserved
for a unit where a wrong answer is expensive rather than merely wrong. On
this slate exactly one row is of that shape today —
`tess-lint-growth-margin-unprotected-from-ceil-quantisation`, because
what it moves is the margin the budget gate reds on and every committed
budget figure is read through it. A re-cut of
`docs/tess-budget-data/tess-budget-baseline.csv` is a PROPS
coordination, never a lane's call.

## Opening slate

**Twenty rows**, moved from `work/meter/` by `git mv` (ids unchanged) in
the PR that opened this program: the nineteen whose fix lands inside this
program's `paths`, plus `C15`, which Ev put here on 2026-09-08 reversing
the walk's own first ruling of code-quality Track X. Two of the nineteen
are blocked on a fence no lane here may cross and stay here anyway, by
Ev's ruling of the same day (*"giving them to the successor sounds
good"*): the row-shaping half is the instrument's.

## Unit order

The first lane is the document lane, because METER's unit 10 has just
deleted `tools/tess-lint/tests/baseline_sizing_census.rs` and five rows
point at it or at figures it used to own.

**Lane A — `docs/TESS-BUDGET.md` and the two straddlers.**

0. `baseline-sizing-census-pointers-stale` — the seven surviving mentions
   of the deleted census across six files, one of them code in
   `tools/tess-lint/tests/`, plus the line citation that had already
   moved. Cheapest, and it is the one row whose staleness grows with
   every other unit here.
1. `baseline-sizing-census-second-copy` — the four asserted figures
   `docs/TESS-BUDGET.md` carries present-tense, which the census cannot
   edit. Lands with unit 0 or straight after it; the standing rule
   decides the shape (one executable home, the document points at it).
2. `tess-budget-doc-identity-column-list` + `tess-budget-doc-note-finding-rule`
   — one lane in the document: the two identity-column enumerations that
   still name an entry `tess-lint` no longer carries, and the
   note-vs-finding rule's fourth home, which carries only the short form.
3. `tess-lint-recourse-quote-half-pinned` — the document is the unpinned
   half; the pin lands in `tools/`, so this closes the lane by crossing
   back into code.

**Lane B — `tess-lint` / `tess-meter` execution.**

4. `baseline-census-partition-assert-cannot-fail` — the assertion that
   cannot fail, the defect class METER named in its walk §6.1. It goes
   first in this lane because the census is what every later row's
   evidence is read through.
5. `tess-lint-ungated-columns-fold-silently` — six CSV columns reach the
   gate unparsed and unrefused. `C15` refs this row; closing it is what
   makes `C15`'s residue readable rather than argued.
6. `gate-findings-name-no-columns-and-recoverable-has-two-aggregations`
   + `report-constraint-activity-line-names-no-columns` — one lane: the
   report names its columns, and `recoverable` stops being one name over
   two aggregations. Both edit `tools/tess-lint/src/lib.rs`, so they
   land together or they conflict.
7. `tess-lint-growth-margin-unprotected-from-ceil-quantisation` — the
   half `D206` did not close: the continuous envelope is under the
   margin, the `ceil`'d column the gate READS is not. **The full
   falsification review.**
8. `tess-meter-sampled-retune-figure-unreproducible` — the 21-sample
   retune figure names a draw nothing records, and its reference moved
   with `SPLIT_SCAN_SAMPLES`. Follows unit 7, which settles what the
   reference is.
9. `cut-line-commit-names-no-baseline-change` — the cut line stamps the
   sweeping tree's HEAD. Needs the CIW seam drawn in the PR: the three
   cut scripts are CIW's and this pin reads them as text.
10. `tess-lint-zero-certificate-two-meanings` — **blocked outside the
   fence.** The producer separating "certified nothing" from "certified
   zero" is `crates/mesh/src/budget.rs`, S-MESH's. The row-shaping half
   is the instrument's, which is why it sits here; dispatch it only
   after S-MESH lands the separation, and announce it to S-MESH first.

**Lane C — `k-lint`.**

11. `k-lint-csv-header-unpinned-against-five-producers` — `EXPECTED_HEADER`
   hand-copied at five producer sites with no pin in either direction.
   The `CHART_TAGS` precedent METER used for the cut prefix applies.
12. `k-lint-last-round-is-eps-coupled-but-unrostered` — `props_quad_last_round`
   is eps-coupled by the criterion the roster pin already applies and is
   not on the roster. One name; it is the cheapest evidence that the
   roster's ADDED direction is unpinned.
13. `k-lint-gate-described-as-diffing-the-committed-baselines` — two sites
   say the gate diffs the fresh sweep against `docs/k-report-data/`;
   nothing is diffed. A document row against `docs/K-REPORT.md`.
14. `k-lint-rule-1-prose-assumes-every-in-band-site-refuses` — rule 1's
   prose says an indeterminate sample means the kernel refused typed; a
   folding site records one and does not refuse.
15. `k-lint-eps-coupled-criterion-unwritten` + `k-lint-roster-wants-a-kernel-side-vocabulary`
   — **the PROPS pair, blocked outside the fence.** Writing the criterion
   is what lets the roster be pinned in the ADDED direction, and a
   `const` slice from `geom_core::k_stats` would retire the hand-rolled
   parser entirely; both are PROPS units and one PROPS unit could land
   both. They stay here by Ev's ruling; what this program owes is the
   criterion in prose and the ask, not the kernel edit.

**Last, and not in a lane.**

16. `C15` — waits on a `demos/tour` change no lane here makes (a scene
   carrying a pair becoming document-built, so the sweep has an
   evaluation to take names from). `demos/tour` is in no program's
   `paths` at all. Its residue is guarded meanwhile by execution in
   `tools/tess-lint/tests/baseline_census.rs`, which fires either way, so
   nothing rots while it waits. Re-read it after unit 5.

## Exit shape

Lanes A, B and C land; `C15` closes on the corpus change or is ruled
not-now with the ratification cited. The instruments then say only what
they can execute, and the walk convention applies.

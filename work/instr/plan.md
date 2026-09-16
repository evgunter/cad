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
because the census that detects it is here rather than because the scene
is. The corpus change `C15` waited on has landed (the teapot's spout is
a lofted canal), so it is now a unit like the rest. The standing rule from
`baseline_census.rs` carries over: **a census has one executable home and
every other site points at it.**

## Review posture

**Settled for this program on 2026-09-16 (Ev), replacing the question
METER's walk §8 left open.**

- **No A/B protocol.** METER's posture (Ev, 2026-09-07) is not merely
  inherited pending a ruling any more: `instr` runs none. No draws, no
  arms, no `docs/MODEL-AB-LOG.md` rows beyond the band claim itself.
- **One style review per unit is the default**
  (`docs/prompts/reviewer-style-lane.md`, dispatched per
  `docs/REVIEW-STYLE-DISPATCH.md`). Most of this slate is documents,
  pins and small table work, where a wrong answer is visible and cheap.
- **A full falsification review is reserved for the three units where a
  wrong answer is expensive rather than merely wrong**, and they are
  named here rather than judged unit by unit:
  - unit 7, `tess-lint-growth-margin-unprotected-from-ceil-quantisation`
    — it moves the margin the budget gate reds on, and every committed
    budget figure is read through it;
  - unit 15, `no-guard-reds-a-decide-name-missing-from-the-k-report` —
    a new source-walking census, whose failure mode is failing open and
    staying green, which is the shape the style lane says only a
    non-author has ever caught;
  - unit 18, `C15` — two join regimes that have to agree on a scene
    carrying both, where a wrong join mis-attributes per-face rows
    silently instead of reddening.

  Unit 5 (`tess-lint-ungated-columns-fold-silently`) was weighed for the
  third slot and ruled style-only (Ev, 2026-09-16): it changes what the
  gate refuses, but the logic itself is plain.

A re-cut of `docs/tess-budget-data/tess-budget-baseline.csv` is a PROPS
coordination, never a lane's call.

## Slate

**Twenty rows** opened the program, moved from `work/meter/` by `git mv`
(ids unchanged) in the PR that opened it: the nineteen whose fix lands
inside this program's `paths`, plus `C15`, which Ev put here on
2026-09-08 reversing the walk's own first ruling of code-quality Track X.

**Five have arrived since**, filed by other programs' lanes onto this
program's ground rather than opened here:
`pin-table-has-no-whitespace-tail-row` (routed by CIW),
`c15-is-dischargeable-now-that-a-sized-scene-carries-names` (the demo
re-cut that discharges `C15`),
`no-guard-reds-a-decide-name-missing-from-the-k-report` (TOPO, PR 2529),
`k-report-bit-identity-claim-has-no-citation` (SUITE/D114) and
`frame-mint-funnel-names-outside-every-sweep-corpus` (FRAME-WITNESS,
PR 2675). **Twenty-five today**; the unit order below places all of them.

**The two fences the opening slate was blocked behind have moved**, both
by Ev's ruling of 2026-09-16:

- **S-MESH** (`work/mesh/`) is open but not in active development, so
  the producer-side column `tess-lint-zero-certificate-two-meanings`
  waits on may be written here. It is a territory crossing, not a claim:
  `crates/mesh/*` stays S-MESH's `paths`, `work.py territory` names it,
  and the unit's PR says so.
- **PROPS** is live and reachable. What this program owes the PROPS pair
  is still the criterion in prose and the ask — and if the ask is better
  sited on PROPS' own slate, it MOVES there (`git mv` plus header edit),
  which is the ruling's other half.

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
   margin, the `ceil`'d column the gate READS is not. **Full
   falsification review.**
8. `tess-meter-sampled-retune-figure-unreproducible` — the 21-sample
   retune figure names a draw nothing records, and its reference moved
   with `SPLIT_SCAN_SAMPLES`. Follows unit 7, which settles what the
   reference is.
9. `cut-line-commit-names-no-baseline-change` + `pin-table-has-no-whitespace-tail-row`
   — one lane on the cut line and its pin. The first: the cut line
   stamps the sweeping tree's HEAD, so the commit it names need never
   have touched the baseline. The second: `cut_line_pin.rs`'s `TABLE`
   has no row whose date carries a non-space whitespace tail, so the
   `[^ ]*$` anchor CIW chose is load-bearing in the re-stamping
   direction and unexercised. Both read the three cut scripts as text
   and edit none of them; the CIW seam is drawn once, in this PR.
10. `tess-lint-zero-certificate-two-meanings` — **unblocked.** The
   producer separating "certified nothing" from "certified zero" is
   `crates/mesh/src/budget.rs`, S-MESH's ground, and S-MESH is not in
   active development (Ev, 2026-09-16), so this unit writes both halves:
   the producer's column and the instrument's admission. Announce the
   crossing in the PR with `work.py territory` output; claim no path.

**Lane C — `k-lint` and `docs/K-REPORT.md`.**

11. `k-lint-csv-header-unpinned-against-five-producers` — `EXPECTED_HEADER`
   hand-copied at five producer sites with no pin in either direction.
   The `CHART_TAGS` precedent METER used for the cut prefix applies.
12. `k-lint-last-round-is-eps-coupled-but-unrostered` — `props_quad_last_round`
   is eps-coupled by the criterion the roster pin already applies and is
   not on the roster. One name; it is the cheapest evidence that the
   roster's ADDED direction is unpinned.
13. `k-lint-gate-described-as-diffing-the-committed-baselines` +
   `k-report-bit-identity-claim-has-no-citation` — one document lane
   against `docs/K-REPORT.md`: two sites say the gate diffs the fresh
   sweep against `docs/k-report-data/` and nothing is diffed, and the
   bit-identity claim the report's own subject rests on can now cite the
   differential that checks it. The citation states what the
   differential does NOT read, or it trades one unchecked claim for
   another.
14. `k-lint-rule-1-prose-assumes-every-in-band-site-refuses` — rule 1's
   prose says an indeterminate sample means the kernel refused typed; a
   folding site records one and does not refuse.
15. `no-guard-reds-a-decide-name-missing-from-the-k-report` — the roster
   claim every lane makes has no mechanical guard: a new `decide("…")`
   name ships with no `docs/K-REPORT.md` row and nothing reds. Two
   precedents are already in the tree (`flagged_census.rs`,
   `recourse_roster.rs`), so this is a third instance of a pattern, and
   `test_utils::source` supplies the code-only walk. **Full
   falsification review**, and the claim to falsify is that the census
   can go red — a guard that fails open passes every test it has.
16. `frame-mint-funnel-names-outside-every-sweep-corpus` — two frame-mint
   funnel names reach no `k_probe_sweep.sh` corpus, one of them
   production (`sketch_plane_frame_norm`, the Python `SketchPlane`
   door). Its third shape — whether a test-support name is roster
   material at all — is a question about the roster's MEMBERSHIP RULE
   and goes to Ev on an `[ev]` PR; the honest-coverage half of the fix
   lands either way and does not wait on the answer.
17. `k-lint-eps-coupled-criterion-unwritten` + `k-lint-roster-wants-a-kernel-side-vocabulary`
   — **the PROPS pair.** Writing the criterion is what lets the roster
   be pinned in the ADDED direction, and a `const` slice from
   `geom_core::k_stats` would retire the hand-rolled parser entirely.
   What this program owes is the criterion in prose and the ask, not the
   kernel edit; where the ask belongs on PROPS' slate rather than this
   one, it moves there.

**Lane D — `C15`.**

18. `C15` + `c15-is-dischargeable-now-that-a-sized-scene-carries-names`
   — **no longer waiting.** The teapot's spout became a LOFTED canal, so
   the corpus has its first scene carrying a Hessian-sized row and a
   durable per-face name at once, and four pairs of its laterals are
   indistinguishable by every `IDENTITY_COLUMNS` entry and separated by
   their names. `baseline_census.rs`'s two guards pin exactly that scene
   and red again when a second joins. The unit re-keys rule 4 over the
   rows that carry a name, leaves the ordinal join for the rows that do
   not, makes the two regimes agree on a scene that has both, and makes
   the re-key detector say which regime it read a row under. The
   evidence row closes with it. Re-read both after unit 5. **Full
   falsification review.**

## Exit shape

Lanes A, B, C and D land. The two rows the opening slate could only
park — the zero certificate and the PROPS pair — land or move rather
than waiting, per the 2026-09-16 rulings. The instruments then say only
what they can execute, and the walk convention applies.

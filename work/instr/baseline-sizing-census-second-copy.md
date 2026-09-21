---
id: baseline-sizing-census-second-copy
kind: issue
title: docs/TESS-BUDGET.md carries four of the sizing census's asserted figures present-tense, and the census cannot edit it
status: closed
opened: 2026-09-08
closed: 2026-09-17
---


## Was

disclosed by METER unit 10's style review, on the fold of the two
baseline census files, and filed in the PR that carries the fold. The
defect is INHERITED — the sizing census's own file carried the same
false sentence before the fold absorbed it into `baseline_census.rs` —
but unit 10 restated it and briefly widened it into a claim about both
censuses, which is what made a reviewer read it.

## Finding

**Re-measured on the live tree, 2026-09-16, and the row understated
itself.** `tools/tess-lint/tests/baseline_census.rs`'s opening states
the doctrine: *"a number transcribed into prose is a number nothing
can check"*, and the census exists so that no prose can go on
describing a file it no longer describes. Three LIVE sites transcribe
numbers that census asserts — the governing document, this file's own
header, and a doc comment in the crate the census governs.

**The predicted failure has already happened.** The baseline was
re-cut on 2026-09-15 (`ed1e64202`, cut stamp `5a0048b70e5e`) and the
census re-pinned with it; no prose followed. What
`the_committed_baseline_sizes_this_much` asserts today, and what
`docs/TESS-BUDGET.md` said instead:

| figure | the CSV and the test | the document |
|---|---|---|
| `grid_cells` | 88,036 | 46,019 |
| `patch_cells` | 147,960 | 110,811 |
| `opt_cells` | 127,966 | 93,066 |
| `span_opt_cells` | 76,599 | 44,162 |
| rows | 1605 | 1353 |
| sized rows | 80 | 64 |

The document's four were not a second copy of a live figure; they were
a coherent reading of ONE superseded cut — **cut `3f55f361b22e`
(2026-09-08), rows committed at `715977e6a`** — whose sums are exactly
46,019 / 110,811 / 93,066 / 44,162 over 1353 rows, 64 sized and 286
named. No other blob in the file's `--full-history` matches all six:
the next cut along carries 316 named rows. Every derived factor in the
document (3.52x, 3.35x, 1.042x, 8.5x, 1.19x, and the 2.2% / 0.7%
"tell") was computed from those operands.

**A cut's SHA is not where its rows live, and that trap is now
disarmed at each label.** A `# tess-budget-cut:` line names the tree
the sweep READ, so the baseline committed AT `3f55f361b22e` is the
PREVIOUS one — no `name` column, `opt_cells` 94,154, `span_opt_cells`
44,446. A reviewer who summed that file read the label as wrong when
it was the label that was ambiguous. Every cut citation this unit
added now names the cut AND the commit whose blob a reader should
sum. **The alarm fired, the test
was re-pinned five times, and the document stayed wrong for a week.**

**The second site is the census file's own header**, and it is the
same class: the sizing section transcribed the same four into the
magnitude argument about why the pre-fix block reads as stale when it
is not. They are load-bearing there, so the fix is not simply to
delete them.

**The third site is live code doc, in the crate the census governs.**
`tools/tess-lint/src/lib.rs`: *"Six of the 64 sized rows of the
committed baseline sit exactly there."* `64` is the face-identity
census's sized-row count and is stale (80). **"Six" was asserted
nowhere at all** — a second reading of the committed baseline that no
test re-derives, so a re-cut that changes how many sized rows sit at
`opt_cells == patch_cells` moves it and nothing fires. That is the
weaker half of the same defect: the document's figures at least red a
test somewhere, and this one redded nothing. (Six happens to still be
the right count; it was right by luck.)

## What was fixed in unit 10, and what was not

Two sentences that asserted the opposite were corrected in the fold's
own file, because a fold must not ship a newly-widened false claim:

- the opening no longer says `docs/TESS-BUDGET.md` *"point[s] here
  rather than carrying a second copy"* — a claim that was true of
  `lib.rs` for the face-identity count and false of the document for
  sizing, and that the fold had promoted to a joint claim about both
  censuses;
- the sizing section no longer says *"there is no longer a citation to
  guard: the document holds no live figure from this census"*. It now
  says which four figures the document carries and states plainly that
  the alarm cannot finish the job.

**Nothing else was touched.** `docs/TESS-BUDGET.md` was outside unit
10's fence, and the header's own four transcriptions are a rewrite of
an argument rather than a fold, so both wait for this row.

## Shapes of a fix, none ratified

The document's five sentences could cite the test rather than restate
the sums; or the sums could reach the document through a generated
block; or the comparison figures could be re-framed as ratios that do
not name a current absolute. The header's four are the harder half —
the magnitude argument needs the numbers to be an argument at all, and
the honest options are to derive them in the test or to mark them
explicitly as a dated comparison rather than a current reading.

## Sweep

**The row's original pattern greps for the SUPERSEDED literals**
(`46,019`, `110,811`, `93,066`, `44,162`, `1,552,822`, `164,710`), so
re-running it as written finds the stale copies and proves nothing
about the live figures. Re-derived from what the census asserts today:

```
grep -rnE '362[,_]?154|261[,_]?106|88[,_]?036|147[,_]?960|127[,_]?966|76[,_]?599|1\.6807|1\.1493|29[,_]?726' \
  --include=*.md --include=*.rs --include=*.py --include=*.sh --include=*.toml --include=*.yml .
```

— the eight figures `the_committed_baseline_sizes_this_much` asserts
plus the face-identity census's corpus-wide pair count, each spelled
with a comma, an underscore or nothing. **What it cannot match**: a
paraphrase that gives a ratio instead of an operand; the corpus counts
`1605`, `80`, `14`, `11`, `22` and `78`, which are short enough that a
literal sweep for them is noise; a figure in a non-text artefact; and
any FUTURE transcription, since the pattern is over today's values and
goes stale with the next re-cut — which is the defect one level in,
`baseline-census-transcription-sweep-literals-stale`.

Live, and in this row: `docs/TESS-BUDGET.md`'s name-column bullet, its
"tell" paragraph, the substitution paragraph, the shipped-grid
decomposition and the mis-pairing refutation; the sizing section of
`tools/tess-lint/tests/baseline_census.rs`'s module docs; and
`tools/tess-lint/src/lib.rs`'s doc on
`the_cheapest_split_never_costs_more_than_the_schedule` (found by
reading rather than by any pattern — it names `64`, a count from the
OTHER census, and its own figure "Six" is spelled as a word).

Dated tracker records, NOT in this row and deliberately. Four were
METER's own and **exist nowhere on the live tree**: METER's directory
left the tracker whole at `docs/DOC-LEDGER.md` sweep 10, which is
their done-state of record and names the SHA they stay recoverable at
(`git show 2723839067e80198bec2889d041e490000f02275:work/meter/<FILE>`,
the line numbers below being that tree's) —
`tess-budget-doc-finding-block-stale.md` (`:23`, `:24`, `:95`, `:97`,
`:139`-`:141`, `:147`-`:149`, `:179`),
`report-header-column-phrases-unqualified.md` (`:19`, `:103`, `:104`),
`log.md:49` and `:316`, and
`fold-the-two-baseline-census-files.md:111-112`. Each reports what its
unit read on the day it read it, and `baseline_census.rs`'s own
doctrine is explicit that a dated record may keep the FIGURES it
reported — editing them would make the record say something the unit
did not say. **Only their POINTERS have to follow**, which is what
`work/instr/baseline-sizing-census-pointers-stale` — moved out of
`work/meter/` alongside this row, and still open — is for.

## Moved to INSTR (2026-09-08)

Moved from `work/meter/` to `work/instr/` by `git mv` when METER's exit
walk opened the successor (`docs/METER-EXIT-WALK.md` §4, ratified by Ev on
2026-09-08 at PR #2212). Id, header and body unchanged; the directory is
the claim. This row is one of the twenty on INSTR's opening slate.

## What unit 1 did (2026-09-16)

`instr/u1-sizing-figures-second-copy`. The shape: **one executable
home, and every absolute left in prose is labelled with the cut it
reads.** No live figure from either census is restated outside
`baseline_census.rs`.

- `docs/TESS-BUDGET.md` — the comparison under "The finding" is now
  declared, at its own head, as a comparison between the pre-fix block
  and the baseline at cut `3f55f361b22e` (2026-09-08), rows committed
  at `715977e6a`, with the re-sum command beside it and a warning that
  the cut's own SHA holds the previous file; every "today" reading of
  that section is in the past tense against that cut. The document's own frozen-exception
  enumeration counts three passages instead of two. The `name` column
  bullet's four figures are gone: it now cites
  `no_scene_carrying_a_sized_row_carries_a_name` and
  `the_name_column_separates_pairs_in_exactly_this_scene` and states
  that both are positive, which is what the corpus says and the
  opposite of what the bullet said.
- `tools/tess-lint/tests/baseline_census.rs` — the header's sizing
  argument keeps its two halves and carries no current absolute; the
  numeric form is the document's frozen comparison, cited not
  restated. The CERT-10 and `SPLIT_SCAN_SAMPLES` passages keep their
  figures and are labelled as dated records of single events, which
  the doctrine exempts. Two doc-comment leads that said the `name`
  column separates nothing — contradicted by the assertions directly
  under them — were corrected.
- `tools/tess-lint/src/lib.rs` — "Six of the 64 sized rows" is
  replaced by a citation of a NEW census test,
  `the_committed_baseline_meets_the_split_bound_on_this_many_rows`,
  which counts the sized rows at `opt_cells == patch_cells` (6) and
  reds when one arrives or leaves. The figure has an executable home
  for the first time.
- A citation-by-name is guarded rather than trusted:
  `the_sites_that_cite_this_census_cite_names_it_has` names each cited
  test as a path expression — so a rename is a COMPILE error in the
  census file, and that arm covers every citation anywhere — and
  asserts the citing PROSE still spells it, which reaches
  `docs/TESS-BUDGET.md` alone. Both halves executed red-then-green.
  **Named as unguarded by the prose arm**: `tools/tess-lint/src/lib.rs`
  and `tests/report_columns_pin.rs`, because reading either would make
  the census a site that reads Rust source as text, which
  `crates/test-utils`' reader-census ledger governs and which this
  dependency-free cargo root could only do with a hand-rolled reader;
  and the tracker rows under `work/instr/` and `work/meta/`, which are
  deleted with their program and would red the test on an ordinary
  edit. The guard shape is recorded on
  `work/meta/doc-citations-no-gate-checks-rot-silently`, whose arm B is
  the class it answers.
- **The residue now has a row.** Nothing reds when an unlabelled
  current figure arrives in the document, nothing counts the
  frozen-exception enumeration, and nothing re-checks a label against
  the cut it names:
  `work/instr/no-guard-reds-on-an-unlabelled-figure-in-the-budget-doc`.
  Its first measured instance is this unit's own first push, which
  wrote *"64 there, 80 now"* into the passage explaining why the
  comparison had to be pinned — a current reading, unlabelled, in the
  document whose rule forbids exactly that, invisible to this unit's
  re-derived sweep and caught by a reviewer. Corrected in the fix pass.

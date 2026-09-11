---
id: baseline-sizing-census-second-copy
kind: issue
title: docs/TESS-BUDGET.md carries four of the sizing census's asserted figures present-tense, and the census cannot edit it
status: open
opened: 2026-09-08
---


## Was

disclosed by METER unit 10's style review, on the fold of the two
baseline census files, and filed in the PR that carries the fold. The
defect is INHERITED — `baseline_sizing_census.rs` carried the same
false sentence — but unit 10 restated it and briefly widened it into a
claim about both censuses, which is what made a reviewer read it.

## Finding

`tools/tess-lint/tests/baseline_census.rs`'s opening states the
doctrine: *"a number transcribed into prose is a number nothing can
check"*, and the census exists so that no prose can go on describing a
file it no longer describes. **Three LIVE sites transcribe numbers
that census asserts** — the governing document, this file's own
header, and a doc comment in the crate the census governs. The sweep
finds more, and the rest are dated tracker records, which the same
doctrine exempts; they are listed under "Sweep" below rather than
counted here.

`the_committed_baseline_sizes_this_much` asserts the four cell sums at
`tools/tess-lint/tests/baseline_census.rs:853-862` — `grid_cells`
46,019, `patch_cells` 110,811, `opt_cells` 93,066, `span_opt_cells`
44,162. All four appear present-tense in `docs/TESS-BUDGET.md`:

- `:332` — *"they still read within 2.2% and 0.7% of the figures below
  (93,066 against 95,090; 44,162 against 44,457)"*;
- `:371` — *"390,100 → 110,811 is **3.52x**"*;
- `:377` — *"against today's `grid_cells` 46,019, **3.35x**"*;
- `:380` — *"sits 1.042x above it today (46,019 / 44,162)"*;
- `:389` — *"the 8.5x is 390,100 / 46,019"*.

**The alarm fires and the document stays wrong.** A re-cut moves those
sums, reds the test, and leaves five present-tense sentences standing
in the document that governs the instrument — which is the exact
failure mode the census was written to prevent, one level up. The test
is the prompt to go and edit them; nothing makes that happen, and
nothing notices if it does not.

**The second site is the census file's own header**, and it is the
same class: `tools/tess-lint/tests/baseline_census.rs:199`, `:200` and
`:236` transcribe 46,019, 110,811, 93,066 and 44,162 into the sizing
section's prose, 190 lines after the doctrine sentence at `:9-10` that
forbids it. They are load-bearing there — they carry the magnitude argument
about why the pre-fix block reads as stale when it is not — so the fix
is not simply to delete them.

**The third site is live code doc, in the crate the census governs.**
`tools/tess-lint/src/lib.rs:2682-2683`: *"Six of the 64 sized rows of
the committed baseline sit exactly there."* `64` is asserted at
`tools/tess-lint/tests/baseline_census.rs:437`. **"Six" is asserted
nowhere at all** — it is a second reading of the committed baseline
that no test re-derives, sitting in a doc comment above
`the_cheapest_split_never_costs_more_than_the_schedule`, so a re-cut
that changes how many sized rows sit at equality moves it and nothing
fires. That is the weaker half of the same defect: the document's
figures at least red a test somewhere, and this one reds nothing.

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

`grep -rn "46,019\|110,811\|93,066\|44,162\|1,552,822\|164,710"`
over `*.md` and `*.rs` from the repo root — the six figures
`the_committed_baseline_sizes_this_much` asserts. **What the pattern
could not match**: a figure written without the thousands comma or
with different digit grouping, a paraphrase that gives a ratio instead
of an operand, and the two factors (2.408, 1.0420), which are short
enough that a literal sweep for them is noise.

Live, and in this row: `docs/TESS-BUDGET.md:332`, `:371`, `:377`,
`:380`, `:389`; `tools/tess-lint/tests/baseline_census.rs:199`,
`:200`, `:236`; `tools/tess-lint/src/lib.rs:2682-2683` (found by
reading rather than by this pattern — it names `64`, a face count from
the OTHER census, and its own figure "Six" is spelled as a word).

Dated tracker records, NOT in this row and deliberately:
`work/meter/tess-budget-doc-finding-block-stale.md` (`:23`, `:24`,
`:95`, `:97`, `:139`-`:141`, `:147`-`:149`, `:179`),
`work/meter/report-header-column-phrases-unqualified.md` (`:19`,
`:103`, `:104`), `work/meter/log.md:49` and `:316`, and
`work/meter/fold-the-two-baseline-census-files.md:111-112`. Each
reports what its unit read on the day it read it, and
`baseline_census.rs`'s own doctrine is explicit that a dated record
may keep the FIGURES it reported — editing them would make the record
say something the unit did not say. **Only their POINTERS have to
follow**, which is what
`work/meter/baseline-sizing-census-pointers-stale` is for.

## Moved to INSTR (2026-09-08)

Moved from `work/meter/` to `work/instr/` by `git mv` when METER's exit
walk opened the successor (`docs/METER-EXIT-WALK.md` §4, ratified by Ev on
2026-09-08 at PR #2212). Id, header and body unchanged; the directory is
the claim. This row is one of the twenty on INSTR's opening slate.

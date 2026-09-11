# Bayesian re-analysis of the Opus 5 vs Fable 5 implementation A/B

Scratch analysis of `docs/MODEL-AB-LOG.md`. **Not for merge** — this is
process telemetry about the experiment, not kernel content.

The log's own M4- and M5-close readouts concluded "no evidence either arm
produces more bugs or worse code" without fitting a model. This fits
models: one per outcome, each estimating how much the arm shifts it, so
the conclusion carries an interval rather than an impression.

## Output

`report.html` — the report (self-contained, theme-aware, no external
assets). `artifact.html` is the same body without the page shell.

## Pipeline

```
docs/MODEL-AB-LOG.md
  │
  ├─ blind_extract.py ──→ blinded-rows.md      arm column dropped,
  │                                            model names redacted,
  │                                            zero-leak verified
  │      │
  │      └─ 4 blinded labelling passes ──→ labels/*.csv
  │             maj-severity      per-MAJOR severity classification
  │             task-character    build-new vs diagnose-repair, load ratings
  │             fix-convergence   fix size, red-gate rounds, verdicts
  │             cost-parsed       tokens/wall-clock parsed out of prose
  │
  ├─ core-rows.csv          arm, difficulty, recorded counts, rubric
  │                         (the only file that knows the arms)
  │
  ├─ merge_new.py           folds second-pass labels into the masters
  ├─ analyze.py ──→ results.json      ~34 models
  ├─ refit.py                re-runs any model missing the R-hat bar
  ├─ concordance.py ──→ concordance.json   reviewer-agreement analysis
  └─ report.py  ──→ report.html
```

## Second pass (2026-08-11)

Adds 38 rows (protocol v3 = {opus,opus,fable} triples), and the
**reviewer-concordance** experiment: every 3rd merged row gets a second
independent blinded reviewer on the same code head. `concordance.py`
estimates how much of a finding count is the reviewer rather than the
code — the confound the first readout named as its binding measurement
problem.

Headline changes from the first readout: the MAJOR-finding rate now
separates (0.51, 95% CrI 0.29–0.90); the earlier cost advantage
evaporated under better recording; the difficulty sign-flip resolved; and
the test-quality rubric signal is retired, because reviewer noise on that
dimension is 2.4x the effect.

## Why it looks like this

No `numpy`, `scipy`, `pymc`, `pip`, `node`, or any plotting library is
available on this machine, so:

- `mcmc.py` is a hand-written componentwise adaptive random-walk
  Metropolis sampler with split-R-hat and ESS diagnostics.
- `validate_sampler.py` independently refits the headline model by
  deterministic 2-D grid quadrature and compares marginals, as a check
  that the sampler is not lying.
- `palette_check.py` reimplements the chart-palette accessibility gate
  (OKLab ΔE under simulated colour-vision deficiency + WCAG contrast).
- `svglib.py` emits the charts as inline SVG.

## Blinding

The labelling passes are the only place subjective judgment enters the
data, so they were run against a generated extract with the arm column
removed and every residual model name redacted, by agents instructed not
to open the source log. `blind_extract.py` fails loudly if any model name
survives redaction.

The v6 unilateral-MAJOR adjudication is blinded separately and more strictly,
because its material is the review prose rather than the dispatch table — see
the next section.

## The v6 blinded adjudication (protocol v6 item 4)

Separate pass, separate tooling, and it does not feed `report.html`. Protocol v6
pre-registers a stopping rule (item 2) whose readout is the adjudicated
unilateral-MAJOR tally under the item-3 instrument, and item 4 requires that
tally to be coded **attribution-stripped**: the coder sees reviewer A/B with the
A/B-to-R1/R2 mapping re-randomized per pair. That is a different object from
what `blind_extract.py` blinds — it strips the *dispatch rows*, this strips the
*review prose* — so it is a different script.

```
docs/MODEL-AB-LOG.md
  │
  ├─ blind_reviews.py ──→ blinded-reviews.md        R1/R2 -> RA/RB per a byte
  │                       keys/blind-key-<stamp>.csv    drawn per pair; names,
  │                       labels/v6-*-BLANK.csv          bytes, parity, block
  │                                                      and slot records gone
  │      │
  │      └─ blinded coding pass ──→ labels/v6-unilateral-adjudication.csv
  │                                  (raiser is A or B, never a slot or model)
  │
  └─ unblind_adjudication.py ──→ the readout
         joins the coded findings to the key, applies item 3 (a)-(e)
         mechanically, reports the tally against the item-2 stopping rule,
         split by model, by slot, and by 5.1 era
```

Three things beyond dropping a column, because the review prose leaks three
ways that the dispatch table does not:

- **Slot labels are rewritten, identifiers included.** `r1_dual_probes` names
  the slot as loudly as `R1` does.
- **The draw record is redacted, not just the names.** "byte 146 parity 0 => R1
  OPUS + R2 FABLE" survives name-redaction as "parity 0", which item 1 of the
  protocol decodes.
- **Cost and rubric cells are withheld entirely.** Per-reviewer token counts are
  an arm signal, and item 3 needs the findings, not the price.

`keys/` is withheld from the coder and committed anyway: the drawn byte is the
record of the randomization, like a dispatch draw.

**The review instrument is not the protocol version.** Protocol v5 widened the
review SCOPE (the brief and rubric); protocol v6 is the dual-review DESIGN, and
its item 1 keeps "the v5 instrument ... unchanged". A row reading "v5 instrument"
is therefore a v6 dual carrying the brief v6 tells it to carry — nine rows say
both in one cell, `GUI-4` recording "v5 instrument" alongside "the GUI program's
first v6 TALLY CANDIDATE". An earlier version of this tooling warned that 26
such rows were "not declared v6", which was wrong and would have dropped a
recorded tally candidate; selection is by date from the first v6 pair, and the
review instrument is recorded in the key for reference only.

Neither script rules on anything. `blind_reviews.py` reports pairs whose
reviewer assignment did not parse rather than guessing one, and pairs whose row
is not its table's width rather than trusting their later columns;
`unblind_adjudication.py` stops on a coding inconsistency rather than averaging
it. Both carry `--selftest`, which executes the blinding claims
in both directions — including that the leak scan fires when redaction is
removed.

Main was merged in on 2026-09-11, so the in-tree `docs/MODEL-AB-LOG.md` is
current and `--src` can be left at its default; pass it only to read a log from
somewhere else. Run from the repo root:

```sh
python3 analysis/model-ab/blind_reviews.py
python3 analysis/model-ab/unblind_adjudication.py \
    --key analysis/model-ab/keys/blind-key-<stamp>.csv \
    --coded analysis/model-ab/labels/v6-unilateral-adjudication.csv \
    --coder-model <model>
```

Note that the branch carries the whole tree now, and the rest of the analysis
pipeline still reads the numbers it was cut against — merging main updates the
log, not `core-rows.csv`.

### Known defect in `blind_extract.py`, not fixed here

`blind_extract.py` splits body rows on a bare pipe and keeps only rows of
exactly 14 cells, so it **writes 257 of the log's 310 body rows and says nothing
about the other 53**. Two separate things cost it those rows:

- **Cells carrying pipes.** 21 rows wrote maths as `|Δ|≤π−δ`, which a bare split
  shreds. Those pipes are escaped now (`\|`), which fixes the rendering and lets
  a correct reader recover the cells — but escaping does not help a naive
  `split("|")`, which breaks on the pipe in `\|` just the same. `blind_extract.py`
  needs the one-line change to `re.split(r"(?<!\\)\|", ...)` to benefit.
- **Rows that are not their table's width.** 32 rows carry 6, 9, 10, 15 or 16
  cells under the 14-column header. No reader can repair those; they are filed
  as `work/meta/ab-log-rows-do-not-match-their-tables-declared-width` and belong
  to the programs that wrote them.

`blind_reviews.py --audit-widths` prints the census both numbers come from.
Neither change is applied to `blind_extract.py` here, because its output feeds
labelling passes already run and re-cutting those is a decision, not a repair —
the escaping pass was checked against it for exactly that reason and leaves its
output byte-identical.

## Reproduce

```sh
cd analysis/model-ab
python3 blind_extract.py     # regenerate the blinded extract
python3 analyze.py           # ~20 min, writes results.json
python3 report.py            # writes report.html
python3 validate_sampler.py  # sampler correctness check
python3 palette_check.py     # chart accessibility gate
```

## Judgment calls

Every decision that could have gone another way is in `DECISIONS.md`,
with reasoning, so it can be overturned by re-running.

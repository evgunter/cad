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
  ├─ analyze.py ──→ results.json    ~30 models
  └─ report.py  ──→ report.html
```

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

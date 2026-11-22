# Model-compare readout 1: Opus 5.5 vs Fable 5.2

Off-file readout of `docs/MODEL-COMPARE-LOG.md` (protocol rule 11), with a
tally of the Fable-5.2-era rows of `docs/DESIGN-FORK-LOG.md`. **Not for
merge**; an orchestrator with a dispatch or a fork in flight should not
read it.

Source: main at `e44e340032` (2026-11-22): 73 model-compare rows
(PRs 6854–7346) and the design-fork rows dated 2026-11-08 on.

```
docs/model-compare-log/rows/*.md
  └─ 4 extraction agents, SCHEMA.md ─→ rows.json        (73 rows; spot-checked by hand)
docs/DESIGN-FORK-LOG.md
  └─ 1 extraction agent ─→ designfork.json              (57 rows; 50 in the 5.2 era)
analyze.py ─→ results.json                              (numpy, seeded)
report.py + template.html ─→ report.html                (self-contained, theme-aware)
```

Run: `../../demos/.venv/bin/python analyze.py .` then `python3 report.py .`.

The extraction is a reading of free-form rows. Where a row did not state
a value it is null, and each agent's `notes` field says why; see
`SCHEMA.md` for what each field means and how re-reviews, sequential
second reviews and excluded halves are coded.

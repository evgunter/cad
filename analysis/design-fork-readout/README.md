# Design-fork readout 1: Opus vs Fable designers

Off-file readout of `docs/DESIGN-FORK-LOG.md` (protocol rule 7). **Not for
merge**; an orchestrator with a fork in flight should not read it.

Output: `report.html` (self-contained, theme-aware).

```
docs/DESIGN-FORK-LOG.md
  ├─ parse_log.py ─→ rows.json (scratch)      230 rows, keyed by line
  ├─ blind_extract.py ─→ 4 chunks              A/B column dropped, names redacted
  │     └─ CODING-PROMPT.md, 2 passes ─→ labels/p1.csv (Opus coders)
  │                                      labels/p2.csv (Fable coders)
  ├─ key.py ─→ key.csv                         the only file naming models;
  │                                            checked against byte parity
  ├─ analyze.py ─→ results.json, run.log       Beta/Dirichlet, logistic trend
  └─ report.py ─→ report.html
```

Run with the demos venv (numpy): `demos/.venv/bin/python analyze.py .`
then `report.py .` from this directory.

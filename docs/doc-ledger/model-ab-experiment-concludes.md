# The model A/B experiment concludes — 2026-11-10

The Opus 5 vs Fable 5.x implementation A/B (opened 2026-07-25,
suspended 2026-09-23) was concluded by Ev on 2026-11-10, when the
model comparison (`docs/MODEL-COMPARE-PROTOCOL.md`, implementers and
reviewers) succeeded
it on the current models. Its one file, `docs/MODEL-AB-LOG.md` — the
protocol as amendment history (v1–v7, the suspension and the release
instrument notes), the block and band records and every row — is
deleted.

**Recovery:** `git show b5075b345e:docs/MODEL-AB-LOG.md` (the last commit on
main that touched it). Its readouts were never on main; they are on
branch `ev/ab-bayes-analysis` under `analysis/model-ab/`.

What survives elsewhere:
- its review instrument (correspondence coding, pre-registered
  adjudication, blinded coding, lane isolation, the fair-pair rule)
  lives on in `docs/DUAL-REVIEW-PROTOCOL.md`; its blinding, per-phase
  recording and fair-row rules in `docs/MODEL-COMPARE-PROTOCOL.md`;
- the `ab_band` field in older programs' `program.md` names a band
  of the log's ordinals; it is historical and claims nothing now.

Its rows are not pooled with the successor's: different models, a
different population, a different instrument. Every row in it is
Opus 5 / 5.5 or Fable 5.0 / 5.1, never Fable 5.2 (released
2026-11-09).

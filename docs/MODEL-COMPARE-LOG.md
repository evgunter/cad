# Model comparison — experiment log

**This is process data (an experiment log), not a design
reference** — nothing here binds kernel design; it moves out of
`docs/` when the experiment concludes.

Standing experiment (Ev, in-chat, 2026-11-10). **The protocol in
force lives in `docs/MODEL-COMPARE-PROTOCOL.md`.** Each row is one file,
`docs/model-compare-log/rows/PR-<n>.md`, naming the protocol it was
recorded under by commit hash, so `git show
<hash>:docs/MODEL-COMPARE-PROTOCOL.md` reads that version. This file
holds the dated entries: opening, instrument notes, protocol changes,
readouts' rulings and the close.

## Entries

- **Opened (Ev, in-chat, 2026-11-10).** Opus vs Fable implementers,
  compared on quality, time and token cost; widened the same day,
  before any row, to Opus vs Fable reviewers too (single-review
  units draw the reviewer's model independently of the
  implementer's). Successor to the model
  A/B (suspended 2026-09-23, concluded the same day as this opened;
  `docs/doc-ledger/model-ab-experiment-concludes.md`), not a
  continuation of it: its rows are not pooled with these.
- **Dual tier joins (Ev, in-chat, 2026-11-10, on dual-review readout
  6, PR 6797).** The dual Opus review stream
  (`docs/DUAL-REVIEW-PROTOCOL.md`) is paused in favour of this
  experiment: from now on every dual-tier unit gets a concurrent
  Opus + Fable pair under rule 2, with the dual-review instrument
  carried over, before any row here was recorded.
- **Instrument note (Ev, in-chat, 2026-11-10): FABLE 5.2 RELEASE
  BOUNDARY.** Fable 5.2 released 2026-11-09, the day before this log
  opened, so every Fable row here is expected to be Fable 5.2 and
  every Opus row Opus 5.5. The rows record the version each agent
  actually ran (protocol rule 4); an analysis must check it rather
  than assume it, keep any row whose agent reports another version
  (or "unknown") apart, and report each model version separately.

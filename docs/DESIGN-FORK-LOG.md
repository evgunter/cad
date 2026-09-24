# Design-fork review — recommendation log

**This is process data (an experiment log), not a design
reference** — nothing here binds kernel design; it moves out of
`docs/` when the experiment concludes.

Standing experiment (Ev, in-chat, 2026-09-24). **The protocol in
force lives in `docs/DESIGN-FORK-PROTOCOL.md`.** This log holds the
rows; each names the protocol it was recorded under by commit hash,
so `git show <hash>:docs/DESIGN-FORK-PROTOCOL.md` reads that version.

## Rows

Columns: **protocol** (commit hash of the protocol in force when the
row opened); **fork** (the `work/` item and the `[ev]` PR); **question**
(one line, as the orchestrator posed it); **Opus** and **Fable** (first
recommendation, one line of argument, confidence, framing rejected
y/n); **agree** (on the first reports); **reconciliation** (each round:
who, shown what, what moved — or "none"); **to Ev** (the
recommendation or stated split the PR carried); **Ev** (the decision,
and date); **match** (which of the first recommendations, if any, Ev's
decision took).

| # | date | protocol | fork | question | Opus | Fable | agree | reconciliation | to Ev | Ev | match |
|---|------|----------|------|----------|------|-------|-------|----------------|-------|----|-------|

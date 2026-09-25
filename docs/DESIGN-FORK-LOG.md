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
row opened); **fork** (the `work/` item and the `[ev]` PR); **problem**
(one line, as the orchestrator stated it); **A** and **B** (first
recommendation, one line of argument, confidence, framing rejected
y/n, ratified text challenged y/n); **agree** (on the first reports);
**reconciliation** (each round: who, shown what, what moved — or
"none"); **to Ev** (the recommendation or stated split the PR
carried); **Ev** (the decision, and date); **match** (which of the
first recommendations, if any, Ev's decision took); **A/B** (the
urandom byte and which model was A — filled in only after Ev decides).

| # | date | protocol | fork | problem | A | B | agree | reconciliation | to Ev | Ev | match | A/B |
|---|------|----------|------|---------|---|---|-------|----------------|-------|----|-------|-----|
| 1 | 2026-09-25 | bb10a4cdd | `work/carve/full-revolve-emits-split-planar-walls.md`, PR 3258 | A full revolve emits each planar wall as two same-key halves, so F7 refuses it as a boolean operand above the kernel: what should the revolve produce, and where is maximality owed? | The revolve runs the structural merge as its own final stage; widen F7's sweeps clause by one clause; plain `Band(s)`; π revolve separate (lean: recognise a half turn, or park). Likely. Framing n; ratified y | The same final stage; rewrite F7 as the general rule; `Band(s)` (unsure); π now (decide θ vs π, mint the end cap Shared). Likely. Framing n; ratified y | yes on the seat and naming; split on F7 wording and π timing | none | Merge stage, F7 wording carrying both, `Band(s)`, π parked (the orchestrator's lean) |  |  |  |

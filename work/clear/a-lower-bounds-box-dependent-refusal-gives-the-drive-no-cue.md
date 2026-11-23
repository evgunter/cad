---
id: a-lower-bounds-box-dependent-refusal-gives-the-drive-no-cue
kind: issue
title: A lower bound's box-dependent refusal refuses budget and never cues the drive to bisect
status: open
opened: 2026-11-02
priority: P2
cost: M
---


Found by the correctness re-verify of PR 6292 (R3, sure, probed;
https://github.com/evgunter/cad/pull/6292#issuecomment-6574195890).
Only a final `Refused(BoxDependent)` is logged as the drive's bisection
cue (`wire.rs`, after `unlogged("F30")`), and only an upper bound's
station-pair proof produces it. A lower bound whose cells cannot
separate over a wide box refuses `budget` instead: blocks over
`gap ±0.05` with `≥ 0.3` (true everywhere, min 0.35) refuse `budget`
with 16 candidates, while the same bound over `±0.0016` Holds with 0.
Through the drive, with only `≥ 0.3` in the document, there is one
certified leaf, refused, and no bisection. A band whose lower refusal
outranks the upper's `BoxDependent` does the same.

Sound — the leaf reports truthfully — but a plain `≥ c` over a
macroscopic box (E12's case) never gets narrowed. Related: CLEARTAIL
`a-lower-bounds-violated-over-a-box-is-existential-but-priced-as-the-leaf`.
(DOORS orchestrator)

- 2026-11-02 — moved from CLEARTAIL to CLEAR when CLEARTAIL merged back into CLEAR (the budget rule now counts P0/P1 only); id and body unchanged.

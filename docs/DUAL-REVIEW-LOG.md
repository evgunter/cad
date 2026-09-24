# Dual Opus review — reviewer concordance log

**This is process data (an experiment log), not a design
reference** — nothing here binds kernel design; it moves out of
`docs/` when the experiment concludes.

Standing experiment (Ev, in-chat, 2026-09-23), opened the day the
model A/B protocol was suspended (`docs/MODEL-AB-LOG.md`, the
suspension entry). **The protocol in force lives in
`docs/DUAL-REVIEW-PROTOCOL.md`.** This log holds the rows; each names
the protocol it was recorded under by commit hash, so `git show
<hash>:docs/DUAL-REVIEW-PROTOCOL.md` reads that version.

## Rows

Columns: **protocol** (commit hash of the protocol in force at merge — the last
commit on main that touched `docs/DUAL-REVIEW-PROTOCOL.md`); **unit** (program, PR); **class** (difficulty S/M/L and task
class, logged at spec time) and the **triage reason**; **head** (the
frozen commit); **R1** and **R2** (verdict, MAJ/MIN/NOTE, one line of
prose per MAJOR, tokens, wall-clock); **correspondence** (bilateral
headline; unilateral findings R1→ and R2→; severity divergence);
**tally** (candidates, with the 6(a)–(e) disposition of each);
**fair** (yes, or the relaxation / interruption that excludes it);
**fix pass** (size, and who executed it).

| # | date | protocol | unit | class · triage reason | head | R1 | R2 | correspondence | tally | fair | fix pass |
|---|------|----------|------|-----------------------|------|----|----|----------------|-------|------|----------|

**Tally: 0 of 8. Fair pairs toward twelve: 0.**

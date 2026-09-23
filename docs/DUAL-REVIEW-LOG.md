# Dual Opus review — reviewer concordance log

**This is process data (an experiment log), not a design
reference** — nothing here binds kernel design; it moves out of
`docs/` when the experiment concludes.

Standing experiment (Ev, in-chat, 2026-09-23), opened the day the
model A/B protocol was suspended (`docs/MODEL-AB-LOG.md`, the
suspension entry). **The question: what does a SECOND independent
review buy?** The instrument is the one the A/B protocol's v6 dual
review stream ran on — the correspondence coding, the pre-registered
adjudication instrument, blinded adjudication, lane isolation and the
fair-pair rule — carried over unchanged except where a clause existed
only to compare models. This document is the SINGLE normative source
of the protocol; nothing is copied into memories, plans or program
logs.

## Protocol

1. **Population.** Every unit the suspension entry sends to a DUAL
   OPUS REVIEW — especially tricky logic, or an architectural or
   design decision with broad or hard-to-reverse impact (the v7
   triage test) — enters this log. The triage call is made at spec
   time and recorded where the unit lives (its program's `log.md` or
   item file), naming the tier and its one-line reason. Units that
   get a single review or the orchestrator's read do not enter.
2. **The pair.** Two Opus reviewers, dispatched CONCURRENTLY on the
   SAME FROZEN HEAD (the commit is named in both briefs and in the
   row). Identical brief and claims to falsify, each also pointed at
   `docs/prompts/reviewer-style-lane.md` by path. R1/R2 are
   dispatch-order labels and nothing else. Neither sees the other's
   report. The fix pass runs off the orchestrator-adjudicated UNION
   of both reviews' findings. Late-trigger fallback: if the second
   review cannot dispatch concurrently, it runs on the same frozen
   head R1 reviewed, retrieved from git history, and its findings
   are adjudicated retroactively.
3. **Lane isolation, READ side (v6 item 5).** Pushing is NEVER
   delayed — preemptible containers lose unpushed work. Instead, until
   its own report is delivered, a reviewer must not fetch, check out,
   or read the other review lane's branches, scratchpads, CI artifacts
   or PR comments. Any accidental glimpse is DISCLOSED in the report
   and the adjudicator flags the pair (item 6(e) applies if the glimpse
   included findings). Re-read the PR thread immediately before
   briefing: a comment summarising one review, posted where the other
   reviewer reads, is a glimpse.
4. **Fair pair (the 2026-08-27 method-relaxation rule).** Any
   orchestrator relaxation of a reviewer's method — scope narrowing, a
   lifted battery requirement, a permitted substitution — is written
   into the row, naming WHICH reviewer received it. **A pair whose two
   reviewers ran under different methods is excluded from the tally
   and from the pair count**, though recorded in full. A CORRECTION
   back to the brief (a nudge out of a background waiter, a battery
   moved to the right lane) is not a relaxation; the test is **"do the
   two reviewers now differ in what they were permitted to do".**
   Under build-mutex contention, reduce the number of simultaneous
   duals rather than narrowing one reviewer's method.
5. **Verdict ladder (v4 item 2), verbatim.** APPROVE ·
   APPROVE-WITH-FIXES · NOT-MERGEABLE-AS-IS · REJECT. Weight findings,
   not labels: identical substance often draws different verdicts.
6. **Pre-registered adjudication instrument (v6 item 3, fixed before
   any data here exists).** Every finding of both reviews is coded as
   BILATERAL (both reviewers raised it, at any severity — record
   severity divergence), or UNILATERAL (one raised it, the other never
   mentioned it at any severity). A finding enters the **tally** iff
   ALL of:
   (a) UNILATERAL, and raised as MAJOR;
   (b) DEFECT CLASS code / test-gap / contract-API — doc- or
       claim-only findings are recorded but excluded;
   (c) DEDUP — findings tracing to one underlying defect count ONCE,
       however many ways it is described;
   (d) DEMONSTRATED BY EXECUTION — a red probe, a surviving mutant gone
       red, a compile-fail pin, a red CI row, or a measured wrong
       value; accepted-by-inspection findings are recorded but do not
       count;
   (e) FAIR PAIR — pairs where either review was interrupted or
       truncated (outage, kill, lost session), or which fail item 4,
       are excluded though still recorded; unrecoverable counts are
       missing data, never zeros.
   **A tallied finding is the measurement**: a real, demonstrated,
   blocking defect that one competent review would have let through.
   Unilateral MINORs are recorded both directions — they are the
   plentiful signal on review variance.
7. **Blinded adjudication (v6 item 4).** The correspondence coding
   and the item-6 adjudication run ATTRIBUTION-STRIPPED: the coder
   sees reviewer A/B with the A/B-to-R1/R2 mapping re-randomized per
   pair by a recorded `/dev/urandom` byte. At merge the orchestrator
   records a CORRESPONDENCE PRE-NOTE (bilateral headline, unilateral
   findings each direction, tally candidates); the tally itself is
   what the blinded coding confirms.
8. **Record at merge.** The row rides the unit's own PR as its LAST
   commit, after both reviews are delivered, and a missing field is a
   merge blocker for the row. **Pair numbers are assigned AT MERGE in
   main's merge order** (DR-1, DR-2, …); a collision on a concurrent
   merge is resolved by renumbering the later merge, never by
   renumbering a row already on main.
9. **Pre-registered readout point: the first readout is owed when the
   tally reaches EIGHT, or at TWELVE fair pairs, whichever comes
   first** (v6 item 2's thresholds: the informative unit is the
   tallied unilateral MAJOR, and zero-MAJOR pairs are nearly
   uninformative). This is a readout, not a stop — duals continue
   until Ev rules on the result. The orchestrator recording the
   triggering row asks Ev per `CLAUDE.md` "Asking Ev". The running
   tally and fair-pair count are kept at the foot of the rows table.
10. **Readouts are off-file**, on the standing A/B rule: a directional
    result ("the second review rarely finds anything") creates
    expectancy effects on how pairs are adjudicated and triaged. They
    live on a branch under `analysis/dual-review/`; an orchestrator
    with a dual in flight should not read them.
11. **Later escapes.** When a defect is later traced to a unit in this
    log (an issue, a red row, a fix PR), append a line under the unit's
    row naming it and whether either review raised it. Escapes that
    both reviewers missed bound what a second review can buy from
    above.

## Seams

- **Not comparable with the A/B log's duals.** Those were cross-model
  (or fable/fable) pairs over a different population; this stream is
  Opus/Opus over the hard units only. The v4 same-model stream
  (fable/fable, closed at sample #16) is the nearest precedent and is
  still a different model.
- **Opus 5.5 era.** Every pair here post-dates the 2026-09-22 Opus 5.5
  release; any future model change is an instrument seam recorded as a
  dated note here, and a readout spanning it reports the eras
  separately.

## Rows

Columns: **unit** (program, PR); **class** (difficulty S/M/L and task
class, logged at spec time) and the **triage reason**; **head** (the
frozen commit); **R1** and **R2** (verdict, MAJ/MIN/NOTE, one line of
prose per MAJOR, tokens, wall-clock); **correspondence** (bilateral
headline; unilateral findings R1→ and R2→; severity divergence);
**tally** (candidates, with the 6(a)–(e) disposition of each);
**fair** (yes, or the relaxation / interruption that excludes it);
**fix pass** (size, and who executed it).

| # | date | unit | class · triage reason | head | R1 | R2 | correspondence | tally | fair | fix pass |
|---|------|------|-----------------------|------|----|----|----------------|-------|------|----------|

**Tally: 0 of 8. Fair pairs toward twelve: 0.**

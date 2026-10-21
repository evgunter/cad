---
id: m-tier-miss-at-dr-321-the-backstop-passes-a-violated-bound-on-rounding
kind: ruling
title: Short readout: M-tier miss at DR-321 (PR 5576), a holdout pair's tallied MAJOR: the volume backstop passes a violated bound when its f64 margin rounds to holding, which the other review did not raise
status: open
needs_ev: true
opened: 2026-10-21
---

**FYI, no options** (dual-review protocol rule 9: a short readout at every M-tier miss). Acknowledging it closes this item.

- **The unit.** PR 5576 restates what a zero `volume_pad` means: "every face took a closed form", not "the value is exact". It gives readers a rule (re-derive, or decide on an enclosure) and audits every reader. It is an M unit, drawn into the HOLDOUT arm (byte 120), and recorded as DR-321.
- **The miss.** The PR's audit said one reader broke the rule: the boolean volume backstop's magnitude arm, which it filed on CLEAVE. R1 showed by execution that the backstop's Bound arm also reads an f64 margin as exact.
  - A truly violated `vol(A ∩ B) ≤ vol(A)` whose f64 margin rounds to holding passes with no re-derivation: 3 of 60 rows, at every ε.
  - For example, at far 1e4, t 1e-2, k 1e-9, the exact margin is −1.0e-11 and the f64 margin +6.35e-9, and the bound passes.
  - The arm-2 fix the PR proposed does not reach it.
  - R2 raised no MAJOR, and its report says it found no other f64-as-exact read in `bound_holds`. Had R2 been the sequential arm's only review, the audit would have shipped with its census short.
  - The blinded coding (byte 143) tallied it. The defect is in pre-existing CLEAVE code, but rule 6(b) classes the defect itself, and this one is a wrong Ok on a violated bound.
- **What happened next.** The fix pass replaced the type doc's census with the rule. It filed the held read, the backstop's operand gate (seen by both reviews: it can switch every arm off) and its +V posture as a second CLEAVE P1, `work/cleave/the-backstop-reads-an-unpadded-f64-volume-as-exact-where-it-passes-or-switches-off-a-bound`, with witnesses and fix shapes. The backstop itself is CLEAVE's to fix.
- **The count.** The foot of `docs/DUAL-REVIEW-LOG.md` gives the running tally and the M-tier misses since the last full readout.


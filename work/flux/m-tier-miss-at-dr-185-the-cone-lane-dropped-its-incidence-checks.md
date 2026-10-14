---
id: m-tier-miss-at-dr-185-the-cone-lane-dropped-its-incidence-checks
kind: ruling
title: Short readout: M-tier miss at DR-185 (PR 4696) — a holdout pair's single MAJOR, the cone lane's dropped incidence checks, which the other review never raised
status: closed
opened: 2026-10-11
closed: 2026-10-12
---


**FYI, no options** (dual-review protocol rule 9: a short readout at every M-tier miss). Acknowledging it closes this item.

- **The unit.** PR 4696 measures every one-nappe cone face by its vector area over all its loops, and retires the S58 iso premise from the cone arm. It is an M unit, drawn into the HOLDOUT arm (byte 120), and recorded as DR-185.
- **The miss.** R2 found that the rewrite had dropped the on-surface premises (incidence) that `cone_boundary` used to decide, and showed it by execution: a rectangle with straight chords in place of its rims measured 0.6051, where the true cone wall is 0.7191 and the base refused it `OffSurface`. R1 raised no MAJOR and never mentioned incidence. Had R1 been the sequential arm's only review, the gap would have shipped. The blinded coding (byte 154) tallied it.
- **What happened next.** The first fix pass restored every on-surface premise. Verifier 1 then caught a regression that the restoration itself introduced: the rim-fit lever was off by 1/cos α, so 316 legal near-flat faces escalated. The second fix pass levers every incidence decide at its true distance from the cone. Verifier 2 confirmed it. Its two fail-closed MINORs are filed as `cone-incidence-decides-over-lever-toward-the-needle-and-the-flat`.
- **The count.** This is the first M-tier miss since readout 5's trigger (DR-165), among 12 M-tier units in that window. The foot of `docs/DUAL-REVIEW-LOG.md` gives the full count.

## Closed (2026-10-12, PR 4738)

Ev acknowledged the readout ("thanks for letting me know"). Nothing further is owed.

---
id: m-tier-miss-at-dr-208-the-booleans-carry-refused-a-pit-under-a-carried-rest
kind: ruling
title: Short readout: M-tier miss at DR-208 (PR 4820) — a holdout pair's single MAJOR, a pit under a carried rest refusing JoinDesync, which the other review never raised
status: closed
opened: 2026-10-12
closed: 2026-10-12
---


**FYI, no options.** Dual-review protocol rule 9 calls for a short readout at every M-tier miss. Acknowledging it closes this item.

- **The unit.** PR 4820 makes the boolean's carried contact records go through the one carry, per your ruling on `[ev]` PR 4749. It retired `ledger`, `ee_lineage` and `holder`. It was reviewed together with PR 4809 (the surgery verbs' carry) as one stacked unit. It is an M unit, drawn into the HOLDOUT arm (byte 36).
- **The miss.** R1 found a regression and showed it by execution on both heads. The case: a tilted cube resting its corner on a slab, with the record of that rest carried, minus a small pit under the corner.
  - It built and validated before the change. Afterwards it refused with `JoinDesync`, the kernel-bug kind.
  - The cause: the boolean counts a missing cell as removed only if the finish's discard list names it. Two join routes (a sliver face merged into its neighbour, and a hole face turned into a ring) removed faces without stating it.
  - R2 raised no MAJOR: it found the neighbouring weaknesses (`place_found` reading removal off a missing image, and remainder mutants surviving), but not this case.
- **What happened next.** One fix pass made every removing route state the face that holds the killed face's region, so the pit now builds and drops the rest. R1's delta check came back clean, with every mutant red. After FUSE's PR 4868 landed, the join's rows became a checked `Substitutions<FaceKey>`.
- **The count.** This is the second M-tier miss since readout 5's trigger (DR-165), after DR-185, among 28 M-tier units in that window. The foot of `docs/DUAL-REVIEW-LOG.md` gives the full count.

Acknowledged by Ev on PR 4899 ("thanks for letting me know!", 2026-10-12).

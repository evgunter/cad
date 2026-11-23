# CLEAVE leaves the tracker — 2026-11-03

CLEAVE (split and boolean results where solids meet along a line) opened 2026-10-01 in REACH's priority-seam cut. It was itself cut on 2026-10-26 and 2026-11-01. Those cuts opened ONEHOME, SPLITDOOR, HONEPAIR, HONEPOSE, CLEAVETAIL, TANGRIM, PARTING, TOUCHREC, ONEREAD and SEAL. It closed on 2026-11-03 (Ev, in chat) with fourteen live rows behind 137 closed ones and no dispatchable row above P2. Its one P1 is parked on GAUGE. Its plan said it closes when its slate is empty and set no exit criteria, so no exit walk is owed.

**Where its live rows went.** All fourteen moved to **CLEAVETAIL**, its own tail on the same `paths`, by `git mv`, ids and bodies unchanged, each with a dated note. CLEAVETAIL's title and plan now carry the line-meeting slate (line wedges, fate cuts, slits and rings), and the plan names the seam to cut on if it grows.

**Citations.** Citations of the live rows were rewritten to `work/cleavetail/`. In shipped code and docs, a citation of a closed CLEAVE row now names the row's id and this note. Header references from other programs' rows to closed CLEAVE rows were dropped, among them HONEPAIR's `parent`. Logs and other programs' row bodies were left as written.

Its A/B band 9800-9899 in `docs/MODEL-AB-LOG.md` closes with it.

Recover its program, plan, log and closed rows at 0f01217666:

    git show 0f01217666:work/cleave/<FILE>

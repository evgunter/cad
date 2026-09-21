# DECIDE — the log

## 2026-09-20 — opened

Cut out of SYM, which was carrying 78.5 budget points, when Ev
ratified the priority and track-size conventions in chat the same day.
The cut followed SYM's PRIORITY seam per `work/README.md` "Track
size", into several tracks at once so they can run in PARALLEL — Ev, in
chat: *"for these high priority tracks it's ideal to have several
components that can be worked on in parallel."*

8 rows arrived by `git mv` with their ids, bodies and history
unchanged. SYM keeps its band 5800-5899; band 8600-8699 is claimed
for this program in the same commit (`docs/MODEL-AB-LOG.md`). Nothing
dispatched.

## The fork reconciliation, SYM-8's dual, SYM-10's hand-over (2026-09-21)

The SYM orchestrator session was forked on 2026-09-19 (one copy on
Ev's local machine, one in the cloud). The local copy dispatched
SYM-8's v6 dual on 2026-09-19 (ordinal 4704, claimed in SYM's band
before the 09-20 cut; byte 98 ⇒ R1 = OPUS, R2 = FABLE; both arms on
the shared local box, editor-core re-takes riding the hosted gate),
dispatched SYM-10 to slot 2 with SYM-9 displaced, and answered PROPS's
seams on #2468. The cloud copy, out on its usage limit from 09-15 to
09-21, merged main into #2616 on its return. The two reconciled on
`[ev]` #2949 with Ev's delegation ("don't wait on me"): the local side
adjudicated SYM-8's union (both reports verbatim in
`work/sym/logs/fork-state-local.md` on that branch; R1 OPUS
MERGEABLE-AFTER-FIXES 2/3/4, R2 FABLE 1/4/7, the same two defects found
independently, no unilateral MAJOR) and runs the R1 delta; the cloud
side runs the fix pass, state-sync, merge and the row, takes SYM-10
from `fb01a201c`, and is the orchestrator of DECIDE and SYM's remainder
going forward. The block record is `sym/b2-block` (the local side's
"slot 1 dual concluded" at `2fd3edf11`).

**Protocol v7 triage, recorded where the unit lives:** SYM-8 is triaged
IN — a rule of the atom algebra, H / NUMERIC, spec'd 2026-09-14 under
v6 before the seam, its dual dispatched 2026-09-19 22:50Z. **The
orchestrator's model, per phase:** spec, dispatch and adjudication
FABLE (the local fork); fix pass, state-sync and merge FABLE (the cloud
session); the unit changed hands at the fork. SYM-10's implementer arm
is FABLE on both sides.

---
id: ab-log-v6-stream-is-past-its-stopping-rule-unadjudicated
kind: issue
title: The v6 dual-review stream passed its twelve-pair stopping rule around 2026-08-29 and its unilateral-MAJOR tally has never been reconciled
status: open
opened: 2026-09-04
---


## What

`docs/MODEL-AB-LOG.md` Protocol v6 (lines ~273–340) pre-registers a
stopping rule for the cross-model dual stream: it STOPS when the
adjudicated unilateral-MAJOR tally reaches EIGHT, or at TWELVE new
pairs, whichever first, and the orchestrator recording the triggering
row notifies Ev. Item 4 says the tally's coding and adjudication run
attribution-stripped in a blinded session.

Measured over the file on 2026-09-04 (a read-only pass over every
record row dated 2026-08-27 or later):

- **109 v6 dual rows** are recorded, PIERCE (2026-08-27, "the FIRST
  v6 pair") through DOCM-3 (2026-09-04). The twelfth fair pair was
  passed around 2026-08-29/30; no row records the rule firing, no
  notification is recorded, and there is no Protocol v7.
- **The running tally is stated inconsistently and then abandoned.**
  F7POLE (2026-08-29) declares "tally 1/8"; BLEND3 (2026-08-31)
  independently declares "1 (from 0)"; C5-1 (2026-09-02) reads "tally
  1/8, candidates 5"; H4 (2026-09-04) records the contradiction and
  declines to resolve it. Roughly 40 rows carry no tally language at
  all; about 20 more defer to "coding at the blinded adjudication".
- **Confirmed tally entries: 1** (F7POLE). **Named candidates never
  adjudicated: at least 12** (GUI-4, P-1b, M10-1, CERTM2, CERTN2 ×4,
  MESH6, MESH7, CS-1, SHELL-1, RC-1, H4) — plus rows with a unilateral
  executed MAJOR and no tally sentence (CERT1, M10-2, QA-1, QA-3,
  CERT6, CERT10, MATE6, SEAT5, M10-4, M10-5, K3 among them). DOCM-1
  adds two candidates at its merge.
- Excluded pairs, correctly recorded as such: PIERCE, P-1a, CENSUS-G2,
  MESH3, BOOL3 (item 3e).

So the stream has run about nine times past its stopping rule with
the readout that was the point of the rule never taken; every
orchestrator has kept dispatching duals because no row said to stop.
The cost is real (two reviews per unit at ~250–320k tokens each).

## What it is not

Not a claim that the duals were wasted: the union fix passes catch
defects every time (DOCM-1's pair alone found a code defect and a
test gap, one per slot). The finding is that the DATA the rule exists
to produce — the slot-vs-model contrast — has not been read, and the
rule that would have stopped the spend has been passed unnoticed.

## Where it stands

Ev's call, two parts: (1) run the blinded adjudication (item 4) over
the ~109 pairs, or over the candidates list above as the tally's
input; (2) rule on the stream — STOP now per the pre-registered rule,
or write Protocol v7 with a new rule and say why. Until ruled, DOCM
keeps dispatching duals as every other program does, and records
"+N candidate" per the H4 precedent. Orchestrators dispatching a dual
after this filing should read it.

## Re-homed (2026-09-04)

Moved from `work/issues/` to `work/meta/` in the tracker-wide
re-home sweep of 2026-09-04 (Ev's direction, in-chat), which read every
open `work/issues/` file against every open program's `paths` and
against the code-quality K–X fences. Id, body and header are unchanged;
the directory is the claim (`work/README.md`). Any `## Home` section
above naming `work/issues/` is superseded by this line and is kept as
the record of why the file was parked there.

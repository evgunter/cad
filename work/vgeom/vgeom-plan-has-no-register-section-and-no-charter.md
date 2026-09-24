---
id: vgeom-plan-has-no-register-section-and-no-charter
kind: issue
title: work/vgeom/program.md points every lane at a plan.md section that does not exist, and the priority-seam cut took VGEOM's charter with it
status: closed
opened: 2026-09-21
priority: P4
cost: E
closed: 2026-09-21
branch: claude/view-orchestrator-review-2lnz3t
---


Found by the VIEW lane `view/cut-residue` while doing the archaeology
for `work/view/the-lane-register-has-no-home-after-views-directory-goes`,
which had to count who inherits VIEW's rule register and how.

## The finding

`work/vgeom/program.md`'s closing paragraph says:

> The lane register that binds every lane dispatched here is
> `work/view/plan.md`'s, inherited by reference and not copied —
> `work/vgeom/plan.md` §The register says why and what happens to it
> when VIEW's directory goes.

**There is no §The register in `work/vgeom/plan.md`**, and there is no
§Charter either:

    git show origin/main:work/vgeom/plan.md | grep -n '^## '
    # 8:## The slate
    # 33:## Order
    # 48:## Review posture

Its three sibling successors all have both — `work/vnews/plan.md`,
`work/vseam/plan.md` and `work/vdoc/plan.md` each carry a §Charter and
a §The register, and the three §The register sections are byte-identical
to each other.

## How it happened, and why it is not the cut's defect

The cut of 2026-09-17 (`f8a822e8c1`) wrote all four plans with both
sections. `work/vgeom/plan.md` was then rewritten whole by **VGEOM's own
priority-seam cut of 2026-09-20** (its header says so: *"Re-scoped
2026-09-20 by VGEOM's priority-seam cut"*), which replaced the file with
a slate table, an Order and a Review posture. The two sections went with
the rewrite and `program.md`'s sentence was not re-read against it.

## What it costs

Two things, and the second is the one that matters:

- **The pointer dangles today**, not on some future sweep day. A VGEOM
  lane following `program.md` to find out what binds it lands on a
  section that is not there — and the register it is being pointed at
  is 87 rules of operational discipline that the other three programs'
  lanes are given.
- **VGEOM is the only one of the four with no written charter test.**
  The other three each state the sentence that separates them from
  their siblings (*the word a fact is spelled in*, *state that outlives
  the frame that made it*, *no viewer behaviour changes at all*). VGEOM's
  nearest equivalent is its Order's *"every row here is a non-finite
  value reaching a place that assumed it could not"*, which describes
  the CURRENT slate rather than the program — it is false of
  `the-one-free-transform-is-the-only-total-door-in-camera` and of
  `viewer-array-lowered-vector-ops-escaped-the-hand-rolled-sweep`,
  both on that slate. A row arriving from outside has nothing to be
  tested against; the `view/cut-residue` lane assigning ten unclaimed
  files had to reconstruct VGEOM's test from `program.md`'s description
  (*"the path from a document's geometry to the picture and to the
  figures printed beside it"*) because the charter it was told to apply
  does not exist.

## What resolving it looks like

Restore both sections to `work/vgeom/plan.md`. §The register is
verbatim from any sibling (the three copies agree byte for byte, which
is the property the by-reference choice exists to keep). §Charter is
VGEOM's own to write and is not a copy — one sentence that separates
this program from VNEWS, VSEAM and VDOC, applied the other way as the
siblings do, against a row on the slate that does not meet it.

**Not fixed from here**: `work/vgeom/plan.md` is VGEOM's, the charter
sentence is its orchestrator's to write, and a lane inventing one would
be the same defect as the cut writing a fence nobody re-derived
(`work/view/plan.md`, *a fence written in the same commit as the
program it fences has no independent authority*). The §The register
restoration is mechanical and could ride any VGEOM PR.


## Closed (2026-09-21) — both sections restored, and the posture with them

`work/vgeom/plan.md` carries §Charter and §The register again, and a
third section the row did not ask about turned out to be the same
defect: **§Review posture had been overwritten with the template for a
newly opened program.**

The 2026-09-20 cut applied the new-program shape to VGEOM, which is a
parent of that cut rather than a child of it. `docs/MODEL-AB-LOG.md`'s
own entry for the cut names which programs the v7 triage question is
open for — EMIT, GATHER and FIT, the three it opened — in the same
sentence that says *"WIRE and VGEOM are NOT closed and keep their bands
3700-3799 and 5300-5399"*. The roster's 2026-09-17 clause is explicit
the other way: *"All four inherit VIEW's posture verbatim … no duals
and no row recorded … Each program's `plan.md` §Review posture states
it."*

So the corrected text asserts nothing new. It restores what the roster
already records, quotes it, and says where the template came from.

**The head line was stale the same way.** *"Nothing dispatched"* was
true on 2026-09-20 and false by 2026-09-21, when #2967 closed two rows
and #3000 four more. Re-pointed at the log's tail.

The class is one the register now names: **a fence, a posture or a
status written by a cut has no independent authority.** Nothing
re-derives what a cut writes, and this one silently reopened a question
Ev had answered twice.

---
id: an-operand-slot-is-re-pointed-by-the-slot-door
kind: issue
title: Stage 2 FORK-4: may an edit re-point an operand slot (REFERENCES DM6)?
status: open
opened: 2026-10-07
priority: P0
cost: E
needs_ev: true
refs: [d10-one-way-to-say-intent-is-unbuilt, no-docedit-splices-a-deleted-node]
---

REFERENCES DM6 rules that no edit rewires a live node's inputs. After stage 2 an operand is a slot holding a variable, and stage 1's slot edits already re-point every other kind of slot. Raised as FORK-4 by the stage-2 spec (`docs/INTENT-STAGE2-SPEC.md` §11, PR 4216). Unit B (`operands-are-reads`) keeps DM6 until this is ruled.

A designer pair agreed on the first reports. An operand slot is written by the one slot door, under the checks the insert door and `SetMembers` already make, and reports the names it strands rather than refusing them. DM6 becomes "no edit infers a re-point". The question and both reports are in the `[ev]` PR. Ev's answer closes this row and rewrites DM6.


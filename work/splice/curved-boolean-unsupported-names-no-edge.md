---
id: curved-boolean-unsupported-names-no-edge
kind: issue
title: CurvedBooleanUnsupported names the face and its kind but not the edge read against it, unlike EdgeCarrierUnsupported
status: open
opened: 2026-10-26
priority: P3
cost: E
refs: [an-analytic-edge-against-a-nurbs-face-has-no-crossing-lane]
---


Filed by the SPLICE orchestrator from the design weighing on PR 5727.

`BooleanError::CurvedBooleanUnsupported { operand, face, kind }` at the
curved arm names no edge. With the operand edge gate gone it is a pair
refusal, so it should carry the edge too, as `EdgeCarrierUnsupported`
does. PR 5723's review had to instrument the arm with an `eprintln` to
learn which edge each demo wall reaches. Ride it with whichever row
next edits the curved arm.

Moved from SPLICE to SPLICESEAM on 2026-10-28, by `git mv` in SPLICE's split along its seams (Track size; Ev, in chat, 2026-10-28).

Moved from SPLICESEAM to SPLICE on 2026-11-02, by `git mv`, when SPLICESEAM merged back into SPLICE: the track budget now counts only the P0, P1 and unbanded rows a sitting dispatches (Ev, in chat, 2026-11-02), so the four tracks of SPLICE's 2026-10-28 split came in under one budget.

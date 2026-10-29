---
id: d10-hold-reread-2026-10-29
kind: issue
title: Re-read of the rows held on the D10 refactor hold (2026-10-29, against main at 9ab1fa00)
status: closed
opened: 2026-10-29
closed: 2026-10-29
refs: [d10-one-way-to-say-intent-is-unbuilt]
---

Every row whose `blocked_on` named `d10-one-way-to-say-intent-is-unbuilt`
on main at `9ab1fa00` was read against main. There were 11; the rest of
the hold's rows had been released at earlier stage closes. Built at the
time of the read: D10 stages 1–5. Stage 6 is in flight
(`intent-stage6-is-built`), and the exit walk's declared-rung row
(`a-glued-pair-is-re-read-through-the-declared-rung-which-bridges-in-band`)
is open. Each row carries a dated **Re-read** line citing the code.

Totals: 8 released, 3 re-parked, 0 closed, 0 kept on the hold. No row
needed stage 6.

| row | program | disposition | reason |
|---|---|---|---|
| `parametric-polygon-loop-certifies-nothing` | nurbs | released → open | Stage 1 (`polygon_expr` takes `Formula`, `program.rs:3104`) and stage 4's symbolic tier are both built. Re-measure first |
| `a-two-carrier-curve-touch-has-no-bounded-locus` | contacthold | released → open | Records and at-rest findings are rebuilt. The midpoint read is still `census.rs:9688`. The ends are independent of the declared posture |
| `an-edge-face-record-off-a-line-and-a-plane-has-no-curve-touch-lane` | contacthold | released → open, `design` | D10 ground is built. The open question is the row's own design question, what the lane certifies for an edge. Released on the INTENT orchestrator's call |
| `census-face-pair-confirm-is-a-trial-ladder` | contacthold | re-parked → declared-rung row | Rung 2 sends an in-band pair to the declared rung (`census.rs:9712`). That is the rung the row decides |
| `an-escalated-census-finding-names-no-pair` | contacthold | released → open | Stage 5 is built. `CensusEscalated { cause }` still names no pair (`validate.rs:1967`) |
| `the-per-instance-tier-3-gate-reads-every-assembly-face-twice` | importgate | re-parked → `step-import-anchor-rows-are-recorded-and-not-linted` | The premise is an import that has become a document operation. That is not D10's build, and that row is where it is asked. Still `design` |
| `contact-verify-spells-the-orientation-and-second-order-margins-beside-their-homes` | predtail | re-parked → declared-rung row | Its sites survived stage 4 (`contact_verify.rs:553`, `:634`, `:651`). The `declared` posture (`:291`) is the declared-rung row's question |
| `add-parameter-create-owes-the-name-doors-sentence` | vnews | released → open | Carried over to `VarName`: `properties.rs:317`, `:335` |
| `the-add-parameter-create-button-drops-a-count-it-cannot-carry` | vnews | released → open | The same chain on the variable form (`properties.rs:344-350`) |
| `a-drag-on-a-deleted-nodes-slot-says-the-slot-is-missing` | vnews | released → open | `driver_of` still answers `NoSuchSlot` (`session.rs:232`), and the pane still says *deleted* (`properties.rs:1249`) |
| `a-vector-unit-pick-is-one-undo-per-component` | vnews | released → open | Still one `SetSlotUnit` per component (`properties.rs:911-917`) |

---
id: split-and-chord-coincidences-leave-two-records
kind: issue
title: The split's ON section face and the chord join's tangent plane decide no face-vs-tool coincidence, so they mint a second record of a carrier an operand face already wears; [coincidence.door] needs the decision, its cells and its margin named before they can unify
status: open
opened: 2026-11-03
priority: P2
cost: M
design: true
---


Filed by the ONEDOOR orchestrator on 2026-11-03, from the `coincidence-leaves-one-surface-record` unit (`onedoor/one-surface-record`), which stopped these two producers with reasons.

**The split.** It decides vertices ON the tool plane. It never decides that an operand face lies in the plane. When it re-charts a section face, `Rechart::new(plane_for(side), …)` mints a fresh plane, while an operand face lying in the plane keeps its own key.

**The chord join.** Its tangent section mints an auxiliary `plane_key`, which ends orphaned when the section re-chart runs.

To state `FaceSurface::Shared` on the operand's key, each would need a new face-vs-tool decision. The ratified text settles neither which cells its `Coincidence` row names nor which margin decides it: the vertex margins, or a plane-ladder reading against the tool.

**Why P2, not P1.** The lane checked the result gate on the whole topo suite and no adjacent pair of equal records survives today: planar merges always merge or refuse. So this is a latent second record, not a live one, and nothing reads it wrongly today.

**`design`.** The decision and its row need weighing (designers first) before a lane builds it.

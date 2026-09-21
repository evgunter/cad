---
id: chord
kind: program
title: CHORD — the tessellator's arithmetic and its certificates: the chord counts, the sizing schedules and the guards over them
status: ready
opened: 2026-09-20
area: kernel
prefix: chord/
tag: (CHORD orchestrator)
ab_band: 6900-6999
paths: [crates/mesh/src/chords.rs, crates/mesh/src/sizing.rs, crates/mesh/src/cert.rs, crates/mesh/src/nurbs_cert.rs]
keep_out: [opened by TESS's 2026-09-20 priority-seam cut (Ev, in chat) per work/README.md Track size - TESS keeps the rows where the mesh LIES or REFUSES and its band 5100-5199, this program takes the arithmetic beneath them, the two share crates/mesh whole so neither fences the other and a unit crossing announces the seam in its PR, crates/geom-brep is CHART's for the pcurve side and props/ is PROPS' - the Approx-tolerance row sits on both and rides whichever opens first]
priority: P1
---

The layer beneath TESS's refusals: **what the tessellator computes a
count from, and what certifies the count.** Every chord-count arm
multiplies a certified sup by a span in plain `f64` and ceils it, with
`next_up` on the sup as the only outward pad; the three tessellation
lanes have no shared core; the chart-frame triple is hand-rolled with
no length decided; and the certificate rows above them carry ceilings
with no measured floor beside them, so a bound that silently went
vacuous would read the same as one that held.

Split from TESS because the two answer different questions. TESS's
rows say *this body cannot be meshed, or is meshed wrongly*; these say
*the number we meshed it at was reached by arithmetic nothing holds to
account*. The first is P0 and the second is the architecture under it,
which is why one budget could not hold both.

Charter and order: `work/chord/plan.md`; narrative in
`work/chord/log.md`.

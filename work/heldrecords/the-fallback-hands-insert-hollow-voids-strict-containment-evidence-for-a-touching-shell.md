---
id: the-fallback-hands-insert-hollow-voids-strict-containment-evidence-for-a-touching-shell
kind: issue
title: The containment fallback hands insert_hollow_voids Probed(In) evidence, which its door reads as strict containment, for a shell that touches the outer shell and that the parting then opens
status: open
opened: 2026-11-25
priority: P3
cost: E
refs: [a-tube-touching-a-ball-from-inside-along-its-rim-refuses-the-extent-scan]
---

Filed by INSIDE's orchestrator from the two-designer weighing of
`work/inside/a-tube-touching-a-ball-from-inside-along-its-rim-refuses-the-extent-scan.md`
(2026-11-25). Unsure it is a defect; no wrong body ships from it.

`boolean/ops.rs`'s containment fallback hands
`voids::insert_hollow_voids` the evidence
`VoidContainment::Probed(SolidContainment::In)` from the shell
witness's verdict. The door reads that as strict containment (its
`NotStrictlyContained` refusal). For a shell that touches the outer
shell (a keel on the floor, a tube rim on the sphere) the shell is not
strictly contained, and the parting then opens the cavity it just
grafted. Either the evidence type should say what it is (a witness's
verdict, touches left to the parting), or the fallback should graft
shells and let the parting and the piece sort decide.

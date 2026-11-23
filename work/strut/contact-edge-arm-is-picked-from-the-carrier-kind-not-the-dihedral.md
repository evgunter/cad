---
id: contact-edge-arm-is-picked-from-the-carrier-kind-not-the-dihedral
kind: issue
title: sweep: attach_contact picks seam/transverse/smooth from the carrier KIND where the sweep verbs ask classify_dihedral first
status: open
opened: 2026-09-13
priority: P1
cost: D
---



## Finding

`attach_contact` (`crates/sweep/src/blend/surgery.rs`, the `is_seam` /
`transverse` flags read off the `ContactCarrier` just below the endpoint
reads) chooses which description ARM a contact edge takes — seam,
transverse `Intersection`, or the must-carry rule's smooth arm — from
the carrier's KIND: `SeamArc` is a seam, `Chord` and `TransverseArc` are
transverse, everything else is smooth. The two sweep verbs decide the
same question by METERING it: `extrude.rs`'s strut arm (`sweep_loop`,
`classify_dihedral(&s_prev, &s_next, mid, w_norm, band)`) and
`revolve/upgrade.rs::upgrade_intersection` (`classify_dihedral(&surf1,
&surf2, data.witness, data.extent, band)`) ask the first-order
classifier first — Transverse upgrades to `Intersection`, Smooth
descends to `must_carry_over_edge`, in-band escalates — and only then
store. Here the structural pair decides what a rule could decide, and
a carrier mis-tagged `TransverseArc` is described `Intersection`
unmetered (the transverse arm's own comment records that a cut-off arc
mis-described as a TANGENT intersection certifies and passes tier 3,
which is the cost stated from the other side).

Both BLEND-14 reviewers raised it (R1 Q1, R2 S1) and the unit did not
act on it: BLEND-14's spec fixed the seam and transverse arms as
ratified and `dihedral.rs` as untouched, and routing the arm CHOICE
through `classify_dihedral` is a design question for the program — the
surgery would then meter a first-order dihedral per contact edge (the
K cost, and a fourth site of the `classify_dihedral → must_carry` pair
BLEND-9 counted at two) where today it meters nothing on the seam and
transverse arms.

## Disposition

Open on BLEND's slate for the program to decide; not this unit. The
smooth arm is metered as of BLEND-14; what is unmetered is the CHOICE
between arms.

## Re-homed at BLEND's exit (2026-09-17)

Filed by BLEND unit 14's fix pass (a program decision, not acted on) and merged with that unit on 2026-09-17, after the cut branch was drawn; moved here at BLEND's exit walk. `attach_contact` is `crates/sweep/src/blend/surgery.rs`, CARVE's ground, and the walk's evidence names this row as the description rule's remaining structural decision.

---
id: assembly-recourse-omits-the-transverse-cap-open-chain
kind: issue
title: FILLET3_ASSEMBLY_RECOURSE's open clause omits the transverse-cap chain its sibling endorses
status: open
opened: 2026-09-08
---

Found by the unit-3 sweep over `ALL_RECOURSES`
(`crates/sweep/src/blend/mod.rs:1425`) for sentences that restate a set
the code holds elsewhere. Not a dead recourse — following the sentence
succeeds — and the same shape as
`blend-recourses-under-describe-their-doors` §1: a sentence naming a
strict subset of what its door admits.

## The two sentences disagree about one door

`FILLET3_ASSEMBLY_RECOURSE` (`crates/sweep/src/blend/mod.rs:709`) names
the open chains that carve as

    open chains [that] are single plane–plane links ending at
    fully-requested trivalent corners

`FILLET3_CORNER_RECOURSE` (`crates/sweep/src/blend/mod.rs:634`), on the
same door, names a second termination for a second support pair:

    or, for a straight edge between a cylinder and a plane or cylinder
    sharing its ruling, in TRANSVERSE CAPS (plane faces perpendicular
    to the ruling), where the band is cut off in the cap's own section
    of it, on either material side

That second door exists and carves: `BlendArm::CylinderPlaneCylinder`
and `BlendArm::CylinderCylinderCylinder`
(`crates/sweep/src/blend/arms.rs:178`, `:174`) are the ruled arms, and
`crates/sweep/tests/fillet_h7_transverse_cap.rs:305`
(`the_rod_with_a_flat_fillets_both_creases_at_the_prism_closed_form`)
carves a rod's two cylinder–plane creases between transverse caps
through the assembly door.

So a caller refused at the assembly gate reads that only a plane–plane
link carves, and does not try the ruled crease that would have.

## What it costs and what it does not

The refusal is honest about WHY it refused; only the endorsement is
narrow. A caller who reaches `FILLET3_CORNER_RECOURSE` instead reads
the wider door, so the omission bites exactly the caller whose refusal
routes to the assembly sentence.

## The decision owed

Whether the assembly sentence gains the transverse-cap clause its
sibling already carries, or the two are derived from one statement of
the surgery's open-chain inventory. Unlike the arm table, that
inventory is held as match arms rather than as a roster
(`BlendArm::ALL` and `ALL_RECOURSES` are the module's only two
enumerable rosters), so there is nothing to check a sentence against
today — a row would have to build the chain and read the outcome, as
`blend_recourse_followability` does.

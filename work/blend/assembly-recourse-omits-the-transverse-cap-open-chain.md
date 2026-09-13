---
id: assembly-recourse-omits-the-transverse-cap-open-chain
kind: issue
title: FILLET3_ASSEMBLY_RECOURSE's open clause omits the transverse-cap chain its sibling endorses
status: review
opened: 2026-09-08
pr: 2141
branch: blend/3-spine-recourse
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

## Closed

Closed on PR 2141, the same PR that filed it — the style review read
the two findings as ONE CLASS rather than an item and its sibling, and
that reading is what closed them together.

**The class**: a recourse sentence that names a strict subset of what
its door admits. Not a dead recourse — following any of these sentences
succeeds — so the followability bar FILLET-E2 set cannot see it. What
sees it is asking the door what it tests, and in what ORDER, and
checking the sentence against that.

Three sentences in `crates/sweep/src/blend/mod.rs` carried it, and all
three are fixed on that PR:

- `FILLET3_SPINE_KIND_RECOURSE` named two of eleven pairs. It now names
  the four support KINDS the arm table traces and then the two families
  those kinds may meet in — kinds first, because the door tests kind
  membership (`coaxial_arm`/`ruling_arm`, `Meridian::trace`) before it
  decides the coaxiality hypothesis. Tied to the table by
  `verbs_arms2_arms::the_spine_kind_recourse_names_a_family_for_every_arm`.
- `FILLET3_ASSEMBLY_RECOURSE` — this item — named only the plane–plane
  link at a fully-requested trivalent corner. Its open clause now names
  the RULED link too, ending at transverse caps, conditioned as OQ6 and
  `fillet3_cap_transverse` state it. Held by
  `review_blend3_r3_probes::the_ruled_crease_carves_and_all_three_sentences_name_it`,
  which carves the rod's cylinder–plane creases and then reads the
  clause.
- `FILLET3_GEOMETRY_RECOURSE` named "planes (for a fillet's rim, also a
  sphere cap)" as the surgery's support forms while cylinder and cone
  supports carve. It now names the same four kinds
  (`BlendArm::kinds`), and the same probe row holds it.

The sibling disagreement this item reported — `FILLET3_CORNER_RECOURSE`
endorsing the transverse-cap termination that `FILLET3_ASSEMBLY_RECOURSE`
withheld — is therefore gone by the assembly sentence widening to meet
the corner one, not by the corner one narrowing.

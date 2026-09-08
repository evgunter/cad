---
id: blend-recourses-under-describe-their-doors
kind: unit
title: Two blend recourse sentences under-describe the doors they endorse
status: dispatched
opened: 2026-09-04
branch: blend/3-spine-recourse
---

Neither of these is a DEAD recourse: following either sentence
succeeds, so both pass the followability bar FILLET-E2 set. What is
wrong is narrower and still worth a decision — each names a strict
subset of what its door admits, so a caller who reads it walks away
believing the kernel does less than it does.

## 1. `FILLET3_SPINE_KIND_RECOURSE` names two of eleven pairs

`crates/sweep/src/blend/mod.rs`. The sentence:

    use a chain whose support pairs have analytic blend arms
    (plane–plane or plane–sphere); other pairs need the canal-surface
    approximating blend, which is not implemented

The refusal's OWN payload rosters nine more admitted pairs — `sphere–cone`,
`sphere–sphere`, `cone–plane`, `cone–cone`, `cylinder–cone`,
`cylinder–sphere`, `cylinder–plane`, `cylinder–plane(∥)`,
`cylinder–cylinder`. A caller told "plane–plane or plane–sphere" will
not try the cylinder pair that would have worked.

Measured by
`sweep/tests/blend_recourse_followability::the_spine_kind_recourse_names_an_analytic_pair_that_builds`,
which follows the sentence to a build and therefore does NOT go red on
this.

## 2. `FILLET3_ASSEMBLY_RECOURSE` omits the plane–cylinder closed rim

Same file. The sentence names, as the closed chains that carve,
"circular plane–sphere rims". A cylinder's plane–cylinder TOP rim carves
too, at r = 0.1, tier-3 valid — witnessed by
`sweep/tests/review_fillet_e2_probes::open_plane_sphere_arcs_meet_the_chain_gate_and_a_plane_cylinder_rim_carves`,
whose closing assertion is exactly that the sentence does not name the
rim it just built.

## Why this was not fixed in FILLET-E2

Both are door-inventory questions, not recourse-followability ones. The
honest fix needs the admitted set stated once, in one place, and both
sentences derived from or checked against it — otherwise the roster is
restated a third time and drifts a third way (the same failure
`ALL_RECOURSES` was created to end). Widening `FILLET3_SPINE_KIND_RECOURSE`
inline would duplicate the payload's own roster in prose.

## The decision owed

Where does the admitted-pair roster live, and does a recourse sentence
quote it or point at it? Whichever is chosen, the pin the class asks for
is the same: the second request executed, and the outcome asserted.

## Status of §2 (FILLET-H4 fix pass, PR 1752)

Answered by rewording: `FILLET3_ASSEMBLY_RECOURSE`'s closed clause now
names "circular rims between two coaxial revolution surfaces" — the
plane–cylinder top rim included — with the repaired-pole exception
(README A3-2) stated in the sentence. §1 (the spine-kind sentence) is
untouched by that PR and stays this item's open half.

## Pointer (FILLET sweep, 2026-09-06)

Former `refs` `recourse-sentences-owe-followability-pin` named FILLET items now deleted with that program's directory: `recourse-sentences-owe-followability-pin` — FILLET E2, closed (PR 1753). Recoverable at the sweep SHA in `docs/DOC-LEDGER.md` (sweep 7).

## Re-homed (2026-09-06)

Moved from `work/issues/` to `work/blend/` in the tracker-wide cut of 2026-09-06 (Ev's direction, in-chat), which read every open `work/issues/` file and every open code-quality row against every live program's `paths` and opened four programs for the ground none covered. Id, body and header are unchanged except as noted; the directory is the claim (`work/README.md`). The open half (§1) is a `blend/mod.rs` recourse-table decision, BLEND's ground.

## Landed

§1 closes on branch `blend/3-spine-recourse`, and with §2 already
answered the item closes whole.

`FILLET3_SPINE_KIND_RECOURSE` no longer enumerates pairs. It names the
two FAMILIES the arm table has — a rim between two coaxial surfaces of
revolution, which blends to a torus band, and a straight edge between
two supports sharing one ruling direction, which blends to a cylinder
band — and points the reader at the pair roster the refusal's own
payload carries (`crates/sweep/src/blend/battery.rs:914`,
`ARM_ROSTER`). The eleven pairs are therefore still in ONE place: the
roster, checked against `BlendArm::ALL` by
`verbs_arms2_arms::the_refusal_roster_names_every_arm_and_nothing_else`.
That is the decision the item asked for — the sentence POINTS at the
roster rather than quoting it, so there is no third copy to drift.

The tie the item asked for is
`crates/sweep/tests/verbs_arms2_arms.rs::the_spine_kind_recourse_names_a_family_for_every_arm`:
every fillet arm in `BlendArm::ALL` maps by its band kind (the band
half of `BlendArm::name`) to a family, and the sentence must carry that
family's phrase. A third band goes red at the map itself, so a family
added to the table without its clause in the sentence cannot ship.

`blend_recourse_followability::the_spine_kind_recourse_names_an_analytic_pair_that_builds`
now follows the sentence once per family — the dome's plane–sphere rim
and the cube's plane–plane chain — instead of following its former
literal pair list.

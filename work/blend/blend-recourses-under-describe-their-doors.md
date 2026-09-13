---
id: blend-recourses-under-describe-their-doors
kind: unit
title: Two blend recourse sentences under-describe the doors they endorse
status: closed
opened: 2026-09-04
branch: blend/3-spine-recourse
pr: 2141
closed: 2026-09-08
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
answered the item closes whole. The style review widened it: the same
defect sat in THREE sentences, so all three are fixed here and
`work/blend/assembly-recourse-omits-the-transverse-cap-open-chain.md`
closes with this one.

**The decision: a recourse sentence describes its door's TEST, in the
door's own order.** The first fix named the two spine-symmetry families
and stopped there, which was wrong for a reason worth recording: the
door tests KIND membership FIRST — `coaxial_arm`/`ruling_arm` match on
stored variants and `arms::Meridian::trace` has rows for plane, sphere,
cylinder and cone and none for a torus — and only then decides the
shared-axis hypothesis (`fillet3_support_coaxiality`). A sentence
leading with "a rim between two coaxial surfaces of revolution"
therefore endorsed the spool's coaxial torus–plane rim, which the door
refuses on kind: the refusal's own advice recommended the request just
refused.

So `FILLET3_SPINE_KIND_RECOURSE` now names the four KINDS, then the two
FAMILIES those kinds may meet in, then what is left over (a torus or
any other stored surface, and the one coaxial pair with no row — two
coaxial cylinders). The eleven PAIRS stay in the refusal's own payload
roster (`crates/sweep/src/blend/battery.rs:914`), which the sentence
points at rather than copying.

Four words are a KIND SET, not a roster, and they now have one home:
`BlendArm::kinds` (`crates/sweep/src/blend/arms.rs`), checked against
the pair half of `BlendArm::name` so the two spellings cannot drift.

**The tie** is
`crates/sweep/tests/verbs_arms2_arms.rs::the_spine_kind_recourse_names_a_family_for_every_arm`:
for every arm, both of its `kinds()` must appear in the sentence's kind
clause (its first `;`-delimited segment) and its family — read off
`is_coaxial_torus` / `is_ruled` / `is_plane_plane`, never off a re-split
of the display name — must be named after it. It collects failures and
asserts on the SET. Its negative half is that `torus` must NOT appear
among the named kinds, which is what stops the defect being re-opened
by widening the list.

**The sibling sentences.** `FILLET3_ASSEMBLY_RECOURSE`'s open clause
now names the ruled link ending at TRANSVERSE CAPS beside the
plane–plane link at a trivalent corner, and `FILLET3_GEOMETRY_RECOURSE`
names the same four support kinds instead of "planes (for a fillet's
rim, also a sphere cap)". Both are held by the reviewer's carve probe,
adopted at `crates/sweep/tests/review_blend3_r3_probes.rs`.

`blend_recourse_followability::the_spine_kind_recourse_names_an_analytic_pair_that_builds`
follows one representative request per family — the dome's plane–sphere
rim and the rod's cylinder–plane creases, the ruled door's own pair —
and its doc now says that is what it pins, leaving completeness over
the arm table to the map row.

`test_support::spool`'s doc said the fixture's spine "is neither a line
nor a circle"; its torus–plane rim's spine IS a circle, and the refusal
is on kind. Corrected.

## Closed (2026-09-08, PR 2141)

§1 closed with the class: `FILLET3_SPINE_KIND_RECOURSE` names the four
kinds the arm table traces and then the two families, in the door's
order (the review corrected the first cut, which named families
alone and endorsed a coaxial torus–plane rim the door refuses on
kind); `FILLET3_GEOMETRY_RECOURSE` names the four kinds;
`FILLET3_ASSEMBLY_RECOURSE`'s open clause names the ruled link ending
at transverse caps. One row maps every fillet arm to a family through
`is_coaxial_torus`/`is_ruled`/`is_plane_plane` and its kinds through
`BlendArm::kinds()`; the followability row follows one request per
family. Four stale negative asserts of the class in the suites turned
positive (the last one found by CI, not the sweep — said in the PR).

---
id: topo-mints-indeterminates-outside-the-funnel
kind: issue
title: topo mints Indeterminates outside the funnel after a definite sign, in two spellings, at eleven shipped sites
status: open
opened: 2026-09-20
priority: P0
cost: D
---



## What

`topo` builds `Indeterminate`s of its own after a DEFINITE sign, at
sites the funnel never sees — so the escalation (or contradiction) the
caller receives is on no frame's escalation log, and a consumer reaches
it only through the op's error arms. Found by the PROPS
escalation-channel unit (PR 2928), which retired exactly this shape at
the eight `geom-brep` sites it owned and then swept for the SHAPE rather
than the spelling. These are `curved`'s ground, so they are filed.

**Spelling 1 — the struct literal**, `crates/topo/src/boolean/contact_verify.rs`,
**seven** shipped mints (an eighth `Indeterminate {` in that file is a
fixture inside `#[cfg(test)]`):

- `contact_rest_senses_opposed` — the `CarrierRelation::SameOriented`
  arm → `ContactRefusal::Contradicted`
- `contact_rest_ladder_invariant` — the `CarrierRelation::Distinct` arm
  → `ContactRefusal::Escalated`
- the residual arm, under the decided predicate's own name
  (`contact_tangent_on_1` / `_on_2`), after `Ok(Sign::Positive)` →
  `Contradicted`
- `contact_tangent_opposed`, after `Ok(Sign::Positive)` → `Contradicted`
- `contact_tangent_independent` → `Contradicted`
- `contact_tangent_parallel` → `Contradicted`
- `contact_tangent_second_order` → `Escalated`

**Five of the seven are `Contradicted`, not `Escalated`** — see the note
below, which is the harder half.

**Spelling 2 — a local `invalid(band, name)` helper** returning an
`Indeterminate` (or a fault wrapping one). `topo` holds four separate
definitions of this helper, and the census PR 2928 added over
`geom-brep` — a scan for the `Indeterminate {` literal — would not see a
single one of them even if it were pointed at this crate. On curved's
ground:

- `crates/topo/src/census.rs` — `undecided.push(invalid(band,
  "material_wedge_side"))` after a definite `DihedralClass::Transverse`,
  and a second at the `cause: invalid(band, name)` site above it.
- `crates/topo/src/boolean/contain.rs` — `Ok(Sign::Negative) =>
  Err(invalid(band, "bool_contact_arc"))`, plus the
  `ContainError::Escalated(invalid(band, "bool_contact_edge"))` site.
- `crates/topo/src/boolean/sectors.rs` — `invalid_escalation(band,
  "bool_dir_same")` after `Ok(Sign::Zero)`, and `"bool_faces_parallel"`.
- `crates/topo/src/splitting/rules.rs` — two: a hand-built
  `split_sector_extent` refusal after a definite sign, and an
  `enters_material` refusal minted after a definite
  `EntersMaterial::Tangent`.

(`crates/topo/src/sector_shape.rs` holds two more of spelling 2, on
PROPS' ground:
`work/props/sector-shape-mints-indeterminates-through-an-invalid-helper.md`.)

## Note on the payload, which is the harder half

Five of the seven `contact_verify` mints are not escalations at all. A
DEFINITE `Positive` residual is a CONTRADICTION of a declared contact,
and dressing it as `MarginDiag::Invalid` says "the margin was poison"
about a margin that was measured.
`crates/geom-brep/src/props/quad.rs` names this class in prose — *"turns
into `Indeterminate{margin: Invalid}`: a MIS-TYPED refusal"*.

**This is also a correction to PR 2928's own sweep, and it is the
sharper finding.** That sweep keeps a group of variants on the reason
that they "carry the classifier's diagnostic as evidence"
(`ContactRefusal::Contradicted` among them). For these five the
diagnostic is FABRICATED — an `Invalid` nobody classified — so for them
that reason is hollow, and what actually keeps the variant is the
declaration and the steer it carries beside the diagnostic, not the
diagnostic. `work/props/indeterminate-error-arms-sweep.md` now says so.

So the fix is not only to route the mint through a funnel door; it is to
decide, per arm, whether the fact being reported is an indeterminacy
(route it, and it belongs on the log) or a definite contradiction (give
it a payload that says what was measured, and it belongs on no
escalation log at all).

`geom_core::k_stats` gained `decide_positive`, `decide_nonzero` and
`gate_measured` in PR 2928; those are the doors for the arms that really
are gates.

## A third file, found by EMIT (2026-09-23)

`crates/topo/src/boolean/plane_eq.rs` holds seven more `Indeterminate {
margin: MarginDiag::Invalid, .. }` struct literals minted AFTER a definite
sign — spelling 1, in a file the list above does not name (no open
program's territory claims it):

- `oriented_plane_eq_verdict`: `bool_plane_parallel` `Ok(Negative)`
  → `Escalated`; `bool_plane_orient` `Ok(Zero)` → `Escalated`;
  `bool_plane_offset` `Ok(Zero)` → `PlaneEqError::Undeclared` (rung 4).
- `declared_rung`: `bool_plane_parallel` `Ok(Positive)` → `Contradicted`,
  `Ok(Negative)` → `Escalated`; `bool_plane_orient` `Ok(Zero)` →
  `Escalated`; `bool_plane_offset` `Ok(Positive | Negative)` →
  `Contradicted`.

The rung-4 site's payload is user-visible and the same shape as the note
above. Measured on `tests/fixture/pr4.rs`'s sliding union, with B's
transform moved to x = 1.0 so B's −x wall rests on A's +x wall: the union
refuses, as the contact contract requires (the declaration covers only
the flush planes), and the refusal reads *"… coincident with opposed
orientations (resting contact) … predicate 'bool_plane_offset'
indeterminate: margin is invalid (NaN or a poisoned enclosure) …"*
about a margin that DECIDED exactly Zero. The same sentence appears on
the undeclared flush case at x = 0.5 and 0.99. The refusal is correct;
its diagnostic states a poison that did not occur.
`work/stack/certified-lane-non-real-contract-audit.md` records the same
payload from the M10-DI review as a member of its class; this is its
minting site.

## Also reached from a union's declaration channel (GATHER, 2026-09-24)

`declared_rung`'s `Contradicted` arm (`bool_plane_offset`
`Ok(Positive | Negative)`) is user-visible through a union too. R2's
`r2_p7` declares `a`'s x = 1 wall Rest against `far`'s x = 6 wall.
In the four member orders that feed the pair while the wall is still a
row, the union refuses `ContactContradicted`, and the refusal reads
*"… contradicted by predicate 'bool_plane_offset' indeterminate: margin
is invalid (NaN or a poisoned enclosure) …"* about two planes 5 units
apart whose offset DECIDED nonzero. Pinned (by kind, not text) in
`docm8_flat_merged::a_member_face_inside_a_later_member_keeps_the_vanished_refusal`.

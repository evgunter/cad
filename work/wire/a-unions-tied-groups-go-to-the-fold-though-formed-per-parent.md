---
id: a-unions-tied-groups-go-to-the-fold-though-formed-per-parent
kind: issue
title: A union leaves its tied parents' face and seam groups to the fold, though it forms them per parent now
status: closed
opened: 2026-10-27
priority: P3
cost: M
closed: 2026-10-28
---


Filed by WIRE's `wire-seamgroups` lane (the PR that keys seam groups
per candidate). Found reading the group record; not measured on a
fixture.

## What

`crates/editor-core/src/names/emit_union.rs` `name_by_parents`: the
`record` closure sends every group whose parent is tied to
`Rederived::by_fold`, so the diagnosis rung reads the fold's
entity-followed groups for that base instead of the union's own. Its
comment, and `groups.rs` `Rederived`'s doc, give the reason: "A group is
formed by parent NAME, so tied parents share one and it holds no one
parent's count."

That reason no longer holds. The face groups iterate `Parents::all`,
one group per parent (each a set of member faces), and the seam groups
are keyed by the two parents' member-face sets beside the base. A tied
parent's group is one parent's, and its count could be on record as the
pair boolean's now is (`name_boolean_edges` records each seam group with
`GroupRecord::record`).

## What it needs

Decide whether a union's tied groups should be recorded as its own
(`record`, as untied ones are) or whether the fold's reading is still
wanted for another reason; then make the code and the two comments say
the same thing. A witness: `lib_g14_split_walls`'s
`a_unions_seams_of_a_tied_face_tie_under_the_bare_name` document, with
the group-size rung queried on one of the tied seams.

## Closed

2026-10-28 (WIRE orchestrator): PR 5902 merged. A union records its tied parents' face and seam groups as its own, and `Rederived`/`by_fold` are deleted. When tied groups resize unequally the rung now declines in every member order; base's answer depended on the order. That case is pinned by `tied_groups_that_resize_unequally_decline_in_every_member_order`, which is red on base.

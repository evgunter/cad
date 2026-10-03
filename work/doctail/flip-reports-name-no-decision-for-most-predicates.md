---
id: flip-reports-name-no-decision-for-most-predicates
kind: issue
title: edit: a flip report names no decision for most predicates a subtract logs
status: open
opened: 2026-09-29
priority: P3
cost: M
---


(EDIT, found by the recourse fix pass of
`edit-refusals-short-of-the-shape-guard`, PR 3490.)

## What

A flip report (`crates/editor-core/src/resolve/mod.rs`, `FlipSubject`)
names the margin whose sign flipped by what its predicate decides,
read from one lookup (`crates/editor-core/src/decision.rs`,
`decision::words`) over each owner's words. Of the 60 predicates a
plain subtract of two blocks logs, 15 have words. The other 45 render
as "the margin of an unnamed decision". Three of those are
deliberately wordless (the invariant lane's `volume_backstop*`). The
other 42 have no words yet, and their owners are spread across crates:

- `topo` (REACH): the `bool_*` point-in-solid, contact, join, sector
  and pierce predicates, `point_in_loop_*`, `sector_*`, `dihedral_*`,
  `enters_material*`, `carrier_*`, `interval_span_*` and
  `split_edge_param_interior`.
- `sweep` (CARVE/STRUT): `extrusion_normal_component`,
  `side_planes_cosurface`.
- `geom-brep`: `newell_plane_residual`, `witness_at_mid_parameter`,
  `witness_on_surface_1`, `witness_on_surface_2`.

`crates/editor-core/tests/edit_refusal_recourse.rs`,
`every_predicate_a_subtract_logs_has_words_or_a_reason`, lists them in
`WORDLESS` under "no words yet", citing this file.

## Repair shape

Give each predicate its words beside the decision, in its owner's
table (`topo::decision_words`, or a new `decision_words` in `sweep`
and `geom-brep` that `decision::words` then consults). Then drop its
`WORDLESS` entry. The row reds a listed predicate that renders words,
so an entry left behind goes red.

## 2026-09-30 — the counts after PR 3493

PR 3493 gives `topo::decision_words` words for the containment walk's
predicates (`point_in_loop_*` among them), the sector rungs and
`split_edge_param_interior`, each read from the closed decision type the
Boolean's refusals carry. Of the 60 predicates a plain subtract logs, 24
now have words and 36 do not: the three invariant-lane ones, 32 with no
words yet, and `bool_contact_vertex`, which is raised under two
decisions (the Boolean contact sweep's vertex-to-vertex coincidence and
containment's boundary pre-pass), so no one decision's words are true of
it. `WORDLESS` lists it under its own reason. Giving it words needs one
predicate name per decision; `bool_contact_arc` is the same case,
though a subtract does not log it.

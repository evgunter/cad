---
id: named-copies-of-fixture-doors-under-other-names
kind: issue
title: 121 StableName literals in editor-core's suites spell a fixture authoring door by hand
status: open
opened: 2026-09-19
priority: P4
cost: E
---


## Finding

`crates/editor-core/tests/fixture/mod.rs` homes the name-authoring
shorthands — `minted`, `fname`, `ename`, `vname`, `rim_edge`,
`cap_vertex`, `pole`, `in_copy`. A suite that wants a one-segment name
can call one. **121 sites in 50 files outside `tests/fixture/` write
the struct literal instead**, measured on `edit/suite-helpers-one-home`
at PR #2867's fix-pass head:

```
StableName { kind: EntityKind::<K>, node: <expr>, path: vec![<one seg>] }
```

74 Face, 25 Body, 22 Edge. Each is exactly `fname` / `minted` /
`ename`. A further 9 literals carry a multi-segment `path` (a `FromA`
plus a `Fragment`, and the like), which no shorthand spells and which
are not this row's subject.

Twenty-two of the 121 sit inside a NAMED helper whose whole body is
the literal — `corpus/slots.rs::cap`, `corpus/part_select.rs::section_face`,
`display_contract.rs::face_name`, `eval4_accept_funnel.rs::local_cap`,
`m4_pr4_edits.rs::cap`, `rv_matehead_probes.rs::face_name`,
`docm6_seam_declarations.rs::wrap`, `docm8_flat_merged.rs::from_a` and
`::from_b`, `r2_m10_6_probes_interval.rs::bname`,
`edit_blend_canonical.rs::edge`, `m6_5_selection_refusals.rs::rim`,
`edit_one_predicate.rs::in_part` and `::part_face`,
`asm_r2a_mate_solve.rs::in_part`, `mate6_gather_mints.rs::dangling` and
`::in_part_in_part`, `mate6r1_shared.rs::dangling`,
`mate6r2_probes.rs::vanished`, `dm7_delete_strands.rs::instance_face`,
`rv_dm7_probes.rs::instance_face`, `blend5_rim_support_wire.rs::trim_name`.
Those are suite-local shorthands whose BODY should delegate to the
door; the rest are literals at a call site. Two further named helpers
(`bool7_shadow_exec::frag`, `m4_pr4_resolve::sideof_frag`) are bare
literals with a two-segment `path`, outside this class.

**What this row no longer holds.** Its first filing also named
`fn name1` (eight files, `fixture/pr4.rs` included), `fn in_copy` (five
files, each with `kind: EntityKind::Face` frozen where the door carries
`of.kind`) and `fn shelled` (two files). PR #2867's fix pass took all
fifteen: every one now imports the door or delegates to it, and a
census of every `-> StableName` helper outside `tests/fixture/` (49
helpers, 45 distinct normalised bodies) shows no remaining body equal
to a door's. Only the literal class is left, which is what this row is
now about.

## Why it matters

A literal cannot diverge from the door the way a named copy can, but it
carries the same cost the named copies did: the `in_copy` copies froze
`kind` to `Face` because a literal has to name a kind, and nothing told
their reader that the door propagates the master's kind instead. A
shorthand call says which door the site went through; a literal says
only what the struct's fields are.

## What a taker owes

Read each site before changing it: a literal whose `kind` is a variable
or a functional update (`StableName { kind: EntityKind::Vertex,
..edge_name.clone() }` in `display_contract`) is not this class, and
18 such sites are excluded from the count above. Delegate the twelve
named helpers to the door before rewriting the call-site literals, so a
reviewer can see the shorthand in one place per suite.

**What the pattern cannot match**: a name assembled field by field into
a `let mut`, or built by a helper that returns the literal through a
`match`. The census of `-> StableName` helpers bounds the second class
at 45 distinct bodies and finds none of that shape today.

## Territory

`crates/editor-core/tests/*` — tcost's and tint's. Filed by the
`review/helpers-rv` lane reviewing EDIT's PR #2867 and re-scoped by
that PR's fix pass; sibling of
`inline-name-table-reads-bypass-the-fixture-door`.

---
id: an-orphaned-declare-joins-the-product-root-set
kind: issue
title: The delete that orphans a Declare also re-roots it: a Declare joins the document's product root set
status: closed
closed: 2026-09-19
opened: 2026-09-19
---



(Filed by the style review of PR 2874, lane `orphan-rv`; the row that
PR builds is `a-declare-orphaned-by-a-cascade-is-never-reported`.
Citations at `f942a8dbc` merged with `d65dcdf8c`.)

## The finding

A document's product roots are its SINKS
(`crates/editor-core/src/roots.rs:239` `is_sink`), and
`roots::on_delete` (`roots.rs:221`) splices the deleted root's inputs
that its departure turned into sinks into `doc.roots`. It does not ask
what KIND they are. So the very delete that PR 2874 makes report
`Maintenance::OrphanedDeclare { declare }` also registers that
`Declare` as a product root: deleting the union of a declared union
leaves `doc.roots() == [decl]`.

Measured: `crates/editor-core/tests/dm7_delete_strands.rs`'s
`the_orphaned_declaration_is_re_rooted_by_the_same_delete` (written
on branch `review/orphan-rv` as `rv_orphan_probes.rs`'s
`rv_the_orphaned_declaration_is_re_rooted_by_the_same_delete`, and
adopted into the unit's suite by PR 2874's fix pass).

The behaviour predates 2874 — nothing in that PR touches `roots.rs` —
but it is the fact two of that PR's new sentences are written against:

- the arm's `Display` in `crates/editor-core/src/edit.rs` said
  "nothing reads the declaration until a boolean or union names it
  again"; the root set reads it, so PR 2874's fix pass re-worded it to
  "no node consumes the declaration". The sentence is now true, and
  this row is what it is true BY.
- `crates/pncad-py/pncad.pyi`'s `orphaned_declare` paragraph said "no
  longer read by anything", re-worded the same way in the same pass.

A `Declare` is not a body, so the gather yields nothing for it and the
evaluation stays clean
(`review_decl_r1::a_declare_orphaned_by_a_cascade_is_reported_at_the_delete_that_orphans_it`
asserts exactly that). What is wrong is the root set's own meaning:
A10 says the roots are the document's product solids, and a `Declare`
in that list is a root that is not one.

## The shapes a fix could take

- `is_sink`'s callers filter by whether the node can BE a product (a
  kind question the node vocabulary can answer), so the root set keeps
  its A10 meaning;
- or A10's wording admits non-product sinks and the two sentences
  above are re-worded to say "no node reads it";
- or it is ruled a non-issue and `is_sink`'s doc says why.

Ground: `crates/editor-core/src/roots.rs` — EDIT's.

## Closed (2026-09-19, EDIT orchestrator) — ruled a non-issue, E-class

**Ruling: the third shape.** A10's own text already admits the
state: "together the root set is exactly the DAG's sink set", and
"non-body roots contribute nothing" to the gather, which reads only
body-denoting roots. A mate is the standing precedent (ASSEMBLY.md
A12: "an ordinary non-body root: an isolated sink, listed, ignored by
the gather"), and an orphaned `Declare` is the same thing from the
delete on. The kind question belongs to the gather, not to
`is_sink`; filtering sinks by kind would break the sink-set identity
that makes the root invariants burden-free. `is_sink`'s doc now says
so in one paragraph, citing this row; the two sentences PR #2874
re-worded ("no node consumes the declaration") stay true by it. No
row changes, no design-page text moves. Merged as an orchestrator
E-class commit on green CI.

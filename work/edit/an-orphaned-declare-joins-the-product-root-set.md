---
id: an-orphaned-declare-joins-the-product-root-set
kind: issue
title: The delete that orphans a Declare also re-roots it: a Declare joins the document's product root set
status: open
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

Measured: `crates/editor-core/tests/rv_orphan_probes.rs`'s
`rv_the_orphaned_declaration_is_re_rooted_by_the_same_delete`, on
branch `review/orphan-rv`.

The behaviour predates 2874 — nothing in that PR touches `roots.rs` —
but it is the fact two of that PR's new sentences are written against:

- `crates/editor-core/src/edit.rs:1686` the arm's `Display`: "nothing
  reads the declaration until a boolean or union names it again". The
  root set reads it.
- `crates/pncad-py/pncad.pyi:5267` "no longer read by anything".

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

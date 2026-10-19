---
id: module-docs-on-both-sides-resolve-their-inner-links-in-the-parent
kind: issue
title: a module documented on both sides of its declaration resolves its //! links in the parent's scope, so they can resolve to the wrong item with the gate green
status: open
opened: 2026-10-15
priority: P3
cost: M
---


Found by the rustdoc-gate-red fix (`ciw/rustdoc-gate-red`, 2026-10-15).

## Measured

When a `mod foo;` declaration carries an outer `///` doc and `foo.rs`
also opens with `//!` docs, rustdoc 1.97 resolves the intra-doc links
of BOTH halves in the declaring (parent) module's scope, not in
`foo`'s. Three of the gate's failures on 2026-10-15 had this shape and
no other cause:

- `crates/geom-core/src/sym/revalue.rs` (declared in `sym.rs` with an
  outer doc): `` [`super::SymOp::Opaque`] `` reported *"no item named
  `SymOp` in module `geom_core`"*, i.e. `super` was read from `sym`;
  `` [`record`] `` was not in scope.
- `crates/geom-core/src/sym/deferred.rs`: `` [`with_deferred_session`] ``,
  its own module's function, not in scope.
- `crates/topo/src/census/curved.rs`: `` [`super::on_edge_interior`] ``
  read as `topo::on_edge_interior`.

The fix there was to spell each link crate-absolute (or unlink it).

## Why it matters past those three

A link in such a `//!` half that does NOT error resolves to whatever
the name means in the parent, which may be a different item than the
author meant, and the gate is green over it. `revalue.rs`'s
`` [`revalue`] `` was one: it resolved to the module itself (the name
in `sym`'s scope), where the prose meant the function
`revalue::revalue`.

Sweep (2026-10-15, a script pairing each `mod x;` that has an outer
`///` with the file it mounts, counting `//!` lines that carry a bare
`` [`…`] `` link): 43 files. Blind spots: inline `mod x { … }` blocks
(their docs are all outer), `#[path]` mounts whose target the script
could not find, and links written `[text](path)` (excluded, since an
explicit path is usually absolute already).

## Owed

- Decide the rule: either drop the outer `///` on such declarations
  (the module's own `//!` is its doc) or require crate-absolute paths in
  those `//!` halves; then sweep the 43 files to it.
- Optionally a `scripts/gates/` check that refuses the two-sided shape.

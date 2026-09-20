---
id: python-flush-findings-do-not-carry-their-sites
kind: issue
title: A Python FlushFinding hides the site the kernel's declare doors now depend on
status: open
opened: 2026-09-17
---


## What

A declared pair names SITED entities since EDIT-DECL
(`crates/editor-core/src/node.rs`, `Node::Declare`; DM4): each side is
a `SitedRef { at, name }`, and `at` — the operand or member the entity
is read at — is what says which side of the boolean the name belongs
to. Rust's `FlushFinding` carries both halves, so
`find_flush_candidates` → `declare_all` is total.

The Python binding carries the site but does not expose it.
`crates/pncad-py/src/py/flush.rs`'s `FlushFinding` publishes `a` and
`b` as opaque name text plus `relation`, `class_` and `rung`; the
site travels inside the value and the docstring says so explicitly
("That site is not exposed as an attribute: a name here is opaque
text, and a node id beside it would be the one part a caller could act
on wrongly").

Three things a Python caller cannot do as a result:

- **Say which member a union's refusal is about.** A union's
  `undeclared_contact` carries a finding whose two sides are now
  members of the union (`eval/wire.rs`'s `union_refusal`). The caller
  can declare it back verbatim but cannot report, log or group by the
  member — the one fact the kernel added.
- **Act on a MERGED side.** When the contact is against a face the
  fold merged, the refusal picks one constituent and carries the whole
  flat set (`NodeErrorKind::UndeclaredContact`'s `merged` field). None
  of that crosses: Python sees one pair and no indication that the
  face is a merge of several members' faces.
- **Author a declaration that is not a finding.** `Doc.declare` takes
  findings only, so a caller who knows two faces meet — and which
  nodes they are read at — has no door that takes the two sites.

## Where it lands

LIB's: `crates/pncad-py/src/py/flush.rs` and the `Doc.declare` /
`Node.declare` doors beside it (`crates/pncad-py/src/py/doc.rs`).
`python3 scripts/work.py territory --files -` reports `lib` for both.

## Why it is not this unit's

EDIT-DECL's fence is the kernel payload, its resolver and the
persisted form; the binding followed mechanically (announced on PR
#2809). Whether a site should cross as an attribute — and in what
vocabulary, given that a `NodeId` is a first-class Python value
already — is a binding-design question with its own answer.

Filed by the EDIT-DECL fix pass, PR #2809.

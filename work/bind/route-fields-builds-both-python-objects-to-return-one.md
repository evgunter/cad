---
id: route-fields-builds-both-python-objects-to-return-one
kind: issue
title: route_fields builds of and via on every call and each of its four getter callers discards one
status: open
opened: 2026-09-15
priority: P4
cost: E
---


## Finding

Raised by the full review of PR 2635 (PORT-DOORS-1) and disclosed by
that lane, which added the third and fourth instance.

`crates/pncad-py/src/py/assembly.rs`'s `route_fields` builds BOTH
Python objects a `Route` projects — the document id as a `str` and the
instance list as a `list[NodeId]` — and returns them as a pair. Two of
its callers want the pair (`assembly_err`'s gather arm, and the carried
arm before PR 2635 restructured it). The rest are `#[getter]`s that
want one field each and index the pair:

- `CarriedDeclaration::of` → `route_fields(py, …).0`
- `CarriedDeclaration::via` → `route_fields(py, …).1`
- `CarriedRefusal::of` → `route_fields(py, …).0`  *(added by PR 2635)*
- `CarriedRefusal::via` → `route_fields(py, …).1`  *(added by PR 2635)*

So reading `row.of` allocates and discards a `list[NodeId]`, and
reading `row.via` allocates and discards a `PyString`. A caller looping
over a refusal list and printing `of` pays for a via-list per row.

The cost is small and the shape is what matters: a helper whose
callers index its result is two helpers wearing one name. PR 2635 made
it worse rather than better by following the spelling already in the
file — deliberately, because inventing a second spelling for the two
new getters would have left three ways to read a route in one file, and
the lane judged a consistent wart better than an inconsistent fix in
another program's territory.

## What is owed

Split it, or make the getters ask for what they want:
`route_of(py, route)` and `route_via(py, route)`, with `route_fields`
either built from the two or retired in favour of calling both at the
one site that needs a pair. Four getters and two `assembly_err` arms
change; nothing Python-visible moves, so no tag, stub or census row is
touched and the py suite should be untouched by it.

## Sites

- `crates/pncad-py/src/py/assembly.rs`, `route_fields` — the helper.
- `crates/pncad-py/src/py/assembly.rs`, `CarriedDeclaration::of` /
  `::via`, `CarriedRefusal::of` / `::via` — the four indexing callers.
- `crates/pncad-py/src/py/assembly.rs`, `assembly_err`'s `E::Product`
  arm — the caller that genuinely wants both.

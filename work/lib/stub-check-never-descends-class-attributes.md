---
id: stub-check-never-descends-class-attributes
kind: issue
title: Python stub checking never descends into class attributes — a forgotten .pyi enum member leaves the whole suite green
status: open
opened: 2026-08-30
github: 1309
refs: [1301]
---

## From GitHub issue 1309

opened 2026-08-30, 0 comments.

**Raised by BLEND-5's review round** (PR #1301). The PyO3 mirror of a widened enum is guarded by an exhaustive `growth_tripwire` match (`pncad-py/src/py/select.rs:864`) — good shape. The hand-written stub is not: `test_stubs.py`'s `stub_names()` walks only `tree.body`, so class *attributes* in `pncad.pyi` are never descended into (its own docstring says so: "NAME-level equality of the top-level surface"), and `ty` has nothing to cross-check enum members against. Had BLEND-5's author forgotten to update `pncad.pyi:1624-1628` for `RimSupport::{Host, Mate}`, the entire Python suite would have stayed green while the stub advertised retired variants.

The author did update it correctly — this issue is the class, not an instance: the stub checker's blindness to attribute-level drift means every enum-member and method rename in the bindings relies on authors remembering the `.pyi`. A one-level-deeper walk (class attributes of the top-level classes) would close most of it.

S-QA-shaped (test-infrastructure honesty); flagged for that program's backlog rather than any blend unit.

## Home

`work/lib/` — `crates/pncad-py/tests/test_stubs.py` and `pncad.pyi` are inside LIB's `crates/pncad-py/*` territory glob and the bindings' census/audit gates are LIB's charter.

---
id: a-python-pattern-builder-copies-the-whole-pattern-per-wrap
kind: issue
title: "pncad-py: every NamePat/SegPat builder clones the whole pattern it wraps, so building a pattern N levels deep costs O(N^2)"
status: open
opened: 2026-09-30
priority: P3
cost: E
---

(Found by EDIT's name-nesting row, `edit/name-nesting-stack-safe`.)

## What

The Python pattern classes are frozen wrappers, and each builder
returns a new value: `SegPat.of(args)` extracts every `NamePat` in
`args` by clone, and `NamePat.seg(seg)` / `.path(...)` clone the
`SegPat`s they are handed (`crates/pncad-py/src/py/select.rs`,
`SegPat::of`, `NamePat::seg`, `NamePat::path`). A pattern built by
wrapping — `p = NamePat.any().seg(SegPat.any().of([p]))` — therefore
copies everything built so far at every wrap, twice.

Measured on this box with the dev extension: 3 000 wraps take 3.8 s on
`origin/main` and 7.0 s on the name-nesting branch, whose pattern clone
walks from its own stack rather than recursing (`names::select`,
`impl Clone for NamePat`). Quadratic either way.

## What would close it

Builders that move the pattern they wrap instead of copying it (an
`Arc`-shared kernel value, or consuming the wrapped Python object), or
a stated reason the copies are needed. A row that builds a deep pattern
in time linear in its depth.

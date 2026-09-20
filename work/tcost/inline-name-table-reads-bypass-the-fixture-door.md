---
id: inline-name-table-reads-bypass-the-fixture-door
kind: issue
title: 127 inline name-table reads in editor-core's suites bypass the fixture's table door
status: open
opened: 2026-09-19
---


## Finding

`crates/editor-core/tests/fixture/mod.rs`'s `table` is the one home for
"the name table `id` published, or a panic naming what it did instead".
The EDIT sweep of `work/edit/editor-core-suites-redefine-the-name-table-helpers`
retired every NAMED redefinition of it. What it did not touch is the
same read spelled INLINE at a call site:

```
$ grep -rn '\.name_table' crates/editor-core/tests --include='*.rs' \
    | grep -v '^crates/editor-core/tests/fixture/'
127 hits in 50 files
```

Re-measured on PR #2867's fix-pass head, after the named copies went:
every hit is code, none is a comment.

The spellings are not uniform — `ev.value(id).expect("…").name_table`,
`ev.result(id)` destructured to `NodeResult::Ok`, `&v.name_table` off a
value already in hand — so a suite whose node fails reports the failure
in fifty different ways, or drops it. The door reports it one way, with
the node's whole result in the message.

## What a taker owes

The same discipline the named-copy sweep used: read each site before
changing it, because some already hold the `NodeValue` for another
reason and only the `.name_table` projection is the duplicate, and a
site that DELIBERATELY reads a failed node (`ev.result`) is not the
door's subject and stays. The count above is the population, not the
work list.

Sibling class, same file set: an `.iter()…filter(…n.path.first()…)…count()`
chain appears inline **11 times in 6 files** where `fixture::count` is
the door (`blend5_r1_probes` ×2, `docm7_union_declare`,
`docm8_flat_merged` ×2, `lib_g17_shell_node` ×2, `m4_pr3_names_bool` ×2,
`m4_pr5_declare` ×2) — note the door indexes `n.path[0]` where the
inline form uses `path.first()`, so widening the door to the total
spelling is part of the work. A further 39 sites in 20 files test
`n.path.first()` for something other than a count (`.any`, a
`filter_map` that keeps the name, a `match`), and are not the door's
subject.

## Territory

`crates/editor-core/tests/*` — tcost's and tint's. Filed by EDIT's
`edit/suite-helpers-one-home` lane, which found it sweeping for the
SHAPE of the helpers its own row named by symbol.

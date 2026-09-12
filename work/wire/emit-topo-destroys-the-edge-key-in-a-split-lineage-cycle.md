---
id: emit-topo-destroys-the-edge-key-in-a-split-lineage-cycle
kind: issue
title: names/emit_topo.rs:127 discards SplitLineageCycle's EdgeKey, keeping the kind honest and destroying the locator
status: dispatched
opened: 2026-09-11
refs: [2378]
branch: wire/names-vocab
---


## Finding

Found by WIRE's `names-refusal-carries-cause` lane (PR 2378) in its
sweep — inside `names/` but outside the five files that unit touched,
so it is this program's and not a handover. Accurate at `af8bbca`.

`crates/editor-core/src/names/emit_topo.rs:127` `map_err`s away a
`SplitLineageCycle { edge }`, discarding the `EdgeKey`, and raises
`NamingError::Emission`.

The lane calls it a **weaker instance** of the class PR 2378 fixed, and
is right about why: unlike `clearance.rs:1979`, the **kind stays
honest** — an emission fault really is what happened. What dies is the
**locator**. A cycle in split lineage is exactly the failure where the
one thing a reader needs is which edge, and it is the one thing the
refusal no longer says.

## Fence

`crates/editor-core/src/names/emit_topo.rs` is in **no open program's
`paths`**, as are `names/geompred.rs`, `emit.rs`, `discriminate.rs` and
`names/README.md` — WIRE's `paths` name only `flush.rs`, `select.rs` and
`table.rs`. PR 2378 already edits three of those unowned files and draws
the fence in its PR body; this row travels the same way. Worth deciding,
when a lane next takes it, whether WIRE should simply widen its `paths`
to `crates/editor-core/src/names/*` rather than draw the fence one file
at a time — that is a `program.md` edit with no design content in it.

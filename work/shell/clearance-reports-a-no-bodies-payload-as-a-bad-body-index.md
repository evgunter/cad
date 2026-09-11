---
id: clearance-reports-a-no-bodies-payload-as-a-bad-body-index
kind: issue
title: clearance.rs relabels InterrogateError::NoBodies (a datum node has no bodies) as SelectionRefusal::NoSuchBody { index }, reporting a payload problem as an index problem
status: open
opened: 2026-09-11
refs: [2378]
---


## Finding

Found by WIRE's `names-refusal-carries-cause` lane (PR 2378) in the
`map_err(|_| ..)` sweep that unit ran over `crates/editor-core/src/`;
outside its fence, filed here by the WIRE orchestrator because
`crates/editor-core/src/clearance.rs` is SHELL's by its `paths`.
Accurate at `af8bbca`.

`crates/editor-core/src/clearance.rs:1979` calls
`crate::names::interrogate::output_body(&value.payload, sel.body)` and
`map_err`s the whole typed `InterrogateError` into
`SelectionRefusal::NoSuchBody { index }`.

The lane classes this as a **genuine relabel**, not an
information-free discard: when the inner error is
`InterrogateError::NoBodies { payload }` — the referenced node is a
datum, or otherwise denotes no bodies at all — the refusal a user reads
says *the body index is wrong*. The index is fine; the node has no
bodies to index. `memories/refusal-text-is-not-cause.md` is the rule:
the payload and the raising site are the instrument, and the refusal's
text is not evidence of its cause.

It is a strictly stronger instance than the two
`names-flush-and-select-discard-a-refusal-with-map-err-underscore`
named, because those two raised an **honest kind** and lost the detail,
while this one raises a kind that points the reader at the wrong
argument.

## What a taker owes

A `SelectionRefusal` arm that carries the inner `InterrogateError`, or
an arm that says "this node denotes no bodies" in its own words — the
choice PR 2378 faced and answered by carrying. Read that PR's argument
before re-deciding it: the relevant half is that a `Result` whose `Err`
is destroyed one frame up is a bool in enum clothing.

`crates/editor-core/src/names/interrogate.rs` is in **no open program's
`paths`**, so a fix reaching the error type itself draws the fence in
the PR that mints it.

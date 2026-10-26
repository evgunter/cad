---
id: a-blend-of-no-edges-returns-its-input
kind: issue
title: The kernel's fillet and chamfer doors return their input for an empty edge list
status: open
opened: 2026-10-20
priority: P3
cost: E
design: true
---


## Finding

Found by WIRE's `wire/empty-blend-at-the-door` unit, which moved the
editor's empty-blend refusal to the edit and load doors
(`Select::fault`, `crates/editor-core/src/var.rs`: a set of edges holds
at least one name) and kept only an `unreachable!` in
`crates/editor-core/src/eval/wire.rs` `wire_blend`.

`crates/sweep/src/blend/build.rs` `fillet_edges_inner` (and
`chamfer_edges`, through the same preamble) gates the operand, the
size (`nonpositive_size_gate`) and a repeated edge
(`repeated_edge_gate`), and nothing gates an empty `edges`: the battery
returns a verdict with no chains, and `surgery::blend_surgery` carves
nothing and hands the body back. It is pinned as intended by
`crates/sweep/tests/blend_per_shell_carry.rs`
`an_empty_request_returns_the_body_whatever_it_holds`.

So the kernel's public doors (`pncad::sweep::blend::fillet_edges`,
`chamfer_edges`) answer a blend of nothing as the identity, while the
document says no reader gives one a meaning and refuses it at its
doors. One of the two is wrong about what a blend of no edges is.

## What is owed

A decision on the kernel door: refuse an empty request typed (a
`BlendError` arm beside `RepeatedEdge`, in the shared preamble) and
re-baseline the per-shell carry row, or keep the identity and say on
`fillet_edges`'s doc why the kernel's answer differs from the
document's.

---
id: a-pattern-count-has-no-upper-bound-at-the-loop-that-uses-it
kind: issue
title: wire_pattern guards n < 1 and then loops 1..n, and the cancel token is checked between nodes
status: open
opened: 2026-09-21
priority: P1
cost: D
refs: [need-count-spells-every-failure-as-a-pattern-count]
---


Filed by the VGEOM orchestrator while re-deriving the fork on
`work/vgeom/a-count-slots-cast-still-saturates-for-a-finite-value-too-large`
against today's tree. Its sibling is
`need-count-spells-every-failure-as-a-pattern-count`.

## Finding

`crates/editor-core/src/eval/wire.rs`, the pattern node:

```
let n = slots::count(vals, SlotId::Count).ok_or(NodeErrorKind::MissingSlot { ... })?;
if n < 1 {
    return Err(NodeErrorKind::NonPositiveCount { count: n });
}
...
for j in 1..n {
    let map = stepped_map(kind, j, results, vals, tol)?;
    let placement = names::output_body(usize::try_from(j).unwrap_or(usize::MAX)).map_err(naming)?;
    for (i, body) in master.iter().enumerate() { ... instances.push(Arc::new(place(...)?)); }
}
```

**The only guard on `n` is at the bottom.** There is no upper bound
before the loop, so the count's own magnitude is never a refusal; it
is a number of iterations. The same shape is at the stepped node,
which raises the same `n < 1` and no ceiling.

Each iteration allocates and places `master.len()` bodies into
`instances`, so the loop is not merely slow — it grows a `Vec<Arc<Body>>`
without bound.

## The bound exists in this crate, on a different path

`crates/editor-core/src/mate/member.rs` refuses a count that will not
fit the name table's `u32` row width, through the naming layer:

```
let n = usize::try_from(n).ok().and_then(|n| crate::names::output_body(n).ok())
    .ok_or_else(|| ... NamingError::Emission {
        what: "a pattern's count exceeds the table's u32 row width",
    })?;
```

So the tree already knows what a too-large pattern count is and what to
call it. The evaluator's own loop asks the same question **inside** the
loop instead — `names::output_body(usize::try_from(j)...)` is per
iteration — so the refusal arrives after roughly `u32::MAX` placements
rather than before the first. That is the guard sited downstream of the
work it is for, which is the shape
`work/vgeom/`'s `vgeom/refusal-floor` unit named on its own ground
(`work/vgeom/log.md`, 2026-09-21): a guard correct about what it looks
at, sited one step past the thing that hurts.

## And it is not cancellable

`crates/editor-core/src/eval/mod.rs`: *"The cooperative cancel token
(spec D5): checked **BETWEEN nodes**"*. An unbounded loop inside one
node therefore observes no cancel, so the recourse a long evaluation
otherwise has does not apply to this one.

## Reachability, and what was not checked

The producer is established and is not this crate's:
`crates/viewer/src/props.rs`'s `SlotValue::of` casts a typed `f64` to
`i64` with `value as i64`, which **saturates** — `1e30` commits
`i64::MAX` — and `crates/viewer/src/pane/properties.rs`'s `slot_field`
sets no `egui::DragValue` range, so `1e30` is a value the chrome
accepts for a `Count` slot. That chain is
`work/vgeom/a-count-slots-cast-still-saturates-for-a-finite-value-too-large`,
open, and it is why this row was found.

**Stated as a negative result about a search, not as a proof:** no
outer budget, node-size limit or iteration cap was found in
`crates/editor-core/src/eval/`, and the search was a grep of that
module for `budget|deadline|abort|cancel` plus a read of the pattern
and stepped arms. It did not cover the eval SERVICE above the crate
(`crates/viewer/src/session/`, VSEAM's) or any caller-side timeout, so
a reader should not conclude from this row alone that a person's
session hangs — only that nothing inside the evaluator stops it.
Nothing here was driven end to end.

## Fence

`crates/editor-core/src/eval/wire.rs` and `eval/mod.rs` — WIRE's, by
`scripts/work.py territory --files -`. Filed, not fixed: a numeric door
the viewer consumes is a hand-off and never a diff from VGEOM.

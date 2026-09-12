---
id: clearance-reports-a-no-bodies-payload-as-a-bad-body-index
kind: issue
title: clearance.rs discards two typed causes - InterrogateError::NoBodies reported as a bad body index, and a BandError dropped into the unit variant ToleranceHasNoBand
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


## A second, stronger instance in the same file (2026-09-11, from PR 2378's full review)

`crates/editor-core/src/clearance.rs:1076`:

```rust
let Ok(band) = Band::linear(query.tol) else {
    return ClearanceReport::refused(ClearanceRefusal::ToleranceHasNoBand);
};
```

`ClearanceRefusal::ToleranceHasNoBand` is a **unit variant**
(`clearance.rs:583`) whose payload renders as `String::new()` (`:632`)
and whose tag is a flat `"tolerance_has_no_band"` (`:649`). The
`BandError` is destroyed.

**This is bit-for-bit the defect WIRE's PR 2378 just repaired at four
sites**, still live here. And it is not a cosmetic loss: PR 2378
established by measurement that `Band::linear` has **two** reachable
failure arms wanting **opposite** repairs — overflow at ε near
`f64::MAX`, collapse at subnormal ε — so a refusal naming neither sends
half its readers the wrong way. See
`work/props/band-linear-errors-doc-is-false-empty-is-reachable-at-subnormal-eps.md`
for the numbers.

**Why two sweeps missed it**, which is the transferable part. PR 2378
swept `crates/editor-core/src/` for the class and triaged
`clearance.rs:1979` — 900 lines below this — without seeing this one,
because **the discard is a `let-else`, not a `map_err`**. The sweep's
instrument was keyed to one spelling of discarding; the class is "a
typed cause is destroyed", which has more spellings than `map_err`. The
instrument that finds it:

```
grep -rn "Band::linear\|Band::angular_at\|Band::new" crates/editor-core/src/
```

Ten sites in `editor-core`; this is the one live discard among them.

## Two findings, one file, one unit

Both rows are `clearance.rs` and both are a destroyed cause, so they are
one unit's work. The `NoBodies` one raises a kind that points the reader
at the **wrong argument**; this one raises an honest kind and destroys
the **distinguishing detail**. PR 2378's argument for carrying is worth
reading before re-deciding either: the sharp half is that a `Result`
whose `Err` is destroyed one frame up is a bool in enum clothing, and
strictly worse than the panic D9 declined.

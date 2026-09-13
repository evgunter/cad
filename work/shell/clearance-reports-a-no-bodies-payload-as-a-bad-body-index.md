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


## The `:1979` row is reachable from a PUBLIC door, with an executable repro (2026-09-12, WIRE PR 2474 R1)

**Author of the fixture below: PR 2474's R1 reviewer.** Adopted here
with authorship kept, by the PR 2474 lane, which measured it.

The row above says what the relabel costs. What it did not have is that
the path is reachable **without a name**, from a public door, and that
the false sentence is executable today. `clearance::clearance` takes a
caller-authored `Selection { at, body, faces }` and hands
`(payload, index)` straight to `interrogate::output_body` — no name
table lookup in the picture, so a node that names no boundary entity at
all (a datum, a profile, a declaration list, a mate, a measure, a
verdict) reaches the arm.

**Measured on `245e1445d`**: point a clearance at a datum node and the
refusal reads

```
[selection] node 0's value carries no body at index 0
```

which is not merely less specific than `NoBodies { payload: "datum" }` —
it is a **different and false fact**. No index of a datum's value
carries a body, so nothing about index 0 is the problem, and a caller
who believes the sentence goes looking for the right index.

This also settles a claim PR 2474 made and had to withdraw: that
`output_body` is reached "only through `entity_of`, i.e. after a name
has already resolved". It is not. What survives of that PR's conclusion
is narrower and **contingent on this row**: the family word the arm
computes is observable through no door *today*, because the two callers
that carry an `InterrogateError` out intact (`entity_of`,
`names::flush`'s `face_candidates`) both enter through a name-table
walk that a no-body payload never populates, and the two that reach it
without a name — `clearance.rs:1979` and `clearance.rs:2814`'s `.ok()?`
— destroy it. **Repairing this row makes the word observable**, and the
fixture below is then the row that pins it.

The fixture is RED today, and red for this row's reason. It is
deliberately NOT landed in WIRE's suite: the thing that makes it red is
`clearance.rs`, which is SHELL's.

```rust
//! **The no-body family arm is reachable from a PUBLIC door, and the
//! word it computes is destroyed on the way out.** (Reviewer fixture,
//! WIRE PR 2474 R1 — adopt with authorship kept.)
//!
//! PR 2474 argues deliverable 1 can carry no row because
//! `names::interrogate::output_body` is `pub(crate)` and "reached only
//! through `entity_of`, i.e. after a name has already resolved". It is
//! not: `clearance::clearance` takes a caller-authored
//! `Selection { at, body, faces }` and hands `(payload, index)` to
//! `output_body` with no name in the picture
//! (`crates/editor-core/src/clearance.rs:1979`).
//!
//! Point a clearance at a DATUM node and the arm this PR rewrote
//! answers `NoBodies { payload: "datum" }` — and `clearance.rs:1979`
//! `map_err(|_| ..)`s it into `SelectionRefusal::NoSuchBody`, which
//! renders "node 0's value carries no body at index 0". That sentence
//! is not merely less specific, it is a different and false fact: no
//! index of a datum's value carries a body, so nothing about index 0
//! is the problem. It is the same discard `emit_topo.rs` fixes in this
//! same PR, one caller away from the function the same PR touched.
//!
//! MEASURED on head 245e1445d: the row below FAILS with
//! `[selection] node 0's value carries no body at index 0`. It is
//! written for the behaviour the refusal should have and is RED until
//! `clearance.rs:1979` carries the `InterrogateError` through.

#![cfg(feature = "interval")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;

use editor_core::analysis::ParamBox;
use editor_core::clearance::{ClearanceVerdict, Selection, clearance};
use editor_core::{Node, ProfileDoc};
use geom_core::Tol;
use std::collections::BTreeMap;

use fixture::{insert, len, square};

#[test]
fn a_clearance_over_a_datum_node_says_the_value_carries_no_bodies() {
    let doc = ProfileDoc::empty_derived("wire_output_body_public_door", Tol::witness());
    let (doc, plane) = insert(doc, fixture::xy_frame());
    let (doc, profile) = insert(
        doc,
        Node::Profile(fixture::desc(plane, vec![square(0.0, 0.0, 1.0)])),
    );
    let (doc, block) = insert(
        doc,
        Node::Extrude {
            profile,
            distance: len(2.0),
        },
    );
    let leaf = ParamBox::from_axes(BTreeMap::new());
    let report = clearance(
        &doc,
        &leaf,
        &Selection::body_of(plane),
        &Selection::body_of(block),
        0.1,
        Tol::witness(),
    );
    let ClearanceVerdict::Refused(r) = report.verdict() else {
        panic!("a datum has no faces to measure: {}", report.verdict().label());
    };
    let said = format!("[{}] {}", r.name(), r.payload());
    assert!(
        said.contains("datum") || said.contains("no bodies"),
        "the arm computed the family word and the refusal threw it away: {said}"
    );
}
```

---
id: matches-subset-policy-survives-in-four-viewer-modules
kind: issue
title: A matches! over an enum subset stands in for policy in four more viewer modules, and none of them can red when the enum grows
status: open
opened: 2026-09-22
priority: P2
cost: E
refs: [has-faults-cannot-red-on-a-new-rowstatus]
---

## Finding

Filed by the `chrome/rowstatus-exhaustive` lane, from the sweep
`has-faults-cannot-red-on-a-new-rowstatus` asks for. That row's own
point is that the ORIGINATING sweep's criterion — *"a multi-arm
`matches!` over a subset of an EXTERNAL enum"* — had a word in it
doing no work. Swept for the shape instead: **a `matches!` over a
subset of any enum, standing in for a policy, where the excluded
states are a decision stated only in prose.**

`tree::has_faults` is fixed on that lane. Six more sites, in four
modules under `crates/viewer/src`, are outside that lane's fence:

- **`bounds.rs`, `Verdict::of`** — `matches!(result,
  NodeResult::Failed(_))` over `NodeResult` (`Ok`, `Failed`,
  `Poisoned`). The strongest hit: it is `has_faults`'s defect exactly,
  one enum up. The doc directly above it argues the `Poisoned`
  exclusion at length (*"counting it would make one failure register
  as many"*) — so the policy is already written out, and only the
  construct is silent. A fourth `NodeResult` is silently not a
  failure, and the verdict every value-comparison is made on gets it
  wrong with nothing saying so. Cheapest of the four to fix: the arms
  and their reasons are already there.
- **`session/refuse.rs`, the `NodeKindWanted::Frame` arm** —
  `matches!(held, Some(Node::Datum(Datum::Frame { .. } |
  Datum::FaceFrame { .. })))`, with a comment saying why both frame
  kinds count. The outer `match` over `NodeKindWanted` is exhaustive;
  the inner subset over `Datum` is not, so a new datum flavour a
  picker should accept is silently refused.
- **`session/refuse.rs`, the body predicate** — `matches!(payload,
  ValuePayload::Body(_) | ValuePayload::Boolean(BooleanValue::Body
  { .. }))`. A viewer policy (*does this value carry a body*) over a
  kernel enum; a new payload that carries one is silently bodiless.
- **`sketch.rs`, twice** — the same `Datum::Frame { .. } |
  Datum::FaceFrame { .. }` subset, asking which nodes a sketch may be
  drawn on. Two sites, and a third spelling of the same question as
  `refuse.rs`'s first hit, which is its own smaller finding.
- **`tools.rs`, `ToolKind`'s two multi-op arms** — `SessionOp::
  AddPattern { .. } | SessionOp::AddPlacedUnion { .. }` and
  `SessionOp::AddFillet { .. } | SessionOp::AddChamfer { .. }`, each
  deciding *did this tool's edit land*. The outer `match` over
  `ToolKind` is exhaustive; the `SessionOp` subsets are not, so a new
  op a tool commits leaves the tool open after its own edit landed.

## Not defects, recorded so the negative result is a receipt

- `pane/viewport.rs`'s `matches!(row.status, RowStatus::Failed { .. })`
  is a test assertion naming the one state it wants, not a policy over
  a set.
- The single-variant `is_*` predicates (`props.rs`, `pickcache.rs`,
  `bounds.rs`'s `Bound::is_edge`, `platform.rs`) ask identity, not a
  subset.
- `datums.rs`'s `Some(Node::Datum(_))` matches a whole variant.

## What the sweep could not match

The pattern is textual and finds `matches!` only. It cannot see the
same policy written as an `if let` / `let … else` chain, as a
`match` with a `_ =>` wildcard, or as a helper predicate whose own
body is the subset (a `foo.is_bar()` reads as one token at the call
site). A second pass over every `matches!` in `crates/viewer/src`
closed the sub-gap that the first pattern required a `|` alternation,
and a third pass found no policy spelled as two `matches!` joined by
`||`. The wildcard-`match` gap is the one left open, and it is the
larger of the two: `-D clippy::wildcard_enum_match_arm` over the
crate is the instrument, and running it is a unit of its own.

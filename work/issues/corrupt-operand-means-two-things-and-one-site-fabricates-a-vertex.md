---
id: corrupt-operand-means-two-things-and-one-site-fabricates-a-vertex
kind: issue
title: two call sites map ContainError::Corrupt to different BooleanError variants, and one of them mints a VertexKey::default() for a refusal that has no vertex
status: open
opened: 2026-09-12
refs: [2420, contain-error-drops-the-loop-its-carrier-named]
---



(FIX orchestrator, 2026-09-12) Reported by the
`census-containment-flatten-fabricates-its-diagnostic` lane (PR 2420),
which found the disagreement; **the fabricated payload is the
orchestrator's addition on verifying it.** Filed in `work/issues/`
because `crates/topo/src/boolean/*` is claimed by BOTH `bool` and
`curved` — a disputed owner, which is what this directory is for.
Either may claim it by moving this file.

## Two sites, two meanings

`ContainError::Corrupt` is one arm. Its two consumers disagree about
what it says:

- `crates/topo/src/boolean/reduce.rs:1970` →
  `BooleanError::CorruptOperand { operand, vertex: VertexKey::default() }`
- `crates/topo/src/boolean/ops.rs:1642` →
  `BooleanError::ClassificationInvariant { what: "extent scan: contfp met corrupt topology" }`

A corrupt operand and a violated classification invariant are different
claims with different repairs — one says the body handed in is broken,
the other says the kernel's own scan reached a state it holds to be
impossible. One of the two is wrong about what `ContainError::Corrupt`
means, and reading the arm's own declaration is what settles it.

## The sharper half: `reduce.rs` mints a vertex it does not have

`BooleanError::CorruptOperand` carries a `vertex`, and
`ContainError::Corrupt` has none to give, so the site supplies
**`VertexKey::default()`** — a key naming no vertex, in a field a
reader would use to find the corruption.

**That is the third instance of the class PR 2420 just closed**, and it
is the same shape exactly: a refusal reaching a user with a fabricated
value in the field that was supposed to locate the problem. The census
site synthesized an `Indeterminate` whose margin nothing measured;
this one synthesizes a `VertexKey` no vertex answers to.
`memories/refusal-text-is-not-cause.md` is the standing memory, at the
payload rather than the sentence.

**So the repair is probably not "pick one of the two variants".** If
`CorruptOperand` genuinely requires a vertex and this door cannot name
one, the honest answers are a variant that does not demand what the
caller lacks, or an `Option<VertexKey>` — the same door-shape question
PR 2420 answered for `CensusUnsupportedCause`, and the precedent to
read before deciding.

## What a taker owes

Read `ContainError::Corrupt`'s own declaration and decide what it
means, then make both sites say that. Check whether any pin asserts
either message; PR 2420 found that for two of three carriers in the
same family **nothing did**, and recorded the missing pin as part of
the defect rather than as a baseline.

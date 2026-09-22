---
id: has-faults-cannot-red-on-a-new-rowstatus
kind: issue
title: tree::has_faults encodes chrome policy as a two-arm matches! over RowStatus, so a fifth row state is silently not-a-fault — and band-refusal's fix is to add one
status: closed
opened: 2026-09-21
closed: 2026-09-22
branch: chrome/rowstatus-exhaustive
priority: P2
cost: E
---

## Finding

`crates/viewer/src/tree.rs`, `has_faults`:

```rust
rows.iter().any(|row| {
    matches!(
        row.status,
        RowStatus::Failed { .. } | RowStatus::Poisoned { .. }
    )
})
```

under a doc that reads *"Whether any row reports a failure or a
poisoning — what a chrome shows as \"this document is not
building\""*.

That is **the same defect class** `frame::product_badge` was just
fixed for
(`viewer-states-the-empty-document-rule-in-four-places-and-the-one-
that-gates-cannot-red`): a multi-arm `matches!` over a subset of an
enum, standing in for chrome policy, which **cannot red when the enum
grows a state**. A fifth `RowStatus` is silently not a fault, and no
build anywhere says so. `RowStatus` has four states today — `Ok`,
`Failed`, `Poisoned`, `Unevaluated` — and the `Unevaluated` omission
is a deliberate policy call that a reader cannot distinguish from an
oversight, which is the second half of the same problem.

**Why the originating unit's sweep missed it.** That sweep's
criterion was a multi-arm subset of an EXTERNAL enum, and `RowStatus`
is viewer-owned. The word `external` was doing no work: what makes
the construct silent is that it is a `matches!` rather than a
`match`, and which crate declares the enum has nothing to do with it.
Recorded here because the criterion is the defect, not the miss.

## It meets `band-refusal-still-badges-every-row`, and neither row can see that from its own side

`work/chrome/band-refusal-still-badges-every-row` proposes closing
its symptom with a row status that names the CLUSTER rather than the
row — *"may want its own status, which is why it is filed rather than
folded into 1769"*. **That fix is the event this row is about.** The
day a `RowStatus` variant is added for a run-level refusal,
`has_faults` silently answers `false` for every row carrying it, and
a document that is not building reports as building. Nothing reds.

Whoever takes either row should take both, or at minimum make the
`has_faults` match exhaustive first, so the other row's variant
cannot land silently. Cross-referenced on that row too.

## What a taker owes

An exhaustive `match` in place of the `matches!`, with the
`Unevaluated` answer stated as the policy it is rather than left to
the complement. `frame::badge_site` on this branch is the worked
shape, including how it keeps a cited rule load-bearing beside a
local policy.

## Filed from outside the fence

Filed under `docs/prompts/implementer-discipline.md` §6 by the lane
for `chrome/empty-document-gate` (scope: `frame.rs`, `pickindex.rs`,
`crates/viewer/tests/`). `tree.rs` is claimed by CHROME jointly with
VIEW and VNEWS; filed on CHROME because `has_faults`'s subject is
what the chrome shows.


## Closed 2026-09-22 (`chrome/rowstatus-exhaustive`)

`has_faults` is an exhaustive `match`, with `Unevaluated` and `Ok`
each an arm carrying its reason rather than the complement of a
pattern, and the doc says which axis it is and how it differs from
`RowStatus::tone` (the collapse a later reader would reach for).

**Proved red, both ways, rather than asserted.** A scratch fifth
variant made `has_faults` one of five `E0004`s in the crate —
`tree.rs`'s `badge`, `tone`, `message` and `has_faults`, plus
`pane/features.rs`'s draw decision and `frame.rs`'s guard. And the
policy itself now has a test that a bug breaks:
`a_downstream_failure_alone_is_a_fault_the_reader_cannot_act_on`
(`crates/viewer/tests/tree_badges.rs`) is the only place in the tree
where a `Poisoned` row stands alone, so dropping `Poisoned` from the
policy — or rewriting it as *any actionable row* — reds only there.
Confirmed by making that mutation and watching it fail.

The sweep this row asks for is
`matches-subset-policy-survives-in-four-viewer-modules`: four more
live in `crates/viewer/src`, and the closest sibling is `bounds.rs`'s
`Verdict::of`, which is this defect one enum up with its policy
already argued in prose above it.

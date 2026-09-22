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
pr: 3055
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
`RowStatus::tone` (the collapse a later reader would reach for) and
from `bounds::Verdict`, which excludes `Poisoned` for a reason argued
at its own site and never reconciled with this one — filed as
`two-is-this-broken-readings-argue-opposite-on-poisoned`.

**Two claims in the sections above are corrected rather than
inherited.**

- *"what a chrome shows as \"this document is not building\""* — the
  doc sentence this row quotes, and the role the Finding gives the
  function — **describes a role nothing plays**. `grep -rn has_faults`
  over the workspace returns the definition plus
  `crates/viewer/tests/` and `crates/viewer/examples/r1_e2e.rs`, and
  nothing else: there is no `src/` caller and `lib.rs` re-exports
  none. What a chrome shows is decided per row by `pane/features.rs`'s
  badge draw and, for the product, by `frame::product_badge`.
  `has_faults` is a **test oracle** — twenty-two call sites, fifteen
  of them the `!has_faults(…)` "this evaluates clean" gate, across
  seven suites and `r1_e2e.rs`. That is a smaller claim than "the GUI
  would lie to a user" and it is not a weaker reason to fix the
  construct: a silently-wrong oracle lets all fifteen of those gates
  keep passing on documents broken in the new way.
- *"the `Unevaluated` omission is a deliberate policy call"* (Finding,
  above) claims an intent the history does not carry.
  `-S'pub fn has_faults'` over this file's log puts the function in
  the crate's founding commit (`1a7c38d2d`) and nothing since settles
  why. The REASONS the fix writes out ARE corroborated, which is a
  different claim and the one the doc now makes —
  `review_gui3_r2::a_document_with_no_result_yet_reads_unevaluated_and_reports_no_faults`
  pins that an unevaluated tree reports no faults and
  `tree_badges::only_the_row_whose_own_operation_refused_is_actionable`
  pins its tone, so the doc says *the reading the tests pin* rather
  than recovered intent.

**Proved red, both ways, rather than asserted.** A scratch fifth
variant made `has_faults` one of **six** `E0004`s in the crate —
`tree.rs`'s `badge`, `tone`, `message` and `has_faults`, plus
`pane/features.rs`'s draw decision and `frame.rs`'s guard. And the
policy itself now has a test that a bug breaks:
`a_downstream_failure_alone_is_a_fault_the_reader_cannot_act_on`
(`crates/viewer/tests/tree_badges.rs`) is the only place in the tree
where a `Poisoned` row stands alone, so dropping `Poisoned` from the
policy — or rewriting it as *any actionable row* — reds only there.
Confirmed by making that mutation and watching it fail.

**What that test may NOT be built from**, since the first draft of it
was built that way: a poisoned row beside a `Failed` cause. Any tree
carrying a `Failed` row satisfies the policy through THAT row, so the
mutation above stops reddening — measured rather than reasoned: with
the cause row switched to `Failed`, mutating `Poisoned` out of the
policy leaves the test green. The one shape that carries the claim is
`Poisoned { message: None }`, which `poisoned_through` mints when the
chain does not end at a failure and which `RowStatus::Poisoned`'s own
doc names as the reporting of that; so the fixture is a tree state the
module has a rendering for, not one it declares broken.

The sweep this row asks for is
`matches-subset-policy-survives-in-four-viewer-modules`: 21 more
`matches!` sites live in `crates/viewer/src`, the wildcard-`match`
spelling of the same construct has its own row
(`a-wildcard-match-decides-viewer-policy-in-five-places`), and the
closest sibling is `bounds.rs`'s `Verdict::of`, which is this defect
one enum up with its policy already argued in prose above it.

### Raised at review and declined, recorded rather than dropped

- **Moving `has_faults` onto `RowStatus` as `is_fault(&self)`**, beside
  `badge` / `tone` / `message`. Right shape, wrong moment: ~11 call
  sites churned for a taste improvement on a green PR, and the
  reviewer was explicitly unsure. Carried on
  `matches-subset-policy-survives-in-four-viewer-modules` so the next
  lane that opens that impl gets it for free.
- **Splitting `tree.rs`'s 77-line module header**, and the observation
  that `has_faults`'s doc is now the file's largest. No action: the
  header is the second-section argument `blamed_mates` and
  `downstream_of_mate` both cite, and splitting it is a change to what
  those citations point at.
- **The fixture's `kind: "Transform"` / `depth: 0` / `root: false`
  being a shape `rows()` would not build.** `has_faults` reads none of
  those fields, and the reviewer was unsure. No action — but the
  fixture's *status* shape was a real defect and is fixed above.

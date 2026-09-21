---
id: viewer-states-the-empty-document-rule-in-four-places-and-the-one-that-gates-cannot-red
kind: issue
title: The viewer states the empty-document classification in four places, and the one that actually gates (product_badge's matches!) is the one construct that cannot red when a ProductError arm is added
status: open
opened: 2026-09-15
priority: P1
cost: E
---


## Where this came from

WIRE's PR 2629 gave the empty-document classification one home —
`ProductErrorKind::is_empty_document` in `crates/editor-core/src/product.rs` —
and reduced four consumers to citing it, two of them in `crates/viewer`
(announced on `work/chrome/log.md`). Its style review then found that the
viewer keeps arguing the rule anyway, in places the citation did not
reach, and that the site which *decides* anything is the weakest one.

Filed here rather than fixed there: the fix runs **down into the viewer**,
not up into `editor-core`, so it is not WIRE's to make. That direction is
agreed by both the reviewer and the WIRE orchestrator, and it is the same
call `crates/viewer/README.md` already makes one layer up for `Refusal`.

## The instrument finding, which is the reason this is a row

`crates/viewer/src/frame.rs`, `product_badge` — after 2629 the filter
reads

```rust
!(fault.kind().is_empty_document()
    || matches!(fault, ProductError::RootFailed { .. }
                     | ProductError::RootPoisoned { .. }
                     | ProductError::UnknownNode { .. }))
```

**The three arms in the `matches!` are chrome policy and they belong
here** — the Features pane already badges those three at the node with a
typed cause, one of them deliberately quiet, which is a decision about
the chrome and not a classification of the refusal. Nothing about that
should move to `editor-core`, and the dependency should keep running
`viewer` → `editor-core`.

What is wrong is the **instrument**, and it is exactly the cost argument
the `editor-core` half was built on. A `matches!` is the one construct
here that **cannot red when a tenth `ProductError` arm lands**: arm
eleven is silently declined or silently badged, whichever way the
expression happens to be written, and no build anywhere says so. The
`editor-core` side now reds twice by name for a new arm (at `kind()`, and
at `is_empty_document`); the viewer side, which is where a user actually
sees the consequence, reds not at all.

The honest shape is one cited rule plus one local policy **both
compile-checked** — an exhaustive `fn` over `ProductErrorKind` living in
the viewer, beside the badge it serves.

Two smaller things at the same site, for whoever takes it: the expression
reads one value at two levels (`fault.kind()` against
`matches!(fault, …)`), and `!(A || B)` is harder to read than the
`!matches!(…)` it replaced.

## The three restatements the citation did not reach

| site | what it says |
| --- | --- |
| `frame.rs`, `product_badge`'s doc | *"Nothing here is wrong to report"* — a restatement sitting immediately after a citation of the rule it restates, with the *"deleting the last feature"* worked example now duplicated rather than moved (it is also in `is_empty_document`'s doc) |
| `frame.rs`, the in-module test's comment above the four quiet arms | *"An empty document is not malformed, and the three per-node states are the feature tree's to badge"* — disclosed by 2629 and deliberately left, because rewriting another program's test comment is past what an announced seam is for |
| `crates/viewer/src/pickindex.rs` | *"deleting the last feature … a document that has only datums or profiles … An empty document is a state, not a fault"* — the same classification, the same two worked examples, independently written a third time in the viewer |

`pickindex.rs` reads no `ProductError`, which is why 2629's sweep
dismissed it — true of the code and not of the sentence. The originating
row's class is *"the same classification, same reasoning, same worked
example, independently written"*, and that is what this is.

## Why it is scheduled rather than noted

`docs/prompts/reviewer-style-lane.md` Q6: a disclosed deviation owes a
concretely scheduled followup, and *"recorded as a pickup"* is not a
schedule. 2629 disclosed the second row above and left it; this file is
the schedule for all three.

## Cross-program note

`crates/viewer/src/frame.rs` and `pickindex.rs` are claimed by **CHROME
and VIEW jointly** (`work.py territory --files -`). Filed on CHROME
because `product_badge` is the gating site and what the chrome badges is
CHROME's charter; re-home to VIEW if that reading is wrong — the
directory is the claim.

Filed by the WIRE orchestrator under
`docs/prompts/implementer-discipline.md` §6, from PR 2629's style review.

## What was measured, taking it

Taken on `chrome/empty-document-gate`.

**The name in this file is stale and the shape is not.** WIRE renamed
`is_empty_document` to `means_no_body` before this row was written
(its own log records the rename and the argument for it: a kind is not
a document). Every citation above should be read as `means_no_body`.
The filter's shape, the ten classes and the three-arm policy were all
as described.

**The gate.** `frame::badge_site`, a `match` exhaustive over
`ProductErrorKind` returning `frame::BadgeSite` — `Frame`,
`FeatureTree`, `NotAFault`. An enum rather than a `bool` because three
answers are in play and the two silences are silent for unrelated
reasons; a `bool` collapses them and is precisely the assertion the
in-module test could already make and which could not tell them apart.
The three tree-owned classes are answered by the local policy; every
other class reaches `NotAFault` or `Frame` through a call to
`ProductErrorKind::means_no_body`, so the cited rule is load-bearing
rather than re-named. Asked in that order because `means_no_body`'s
own contract says `false` does not appoint the reporter.

**Proof the gate reds.** An eleventh `ProductErrorKind` added locally
(not committed) gives `error[E0004]: non-exhaustive patterns:
ProductErrorKind::Eleventh not covered` at `frame::badge_site`, and
`cargo check -p viewer` fails. What it still does NOT buy, unchanged
from `means_no_body`'s own disclosure: a new `ProductError` VARIANT
under an existing class inherits that class's answer silently.

**The three restatements.** `product_badge`'s doc keeps its badge
argument and its silent-arms section is four lines citing
`badge_site`; the policy prose moved there rather than being copied.
The in-module test comment now pairs each quiet arm with the silence
it gets instead of restating why. `pickindex.rs` cites
`means_no_body` in both `scene_for`'s `# Errors` doc and the
`parts.is_empty()` comment, and the comment's account of what the code
used to do went with it (implementer-discipline §4).

**Rows pinned, and the mutations that redded them.** Extended
`frame::tests::the_gather_verdict_badges_only_the_faults_nothing_else_carries`
rather than adding a second test. `RootFailed` moved to the frame
group reds *which channel reports it ... left: Frame right:
FeatureTree*; `Graft` moved to the tree group reds *no per-node badge
carries it: Graft*; `NoBodyRoots` answered `FeatureTree` instead of
through `means_no_body` reds *left: FeatureTree right: NotAFault* —
that last one is the row the old test could not have: `product_badge`
is `None` either way.

**Found outside the fence**:
`two-more-viewer-sites-restate-the-empty-document-rule` (app.rs's
badge-column comment and `SceneMesh::nothing`'s doc), filed on this
slate in the same PR.

The behavioural half, `at-rest-badge-reports-an-empty-document-as-a-
refusal`, is untouched: `session.rs` was out of this wave.


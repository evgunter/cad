---
id: viewer-states-the-empty-document-rule-in-four-places-and-the-one-that-gates-cannot-red
kind: issue
title: The viewer states the empty-document classification in four places, and the one that actually gates (product_badge's matches!) is the one construct that cannot red when a ProductError arm is added
status: dispatched
opened: 2026-09-15
priority: P1
cost: E
branch: chrome/empty-document-gate
---


## Where this came from

WIRE's PR 2629 gave the empty-document classification one home —
`ProductErrorKind::means_no_body` in `crates/editor-core/src/product.rs` —
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
!(fault.kind().means_no_body()
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
at `means_no_body`); the viewer side, which is where a user actually
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
| `frame.rs`, `product_badge`'s doc | *"Nothing here is wrong to report"* — a restatement sitting immediately after a citation of the rule it restates, with the *"deleting the last feature"* worked example now duplicated rather than moved (it is also in `means_no_body`'s doc) |
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

**The name this file was written with never existed on main**, and
the body above is corrected rather than annotated: WIRE renamed
`is_empty_document` to `means_no_body` inside PR 2629's own lane, on
the argument that a kind is not a document (`work/wire/log.md`). The
old spelling is what rotted this unit's premise in the first place, so
leaving it in place for a later reader to grep for would repeat the
defect the row is about. The shape the row described — the filter, the
ten classes, the three-arm policy, all three restatement sites — was
true of the tree.

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

**Found outside the fence**, all on this slate in the same PR:
`six-viewer-sites-restate-the-empty-document-rule-and-its-badge-policy`
(the class is wider than this row's three — two more in `app.rs`, two
in `scene.rs`, one in `pane/viewport.rs`, one in `session.rs`);
`has-faults-cannot-red-on-a-new-rowstatus` (the same `matches!`
defect in `tree.rs`, cross-referenced to
`band-refusal-still-badges-every-row`, whose proposed fix is to add
the very `RowStatus` variant it cannot see); and
`a-citation-in-a-line-comment-is-not-checked`.

**The instrument sweep's criterion was wrong, and that is recorded
rather than quietly fixed.** It excluded `tree::has_faults` on the
word EXTERNAL — a multi-arm `matches!` over a subset of a
viewer-owned enum is exactly as silent as one over `ProductErrorKind`.
The prose sweep's key phrase also missed `scene.rs`'s *"denoting no
body"* on an inflection of its own stem. Both are stated on the row
above.

The behavioural half, `at-rest-badge-reports-an-empty-document-as-a-
refusal`, is untouched: `session.rs` was out of this wave.

## The fix pass

The gate's doc was itself restating `means_no_body` at three sites in
the diff that closes the row — including at `BadgeSite::NotAFault`,
a brand-new site with no citation on it at all. All three now cite.
Two further things landed with them: the `RowStatus` count the policy
argues from is a measurement of another module's enum, so it has a
guard (`the_tree_still_has_exactly_the_three_states_this_policy_
pairs_with`, which reds both on a fifth variant at compile time and
on a policy edit at runtime); and the doc now says what the compiler
does NOT buy — exhaustiveness over the classes, not liveness of the
cited call, which a later edit could leave unreachable with nothing
but the in-module row to catch it.

`frame.rs`'s module header also named nine of the ten badge doors
(`profiles_badge` was missing) and `datums_badge` carried an
unguarded "beside eight others"; both corrected, the latter by
dropping the count rather than restating it. `crates/viewer/README.md`
remains the only complete enumeration and the only one the repaired
`frame_policy` scan reads.

**Where the census of classes lives, and why not a `const`.** The
guard's ten-class list is an inline array in the row.
`scripts/gates/viewer-vocab-declared-once.sh` reds on a `const` array
of two or more `Type::Variant` entries under `crates/viewer/src` that
`crates/viewer/README.md`'s roster does not ratify, and neither
ratified kind fits a complete census of another crate's enum — a
third kind is an amendment argued on that page, not a table cell. The
page states the alternative it expects of a suite: a hand-written
variant list belongs inline in a row. It is hand-written for the
reason the row states at the site: a class cannot be added without
`badge_site`'s `match` refusing to compile.

---
id: the-split-did-not-separate-indexrequest-and-indexinputs
kind: issue
title: PR 2079 and the closed item both say the split put IndexRequest and IndexInputs in different modules; they were already in different modules
status: closed
opened: 2026-09-06
refs: [2079, index-request-and-index-inputs-are-one-concept-twice]
closed: 2026-09-06
pr: 2079
---



Found by the style review of #2079.

## What

`work/view/index-seam-vocabulary-sits-in-the-wrong-module.md:267-270`
(the `## Closed` section, now on `main`'s history via #2079) says:

> Not taken, deliberately:
> `index-request-and-index-inputs-are-one-concept-twice`. **The split
> does put the two types in different modules**, which makes its cheap
> answer — state the relationship at each type — a one-line reach from
> either file.

It does not, because they already were. At the merge base
(`3f0ee3a`) `IndexRequest` was in `crates/viewer/src/evalseam.rs` and
`IndexInputs` in `crates/viewer/src/pick.rs`. Both stayed exactly
where they were: `evalseam.rs:327` and `pick.rs:64`. The split moved
`PickIndex` out from under `IndexInputs`, which changed nothing about
the distance between the two types the item is about.

The same sentence is in the PR body under *Out of scope, and it stays
that way*.

## Why it is worth a file

The decision it justifies is right — declining
`index-request-and-index-inputs-are-one-concept-twice` is correct and
its own reasoning survives the move intact (`IndexRequest` owns its
copies because the worker holds them across a thread; `IndexInputs`
borrows a landing; that is still why they are not redundant). But the
reason given for declining is a false statement about what this PR
did, and it is now in a closed item's `## Closed` prose, which
`work/README.md` treats as the record of what was believed at that
moment. A later reader costing the cheap answer will start from a
premise that was never true.

## Confidence

`sure`.

## Closed

Struck rather than repaired, in both places: the `## Closed` prose of
`index-seam-vocabulary-sits-in-the-wrong-module` and the #2079 body.
Neither now claims the split did anything to the two types. The
decline stands on the reasoning that was always the real one —
`IndexRequest` owns its copies because the worker holds them across a
thread, `IndexInputs` borrows a landing — and the closed item now says
explicitly that both types stayed where they were at the merge base.

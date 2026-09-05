---
id: news-and-standing-facts-are-orthogonal-axes
kind: issue
title: News and standing fact are orthogonal to subject, and three facts of one class got two answers
status: open
opened: 2026-09-05
---


## What this is

`crates/viewer/src/frame.rs`'s header carries one classification —
**news vs standing fact**, decided by whether a fact outlives the frame
that produced it — and `view/news-and-badges` added a second,
`Subject`, decided by which event stream retires a message. The two
were designed as if they were the same axis. **They are orthogonal.**

A fact whose lifetime is *"until the camera moves"* satisfies the
standing-fact test (it outlives its frame, it survives a mouse drag)
**and** has a perfectly good subject (`Camera`). Nothing forces a
choice, and asking "is it news or a standing fact" answers a different
question from "what retires it".

## The inconsistency it left inside one unit

Three facts of one class — a seam refused, the picture is stale until
that seam succeeds — got two different answers in the same diff:

- `crates/viewer/src/frame.rs`, `scene_refusal` — `Subject::Display`,
  news.
- `crates/viewer/src/frame.rs`, `index_refusal` /
  `unindexed_refusal` — `Subject::Display`, news.
- `crates/viewer/src/frame.rs`, `projection_refusal` —
  `Subject::Camera`, news.

And `status-line-writers-bypass-the-ranking` classifies all three as
**standing facts wanting a badge**, while #1883's ruling names the
projection refusal among its four *news* instances. No rule anywhere
distinguishes them, so the sweep has nothing to sort by.

## Why it is worth more than any line of that diff

`status-line-writers-bypass-the-ranking` sorts nineteen writers into
news and standing facts. If the two axes are orthogonal, that sort is
under-determined for every writer whose fact outlives its frame *and*
has a subject — which is most of the standing-fact list. The sweep
needs the rule before it writes twenty sites against it.

## The shape of an answer

The candidate the reviewer's finding suggests: the axes are
independent, so state both. A fact is a **badge** when it is a read of
held state a reader consults, and a **line message** when it is the
outcome of something that just happened — and *either* can carry a
subject, because a subject is only about what retires it. Under that
reading the three above are all badges (they are reads of seam state)
and their subject is what would retire the badge, not what retires a
sentence. That is a change to `Badge` as well as to `Subject`, which is
why it is a decision and not a fix.

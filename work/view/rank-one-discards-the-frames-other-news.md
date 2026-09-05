---
id: rank-one-discards-the-frames-other-news
kind: issue
title: A frame's refusal discards its notices, and a discarded free-move placement is not recoverable
status: open
opened: 2026-09-04
refs: [opoutcome-superseded-has-no-production-reader, the-news-vocabulary-has-no-expiry, status-line-writers-bypass-the-ranking]
---

Disclosed by `opoutcome-superseded-has-no-production-reader`'s fix,
which put a new kind of message into rank 2 and so made the rank's
cost concrete.

## What happens

`frame::frame_status` (`crates/viewer/src/frame.rs`) ranks a frame's
news, and **rank 1 wins ALONE**: a refusal is shown and every notice
the same frame produced is dropped, not queued, not joined.

```
match batch_status(ops, refusal) {
    refused @ StatusUpdate::Show(_) => refused,
    ...
}
```

That is the ratified ranking and it is deliberate — two sentences
about different things are worse than one about the loudest. The
question this item raises is whether it should still hold when the
dropped notice reports something the user **cannot get back**.

A batch is one frame's ops, and one frame can carry several: a panel
committing a mate and a drag emitting a gesture op that refuses is one
batch. When that happens today:

- the mate lands, `DisplayState::prune` discards the free-move
  placement the user positioned by hand
  (`crates/viewer/src/display.rs`, `prune`),
- `frame::supersession_notice` renders it,
- and `frame_status` drops that notice for the gesture's refusal.

The refusal is about an op that did nothing. The notice is about a
value that is **gone** — discarded, not zeroed and not parked, so no
undo of the refusing op restores it. A declined pick can be re-picked;
this cannot.

## Why it is not simply "add it to the refusal"

The joined form (`NOTICE_SEPARATOR`) already exists for several
notices, so the mechanism is there. What is missing is the JUDGEMENT:
which rank-2 messages are important enough to ride alongside a
refusal, and whether that is a property of the message (a lost value)
or of the pair. Deciding it for supersessions alone would put a second
un-stated rule in a module whose whole point is that its rules are
stated.

The same question is open for the tool notices already in rank 2 —
this item is not about supersessions alone, they are just the case
where the loss is irreversible.

## Where to look

- `crates/viewer/src/frame.rs` — `frame_status`, the ranking and its
  doc comment.
- `crates/viewer/src/app.rs` — `perform_batch`, where a frame's ops
  and its notices are collected together.
- `crates/viewer/tests/frame_policy.rs` —
  `a_superseded_free_move_is_news_the_ranking_shows` is the row that
  would grow a refusing sibling op.

---
id: news-and-standing-facts-are-orthogonal-axes
kind: issue
title: News and standing fact are orthogonal to subject, and three facts of one class got two answers
status: review
opened: 2026-09-05
branch: view/axes-and-badges
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

## Put to Ev (VIEW orchestrator, 2026-09-05)

**This is the sweep's remaining blocker.** #1883 ruled the two
vocabularies and #1933 built them; `status-line-writers-bypass-the-
ranking` is the twenty-site sweep that consumes them, and it cannot
start until the classification rule exists. Nothing else on this board
is blocked.

**What #1933 committed to, so the question is concrete.** Three facts
of one class — a seam refused, and the picture is stale until that seam
succeeds — got two subjects in one diff: `scene_refusal` and
`index_refusal`/`unindexed_refusal` are `Subject::Display`,
`projection_refusal` is `Subject::Camera`. Meanwhile the sweep item
calls all three **standing facts wanting a badge**, and #1883's ruling
text names the projection refusal among its four **news** instances.
Every one of those is defensible and no rule distinguishes them.

**The orchestrator's reading, offered as an argument.** The reviewer's
diagnosis is right and it dissolves rather than decides: *"is it news
or a standing fact"* asks whether a fact outlives its frame; *"what is
its subject"* asks which event retires it. A fact whose lifetime is
"until the camera moves" answers **yes** to the first and **`Camera`**
to the second. They were designed as one axis and are two.

So the answer is probably to state both, with the rule the section
above names: **a badge is a read of held state a reader consults; a
line message is the outcome of something that just happened** — and
either can carry a subject, because a subject only says what retires
it. Under that rule the three above are all badges, and their subject
is what retires the *badge*.

**Why it is a decision and not a fix.** It changes `Badge` as well as
`Subject` — a badge would gain a subject it does not have today — and
it re-sorts most of the sweep's standing-fact list. Getting it wrong
costs twenty sites written against the wrong rule, which is the thing
the fence around #1933 was built to prevent and the reason that unit
stopped where it did.

## RULED (Ev, #1945, 2026-09-05): state both axes

> "1945's proposal sounds good"

**The two classifications are independent and both get stated:**

- a **badge** is a *read of held state a reader consults*;
- a **line message** is the *outcome of something that just happened*;
- **either can carry a subject**, because a subject only says what
  retires it.

So "is this news or a standing fact" and "what is its subject" are
different questions with independent answers, and the sweep sorts on
the first while assigning the second.

### What this settles, concretely

The three facts of one class in #1933 are **all badges** — each is a
read of seam state (`scene_refusal`, `index_refusal` /
`unindexed_refusal`, `projection_refusal`), and each keeps a subject,
which is now what retires the **badge** rather than what retires a
sentence. That agrees with `status-line-writers-bypass-the-ranking`'s
own classification of all three and resolves it against #1883's
example, which named the projection refusal as news.

**#1883's ruling is not overturned by this.** Its subject-carrying
mechanism stands exactly as ruled and as built; what #1945 adds is that
carrying a subject was never what decided *which channel* a fact goes
to. The projection refusal is a badge that has a subject — which was
unavailable as an answer when #1883 was written, because `Badge` had no
subject then.

### What has to change

1. **`Badge` gains a subject.** That is the API change this ruling
   costs, and it is why this was a decision rather than a fix.
2. **Three call sites move** from the line to the badge family, keeping
   their subjects.
3. **The rule gets written where the classification is made** —
   `crates/viewer/src/frame.rs`'s header, which today states only the
   news/standing-fact half, and `crates/viewer/README.md`.

Per CLAUDE.md the README and header prose ride the unit that makes
them true, not this ruling.

### What it unblocks

`status-line-writers-bypass-the-ranking` — the twenty-writer sweep,
the largest item on this board — which has been waiting for a rule to
sort on since #1849 filed it.


## What landed

`Badge` carries a `Subject`; `frame::SeamSubject` states a seam's
subject once, at the type of its refusal, so a seam speaking on both
channels cannot answer twice; `scene_refusal`, `index_refusal` and
`projection_refusal` became `scene_badge`, `index_badge` and
`projection_badge`, held by `ViewerApp` and read at the toolbar; and
the rule is written in `crates/viewer/src/frame.rs`'s header and in
`crates/viewer/README.md`.

**One of the four doors did not move.** `unindexed_refusal` is raised
by a click and takes the frame's own pick stream, so the ruled rule
makes it an outcome and leaves it on the line, against the ruling's
worked example which named it a badge. The unit built the rule and
disclosed the disagreement as
`work/view/unindexed-refusal-is-an-outcome-not-a-read.md`, which is
for Ev.

---
id: the-news-vocabulary-has-no-expiry
kind: issue
title: Nothing owns when news stops being news, so a stale-but-true complaint sits on the line indefinitely
status: closed
opened: 2026-09-04
closed: 2026-09-05
refs: [camera-fold-clears-status-line, status-line-writers-bypass-the-ranking, opoutcome-superseded-has-no-production-reader]
branch: view/news-and-badges
---

## The gap

`crates/viewer/src/frame.rs` states what the status line is FOR — one
frame's news, ranked by `frame_status` — and `camera-fold-clears-status-line`
landed the two rules that follow from it. **Neither says when a piece of
news stops being news.**

`frame_status` owns which of a frame's messages WINS. The only thing
that ever removes one is `batch_status`'s `StatusUpdate::Clear`, which
fires when a batch contains an op `acts` counts as an action — and
`acts` deliberately excludes `Hover`, which is the only op pure
navigation emits. So:

**Refuse a camera operation, then navigate.** `frame::fold_status`
answers `Keep` on every subsequent clean fold, and no batch acts, so
the camera refusal stays on the line for as long as the user orbits.
Before `camera-fold-clears-status-line`, the next clean fold swept it —
which made camera refusals nearly invisible, so this is plausibly the
better trade, but it is a behaviour change that the stated rule does
not cover and the item disclosed rather than resolved.

Same class, and pre-existing:

- `crates/viewer/src/pane/viewport.rs:324` — `projection: {error}`,
  re-written on every frame until the camera moves.
- `crates/viewer/src/pane/viewport.rs:363` — the two picking paths
  disagreed at a cursor that has since moved on.

None of these becomes FALSE. Each becomes stale-but-true, which is a
different failure and one the current vocabulary cannot express:
`StatusUpdate` has `Keep`, `Clear` and `Show`, and nothing that says
"this was news, and is not any more".

## The question

Does a message carry an expiry, and if so, what expires it?

Candidates, none obviously right:

1. **Nothing — accept staleness.** Argue that a true sentence on the
   line costs less than a swept one, and let `Clear` on an acting
   batch be the only sweeper. Cheapest; the status quo after
   `camera-fold-clears-status-line`.
2. **A message carries its subject**, and a later message about the
   SAME subject supersedes it — a camera verdict expires on the next
   camera event whatever it says, a projection refusal on the next
   camera move, a disagreement on the next cursor move. This is the
   typed-status shape `camera-fold-clears-status-line` explicitly did
   NOT take (its candidate 1) — it did not need it then; this is the
   case that would.
3. **A frame count or a wall clock.** Rejected on sight for a fault,
   but arguably right for a report nobody has to act on.

## What this unblocks

`work/view/opoutcome-superseded-has-no-production-reader.md` asks
whether a discarded free-move probe deserves the line, a badge or
nothing. Answering it needs exactly this vocabulary: a superseded probe
is news if the discard is the outcome of an act, a standing fact if
"the probe you are reading is stale" persists, and neither answer is
expressible until "stops being news" has an owner.

## A named instance, now in the tree (VIEW-6, 2026-09-04)

That item answered "news" and shipped `frame::supersession_notice`,
whose doc says the supersession is "true of nothing" after the frame
that carries it.

**Nothing implements that lifetime.** The notice is joined into
`frame_status`'s rank 2 for one frame and then sits on
`ViewerApp::status` exactly as long as any other message: until an
acting batch clears it. So a user who supersedes a placement and then
only orbits reads "free move: the placement on instance 3 was
discarded" for as long as they navigate — a sentence whose own
documentation says it stopped being true one frame in.

It is a sharper instance than the pre-existing ones above, because
here the expiry is **written down as the justification for the
channel**: the argument for the line over a badge is precisely that the
fact does not outlive the frame. Candidate 2 (a message carries its
subject) would expire it on the next document transition; candidate 1
accepts that the stated lifetime is aspirational and the doc should
stop claiming it.

## Put to Ev (VIEW orchestrator, 2026-09-04)

**The question is unchanged; what is new is that it now gates a
19-site sweep.** `status-line-writers-bypass-the-ranking` cannot be
dispatched until this is answered, because eleven of its nineteen
sites are news and the sweep has to know what news *is* before it can
route them through `frame_status`.

`StatusUpdate` is three variants — `Keep`, `Clear`, `Show(String)`
(`crates/viewer/src/frame.rs:67`) — and nothing in it says "this was
news and is not any more". The three candidates in the body above
stand. The orchestrator's reading, offered as an argument:

**Candidate 1 (accept staleness) is defensible and candidate 2 (a
message carries its subject) is what the tree keeps asking for.** The
evidence for 2 is that the instances are not miscellaneous — each is a
message whose subject has an obvious *next event*: a camera verdict
expires on the next camera event, a projection refusal on the next
camera move, a picking disagreement on the next cursor move, a
supersession on the next document transition. That is one rule, not
four special cases, and it is the shape `camera-fold-clears-status-
line` explicitly declined to build because it did not need it then.

The cost of 2 is that `StatusUpdate::Show(String)` becomes
`Show(Subject, String)` or similar, which every one of the nineteen
writers then has to answer — so the fork is really *"is the sweep a
routing change or a vocabulary change"*, and answering it after the
sweep would mean doing the sweep twice.

**One fact that argues against leaving it at 1**: VIEW-6 shipped
`frame::supersession_notice` whose own doc comment says the
supersession is "true of nothing" after the frame that carries it, and
**nothing implements that lifetime** — it sits on the line until an
acting batch clears it. So the tree already contains a written
lifetime with no mechanism. Candidate 1 is not the status quo plus
nothing; it is the status quo plus deleting that sentence.

## RULED (Ev, #1883, 2026-09-05): candidate 2 — a message carries its subject

> "b sounds good"

**A message carries its subject, and a later message about the SAME
subject supersedes it.** A camera verdict expires on the next camera
event whatever it says, a projection refusal on the next camera move, a
disagreement on the next cursor move, a supersession on the next
document transition.

So this is a **vocabulary change, not a routing change**, and the
consequence stated when the question was asked now binds:
`StatusUpdate::Show(String)` grows a subject, and every writer
`status-line-writers-bypass-the-ranking` sweeps has to answer it. That
is why the sweep waited for this: answering it after the sweep would
have meant doing the sweep twice.

**What this settles that was left dangling.** `frame::supersession_notice`'s
doc says the supersession is "true of nothing" after the frame that
carries it, and nothing implemented that lifetime — the notice sat on
the line until an acting batch cleared it. Under this ruling the
written lifetime becomes the implemented one, so the sentence stops
being aspirational rather than being deleted.

Rides with `four-badges-five-spellings`, per the same PR's answer to
question 3.

## Closed by `view/news-and-badges` (PR #1933)

`StatusUpdate::Show(String)` is `Show(Message)`, `Message` carries a
`frame::Subject`, and `StatusUpdate::Expire(Subject)` retires one
subject without touching the rest. `fold_status`'s clean arm and
`cursor_status` are the two issuers; twelve direct writers hand their
typed refusal to a `frame` door that answers the subject for them, so
the decision is asserted rather than chosen in an `app`-gated paint.

**Two gaps are carried forward rather than closed**, each with a file:
`a-supersession-outlives-its-own-frame` (the fourth of the ruling's
four instances is subsumed by `Clear` and changes nothing) and
`one-line-one-subject-loses-a-mixed-frames-expiry`.

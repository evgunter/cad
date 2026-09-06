---
id: a-supersession-outlives-its-own-frame
kind: issue
title: A supersession's stated one-frame lifetime is still not the one implemented
status: open
opened: 2026-09-05
---


## What was closed, and what was not

`the-news-vocabulary-has-no-expiry` was ruled candidate 2 (Ev, #1883):
a message carries its subject, and a later event about the SAME
subject retires it. The unit that built it (`view/news-and-badges`)
delivers exactly that, and it closes the item's own reproduction —
refuse a camera operation, then orbit, and the next camera event
retires the refusal.

**It does not close the fourth of the four instances the ruling
names.** The ruling says a supersession expires "on the next document
transition". A document transition in this crate is an op
`DocSession::perform` accepted, and a frame carrying one already
answers `StatusUpdate::Clear` (`frame::batch_status`) — which sweeps
the whole line. So `Subject::Document` has no `Expire` issuer that
`Clear` does not already subsume, and the behaviour for a supersession
is unchanged: it survives navigation and goes on the next accepted
edit, exactly as before the ruling.

That is not a defect in the ruling's rule; it is the rule being empty
at one of its four sites, and it is worth a file because
`frame::Withdrawal`'s doc still carries the sentence the original item
was filed over.

## The sentence that is still ahead of the tree

`frame::Withdrawal`'s doc (`crates/viewer/src/frame.rs`) says a
supersession is "true of nothing" after the frame that carries it, and
now states plainly that the implemented lifetime is weaker than that.
So the prose no longer outruns the tree — but the gap it names is
real, and this is where it is tracked.

**The candidate the unit rejected, with its reason.** Subject = the
FRAME, retired by the next frame, would implement the doc's sentence
literally. It was not built because a sentence that lives one frame at
sixty frames a second is unreadable: the frame is not a subject a
reader can use, so implementing that lifetime would be a way of
deleting the message rather than of expiring it.

## The fork

1. **Accept it.** The lifetime argument was always about why a
   supersession is NEWS and not a badge — a standing fact would keep
   saying it about a document the user has moved on from — and that
   argument survives whether or not the sentence goes on frame N+1.
   Then `Withdrawal`'s doc should say the lifetime it HAS and stop
   describing one it does not.
2. **A supersession is about the INSTANCE, not the document**, and the
   thing that retires it is that instance's next event — a selection
   of it, a further edit to it, a gesture on it. That is a real
   subject with a real event stream, it is finer than `Document`, and
   it would need `Subject` to carry a payload (which nothing else in
   the vocabulary does today).
3. **Give `Subject::Document` an issuer distinct from `Clear`** — a
   document transition that is not an accepted act. None exists today:
   `perform` is the only door a document changes through, and the
   evaluation seam lands evaluations, not documents. So this fork is
   about a producer that does not exist yet, and taking it now would
   be writing a rule with no event.

Fork 2 is the interesting one and it is the one the dispatch that
built the vocabulary flagged as least sure. It is a design question,
not a bug: no user-visible behaviour is wrong today, and the cost of
answering it late is one `Subject` variant.

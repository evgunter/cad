---
id: a-supersession-outlives-its-own-frame
kind: issue
title: A supersession's stated one-frame lifetime is still not the one implemented
status: closed
opened: 2026-09-05
closed: 2026-09-16
branch: view/supersession-lifetime
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

## Closed — 2026-09-16, `view/supersession-lifetime`

Fork 1 taken. `frame::Withdrawal`'s *Why the line and not a badge*
section states the lifetime the code has, in three legs with the row
that holds each, and argues it — including that `Clear` is tighter
than a per-instance retirement, which the old sentence made
underivable.

**What this file was wrong about.**

- `frame::acts` is at `frame.rs:561-563`, not `537-539`; `frame.rs`
  was split on 2026-09-16 (#2749) and every number written here
  predates it. `batch_status` is at `571-582`.
- The RULED section says `Subject::Document` has no `Expire` issuer
  *"where three other subjects do"*. **Two do.**
  `frame::SUBJECTS_WITH_AN_EXPIRY_ISSUER` is `[Subject; 2]` —
  `Camera` and `Cursor` — and `Document` shares the absence with
  `Display` and `Preferences`. The `Subject` roster's own paragraph
  says so. The conclusion stands and the arm's doc is accurate; what
  is wrong is the claim that `Document` is asymmetric.
- *"Survives navigation"* holds, and for a second reason the file does
  not give: a camera fold is not a `SessionOp` at all, so it never
  reaches `batch_status`. The subject mismatch on `Expire(Camera)` is
  the other, and either alone is enough.

**No new assertion, argued by mutation.** Four falsifications of
`frame.rs` were built and the suite run against each; every leg of the
new sentence reds an existing row, so a row composing them would be
green on every tree where it would have been the one to catch the
break. The table is in `work/view/log.md`'s 2026-09-16 entry. What the
file was actually missing — a stated lifetime nobody could trace — is
answered by the doc naming the five rows.

**Residue, filed rather than disclosed here:**
`a-doc-comment-names-a-test-row-and-nothing-checks-it-exists`.

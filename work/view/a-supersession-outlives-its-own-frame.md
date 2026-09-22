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

## RULED (Ev, in-chat, 2026-09-16): fork 1 — accept the lifetime

**Accept it.** The lifetime a supersession has is the lifetime it
should have, and `Withdrawal`'s doc should state that rather than
describe the shortfall.

### Why fork 1, with the fact this file did not have

The orchestrator checked what actually retires the sentence before
recommending, and the answer changes the argument: **`frame::acts` is
`!matches!(op, SessionOp::Hover(_))`** (`frame.rs:537-539`), so
`StatusUpdate::Clear` fires on the next thing the user does other than
hovering. The implemented lifetime is therefore **tighter than fork 2's,
not looser.**

That inverts the fork. Fork 2 — subject = the INSTANCE — would make a
supersession about instance A survive a selection of B, a hide of C and
an edit elsewhere, and retire it only on A's next event. For a NEWS
channel that is the wrong direction: the sentence would live longer
exactly where the reader has visibly moved on. This file called fork 2
"the interesting one"; it is interesting, and it is worse.

**And nothing semantically needs the instance granularity.** A
supersession reports a completed historical fact — an accepted edit
discarded a committed free-move placement on these instances. Once
true it stays true, so it cannot go FALSE with age, only stale, and
`Clear` retires it at the earliest reasonable moment. The case that
would break this — the sentence persisting while the user re-places A
by hand, so the line contradicts the picture — does not arise, because
re-placing A is a non-hover op and clears the line. **Fork 2 would
INTRODUCE that risk rather than remove it**, since only A's own events
would retire the sentence and "A's event" would then have to be got
right.

What is actually missing is narrower than this file's framing: the
`Subject::Document` arm has no typed `StatusUpdate::Expire` issuer,
where three other subjects do. That is a type-level asymmetry, not a
semantic gap, and `Clear` does strictly more than that `Expire` would.
The arm's own doc already says so.

**The orchestrator miscounted, and the taker caught it — see the
Closed section below.** `frame::SUBJECTS_WITH_AN_EXPIRY_ISSUER` is
`[Subject; 2]`, Camera and Cursor, and `vocab.rs` already says *"names
two of five"*. So `Document` shares the absence with `Display` and
`Preferences` and is not asymmetric at all; the sentence above is
wrong about the count and about the asymmetry. What it is right about
is the only thing the ruling rests on: `Clear` does strictly more than
that `Expire` would, so the missing issuer is not a reason to change
the subject. The ruling stands; this paragraph's reasoning for it does
not, and is kept as written with this correction beside it rather than
edited in place, because a ruling's record is what was decided ON.

### What a taker owes

Small, and `frame.rs` is the whole of it.

- `Withdrawal`'s doc (`frame.rs`, the *Why the line and not a badge*
  section) currently says *"**That is a weaker lifetime than the
  argument above wants**"* and frames the difference as a shortfall
  stated rather than papered over. Under this ruling it states the
  lifetime it HAS and why that is right — including that `Clear` is
  tighter than a per-instance retirement would be, which is the part no
  reader can currently derive from the sentence.
- Check the *"survives navigation"* clause against the tree rather than
  inheriting it: a camera fold is not a `SessionOp`, so the sentence
  surviving an orbit is correct and worth keeping — but verify.
- `Subject::Document`'s own note about having no `Expire` issuer is
  already accurate and is not this unit's to change.

This is a design question answered, so the ruling is Ev's and the
wording is the taker's. Nothing else on the board depends on it.
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

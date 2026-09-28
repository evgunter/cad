---
id: a-refusal-and-a-loss-from-one-fault-say-it-twice
kind: issue
title: A refusal and a withdrawal caused by one fault render that fault twice on one line
status: open
opened: 2026-09-25
priority: P3
cost: E
refs: [rank-one-discards-the-frames-other-news]
---


Disclosed by `rank-one-discards-the-frames-other-news`'s fix, which
lets news nothing will say again ride beside a refusal on one line
(`frame::frame_status`, `# The ranking`).

## What happens

A panel's mate lands on an instance the user had placed by hand, and a
drag on the same instance in the same frame refuses. Both halves are
caused by ONE fault, `AdmissionFault::MateConstrained`, and each
renders it through its own `Display`, so the line reads (pinned
verbatim by `crates/viewer/tests/frame_policy.rs`,
`a_superseded_free_move_is_news_the_ranking_shows`):

> instance 2 is mate-constrained (mate node(s) 3): its pose is
> mate-derived, so the free-move probe refuses — delete the mate(s) if
> free relative motion is intended • free move: a committed placement
> was discarded — instance 2 is mate-constrained (mate node(s) 3): …
> (the same sentence again)

Every word is true, and the second half carries news the first does
not — that a committed placement was discarded. What it repeats is the
CAUSE: the one fault, rendered in full twice, so the reader meets its
long sentence a second time to learn one new clause.

**A second instance of one cause behind both halves: a killed drag
beside its own refusal.** A panel's edit lands while the user is
dragging an instance it makes inadmissible; the prune kills the drag
(`WithdrawalKind::KilledGesture`, worded with the admission fault) and
the drag's `PreviewFreeMove` later in the same frame refuses. That
refusal is the gesture door's "no free move in flight"
(`DisplayState::preview_free_move` checks the held gesture and never
re-reads admission), so here the cause is NOT said twice: the line
reads the symptom first and the cause second, which is the reverse of
the order a reader diagnoses in. Whether one fix answers both — the
refusal naming what it follows from, or the line ordering a cause
before its consequence — is the question this row carries; the
ranking's own "refusal first" is argued at `frame_status`.

## Why it is not simply "de-duplicate"

The join's rule is that each notice is its own typed value's own
rendering and nothing composes prose about another's failure
(`frame_status`'s doc). Dropping the second copy means the chrome
deciding two values say the same thing, which is a comparison of
rendered text or of faults — the question is whether that is a rule
the module can state, or whether the withdrawal's preamble should
refer back ("…discarded, for the reason above") when the refusal
carries an equal fault. `one-line-one-subject-loses-a-mixed-frames-expiry`
(the line as several labels) may dissolve it instead.

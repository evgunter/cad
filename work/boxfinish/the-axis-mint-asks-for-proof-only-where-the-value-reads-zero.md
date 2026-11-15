---
id: the-axis-mint-asks-for-proof-only-where-the-value-reads-zero
kind: issue
title: The profile's axis mint asks whether a chord component is an identity only where its value reads Zero, so an identity that rounds, or escalates in a deferring session, takes the quotient at one valuation and the axis at another
status: open
opened: 2026-11-15
priority: P1
cost: M
design: true
refs: [6963, 6990]
---


Found by PR 6990's review (the repair of PR 6963's direction mint).
PR 6990 lands as a narrowing of the defect it repairs; this row is what
it leaves. The general question it rests on is DECIDE's:
`work/decide/a-construction-can-ask-whether-a-value-is-an-identity-only-by-taking-a-sign-reading.md`.

**What the mint does.** `profile::seg::axis_or` mints a line's director
from its two points. It stores the exact axis where one component of
the chord is zero AS AN IDENTITY of the parameters (a hop `W − W`), and
the quotient everywhere else. It learns "identity" by reading the
component's sign and then asking whether that Zero was proven
(`MarginDiag::proven`, or `sym::deferred::discharge` in a deferring
session).

**What is wrong.** The sign reading consults proof only where the
value channel does not read a definite sign. So whether an identity is
found depends on the value at the valuation:

- **`Sym<f64>`, an identity that rounds** (demonstrated, red probe
  `review_an_identity_that_rounds_takes_one_branch_at_sym_f64` on
  branch `boxfinish/review-axis-mint`, `80ffc4c5`). The hop
  `(x, 0) → ((x + w) − w, 1)`: at `(x, w) = (1, 1)` the chord's `x`
  computes `0.0`, the tier proves it, the mint stores the axis and
  keeps two readings (88 decisions). At `(0.1, 0.3)` it computes
  `2.8e-17`, a definite sign, the tier is never asked, the mint stores
  the quotient and discards (86 decisions). Eager and deferring
  sessions alike. `Sym<f64>` deferring is the coincidence door's replay
  lane and the lane `revalue_corpus` records, so a document with two
  spellings of one real in a hop's ends (`cx + w/2` against
  `cx − w/2 + w`) would turn a corpus row red as the five went red. No
  corpus document has one.
- **A deferring session at `Sym<Interval>` over a box** (demonstrated,
  probe `review_the_slabs_side_takes_one_branch_in_a_deferring_session`).
  The slab's side `(W, 0) → (W, 1)`: eager, the exact axis over a box
  or a point; deferring over a point box, the exact axis; deferring
  over a box of half-width ε/64, the quotient with `x ∈ ±3.1e-11`.
  `W − W` straddles zero, the reading escalates, the deferring session
  returns the escalation unchanged and there is no Zero to discharge.
  Latent: the only production deferring session runs at `Sym<f64>`.

**What still holds after PR 6990.** An axis is minted only where the
tier proved the zero, so no constant is stored where the computation
varies with the parameters (the defect the five corpus rows caught).
What does not hold is that the same document takes the same branch at
every valuation.

**Seen beside it, the tier's and not this mint's:** the identity test
reads the witness's branch of a `max`: `(max(a, b), 0) → (b, 1)` at
`Sym<Interval>` over point boxes mints the axis when `a < b` and the
quotient when `a > b`, and the flip at `max` is recorded nowhere.
Whether that is E12's "wherever the construction takes the decisions
it took" or a hole in it is DECIDE's row's to weigh.

**The question** (a design fork, to designers and then Ev): how a
construction that wants to store an identity as a constant should ask
whether it is one, at every scalar and in every session, so that the
answer is a function of the document and not of the valuation. The
lane's sketch was a non-deciding query on `geom_core::Decide`
(`proves_zero`); the alternatives are the designers'.

**Closed when** the mint's branch is the same at every valuation of
one document in every lane it runs in, the two probes above are green
as ordinary rows, and `axis_or` reads no sign it then throws away.

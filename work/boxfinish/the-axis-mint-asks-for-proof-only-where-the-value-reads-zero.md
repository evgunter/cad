---
id: the-axis-mint-asks-for-proof-only-where-the-value-reads-zero
kind: issue
title: The axis mint asks the tier nothing where the value reads a definite sign, nor on an escalation in a deferring session, so one identity takes two branches at two valuations
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

## Weighed (fork 272, 2026-11-15): three designers agree after five rounds

**The structural question.** `geom_core::is_exact_zero` becomes "is
this value zero at every point its scalar stands for", and is not a
decision. `f64` answers yes at 0; `Interval` only at `[0, 0]`; `Dual`
only where the value and every derivative are 0; `Sym` only where the
tier's normal form proves the expression zero, with every rule that
reads a value off (the certified read at `min` / `max` / `select`; the
registered-identity door) and with rules and a term budget fixed for
the kernel. At `Sym` nothing reads "exactly zero" off the number, so a
new caller cannot write the defect the corpus rows caught. Its one
licence is to choose between two spellings of one real value.

**The mint, one function.** Where one component of the vector is zero
by the question, the axis, signed by a reading of the other component;
otherwise the vector over its length, and no reading. The `line_to`
verb calls it and records the director, and that hop's junction checks
read the same value (today `path::chord_director` and validation spell
the hop's direction twice). Validation calls it only for a table. An
arc's radial uses it. `sym::aside` and the discarded readings go.

**The sign reading** is a new fact (vertex separation decides the
length, not which way the line points). It is kept; it sits outside
the cross-lane verdict log under its audit row, since the `f64` witness
and the leaf can take different arms, and the re-valuation instrument
still records it; where it does not decide, the escalation propagates
as the verb's error, with no fallback to the quotient.

**The other sites.**
- `literal_sign` (a bulge's sign) is a fact about a value: an ordinary
  recorded decision.
- `is_exact_zero`'s other callers (about two dozen) are audited in the
  same change: a read of a literal the kernel wrote answers as today; a
  choice of spelling on a computed value changes only where two values
  agree by coincidence or an identity rounds; a branch that changes
  what is built (a bulge of zero lowering to a line, which repeats
  validation's straightness decision) becomes a margined decision.
- The readers of "was this Zero proven" (`MarginDiag::proven`,
  `deferred::discharge`) and the unproven-coincidence lint stay as they
  are; a construction's registered identities count as established.
- `[verdicts.consumers]`' "Zero takes the closed form" is amended for
  every lane choice, with an audit to follow.

**Rejected as final states, by all three:** PR 6990's spelling (a
decision reads the value first by design); taking the axis rule out
(the slab's wall turns over the box); making the value channel exact
(`h/|h|` still needs a sign); a tagged direction type; a separate
query beside `is_exact_zero` (two functions that must agree at `Sym`).

**What it costs.** Every `is_exact_zero` caller's answer at `Sym` and
`Dual` can change the day it lands, so the audit is part of the
change. With the symbolic tier switched off by its dial the driver
runs plain `Interval`, and a literal-zero component there stores the
axis, so that replay is no longer the pre-E12 one byte for byte on such
a profile. The question needs a memo of its own.

**To trace before building** (none changes the answer): how an
escalation taken outside the verdict log reaches the subdivision
driver; one run of the value-free walk on the corpus's hop identities;
one measured run of the lint; the per-caller audit. Deferred, to be
measured on a real document first: a "constant ratio" form of the
question, which would make a 45° side with a boxed size exact.

**How the rounds went.** The first reports agreed on a question that
is not a decision and split on four things; the box-certified reads
and where the mint lives closed in round 1, the lint and the sign
reading in round 2. One door or two crossed over three times before
all three settled on one in round 4, and the budget in round 5. The
reports are on `boxfinish/fork-identity-reports`
(`probe/fork-identity/round0` to `round4`), by label.


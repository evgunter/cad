---
id: the-axis-mint-asks-for-proof-only-where-the-value-reads-zero
kind: issue
title: The axis mint asks the tier nothing where the value reads a definite sign, nor on an escalation in a deferring session, so one identity takes two branches at two valuations
status: review
opened: 2026-11-15
priority: P1
cost: M
refs: [6963, 6990]
branch: boxfinish/expression-question
pr: 7488
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

## Revised on Ev's first answer (2026-11-15): the question has an answer only at the symbolic scalar

Ev, on `[ev]` PR 6993: "this makes sense at the symbolic tier but sits
uncomfortably with the eps allowance for numeric tiers".

Each of the three designers, asked alone, moved to the same narrower
answer, and each called the discomfort right: at a numeric scalar the
design's only zero is the band's (`[verdicts]`: bit-equality is never
the test), and "`f64`: yes at 0; `Interval`: only `[0, 0]`" would have
ratified a second one, which is this row's own defect one scalar down
(`(x + w) − w − x` reads `0.0` at one valuation and `2.8e-17` at the
next).

**What stands now, replacing the section above where they differ:**
- The structural question (is this value zero as a function of the
  document's variables) is answered only by `Sym`, from the expression,
  by value-free rules under a kernel-fixed budget. `f64`, `Interval`
  and `Dual` hold no expression and answer no, so a numeric lane always
  takes the general spelling: the quotient, which is what PR 6990
  already does and which at `f64` is the axis but for a zero's sign.
- So the `Dual` clause goes (the quotient carries its derivative), and
  so does "every caller audited in the same change": the mint's change
  no longer touches `is_exact_zero`.
- `geom_core::is_exact_zero` is not the door and gets no ratified
  text. The designers audited its callers (17 to 22 sites, counted
  three ways): none asks whether a computed quantity vanishes by
  identity. They are in-band tags, bit-for-bit comparisons of two
  stored copies, and a few branches that change what is built decided
  by bit-equality
  (`work/decide/is-exact-zero-reads-tags-copies-and-decisions-off-a-number.md`).
- Unchanged from the section above: the value-reading rules off and
  the fixed budget; the mint as one function called by the `line_to`
  verb and by validation for tables; the one kept sign reading, outside
  the verdict log; the lint; the `[verdicts.consumers]` amendment.

**The cost named by one designer:** with the symbolic tier switched
off, a plain-`Interval` side between literal-equal coordinates stays
as wide as rounding makes it; that lane already refuses identities
over a box.

## Revised on Ev's second answer (2026-11-22): the question is a method of the expression, and a type error at a numeric scalar

Ev (2026-11-16): "re 'f64, Interval and Dual hold no expression and
always answer no', is it possible to just make it sym only and a type
error for these ones?" Each of the three designers, asked alone and
reading current main, said yes; after three short rounds they
recommend one shape (E12 carries it).

- **Each scalar declares the expression it carries**
  (`Real::Expression`): `Sym` its node; every numeric scalar a type
  with no values. This is the device `crates/geom-core/README.md`
  DL5.3 ratifies for a dual's lanes (`geom_core::NoLane`), and it
  passes DESIGN `[unrepresentable]`'s "a signature is a claim" (Ev,
  2026-11-20): the accessor's sentence at a numeric scalar is "this
  scalar carries no expression", and no impl chooses an answer.
- **The question is a method of the expression** and returns a proof
  only the tier mints. Written against an `f64`, an `Interval`, a
  `Dual` or a generic `T`, it does not compile; no impl anywhere
  answers "no".
- **The mint stays one generic body**: where a component's expression
  is proved zero, the axis signed by a reading of the other component;
  otherwise the chord over its length. At a numeric scalar the axis
  arms are unreachable by type, so the sign reading does not exist
  there. `read_component`, `Component::UnprovenZero` / `Escalated` and
  the `aside` go.
- **Code whose whole meaning is symbolic is bounded** to a trait only
  `Sym` implements (`register_equal` and `free_identity`, moved later:
  not this change).
- **The zero proved is E12's**: it holds at every parameter value at
  which the build takes the decisions it took. "The same at every
  valuation" overstated it; what is the same whatever the value
  channel reads is whether a proof exists for one graph.
- **Weighed and withdrawn by their proposers:** a spelling combinator
  on the scalar trait (`spelled(x, closed, general)`: it is the old
  "answers no" under another name, since `spelled(x, || true, || false)`
  reads the answer back at `f64`, and it cannot serve a site that asks
  about several values); a per-scalar item at each spelling site
  bounded away from the numeric scalars (one trait and five impls per
  site, and one function split in two against `[one-way]`).
- **Not in the ratified text, for the unit:** a helper over the
  expression that takes two spellings and returns one is the natural
  way for most callers to honour the licence; `Dual<Sym<…>>` carries no
  expression, `Sym<Dual<…>>` (instantiated in tests) does and the
  closed spelling is right there; a check in the symbolic lanes that
  the two spellings agree on the value channel.
- **New on main since the audit:** `Real::is_exact_zero_in_every_channel`
  behind `UnitVec3::is_exactly` (the least turn, `mate/value.rs`,
  `sweep/skin.rs`), a second structural door read off numbers; added
  to DECIDE's row.
- **Still owed by the unit, unchanged:** how an escalation from the
  closed spelling's sign reading reaches the subdivision driver.

**What the doors-read lane measured meanwhile bears on the mint's
value, not on this text:** a door that reads the minted axis on one
wall against a `s/|s|` quotient on its partner loses identities that
cancelled when both were quotients (five ring-seat rows; the row that
carries it is the sweep doors' reading row). The mint's unit owes that
measurement before any door reads its record on a line.

## Ev's answer (2026-11-22): the design stands; DESIGN.md is not touched

Ev on PR 6993: "this sounds good, but the additions to DESIGN.md are
much too detailed and may not be needed at all, since the stuff about
making sure that everything that's allowed for all generics actually
makes sense for all of them is already in there."

So the PR changes no line of `docs/DESIGN.md`. `[unrepresentable]`
("a signature is a claim", PR 7189) already says a numeric scalar is
excluded from what has no meaning there, and `[scalars]` already says
a lane scalar's choice is the same across the box, at a `Sym` over the
unknowns' whole range. What is ratified here is ERROR-DESIGN E12's
structural-question bullet and one clause of the profile's README. The
mechanism (`Real::Expression`, a type with no values at a numeric
scalar; the question a method of the expression; a bound for code
whose whole meaning is symbolic) is written beside the code when the
unit builds it, in `crates/geom-core/README.md`, as DL5.3 is.

The earlier sections above that quote text for `[scalars]` and
`[verdicts.consumers]` record what was weighed; none of it landed.


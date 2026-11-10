---
id: an-argmin-by-decided-comparison-is-spelled-three-ways
kind: issue
title: The decided argmin (geom_brep::ladder::argmin_decided) refuses at its first in-band comparison; what an in-band comparison leaves of the minimum, and each site's tie rule, is open
status: open
opened: 2026-11-09
priority: P3
cost: M
---


Found by QML tranche 2 (`work/onecopy/question-mark-ladders-the-gate-registered.md`,
`onecopy/refusal-standing`), reading `offset_derive::nearest`, which the
row listed "for its argument, not as a clear port".

## What

Three folds keep the nearer of an incumbent and a candidate by a
decided comparison:

- `topo::boolean::join::keep_nearer` (≈`join.rs:2267`): the incumbent
  keeps a tie (D9), and a comparison in band refuses by `?`. It is typed
  on `PartnerDistance` and `BooleanError`, so it serves join alone
  (`find_match`, the walk order, its rows).
- `topo::offset_derive::nearest` (≈`offset_derive.rs:601`): a decided
  `Zero` refuses `Unsupported` ("two branches of the section lie equally
  near the edge"), and an in-band comparison escalates by `?`, the site
  `escalation-question-marks.counts` registers.
- `topo::link` (≈`link.rs:1122`, the piece-end turn fold): the incumbent
  keeps anything not `Less`, and the order's escalation returns by `?`.

The three differ in what a tie is (kept, refused, kept), so they are
three answers to one question, spelled apart.

## The argument the register listed it for

`climb`'s rule ("a reading in band is kept, and refuses only where no
later reading answers") does not transfer to an argmin as it stands. An
in-band comparison between the incumbent and a candidate leaves the
minimum open: either may be the nearer. A later candidate decided
nearer than **both** closes it, since the in-band pair is then not the
minimum whichever is nearer. One decided nearer than only the incumbent
does not. So an argmin's kept reading is a *set* of candidates still in
contention, not one rung, and the rule is "refuse only where no later
candidate is decided nearer than every one in contention".

## What closing it takes

One home: an argmin over decided comparisons that carries the
in-contention set and names its tie rule (keep the incumbent, or
refuse). Port the three folds onto it. Then decide, per site, whether
the set rule above is the one it wants. That is a design question, for
the design-fork procedure before anyone builds it. Until then
`offset_derive::nearest`'s `?` stays registered with this row cited.

## Narrowed by `onecopy/argmin-and-leftovers` (2026-11-10)

The designers' reconciled ruling is built:
`geom_brep::ladder::argmin_decided(cands, cmp, FreeOf)`, the set rule
(a candidate drops out only on a decided reading against one; every
pair is compared, so the result is free of order), with the ending by
what the caller is free of (`FreeOf::{Nothing, Ties, Band}`) and its
recording (an undecided reading is spliced superseded where the minimum
does not rest on it). Ported, with no outcome moved over the `ci` sets:

- link's piece-end fold: `Ties`.
- `offset_derive::nearest`: `Nothing`; its tie is
  `SectionVerdict::BranchesTied`, and its `?` has left the escalation
  register.
- join `own_nearest` (was `keep_own`) and `loose_partners`: `Nothing`;
  a tie refuses as `BooleanError::JoinPartnersTied` (standing `InBand`).

## What remains: `first_pair`

`first_pair` stays on its old fold, which keeps the incumbent on an
exact tie in both lanes. Ported at `Band` in its free lane and `Nothing`
otherwise, it refused 759 calls over 525 `ci` test processes that build
today, every one an exact tie (no in-band reading) between mutual pairs.

**The one blocker is a tie with a conic germ in the round**: 663 of 751
tied sets have conic tied pairs, and the other 88 have straight tied
pairs beside another conic germ. Freeing it needs a proof that, where a
germ ranks its partners by travel rather than by chord, committing any
one of a tied set of mutual pairs first gives one matching.

**Draft proof for straight germs** (not a live declaration). Every germ
straight, `K > 2`, and `T` the pairs tied at the round's minimum, each
mutual; commit `P = (g, h)`.
- A germ's nearest partner is decided apart from every other
  (`own_nearest` is free of nothing). On a straight germ the order
  along its line is the chord's, so for a mutual `Q = (p, q)` in `T`,
  every other partner `y` of `p` stands more than `K·zero` farther than
  `q`. A pair `(y, p)` then reads decided farther than `Q`, and cannot
  be committed before it: `Q` keeps its germs until it is committed, and
  stays mutual with its key unchanged.
- A germ `y` whose nearest was `g` or `h` takes a partner decided
  farther than that one, which was not decided nearer than the minimum,
  so its new pair stands past the minimum by more than `(K − 1)·zero`:
  farther than `T`'s pairs at `K > 2`, or in band, which escalates the
  run under either order.
- So both orders commit every pair of `T` before any pair touching their
  germs, and reach one state: the matching is free of which goes first,
  though not the run.

A conic germ ranks by travel, so the first step does not hold for it.


## The tie with a conic germ in the round (`onecopy/first-pair-conic-counterexample`, 2026-11-10)

### A tie between mutual pairs is not enough

The row `boolean::join::first_pair_rows::a_tie_between_mutual_conic_pairs_can_move_the_matching`
is a counterexample. It runs through `Turn::facing`, `germ_arm`,
`own_nearest` and `first_pair` as they are.

**The configuration.** Six germs lie on one ellipse loop of aspect 10.
In counterclockwise order their senses read `+ + − + − −`:
- z+ at −80°
- q+, placed so that chord(q, p) = chord(h, g) = 5.00 m
- p− at 30°
- h+ at 60°
- g− at 90°
- w− at 150°

**The first round.** The nearest partners are z→p, q↔p, h↔g and w→h, so
the two tied pairs are both mutual.
- **`(q, p)` first:** z turns to its next partner by travel, g. Their
  chord is 2.64 m, shorter than the tie: the chord is not monotone in
  travel. So `(z, g)` is committed ahead of `(h, g)` and takes g from h.
  The matching is `{(z, g), (q, p), (h, w)}`.
- **`(h, g)` first:** the matching is `{(h, g), (q, p), (z, w)}`.

Both runs complete. This is where the straight draft above breaks: its
second step needs a germ's next partner to rank past the tie by chord.
Along a conic it ranks past it by travel, and the chord can be shorter.

### What does hold: an all-mutual round

**Premise.** Every germ that has a nearest partner in the round is that
partner's nearest partner: the map from each germ to its nearest
(`own_nearest`'s result, `first_pair`'s `own`) is an involution. Call
its pairs `S`. This is the mutual test `first_pair` already makes on
`own` (`join.rs:2417`).

**Theorem.** Every run from such a round that completes commits exactly
`S`, in some order. This holds whichever pair `first_pair` commits on a
tie or an in-band order, in either lane, at any `K`, and for any mix of
straight germs, conic germs (ranked by travel) and germs running along
an edge (ranked by foot travel).

**What the code guarantees.**
- **G1: a round is a function of the unused slots.** `find_match` reads
  `partners` over the slots `slots` leaves unused (`join.rs:2321`,
  `join.rs:2336`). `partners` reads the records' germs, the reduction
  and the two `Sides`, and never a used flag (`join.rs:2448`).
  `matched_records` changes nothing between rounds but the two flags of
  the committed pair's slots, in both solids (`join.rs:871`).
- **G2: a germ's candidates keep their order.** A germ's partners are
  met in the fixed enumeration order of records and slots. A later
  round's candidates for that germ are a filtered subsequence of an
  earlier round's. So `argmin_decided` reads each pair of them with the
  same roles in every round, and `along_sign` is a pure function of
  those two `PartnerDistance`s and the band.
- **G3: the set rule** (`geom_core::ladder::argmin_decided`,
  `ladder.rs:1042`).
  - Every pair is compared.
  - A candidate drops out only where some candidate is decided nearer
    than it.
  - At `FreeOf::Nothing` (`own_nearest`), the result is the one
    candidate left, or none where none is left. Two or more left refuse
    `Contested`, and an `Aborts` reading stops the argmin.
- **G4:** `first_pair` returns one of `own`'s matches, or `None` only
  where `own` is empty. The commit consumes exactly that match's two
  slots.

**Lemma (a germ's nearest stands while its partner does).** Let germ
`y`'s nearest in round `U` be `x`, and let `U' ⊂ U` hold both `x` and
`y`. Then round `U'` gives `y` the nearest `x`, or it refuses.
- By G1 and G2, `y`'s candidates in `U'` are a subset of those in `U`,
  and every pair among them reads as it did in `U`.
- No reading in `U'` aborts: it would have aborted `U`.
- `x` was decided nearer-free in `U`, meaning no candidate was decided
  nearer than it. So it is nearer-free in `U'`, and at least one
  candidate is left.
- By G3, either `x` is the only candidate left, or the round refuses
  `Contested` (`JoinPartnersTied`, or the in-band reading).
- A germ with no partner in `U` has none in `U'`.

**Proof of the theorem**, by induction on `|S|`.
- **Base.** `S` empty: `first_pair` returns `None` and the run ends with
  nothing committed.
- **Step.** By G4 the round commits some `P ∈ S`. Let `U' = U ∖ P`, and
  take any `y` with a nearest, outside `P`.
  - `y`'s nearest `n(y)` is not in `P`: if it were, `y = n(n(y))` would
    be too.
  - By the lemma, `y` keeps `n(y)` in `U'`, or the run refuses.
  - A germ with no nearest in `U` has none in `U'`.
- So a completing run's next round has the involution `own` restricted
  to `S ∖ {P}`, and the induction applies. ∎

**Scope.**
- **The matching is free of the order; the run is not.** An
  intermediate state can refuse under one order and not another: the
  lemma's "or it refuses" can hold for a germ whose dominators leave
  before its own pair is committed. The order still picks which pair
  the surgery joins first.
- **Nothing here reads chords, travel or `K`.** So for an all-mutual
  round, the free lane's conditions `turn.is_none()`,
  `along.is_none()` and `band.escalate() > 2·zero` (`join.rs:2407`)
  carry nothing.
  - The `K > 2` step of the doc at `first_pair` bounded the old
    incumbent scan in `keep_own`. The set rule's lemma replaces it.
  - The row
    `first_pair_rows::an_all_mutual_rounds_matching_is_free_of_the_commit_order`
    shows this on the `K = 1.1` fixture. Its first round is all mutual,
    and folded with every in-band order kept, in scan order and in
    reverse, it commits that round's four pairs both ways.
  - `at_k_at_most_2_first_pair_reads_the_order_among_mutual_pairs`
    still pins today's gate, which escalates there.
- **Rounds that are not all mutual** (a tie among mutual pairs beside a
  germ whose nearest lies in one of them) are outside the theorem.
  - The counterexample above is one, with conic germs.
  - The straight draft above covers such a round only where every germ
    whose nearest lies in a committed pair ranks by chord, at `K > 2`.
    That holds where those germs are straight: a germ's partners share
    its loci (`partners` requires equal `a_locus` and `b_locus`), so
    the conic germs of other locus pairs in the round do not reach it.
  - That draft was written against the old scan, and the set rule has
    not been re-checked against it here.

### Alternation, and how a check would read it

**The geometric premise.** For each locus pair `(a_locus, b_locus)` and
each loop of its section (the frame `germ_loops` picks by the germ's
`branch`), take that loop's unused germs in rotational order about the
frame's axis. Their rotational senses (`rotational_sense`) alternate
around the loop.

**Alternation implies every round is all mutual, where every reading is
decided.**
- A `+` germ's nearest by travel is the next germ counterclockwise. That
  germ is `−`, and none of its other `−` candidates comes before it.
  - If the neighbour is `Ahead`, it ranks first.
  - If it is `Behind`, so is every other candidate, and travel ranks it
    first among them.
- A `−` germ's nearest is likewise the next germ clockwise, the same
  neighbour.
- Committing a neighbour pair leaves the loop alternating.

So the checkable premise is the weaker one: the mutual test on `own`.
A check of alternation itself would have to run in `find_match`, not in
`first_pair`. `own` holds only germs that have a nearest, and carries no
locus or branch. Run in `find_match`, it would:
1. Group the unused slots of `open` by `(a_locus, b_locus, branch)`.
2. Order each group by angle about the loop's frame. That is a decided
   reading, so a tie or an in-band reading refuses.
3. Read each germ's sense by `rotational_sense`.
4. Require the senses to alternate cyclically.

It costs a sort per loop and a new decision, and it frees nothing that
the mutual test does not.

**Recommendation to the argmin lane.**
- Free `first_pair`'s tie (and its in-band order) exactly where `own` is
  all mutual, citing the theorem above.
- Otherwise, refuse a tie between pairs with a named cause, and pin it
  with the counterexample row.

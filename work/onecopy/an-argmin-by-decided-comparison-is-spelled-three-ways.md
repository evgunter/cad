---
id: an-argmin-by-decided-comparison-is-spelled-three-ways
kind: issue
title: The decided argmin (geom_core::ladder::argmin_decided) is built and ported, first_pair included; the folds left outside it are registered
status: open
opened: 2026-11-09
priority: P3
cost: M
---


Found by QML tranche 2 (`work/onecopy/question-mark-ladders-the-gate-registered.md`,
`onecopy/refusal-standing`), reading `offset_derive::nearest`, which the
row listed "for its argument, not as a clear port".

## What

Three folds kept the nearer of an incumbent and a candidate by a
decided comparison, each with its own tie rule: join's `keep_nearer`
(the incumbent keeps a tie, an in-band comparison refuses by `?`),
`offset_derive::nearest` (a decided `Zero` refuses, an in-band
comparison escalates by `?`), and link's piece-end turn fold (the
incumbent keeps anything not `Less`). They were three answers to one
question, spelled apart. All three now read one door,
`geom_core::ladder::argmin_decided` (below); join's `first_pair` is the
fold left on its old spelling.

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
`geom_core::ladder::argmin_decided(cands, cmp, FreeOf)`, the set rule
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

## `first_pair` ported (`onecopy/first-pair-all-mutual`)

`first_pair` reads the door on the prover's theorem below, in two lanes:
- **free of `Band` where the round is all mutual**, at any `K`, and for straight, conic and along-edge germs alike;
- **free of `Nothing` elsewhere.** There a tie between two pairs refuses (`BooleanError::JoinPairsTied`, standing `InBand`), and an in-band reading left in contention refuses on its margin.

**The old gate is gone.** It asked for straight germs and `K > 2`. Those premises bounded the per-germ incumbent scan that `own_nearest` replaced, and the theorem reads no chord, travel or `K`. The incumbent fold (`nearer`, `nearer_or_scan`) is gone too.

**Measured over the whole workspace `ci` set, both ε:**
- about 31 000 `first_pair` calls read an exact tie, every one in an all-mutual round;
- the 525 conic-tie processes (526 at default ε, 519 at `1e-12`) are all among them;
- no production round that is not all mutual meets a tie or an in-band reading. Only the counterexample's pin and the premises row do.

**Not all mutual: a tie refuses, measured and unguarded on purpose.** The `ci` set holds no production round that is not all mutual and meets a tie or an in-band reading; that is a measurement, not a guarantee. No counter guards it, because the lane cannot go wrong silently: such a round's tie refuses, typed (`JoinPairsTied`), so a production reach shows as a refusal naming its cause, never as a matching chosen by scan order. A straight-germ argument that would free such a tie at `K > 2` was drafted against the old scan and is not carried: nothing measured needs it.

## The tie with a conic germ in the round (`onecopy/first-pair-conic-counterexample`, 2026-11-10)

### A tie between mutual pairs is not enough

The row `boolean::join::first_pair_rows::a_tie_between_mutual_pairs_beside_a_non_mutual_germ_refuses`
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

Both runs complete: a germ's next partner ranks past the tie by travel
along a conic, and its chord can be shorter than the tie's.

### What does hold: an all-mutual round

**Premise.** Every germ that has a nearest partner in the round is that
partner's nearest partner: the map from each germ to its nearest
(`own_nearest`'s result, `first_pair`'s `own`) is an involution. Call
its pairs `S`. This is the mutual test `first_pair` makes on `own` to
choose its lane.

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
- **Nothing here reads chords, travel or `K`.** So an all-mutual round
  is free whatever its germs and band, and `first_pair`'s free lane asks
  no more than the involution: the old gate's straight-germ and
  `K > 2` premises are dropped.
  - The `K > 2` step bounded the old incumbent scan in `keep_own`. The
    set rule's lemma replaces it.
  - The row
    `first_pair_rows::an_all_mutual_rounds_matching_is_free_of_the_commit_order`
    shows this on the `K = 1.1` fixture: its first round is all mutual,
    and `first_pair`, in scan order and in reverse, commits that round's
    four pairs both ways.
  - `first_pair_rows::an_in_band_order_is_free_exactly_where_the_round_is_all_mutual`
    pins the lane: free at `K = 2` and on conic germs, escalating where a
    germ's nearest is not mutual.
- **Rounds that are not all mutual** (a tie among mutual pairs beside a
  germ whose nearest lies in one of them) are outside the theorem.
  - The counterexample above is one, with conic germs.
  - They refuse a tie (`JoinPairsTied`), whatever their germs.
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

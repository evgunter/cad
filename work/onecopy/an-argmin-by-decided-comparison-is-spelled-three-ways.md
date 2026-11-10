---
id: an-argmin-by-decided-comparison-is-spelled-three-ways
kind: issue
title: The decided argmin (geom_core::ladder::argmin_decided) is built and ported; first_pair's tie between mutual pairs stays on its old fold
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

## What remains: `first_pair`

`first_pair` stays on its old fold, which keeps the incumbent on an
exact tie in both lanes. Ported at `Band` in its free lane and `Nothing`
otherwise, it refused 759 calls over 525 `ci` test processes that build
today, every one an exact tie (no in-band reading) between mutual pairs.

**Measured: a tie with a conic germ in the round.** 663 of 751 tied
sets have conic tied pairs, and the other 88 have straight tied pairs
beside another conic germ. **Not measured:** ties in a round that is
not all mutual, and ties at `K ≤ 2`; the 525 processes are the `ci`
sets at the default band, whose `K` exceeds 2. Freeing it needs a proof that, where a
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

**`K > 2` in the gate.** The K-gate row
(`at_k_at_most_2_first_pair_reads_the_order_among_mutual_pairs`) pins
that the gate is read: on its seed-3118 fixture every rotation of the
scan builds the same matching. The gate's `K > 2` premise bounded the
per-germ incumbent scan that `own_nearest` replaced. Whether an
all-mutual round needs it at all is the follow-up's to settle, on the
prover's theorem that such a round's matching is free of the commit
order at any `K`.

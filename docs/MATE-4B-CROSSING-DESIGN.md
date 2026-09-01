# At-rest crossing backability (issue 973 part (b)) — design conversation

**STATUS: DRAFT — for Evan.** The MATE-4b design pass the Q2(b)
direction ordered (in-chat, 2026-08-31): the side/region-aware
crossing machinery is needed eventually; this document proposes its
SHAPE and STAGING. Two constraints from that conversation bind
everything below: interpenetration may eventually be LEGAL when
explicitly declared (A5/C6's interference-fit gate-skips are the
ratified anchor), so no vocabulary here may foreclose that class;
and the #943 constraint holds stream-wide (the census consults the
mate's own declaration — contact machinery is never re-implemented
as mates).

## The question

Issue 973(b): `EdgeEdgeCross` and `EdgeFacePierce` are pushed as
"categorically undeclarable" on the reduction lane's "3′ allows
touching, never crossing" premise — a statement about
interpenetration. At rest, two coplanar boundary edges CROSSING at
a declared seat is not interpenetration: it is what an overhanging
seat looks like from the census's side, and it is ordinary
authoring (drag a part until it overhangs). MATE-4a (PR #1432)
restated the licences on an at-rest premise and narrowed (b)'s live
surface: with the ef rung landed, the overhang class's remaining
hard findings are exactly the crossings.

The geometric fork the current rungs cannot see: an IN-CONTACT-PLANE
crossing at a declared seat is legal (the bodies' material lies on
opposite sides of the shared carrier); a TRANSVERSE crossing means
interpenetration. Distinguishing them needs side and region
awareness — precisely the strength CENSUS-REST Q3 deliberately
declined for the existing rungs.

## Options

**A (recommended) — a side-aware, region-confined backing rung, as
a SECOND NAMED STRENGTH TIER, planar-first.** A declared face pair
backs an `EdgeEdgeCross` of two coplanar boundary edges iff:
1. the crossing point lies within the pair's VERIFIED overlap
   region (door 2's machinery — the region test), and
2. the two bodies' material lies on opposite sides of the shared
   carrier at the crossing — the side test, three-valued
   (opposite-sides / same-side / undecided), decided by the same
   outward-normal sense algebra the tier-3 wedge pass now carries
   (`classify_material_pairing`'s family), escalating on undecided.

The existing rungs' region-unconfined strength is UNTOUCHED: this
is not a strengthening of the ratified rungs but a new rung at a
stronger, named tier ("region-confined"), and C3/C4 gain one
sentence for the tier. `EdgeFacePierce` stays categorical in this
stage — a transverse dive is interpenetration until C6 speaks.

Honest cost: the rung inherits door 2's witness layer, whose fixed
candidate schedule is measured incomplete (issue 1435 — legal seats
bifurcate per-fixture today). **Issue 1435 is therefore a
precondition, not a caveat**: building the confined rung on the
current schedule would reproduce the bifurcation one tier up.

**B — crossings stay categorical**; the justifications stay on the
at-rest premise MATE-4a already gave them, and the overhang class
permanently parks at `Uncertified`. Zero machinery, fully honest —
and an ordinary declared seat can never certify. This is the
do-nothing floor the eventual-machinery direction already rejected,
recorded here so its cost is explicit: B is where we stand while A
is unbuilt.

**C — a declared crossing-contact vocabulary** (a new contact class
naming the overhang). Rejected under the #943 constraint: the mate
already said the right thing (one face pair, declared once); a
second declaration for the events the seat induces is contact
machinery re-implemented as mates. Recorded so it is not
re-proposed; if C6's era wants richer declaration vocabulary, that
is C6's conversation.

## Interpenetration forward-compatibility (the recorded constraint)

The side test's verdict is deliberately three-valued rather than
boolean: a future declared-interpenetration class (C6's
`OverlapUncorrected` recorded gate-skips, the interference-fit
representation A5 already names) consumes the SAME-SIDE verdict as
its admission evidence instead of being fenced out by a bool. No
other part of this design speaks for C6; the hook is the verdict
shape, nothing more.

## Staging (the proposal)

- **Stage 0 (precondition)**: issue 1435 — complete or adapt
  `interior_witness`'s candidate schedule, D9-deterministically.
  `chart_region.rs` ground; small, self-contained, and it repairs
  the measured per-fixture bifurcation independently of anything
  here.
- **Stage 1**: the planar side-aware `EdgeEdgeCross` rung (option
  A) + the C3/C4 tier sentence + the (b) fence rows re-blessed onto
  the new outcomes. One M/L unit once stage 0 lands.
- **Stage 2**: `EdgeFacePierce` under the C6 interference-fit era —
  deferred to that era by name (A5's bullet), not scheduled here.

## Questions for Evan

1. **The confined tier**: is a second, named rung strength
   (region-confined + side-aware) acceptable beside the ratified
   region-unconfined tier — with C3/C4 carrying one sentence for
   it — or do you prefer the crossings wait for a single unified
   strength story?
2. **Staging**: build stages 0–1 inside S-MATE (recommendation:
   yes, if stage 0 prices as the small unit it looks like —
   otherwise bank stage 1 and land stage 0 alone, since 1435's
   repair pays for itself), or bank the whole design until C6's
   era?
3. **The hook**: does the three-valued side verdict suffice as the
   declared-interpenetration hook, or should the C6 vocabulary be
   drafted now so the two designs are ratified together?

Recommendation: option A, staging as given, stage 0 funded
immediately either way.

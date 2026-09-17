---
id: pick-tie-break-width-key-depends-on-scene-magnitude
kind: issue
title: the width tie-break's key depends on where the scene sits, so an exact tie between identical faces is decided by coordinate magnitude
status: open
opened: 2026-09-16
---


Filed by EDIT-PICK3's fix pass, on an observation of review lane
`pick3-r2`. A class the ruling
`what-t-the-pick-door-answers-and-with-what-width` did not discuss;
not built, because deciding it is a design question and not a defect
the unit could settle.

## The observation

`pick_face`'s certified tie is decided by `TSpan::width`, and the width
is not a function of the two candidates' SHAPES alone. `t_span`'s
second term is

```
from_rounding = PROJECTION_ERROR_UNITS * EPSILON * m / (d·d)
```

with `m = Σ_i (|a_i| + |e1_i·u| + |e2_i·v| + |o_i|)·|d_i|` — the
MAGNITUDES of the triangle's anchor and the ray's origin, not just the
edges. So two congruent triangles, met by congruent rays at congruent
angles, carry different widths when one sits at the origin and the
other a kilometre away, and the door prefers the one nearer the
origin.

That is arithmetically correct — the far triangle's parameter really is
less well certified, because its coordinates carry more absolute
rounding — and it is also a rule the user did not ask for. Translate
the whole document and a tie can change its answer.

## Why it is a row and not a bug

The first term, `from_barycentrics`, is translation-invariant (it reads
`err_u`, `err_v`, `|e1|`, `|e2|`, `|d|`). The second is not, and it is
the smaller of the two on every candidate the corpus produced — the
class is reachable only where the first term TIES exactly, which is
where the two candidates are bit-identical in shape and differ only in
placement. `pick3_early_out::equal_widths_fall_to_the_earlier_target`
is that case built deliberately, with both triangles at the same
magnitudes so position decides; move one and width decides instead.

## What it would take to answer

Three shapes, none of them free:

- **Say it is right and state it.** The wider claim IS less certain,
  including for a reason the user thinks of as "where the model sits".
  One sentence at `TSpan::width` and the class is documented rather
  than latent.
- **Compare a translation-invariant width.** Drop `from_rounding` from
  the tie-break key while keeping it in the interval. Then the ORDER
  and the ENCLOSURE stop being the same number, which is the thing the
  ruling's (a)–(c) deliberately made one.
- **Anchor the arithmetic.** Evaluate the crossing relative to the
  triangle's own anchor so `m` loses its `|a_i|` and `|o_i|` terms.
  That is a change to `crossing` and `t_span` both, and it would move
  every measured width in the tree.

## Where the evidence is

`crates/editor-core/src/resolve/pick.rs`, `t_span` and
`PROJECTION_ERROR_UNITS` (the `m` sum is written out in both). The
corpus measurement that says the class is not currently reached is
`crates/viewer/tests/pick3_acceptance.rs` — `aim_lost == 0` over
441 126 rays with the widths as they stand.

## Question for Ev (2026-09-17, EDIT orchestrator) — on the fourth `[ev]` PR

Whether the certified tie's second key stays the full interval width
(arithmetically the better-certified claim, but a rule under which
translating the document can change a tie's answer), or becomes the
shape-only term (`from_barycentrics`, translation-invariant) while the
enclosure and the `precedes` order keep the full width. The
recommendation is on the PR; this row is parked on Ev's answer.

## Recommendation and the exchange on the PR (2026-09-17)

The PR recommended the shape-only key. Ev asked whether the door
should "just be refusing in a case like this"; the orchestrator
proposed refusing at an EXACT shape-width tie only (the position key
gone, the width key kept as the informativeness rule for ties the
arithmetic can grade) and asked whether Ev meant that or the wider
scope — refuse at any certified tie and drop the width key too.

## RULED (2026-09-17, Ev on `[ev]` PR #2795): the certified tie is refused and the width key goes

Ev: *"either scope is good, but dropping the width key seems like it
might simplify the code and also not increase refusals where it's
actually clear what the user meant."* The wide scope is the ruling.
Part (c) of `what-t-the-pick-door-answers-and-with-what-width` is
superseded; its (a), (b) and (d) stand. The order is `precedes`
alone, over the set, and the certified tie is not broken:

1. A candidate some other candidate precedes (`t_hi < t_lo`) is out.
   The survivors pairwise overlap — one certified tie, as today.
2. Survivors that name ONE face — the same `(node, body, face)` met
   on several of its own triangles, which is every ray across a
   triangle diagonal or an in-face shared edge — are one answer, not
   a tie. The door answers that face with the HULL of the members'
   intervals (each encloses the crossing of its own triangle, so the
   hull encloses every crossing the tie holds) and, for `t` and
   `point`, the member with the smallest rounded `t` — a function of
   the set, and a point of the face. This is the "clear what the user
   meant" case; refusing it would refuse most picks.
3. Survivors naming MORE THAN ONE face are refused, typed:
   `HitTestError::Ambiguous { hits }`, one `PickHit` per tied face
   (name, node, body, its own hull interval and point), listed in the
   caller's target order and then face-arena order — an order for a
   LIST, which decides nothing. Nothing about width, the scene's
   placement or the target order decides a pick any more; this row's
   class is unreachable by construction.

The width stays what the interval IS — the enclosure, the `precedes`
order, the early-out margin, the acceptance's `wide_winners`
measurement — and stops being a key. `TSpan::best_of` retires for one
set-valued home (the survivors of `precedes`) that the door and every
reference loop and probe call. The early-out is unchanged and carries
a second obligation: `Pruned == Every` is now also what makes the
refusal's list complete.

What changes visibly, told to Ev on the PR before the ruling: a ray
down a cube's shared edge names neither face and refuses with both; a
face the ray meets edge-on (wide) in front of a transversal face
(narrow) no longer loses to the narrow one — the two are refused
together. The corpus's tie-break aim (19 296 rays aimed at shared
edges and vertices by construction) lands in the refusal wherever the
tied triangles belong to different faces; the acceptance re-baselines
to count those and to keep `Pruned == Every` over the refusal's list.

**The viewer** (VIEW's, crossed by announcement). The ray path is the
authoritative one at its own seam (`crates/viewer/src/idpass.rs`, the
recorded role inversion), so an ambiguous ray answer is a refused
click — nothing selected, the status line naming the tied faces — and
`idpass::disagreement` reports no disagreement when the id pass's name
is one of the tied set (the raster chose among faces the kernel says
are tied). The cross-group merge in `PickIndex::pick_for` takes the
same rule — `precedes` over the groups' answers, else the refusal —
which is what `work/view/pickindex-merges-parts-on-a-rounded-t-it-never-converts`
§2 asks for; the unit closes that row's §2 by announcement and says
what of its §1 and §3 stands. **The Python door** (LIB's, mechanical):
the new `HitTestError` variant gets its tag and the exception carries
the tied hits.

**Middle tier** (one implementer, one style review with a correctness
arm), not the E-class this row was first cut as: the door's public
answer changes shape across three crates. Spec'd at the next claim
(branch `edit/pick-tie-refuses`). Rows that re-baseline to pin the
refusal: `pick.rs`'s `a_ray_down_a_shared_edge_…_position_decides`,
`pick3_early_out`'s `equal_widths_fall_to_the_earlier_target` and
`the_certified_tie_is_decided_by_the_candidates_and_not_the_targets_order`,
the width rows of the two PICK3 review-probe suites, `pick3_acceptance`'s
`winner`, and whatever `gui1_pick` / `review_gui1_r1` rows aim at an
edge. `docs/DESIGN.md`'s picking bullet ("a total documented
tie-break") is re-worded by the unit, as the description of what it
built.

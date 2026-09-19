---
id: pick-tie-break-width-key-depends-on-scene-magnitude
kind: issue
title: the width tie-break's key depends on where the scene sits, so an exact tie between identical faces is decided by coordinate magnitude
status: closed
opened: 2026-09-16
closed: 2026-09-19
pr: 2816
branch: edit/pick-tie-refuses
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

## Spec'd (2026-09-17, EDIT orchestrator) — middle tier, branch `edit/pick-tie-refuses`

The `## RULED` section is the spec's premises; this section is the
rows, the mutants and the territory.

**Rows** (red first, then the door):

- `a_ray_down_a_shared_edge_…` (`pick.rs`) becomes "…refuses with both
  faces": the two hits carried by `HitTestError::Ambiguous`, each true
  (same `t`, same point), in target order; and the refusal is the same
  set whichever order the targets are offered in.
- `pick3_early_out::equal_widths_fall_to_the_earlier_target` becomes
  "equal widths refuse": swapping the targets swaps the LIST's order
  and nothing else. `…decided_by_the_candidates_and_not_the_targets_order`
  re-baselines to the refusal's set. `the_early_out_keeps_a_candidate_whose_interval_reaches_below_its_box`
  keeps its claim: the kept candidate now appears in the refusal's
  list (or is the answer, when it precedes) — `Pruned == Every` over
  the list.
- **Same face, one answer**: a ray across a triangle diagonal of one
  planar face answers that face (no refusal), with the hull interval
  and the smaller rounded `t`; a ray down a shared in-face edge the
  same. A mutant that refuses on triangle count instead of face
  count reds it.
- **Wide over narrow refuses**: the `tube_arc` class
  (`pick-a-wide-but-informative-barycentric-…`, the edge-on face in
  front of the transversal one) now refuses with both — re-baseline
  the PICK3 review-probe rows that pinned the width order
  (`review_pick3_r1_probes`, `review_pick3_r2_probes`) and say which
  claim each now pins.
- `pick3_acceptance`: `winner` becomes the door's set rule; the
  tie-break aim counts refusals (edges/vertices aimed by construction
  across distinct faces) and asserts `Pruned == Every` over hit AND
  refusal; `wide_winners` stays a measurement column; the wide aim's
  `aim_lost` re-baselines with the count of aims that became refusals
  stated in the PR body.
- The viewer: an ambiguous ray answer selects nothing and the status
  line names the tied faces (`pane/viewport.rs`'s click path);
  `idpass::disagreement` answers `None` when the id pass's name is one
  of the tied set, and a `Disagreement` whose `from_ray` is the tied
  set otherwise (render "rendered X, ray path tied between X and Y");
  `PickIndex::pick_for`'s cross-group merge applies `precedes` over
  the groups' intervals, else the refusal — closing §2 of
  `work/view/pickindex-merges-parts-on-a-rounded-t-it-never-converts`
  by announcement (say what of its §1 and §3 stands).
- Python: `HitTestError` gains the `ambiguous` tag; the exception
  carries the tied hits; one test in the binding's suite.

**Mutants**: keep the width key (the wide-over-narrow row reds it; the
equal-widths row cannot, both widths being equal there); keep the
position key (the shared-edge row reds); refuse on
triangle count (the same-face row reds); drop the early-out margin
(the below-box row reds); list the hits in flat-triangle order only
(the target-order row reds).

**Territory**: `crates/editor-core/src/resolve/pick.rs` (EDIT),
`crates/editor-core/tests/{pick3_early_out, review_pick3_r1_probes,
review_pick3_r2_probes, gui1_pick, review_gui1_r1, gui1_pick_r2}.rs`
(TCOST/TINT), `crates/viewer/src/{pickindex.rs, idpass.rs,
pane/viewport.rs}` and `crates/viewer/tests/{pick3_acceptance,
index_memo}.rs` (VIEW's, announced on the ruling), `crates/pncad-py/src/{tags.rs,
py/pick.rs, tests.rs}` and the Python pick test (LIB's, mechanical),
`docs/DESIGN.md`'s picking bullet (the description of what is built).
One style review with a correctness arm (opus), then the fix pass.


## Built (2026-09-17, `edit/pick-tie-refuses`)

The ruling is built across the three crates.

**The kernel** (`crates/editor-core/src/resolve/pick.rs`,
`resolve/hit.rs`). `TSpan::best_of` is gone; `TSpan::survivors`
answers the set of candidates no other precedes, in slice order, and
is the one spelling of the rule every reference loop and probe calls.
`pick_face` groups the survivors by `(node, body, face)`: one group is
the answer (the hull of the members' intervals, at the member with the
smallest rounded `t`), several are `HitTestError::Ambiguous { hits }`
— one `PickHit` per tied face, listed in the caller's target order and
then face-arena order. `TSpan::width` stays as the enclosure and as
the early-out margin's quantity, and is a key nowhere. The early-out
is unchanged and now also makes the refusal's list complete.
`HitTestError` loses `Copy`/`Eq` (its new arm carries a `Vec`) and
`PickHit` gains a written-out `PartialEq`, floats included, so a
refusal is a value a row can pin.

**The viewer** (VIEW's, announced on the ruling). `PickIndex::pick_for`
merges the groups' answers with `TSpan::survivors` — the kernel's own
order, not `<` on a rounded `t` — and refuses with the tied groups,
which closes §2 of
`work/vgeom/pickindex-merges-parts-on-a-rounded-t-it-never-converts`.
Its §1 (a moved instance's `t_lo`/`t_hi` carried unconverted) and §3
(`OCCLUSION_SLACK_REL`) stand: neither is touched here, and §3's site
now reads `PickIndex::front_of`, which answers the occlusion question
across a tie rather than refusing it. `face_under_cursor` becomes
`faces_under_cursor` (a list), `idpass::Disagreement::from_ray` becomes
the SET the ray path names, and the id pass counts as AGREEING when it
named one of the tied faces. An ambiguous click selects nothing and the
status line names the tied faces, through the refusal's own `Display`
on the path `frame::pick_refusal` already took.

**The Python door** (LIB's, mechanical): tag `ambiguous`, the
exception's `hits` attribute carrying the tied `PickHit`s, the class
docstring, the stub, the binding census and one test in
`test_picking.py`.

`docs/DESIGN.md`'s picking bullet is re-worded as the description of
what was built.

**Rows re-baselined**: `pick.rs`'s shared-edge row (now the arithmetic
under the door's refusal); `pick3_early_out`'s four rows plus two new
ones (the shared edge refusing with both faces; several triangles of
one face answering that face); `gui1_pick`'s edge ray; `gui1_pick_r2`'s
corner ray; `review_gui1_r1`'s three rows; `review_pick_r2_probes`'s
reference loop; `m4_pr4_hit`'s `HitTestError` census and Display case;
`index_memo`'s reference and its two `tube_arc`/ring probes;
`pick3_acceptance`'s reference, both sweeps and its assertions.

**The refusal's prose.** The message numbers each tied face — "(1)
face name minted by node 2, (2) face name minted by node 2" — and does
NOT carry the role path. Two faces of one node render identically
through `StableName`'s `Display`, which omits the path on purpose; the
path is a `Debug` derivation, and a `Debug` struct dump in a refusal's
message is what the Display contract forbids and what the binding's
own `reads_as_prose` check refuses at the raise. The ordinal is what
ties each phrase to its entry in `hits`, where the path IS carried.
`idpass::Disagreement` renders the path because it has no typed
payload at all; this arm does.

**Not built, and why**: the spec named "the width rows of the two
PICK3 review-probe suites (`review_pick3_r1_probes`,
`review_pick3_r2_probes`)". Those suites carry no width-ORDER row —
their only use of `TSpan::width` is the enclosure's half-width, which
the ruling keeps — so there was nothing to re-baseline there. The
`tube_arc` width rows are in `crates/viewer/tests/index_memo.rs`, and
those are re-baselined.

## Fix pass (2026-09-17, same branch) — the review's findings built

The style review's verdict was NOT-MERGEABLE-AS-IS on one MAJOR, and
that finding is the largest change here.

**MAJOR — the edge pick survives a face tie.** `hovered_for`,
`edge_at_for` and `faces_under_cursor` open on `PickIndex::seed`,
which was `pick_for` and propagated its refusal with `?` — so a cursor
on a shared edge, the pixel a user aims an EDGE with, refused the
edge pick: 18 of the 66 segment-midpoint cursors on the shipped plate.
The seed is now a DEPTH: `PickIndex::front_of` answers the nearest of
the faces the door names plus the tied set beside it, and the refusal
is raised only by `hovered_for`, only for a cursor whose own answer
would have been a face, and only after the edge-priority rule has had
it. The occlusion probe reads the same door, which is why
`work/vgeom/pickindex-merges-parts-on-a-rounded-t-it-never-converts`
§3 now records `front_of`'s two readers and why a depth across a tie
is not a pick. Row: `edge_pick::a_cursor_the_face_pick_ties_on_still_picks_the_edge`
(the reviewer's probe, inverted, with the 18/66 sweep as its fixture
and its premise asserted).

**The group rule has one home.** `resolve::pick::answer_of` over
`(span, face key)` pairs answers `Answer::Miss | One | Ambiguous` —
the survivors of `TSpan::precedes` grouped by face, each face at the
hull of its members' intervals and the smallest rounded `t` among
them. `pick_face`, `PickIndex::pick_for`, `pick3_acceptance::winners`
and `index_memo::FlatReference::pick` all call it; the references keep
their own candidate enumeration, which is what makes them references.

**A refusing group no longer shadows a nearer face in another group.**
`pick_for` collects every group's whole answer — a group that refuses
contributes its tied faces — and runs `answer_of` over the union. Two
rows on a new two-root fixture (`select_pick.rs`):
`a_moved_face_in_front_of_a_tied_batch_is_the_answer` and
`two_coincident_faces_across_groups_refuse_with_both`.

**The smallest-`t` half, pinned.** The reviewer's
`one_faces_members_answer_the_smallest_rounded_t` is adopted, and
`several_triangles_of_one_face_answer_that_face`'s fixture is no
longer degenerate: its two members are a few ulps apart along the ray
(the seam a tessellation actually leaves), the FAR one offered first,
and both premises — the overlap and the different parameters — are
asserted.

**The acceptance's columns.** `aim_lost` reads by IDENTITY: the aim is
kept when the door names the aimed vertex's OWN face at the aimed
parameter, not merely a hit at that depth. `beyond_or_miss` and
`moved` read the NEAREST of the door's list rather than its first
entry. On this head, release, full corpus: `aim_lost 0`,
`aim_gained 1 440`, `moved 1`, `moved_farther 0`; of the 20 475
refusals a hit at the aimed parameter is in, 20 469 name the aimed
face itself. `tie.refused > 0`'s prose says what it is — a liveness
guard on the corpus, not a measurement of the refusal.

**The 4- and 6-face refusals attributed.** 2 351 of the wide aim's
34 934 refusals name four (2 327) or six (24) faces. The four-face
instance is `kitchen_sink` at the origin: four faces of four
different (node, body) pairs — an instance's lateral, a split
fragment of a merged face, a revolve cap and a revolve band — all
containing one point, which is coincident faces of separate
instances. The six-face instance is `cut_cylinder`: three faces
incident at one point (two lateral split fragments and the section
face) in EACH of the two bodies the cut produced, which are
coincident there. Both are the ruling's intended refusal: every hit
listed is true and nothing orders them.

**Q7 — the status line.** The kernel's refusal stays numbered by name
(the prose contract). `frame::pick_refusal` re-renders the tie itself,
each face the way `idpass::Disagreement` renders a name — kind,
minting node, role path — so two faces of one node are two phrases.
Row: `frame_policy::the_status_line_renders_two_tied_faces_as_two_different_phrases`.

**Q3/Q2 and the record.** The `Ambiguous` `Display` comment is one
sentence. The PR body's `prose_census` paragraph is gone (that file is
not in the diff). Three stale citations of the deleted tie-break are
fixed (`pick.rs`'s memo-equivalence and `ray_triangle` boundary
paragraphs, `pickindex.rs`'s edge-determinism paragraph) and the
sweep `tie-break|tie break|narrower` over `crates/*/src`, the pick
test suites and `docs/` is re-run: the surviving hits are the edge
door's own pixel tie-break, `bvh`'s split rule and uses of "narrower"
about intervals and windows.

The review branch is merged authorship-preserving
(`git merge --no-ff origin/review/pickrefuse-rv`), its two probes
re-headed to the invariants they pin.

## Closed (2026-09-19, EDIT orchestrator)

Built and merged as PR #2816 (middle tier: one opus style review with
a correctness arm, then the union fix pass). Ev's ruling on `[ev]`
#2795 is the door: the order is `precedes` alone (`TSpan::survivors`,
the one spelling); survivors naming one face are that face with the
hull of their intervals at the member with the smallest rounded `t`;
survivors naming more than one face are `HitTestError::Ambiguous {
hits }`, one true hit per tied face, listed in target then face-arena
order and deciding nothing; the width and position keys are gone and
`TSpan::width` is a measurement only. The review found one MAJOR the
first build had minted: the refusal reached the viewer's pick SEED, so
a cursor on a shared edge stopped picking the edge — the clear-intent
case Ev asked not to refuse (18 of 66 segment-midpoint cursors on the
shipped plate). The fix pass made the seed a depth (`front_of`: the
nearest of the faces the door names, the tied set beside it — two
readers, the seed and the occlusion probe, recorded on the vgeom
row), so only a FACE answer refuses; gave the group rule one home
(`resolve::pick::answer_of`, called by the door and both reference
loops); merged every display group's answer before refusing (a
two-root fixture: a moved face in front of a tied batch is the
answer, coincident faces across groups refuse with both); measured
`aim_lost` by face identity (0 lost; 20 469 of 20 475 refusals at the
aimed parameter name the aimed face); attributed the four- and
six-face refusals (coincident faces of separate instances; three
faces incident at a point in each of two coincident bodies — both the
intended refusal); and made the status line render two tied faces as
two phrases. Acceptance on the final head: tie aim 19 296 rays,
`pruned_differs 0`, `refused 4 976`; wide aim 441 126 rays, `refused
34 934`, `aim_lost 0`, `moved 1`. `docs/DESIGN.md`'s picking bullet
describes what was built. Territory crossed by announcement: VIEW
(three sources, six suites), LIB (the `ambiguous` tag, `hits`, stub,
censuses, two Python rows), TCOST/TINT. `work/vgeom/pickindex-merges-parts-on-a-rounded-t-it-never-converts`
§2 closed by announcement; §1 and §3 stand.

# Designer B, round 2: shared-vertex pairing

## What moved since round 1

Two round-1 claims were wrong, and both counterexamples come from them.

- Round 1 said either pairing gives a valid body, and that the start rule (pair along A's kept side) existed only because the finish keyed its correspondence by vertex. That is wrong. The start rule's real content is that **every null edge joins two germs on one cone of the result**. Two things depend on that:
  - The finish's old vertex-set check, which the branch rightly retired.
  - The join's one-site loops. Family 2 is this case.
- Round 1 said the shared solid's In arcs never hold another pair's cut, because the other operand's pieces are disjoint at the point. That holds only when the partner vertices' links bound disjoint disks. Family 1 is the counterexample.
- I also retract "the op leaves the insertion". In family 2 the reduction's own records already depend on the op: `recl` reads the coincident crescent as In for ∪. Taking the op out of the pairing therefore removes no dependence on the op. It only removes the one fact the pairing needs from the op: which arcs the result keeps.

## For Ev

**Recommendation (likely).** Pair each crossing pair along the **kept arcs of a solid whose vertex no other crossing pair shares**: the partner's arcs at a shared vertex, and A's where neither vertex is shared. Here a kept arc is a stretch of that solid's link between two of the pair's crossings, on the side the op keeps. At a vertex shared by several pairs, place their null edges as **one non-crossing family across all of them**: a run may sit inside another pair's run and be minted at that run's copy. A run then never has to be "clear of" the others.

The op contributes exactly one thing: which side is kept. It is the same fact the result is selected by, and main already reads it. This keeps the branch's battery gain, keeps the cleave witness building, and builds both families as main does. One part of it is new machinery, nesting runs across pairs. It is unmeasured (see Confidence).

**Terms.**
- *Pair*: a vertex of A and a vertex of B at the point whose boundaries cross.
- *Germs*: the crossings, seen as points of the small sphere round the point.
- *Link*: a vertex's boundary curve on that sphere.
- *Chord*: the null edge between two germs, drawn across a link.
- *Run*: the stretch of the shared vertex's link that a null edge moves to its new copy vertex.
- *Cone*: one boundary curve of the result's material on the sphere. Your PR 4057 ruling makes each cone one vertex.

**Why it is right.**
1. **Every null edge lies on one cone (sure).** A kept arc of either solid is a stretch of one cone of the result. A null edge joining its two ends therefore never merges two cones at one vertex. Two consequences:
   - The join closes a closed section loop through one site on its own null edge, as family 2 needs.
   - `split_cones` becomes a backstop at shared vertices, not something they depend on.
2. **It never crosses at a shared vertex, whatever the partners' shapes (sure).**
   - The partner vertices' links are disjoint curves on the sphere, disjoint pieces and nested cones alike.
   - Within one op, every partner keeps the same side of the shared link.
   - So their kept arcs are disjoint curves inside one disk of the shared link: a non-crossing chord diagram.
   - The measured data fit this. All 16 crossing lines are `ab`, where main pairs along the *shared* solid's kept arcs. None is `ba`, where main already pairs along the partner's.
3. **Why each family failed, read in the model.**
   - *Family 1 (likely).* `y` is a hollow pyramid, so its material near the apex lies between an outer cone and an inner one: an annulus on the sphere. The In side of y's inner vertex is everything outside the inner cone, and that contains the outer link. So the In-arc rule's run holds the outer pair's cuts. The chords themselves need not cross: the fault is the rule choosing the side, not the pose. Under the recommendation, both of y's links meet x's link in disjoint arcs inside x (∖ keeps B's In side), so the chords are non-crossing. Main builds this pose by turning a run, which the nesting placement covers.
   - *Family 2 (sure on the mechanism, likely that it is unshared).* The boss rim and the hole rim are each a closed section loop through the tangent point. In ∪, `recl` reads the crescent In, which gives four germs: a± on the boss rim, h± on the hole rim. The result pinches into two cones: the boss wall with the plate top, and the boss floor over the hole with the hole wall.
     - A's In arcs are the two crescent stretches, so the In-arc rule pairs (a−, h−) and (h+, a+). That joins germs of the two cones and makes one figure-eight polygon through the point twice. The join builds a one-site loop only as one null edge's two ends (`one_site_segments`), so it leaves 4 loose ends.
     - Main, and the recommendation, pair (a+, a−) and (h−, h+): each rim closes on itself.
     - Whichever solid's kept arcs are used, the pairs are the same here, so the answer does not depend on whether that vertex is shared.
4. **The fault behind the original refusal.** Main's pairing at an A-shared vertex runs along the *shared* solid's kept arcs. Each such arc is cut by other pieces, so it is not one pair's stretch, and it can cross another pair's chord (the 16 crossing lines). Separately, main's reconcile requires runs to be disjoint, not just non-crossing. That accounts for all 966 laminar lines, and for every `ba` refusal, where the pairing was already the partner's.

**The answers, as final states.**
- **R (recommended): pair along an unshared solid's kept arcs, and place each vertex's chords as one non-crossing family.**
  - Pairing: unchanged from main at unshared pairs and where B's vertex is shared. Where A's vertex is shared it pairs along B's kept arcs; per pair this equals either main's pairing or the branch's.
  - Placement: across pairs, one non-crossing family per vertex. This generalises `hang_at_shared`, `holds_whole` and the nested holders `b_runs` used to compute into one reading over the vertex's one cut order.
  - Leaves possible: a nesting the placement reads wrongly. That fails loud (`ClassificationInvariant`) and is never a wrong body.
  - Reversible: yes. Which side of a chord becomes the copy does not change the regions, so any root of the family yields the same vertices.
  - `SharedVertexCrossings` keeps only its tie arm, the one-arc reading.
- **H (the branch patched): main's rule at unshared pairs, In arcs at shared vertices, and a turn for a run that holds another pair's cut.**
  - Builds what the branch builds plus, likely, both families. That is not measured for family 1.
  - Leaves two rules.
  - Null edges at shared vertices still merge cones, so the build relies on `split_cones`.
  - Has no answer for nested links with four or more crossings, nor for a link with other links on both of its sides. Those would be crossings, or an invariant firing, on valid input.
  - Cheaper to land. I would accept it only as a step towards R.
- **Op-free In-arc rule (the branch as it stands).** Wrong by meaning: it can merge cones (family 2) and cross (family 1). Not defensible.
- **Typed refusal.** Not acceptable for any class: every refusing pose has a body.

**Ratified text.** None changes. No README design page or DESIGN.md clause states the pairing (round-1 search). The branch's module docs ("the op is not read", "the pieces of one operand at a point are disjoint") are unmerged agent text. Under R both go, and the docs say instead that the kept side picks which germs pair, and that partner links are disjoint curves, not disjoint disks.

**What R does to today's builds.**
- Unshared pairs and B-shared vertices pair exactly as on main.
- A-shared vertices pair as main or as the branch, pair by pair.
- The finish keeps the branch's per-corner correspondence. It is still needed: at an A-shared vertex A's runs can be A's discarded arcs, so one A vertex holds two corners of one pair.
- The `ba` lines and the "same-type" `ab` lines build only once the nesting placement exists. Today main refuses them, and the branch builds them with a pairing that merges cones.

**Confidence.**
- Non-crossing at every shared vertex: sure.
- One cone per null edge: sure.
- Family 2's mechanism: sure.
- Family 1's mechanism: likely.
- R keeps 5,190 SOUND and builds the cleave witness: unsure. I could not measure it: the build slot never came free (see the orchestrator section).
- R over H: likely.
- Both families build under R as on main: likely.

## For the orchestrator

- **Incident: I ran commands in `/home/user/cad` by mistake.**
  - At 12:52, a `cd` into my not-yet-created directory failed. Commands after a `;` then ran in the main checkout. They fetched `gauge/in-arc-pairing` and `main` into its `.git` and checked out a new branch `inarc` there.
  - About 25 seconds later another process, not me, moved that checkout back to `seal/orchestrator` and deleted `inarc`.
  - I verified: on `seal/orchestrator` at `c23d139129b`, working tree clean, no `inarc` or `mainline` branch.
  - Left behind: `FETCH_HEAD` and the fetched objects in `/home/user/cad/.git`.
  - Every later command used absolute paths.
- **Not measured.** I wrote a probe of R's pairing on main, with main's reconcile and no nesting placement:
  - It is in `af-svc/B-r2-probe-R.patch`, with the lane's refusal probe. It type-checks: `cargo check -p topo`, express slot.
  - It is env-gated by `SVC_R` and `GAUGE_SVC_PROBE`.
  - The release `sweep` build waited on the shared slot from 13:00 to 14:22 (fork-svc-a and fork-svc-c held it). I stopped.
  - What it would show on `four_pairs_battery`:
    - **(a)** No refusal of the crossing kind. A crossing would refute the non-crossing claim. Read it with `gauge-svc-classify.py`.
    - **(b)** Which lines R builds without cross-pair nesting, and that they are SOUND.
    - **(c)** That every remaining refusal is the laminar kind (non-crossing but not disjoint), which is what the placement must build.
  - Family 1 also needs an editor-core run.
- **Family 2 is likely unshared.** I read one boss vertex and one plate vertex at the tangent point, but I did not trace it. It does not matter for R: A's kept arcs and B's kept arcs give the same pairs there, each rim with itself.
- **Family 1's crossing count was not read.** If each pair crosses twice, R's pairs equal main's, and placement alone decides. If more, R pairs along y's arcs inside x, which may differ from main.
- **For the implementer, if R is chosen:**
  - At an A-shared vertex with four crossings, B-kept pairs are A-consecutive (F12). At six or more they can nest in A's walk. The branch's per-solid `run_holders` already reads holders in both solids, so carry it over.
  - The cross-pair placement should be one `arc_holders` over every chord at the vertex in `cut_order`'s single order, replacing turning.
- **Round-1 error, stated plainly:** I claimed either pairing is legal given `split_cones`. That rested on the battery's 458 SOUND lines, which happen to contain no one-site loops.

# Fork `shared-vertex-pairing` — designer B, round 1

Harness token count (first report): 255,096.
## For Ev

**Recommendation (likely).** At a vertex that other crossing pairs share, pair each pair along the *shared* solid's In arcs: the stretches of the shared corner that lie inside that pair's partner piece. Those stretches contain no cut from any other pair, so every run there is clear by construction. Then change the finish so it records which partner vertex goes with each null-edge corner, rather than which partners go with each vertex. Both classes, laminar and crossing, then build and keep no typed refusal. The refusal comes from the plan, not from the pose: the pairing today is chosen to suit a representation in the finish that the zip has already outgrown.

Terms used below:
- **Pair**: one vertex of A and one of B at the point, whose boundaries cross.
- **Germs**: those crossings.
- **Pairing**: which germs a null edge joins.
- **Run**: the stretch of a vertex's orbit that a null edge moves to the new copy vertex.
- **In arc / Out arc**: a stretch of one solid's orbit inside / outside the partner.
- **Laminar**: the null edges at one vertex, drawn as chords of its orbit, cross nowhere.
- **Kept side**: the side the op keeps.

**Premise check.**
1. **The question asks for the wrong thing (sure).** The row asks which pairing to fall back to when the start rule has no clear way round. But the start rule picks A's kept side only for the finish's sake. `pairing_start_turns`' doc says so: "A's is the one the finish needs". The reason is that the finish's seam correspondence maps each kept vertex to a set of partner vertices, and refuses when a vertex has two partners from one pair. That doc was written 2026-10-05, the day before your PR 4057 ruling (a pinch is one vertex per cone) landed as `zip::split_cones`. `split_cones` now works out the result's vertices per cone from the section corners, and rebuilds the correspondence per corner. So the insertion is shaped for a constraint downstream no longer needs.
2. **Both pairings are valid inputs to the build (sure).** The insertion needs only one thing: at every vertex, all the null edges form one laminar family. The partner pieces at a shared vertex are disjoint, so the shared solid's In arcs hold no other pair's germ. Pairing along them is therefore laminar for *both* classes. The 16 "crossing" lines are not a different kind of problem: with this pairing they too fail only at the finish.
3. **The `JoinDesync` follows a rule; it is not pose luck (sure on the mechanism).** With the other pairing, one solid's kept vertex holds both null edges of the pair. When the partner's runs are its own kept arcs, those two edges lead to two different partner copies. The vertex-keyed check then refuses, even though the body is right. This predicts the measured split exactly:
   - `ba ∩`: 18 of 18 refuse.
   - `ba ∖`: 236 of 236 refuse.
   - `ba ∪`: 230 of 236 build.
   - `ab ∪` and `ab ∖` are complementary, pose by pose: 222 poses build `∪` and refuse `∖`, 6 the reverse, and 18 refuse both (the six-crossing and B-nested lines).

   Every build passed tier 2, which rejects a vertex whose orbit is split. So `split_cones` really does sort out the cones even when one vertex holds two of them.
4. **Nesting across pairs (the row's option 3) is real, but only the start rule makes it necessary.**

**Ratified text: none needs to change (likely).** The pairing rule lives in the module docs of `insert.rs`, which agents wrote. No `topo` README clause and no DESIGN.md clause states it (searched). Your PR 4057 ruling supports this answer. Under `[failure.taxonomy]`, refusing valid input that has a defined body is row 2 (valid but unbuilt): acceptable as an interim limit, not as a final state.

**The answers, as final states.**

**A (recommended): pair along the shared solid's In arcs, and record the finish's correspondence per corner.**
- *Insertion:* the run at the shared vertex is the In arc itself. The copy takes the stretch inside the partner, and the shared vertex keeps whatever lies outside every partner. At the partner's own vertex the run is the adjacent arc there, as F12 already says.
- *What retires:* turning a run the other way round, the reconcile's fixed-point loop, `hang_at_shared`'s re-reading of turned runs, and `SharedVertexCrossings`' fan arm.
- *What stays:*
  - Strut-in-strut nesting (`holds_whole`), for an In arc that crosses every edge and is therefore minted as the strut on its Out side.
  - The tie reading for cuts along one line.
  - The one-arc refusal, which has its own closed row.
- *Finish:* the correspondence is keyed by null-edge pair: each pair's surviving A end matches its surviving B end at that corner. The vertex map becomes something derived from that. The finish's list of exceptions (shared cut, one ring, welds) collapses into the check that already exists: each pair has exactly one surviving end on each side. The zip's `align` matches by corner.
- *Also changes:* `plan_null_pairs` stops reading the op, and `pairing_start_turns` retires.
- *Still possible:* an error in the per-corner alignment. It would surface as `ZipCorrespondence`, a fault the kernel announces, never a wrong body.
- *Evidence so far:*
  - With this pairing for the refusing pair, 458 of the 982 lines build SOUND (volume, tier 2, 3′ and certificate all pass), and so does `cube ∖ y`.
  - The other 524 lines stop at the finish's vertex-keyed check.
  - **Not measured:** whether those 524 build once the check is keyed per corner.
  - JOIN tried the same pairing at every shared pair on 2026-10-05, before `split_cones` existed. It hit `JoinDesync`, `PinchUncrossed` and an unexplained `ClassificationInvariant`. `PinchUncrossed` has since been retired; the other two must be re-measured before building.
- *Reversible:* yes. The pairing rule is one function, and a per-corner key carries strictly more than a vertex key.
- *Sub-choice (unsure):* apply the In-arc rule everywhere (A's In arcs where no vertex is shared), or only at shared vertices. I lean to everywhere: one rule, with no coupling to the op. The cost is that every `∪` and `∖` vertex pair with four or more crossings moves onto the per-corner path, so it must be re-measured. Applying it only at shared vertices is the safer step and easy to widen later.

**B: keep the start rule and nest every pair's runs at the shared vertex as one laminar tree.**
- It would build the 966 laminar lines and the cleave witness without changing the finish (by construction; not measured).
- The 16 crossing lines stay refused for good: no laminar family exists for the pairings the start rule takes. Re-pairing them needs A's finish change anyway.
- The pairing stays tied to the op and to the finish, and the nesting machinery grows rather than shrinks. Not recommended.

**C: keep refusing, as today.** Not acceptable for either class. Both have a body the kernel can build, and 458 lines have already been built that way, soundly.

**D: the row's option 2 as stated (re-pair only after a refusal, finish unchanged).** It moves 524 lines and two cleave ops from a typed limit to a `JoinDesync` fault, which is strictly worse. A fallback that triggers on a refusal also makes the pairing depend on the order in which plans are tried.

**Worked example: the cleave witness.**
- The cube's corner is the shared vertex. `y` is three corners touching at it. The band, one of them, crosses the cube's corner four times.
- Each of the cube's two Out arcs holds one of the other corners' cuts, so the start rule cannot place the band's pair. The cube's two In arcs, the stretches inside the band, hold nothing.
- With In-arc pairing, `cube ∖ y` builds today: volume 0.97142669, which is 1 − `cube ∩ y`.
- In `cube ∪ y`, the cube's kept vertex holds both of the band's null edges, and the band's kept runs are its two copies. The vertex map sees one cube vertex with two band partners and refuses. A per-corner map says corner 1 goes with copy 1 and corner 2 with copy 2, and `split_cones` then sorts out the cones.

**What A does to poses that build today.**
- `∩` at unshared vertices is unchanged, since A's In arcs are A's kept side for `∩`.
- Under the "everywhere" variant, `∪` and `∖` pairs with four or more crossings switch pairing.
- Shared-vertex pairs that build today by turning a run switch to In arcs.

The bodies should not change, because the cones are worked out downstream either way, but every line must re-measure SOUND. The experiment's `JoinDesync` goes away, since the check it came from becomes per corner. Anything left over would be a `ZipCorrespondence` fault to fix, not a limit to keep.

**Confidence:**
- In-arc pairing is laminar at every shared vertex: sure.
- The `JoinDesync` mechanism and its split by order and op: sure.
- A is the better final state than B, C or D: likely.
- The 524 lines build once the correspondence is per corner: unsure (not measured).
- Everywhere versus shared vertices only: unsure.

## For the orchestrator

- **The decisive experiment was not run.** The build slot was held continuously from 09:20 to 10:40 by fork-svc-c, fork-svc-a, tesswalk-ball and a mesh nextest run. My release build was killed by timeouts while waiting for the slot.
  - The probe is ready: `/root/.local/share/cad-work/fork-svc-b/svc-b-probe.diff`. It is the lane's probe-and-retry patch plus an env gate, `SVC_B_LOOSE`, that skips finish's "conflicting seam vertex correspondence" check.
  - To run it: `GAUGE_SVC_RETRY=1 SVC_B_LOOSE=1 cargo test -p sweep --release --test all four_pairs_battery -- --ignored --nocapture`.
  - If the 524 lines go SOUND, `align`'s disambiguation by the far end already suffices, and per-corner keying only has to restate the guard. If they go `ZipCorrespondence`, `align` itself needs the per-corner key.
  - Second run: In-arc pairing for every pair at a shared vertex, not only the refusing one. This repeats JOIN's 2026-10-05 experiment now that `split_cones` exists, and tells us whether its `ClassificationInvariant` remains.
- **Prior art the brief does not mention:** `work/join/a-vertex-two-crossing-pairs-cut-is-the-in-end-of-one-null-edge-and-the-out-end-of-another.md`, Measured section (commit 10fb9f3cdb8). It records that In-start experiment from before `split_cones`: 45 of 217 lines SOUND, and 414 lines that built SOUND broke.
- **Error in the brief and the row:** "whether the other pairing builds depends on the pose, not on the op". It is predictable from the operand order, the op and which matching the partner's In arcs coincide with (the four-crossing analysis above). The 40 six-crossing lines and the 2 B-nested lines refuse in both `∪` and `∖`. That fits B's runs being a mix of kept and discarded arcs, but I did not trace it.
- **Provenance checked:**
  - `pairing_start_turns`' doc: 3e5cddc2de4, 2026-10-05, agent-written.
  - The `one_each`/`shared_cut` guard: ef483103e75, agent-written. The base vertex-keyed desync is from PR5 (898f114c665).
  - Ev's PR 4057 ruling: 357f10f4eff.
- **Assumed:** tier 2's `SplitVertexOrbit` would catch merged cones, so the 458 SOUND builds show `split_cones` doing the per-cone work. Not checked: what `align` does when both ends of a seam edge have several correspondents.
- **Off the question, for whoever builds A or B:**
  - `holds_whole`'s doc says one tree over every chord at a vertex needs every pair's germs in one order. `cut_order` already is that one order, so the stated obstacle no longer holds. `holds_whole`, `hang_at_shared` and `b_runs`' holders could become one `arc_holders` over all the chords at a vertex.
  - `mint_directed`'s rule that no vertex is the In end of one null edge and the Out end of another holds automatically for any laminar family of properly paired chords. It is a check, not a constraint on the pairing.
  - `SharedVertexCrossings` is documented as row 2 but its name does not follow row 2's `Unsupported*` naming rule. It is minor and I did not file it.
- **Housekeeping:** the target directory is deleted and the clone is restored to a clean checkout. Notes are in `/root/.local/share/cad-work/fork-svc-b/notes.md`.

# Lane `heldrecords/parting-division` — P1

**Row:** `work/heldrecords/the-partings-curved-face-gate-refuses-a-division-its-reason-does-not-cover.md` (P1, M). Read it in full, including its `## Design note for PR 7481's review`. INSIDE filed it from a two-designer weighing, and both designers agreed it is not a fork.

**Stacked:** branch `heldrecords/parting-division` off `origin/heldrecords/split-on` (PR 7481 at `80f11deb86`). The chain under it is `heldrecords/split-keys` (PR 7421), then `embed/records` (PR 7124, EMBED's, still open).
- Open your PR with base `heldrecords/split-on`.
- Merge each base in whenever it moves. Do not push to any base.
- 7481 lifts `EdgeInFaceGap::Curved`, which is what makes this row's second half live.

Owed:
1. **Narrow `EdgeInFaceGap::CurvedFace`** (`boolean/slit.rs`) to the arms that would leave a zero-area ring or a slit in a curved face. A parting that divides a curved face builds:
   - an arc from one boundary loop to another;
   - **a closed rider edge inside one face**, which needs a division arm, since tier 3 allows one region per face.
   Re-word `CurvedFace`'s doc and words to the narrowed reason.
2. **Witnesses pin SOUND, with solid counts and records:**
   - `crates/sweep/tests/a_tube_ending_on_a_ball.rs` `the_probe_family_ships_no_wrong_body`: the poses `inside, both rims`, `inside, one rim` and `inside, one rim tilted`, for `ball ∖ tube`.
   - Expected: 3 solids for both rims and 2 for one rim. The volume is the ball's less the tube's. Each rim carries one edge–edge record.
   - Report which arm each rim takes: loop to loop, or closed in one face.
   - Also try SECT's torus less a puck on a parallel (see `a-curved-slit-fails-tier-3-prime-on-its-two-coincident-arcs`, 2026-11-20 note) and the four rows PR 5793 moved onto this gap. Pin those that build, and report those still refused, with the next refusal.
   - Every result passes t3 and t3′ at three ε.
3. **The design note:** the parting re-derives "this edge lies in that face" after the reduction sweep already decided it (`reduce.rs` `lying_on`, `SpanVerdict::LiesOn`).
   - Measure whether the parting can cite the sweep's row instead of re-deciding it.
   - If citing it is a contained change inside this diff, do it. If not, file a row on this program and report.
   - Do not touch the census's curve-touch lane (`an-edge-face-record-off-a-line-and-a-plane-has-no-curve-touch-lane`).
4. **No valid body moves to refusing.** Diff verdicts against the base across topo, sweep and editor-core `ci`. Report every moved test. Refused → built is expected; built → refused needs a reason.

Review tier: single FULL review.

Standing rules: `briefs/common.md` on this branch.

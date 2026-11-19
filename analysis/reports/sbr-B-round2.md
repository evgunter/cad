## Reconciliation, round 2

Your read of where we have met is right.

**(1) A vertex's point: forward the three-asker state (certification, the finished-body gate, the placing door).**
- **What is true afterwards without the vertex ask.** Every certified edge of any body is in the box, by type, because `EdgeCurve::certify` is the only way to build one and it reads the edge's ends. Every finished or placed body is in the box.
- **What the vertex ask would add.** "Every vertex of any body, mid-build, is in the box." Only a vertex on no certified edge is outside the first guarantee: a lone-vertex loop or null scaffolding, and the finish refuses the body anyway.
- **What leans on such a vertex.** I looked for a decision that does and know of none. The points a construction decides on before any edge exists are profile or sketch values. Those are a door's inputs, not regions of a body, and a vertex ask would not cover them either.
- **Why drop it.** No type holds the ask. Holding it would make every point write fallible and banded, while the raw builder and import adoption still bypass it. A refusal nothing leans on does not earn its place (designer brief §4).
- I withdraw my round-1 addition. This is a detail, not Ev's choice.

**(2) The uniform placing ask: its own row, not a secondary choice in this question.**
- This change makes PR 7088's hull argument sound, so the hand-kept list of placing doors is correct after it. It is fragile, not wrong.
- Neither of us can say what the uniform ask costs: it is unmeasured on the box-scalar lanes, where an ask already starved a row. Ev would be asked a question nobody can yet answer well.
- A row can measure the cost and then propose either the uniform constructor or a gate on the class. The For-Ev text keeps one sentence saying the list remains and why it is sound.

**(3) The smaller points.**
- **Spline faces.** Refuse a straddle as a limit, with no subdivision loop in the predicate. Clipping the net to the face's chart window is still needed: today's whole-net cover would refuse a small face trimmed from a large net. That is a build stage, not Ev's question.
- **The unasked bulge.** I accept the correction. A small loop on a very large sphere bounds its far side, so an unasked bulge inside a build can be as large as its carrier, not "a few km". The conclusion stands: the finish refuses the body, and no certificate reads that face's far side first.

---

## For Ev

**Recommendation (likely).** Three changes to the session box. In effect you have already ruled on the first.

1. **It is a predicate of the region a piece of geometry occupies, faces' regions included.**
   - Its measure is the region's largest |coordinate|.
   - Today it reads a body's vertices and edges only.
   - On PR 6596 you approved this same correction for the consumed extent ("i agree that there's only one right solution here"): it reads "a face's region, not its boundary". That PR left the session-box bullet, which has the same flaw. *sure*
2. **It decides on bounds from both sides, and admits only on the outer one.**
   - The outer bound is the region's *cover*: the one reading of the region an edge or face can occupy, which contains it.
   - The inner bound is points known to lie on the body.
   - "Admitted" then means "no point past the box". That is the premise the run's admission rule on (ε, K) rests on.
   - Today only a point proven past the box refuses, and everything else is admitted. *likely*
3. **Three doors ask it, each before anything is decided on what it asks about.** *likely*
   - **An edge's certification**, of the edge's cover with its ends, before it reads a residual. Certification is the only way to make an edge, so this holds by type.
   - **The finished-body gate**, the check every operation's result passes, of the body's faces.
   - **The door that stands a body by a map**, the *placing door*, of the body where it stands, as now.

   The first ask is new. On PR 6329 you chose the gate alone "unless we actually find it causing a problem, at which point we can think about our strategy for making the checks cheap". The third defect below is that problem. The check costs one box read per edge.

### What is wrong today
- **A curved face's far side is not read.** A sphere of r = 50 centred at (0, 0, 975) reaches 1025 m. It is admitted when its poles are on y and refused when they are on z, because only then is its top a vertex.
- **Edges made later on that far side are never asked.** A placed copy is asked once, at its placement. A split or a concave fillet then makes edges on the unread bulge.
- **A body far past the box is refused by something else first.** A 1e8 m cube fails an edge's certification by one ulp before the box is asked, and which refusal it gets depends on how a line carrier is spelled.

### Premise check
- **The bullet's operative words were never put to you.**
  - "Read from below" and "one it admits may still reach past it" were written in PR 6608, a fix for red tests with no `[ev]`.
  - The same edit deleted the sentence recording your PR 6329 answer on who asks.
  - What you ratified read "one predicate over a body's consumed extent".
- **The first two defects are one.**
  - The placing rule says a result inside its operand's hull stands where the box admitted the operand. The cube is convex, so the rule is sound, but only if "admitted" means the whole region is in the cube.
  - With changes 1 and 2 it does, so the split and the fillet need no new door. *likely*
- **The third defect is about order.**
  - A certificate decides "within ε" only where the admission rule holds, which is inside the box. So the box is a precondition of every certificate.
  - Today it is asked after them. *sure*
- **Only sphere, torus and spline faces can bulge.** A plane, cylinder or cone face is ruled, so its farthest coordinate lies on its boundary, or at a cone's apex where the face holds it. *sure*

### What it reads, by kind of face
- **Plane, cylinder, cone and sphere faces** read the same from both sides, wherever their edges are lines or circle arcs.
  - A sphere face's only interior extreme along an axis is that axis's pole. The existing directional cover read classifies the pole against the face: In makes it a point of the body, Out leaves the boundary as the bound. *sure*
  - An ellipse or spline edge is read as sampled points from inside and its cover from outside.
- **A torus** needs new closed-form code for the points where its normal is parallel to an axis, each classified against the face. *likely*
- **A spline face's** cover must be its control net clipped to the face's own chart window. Today it is the whole net, which would refuse a small face trimmed from a large surface.
- **A straddle**, where the cover reaches past and no point does, is refused as a kernel limit ("cannot show it lies in the box"), naming the face. It is never admitted. It needs a spline face within its net's slack of 1 km.
- **One recourse on both refusals:** move the body toward the origin by the margin. `LeverBounds` is already read this way: act only on a verdict both bounds share.

### The final states for what it reads
- **Both bounds (recommended):** as above.
- **The outer bound only.** One arm fewer, but its refusal says "past the box" of a body that may be inside it, telling a kernel limit as the input's fault. Not recommended.
- **The inner bound only, with faces' far points added.** A body it admits may still reach past wherever nothing was sampled, so the premise stays unproved. Not recommended.

### The final states for who asks
- **Certification, the gate and the placing doors (recommended).**
  - **What it makes true.** Every certified edge of any body, finished or not, is in the box. So is every finished or placed body. A residual read during a build reads in-box geometry, and a body past the box is told so by the box.
  - **What it leaves.** Inside one build, a curved face's far side and a vertex on no edge are unasked until the finish, which refuses the body. No certificate reads either one first.
  - **Reversible:** remove the certification's ask and the gate remains.
- **The gate and the placing doors only** (your PR 6329 choice).
  - It closes the first two defects.
  - A body past about 1e7 m still gets the certificate's refusal or the box's by rounding luck, which breaks "which kind a failure is is always plain" (`[loud-failure]`).
  - Defensible if you want one asker per body; I lean against it.
- **Also weighed and not recommended:** asking the box on every failure inside a build before it ends (every refusing site must remember to), and each building door asking its output's extent up front (declined on PR 6329).

### Worked examples
- **The sphere above.** Read along +z, its pole (0, 0, 1025) lies in the face. It refuses at −25 whichever axis its poles are on.
- **A torus, r = 400, R = 450, centred at 250 m.** The gate reads about 806 m off its seams; its true reach is about 1100 m, so it refuses. (These are the open issue's figures, not run.)
- **The 1e8 m cube.** Its first edge's certification asks the box and refuses as past it, however the line is spelled.

### Ratified text I would change
- **The session-box bullet in `crates/geom-core/README.md`, "Float rounding",** to:
  > The session box is one predicate over a region's largest `|coordinate|`, never a carrier's stored terms: the box bounds each coordinate (`[tolerance.units]`), so it is a cube. It is decided on bounds from both sides, from below off points of the region and from above off its cover. A region is in the box only where its cover is. One with a point past it is refused as past the box, and one whose cover alone reaches past it as a limit. An edge's certification asks it of the edge before its residual, the finished-body gate asks it of a body's faces, and the door that stands a body by a map asks it of that body where it stands.
- **`[tolerance.units]`: no change.** "Rejected at construction" becomes true as written, and DESIGN keeps pointing at the README for the specifics, as you asked on PR 6329.

**Left as it is.** The placing doors stay a hand-kept list: a placement, a sketch frame, a lift and a mirror, the doors whose result can leave its operand's hull. These changes make its argument sound. Whether one constructor that asks should replace the list is a separate question, with its cost to be measured first.

**Confidence.** Change 1: *sure*. Changes 2 and 3: *likely*. I did not check whether any construction certifies helper edges past 1 km on purpose. The new ask would refuse such an edge, and I would treat that as a finding.

## For the orchestrator

- **Remaining difference from the other designer: none of substance.** We have met on all three points. The README wording above is the merged wording.
- **To file.**
  - A row on the placing doors' hand-kept list: either the uniform constructor that asks, or a gate on the class. The list missed the mirror once (PR 7088). Measure the cost on the box-scalar lanes first: PR 7088's ask starved `m10_3::a_terminal_sliver_refuses_naming_its_predicate_and_is_not_refined`.
  - The `crates/editor-core/src/eval/framed.rs` header states the hull argument as if the box admitted the operand's hull. That is false today and becomes true with this change.
- **Provenance.**
  - `47e9bbfde` (PR 6608) wrote "read from below" and deleted "…takes 'in the box' as its premise".
  - `d0c17dee4` is PR 6596's `[ev]` consumed-extent change, with Ev's approval in words.
  - The checkout is shallow, so `[tolerance.units]` "rejected at construction" resolves only to the graft (`dcd012f6b`). Taken as v0 text per the closed shell row.
- **Not checked.**
  - Helper or cutter edges certified past 1 km on purpose.
  - That every finished face has a cover. `work/boxes/face-uncovered-stands-as-a-limit-whatever-its-cause.md` is the row on that.
  - Interval-lane cost of a face-cover read at every finish.
- **Build notes.**
  - The face ask stays after `validate_closed` and before tier 3's first geometric verdict.
  - The placed read's bit-pin (`a_placements_reach_is_its_images_to_the_bit`): `Cover` has no moved form, so decide whether the pin is on bits or on the verdict.
  - `edge_coordinate_reach_below`, `sampled_reach` and `cover_is_exact` fold into the inner bound.
  - `boxes.rs`' door-inventory line on the "unsound direction" reverses.
  - `EdgeCurve::carried` is covered by the move's ask.
  - The certification ask can read `geom`'s `Curve3::cover_over` hulled with the ends, inside `geom-brep`.
- **Rows that move.** `newell_probes.rs` `probe_d_huge_offset_tiny_face` and `r2_cert9_probes.rs` `p6_reproduce_the_huge_offset_red_digit` return to `OutsideSessionBox`.
- **Wording.** The README bullet avoids "world" so the far-apart-copies fork (`work/onehome/an-operation-over-far-apart-copies-has-no-conditioned-computing-frame.md`) can name the frame.

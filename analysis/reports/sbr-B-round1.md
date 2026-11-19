## Reconciliation, round 1

Your summary of what we agree on is right. I checked the other report's provenance and it holds:
- `d0c17dee4` is PR 6596's `[ev]` change to the consumed-extent bullet ("a face's region, not its boundary"). Ev approved it in words: "yeah i agree that there's only one right solution here".
- `47e9bbfde` (PR 6608) deleted the sentence recording Ev's 6329 answer ("…takes 'in the box' as its premise").

Both are now in my report.

**(1) The construction tier.** I have partly moved.
- A's point I accept: a vertex is a region too, and a construction can decide on points before any edge exists. So a vertex's point should be asked when it is written. I add that ask.
- My argument it does not answer: A leaves a line edge between admitted vertices unasked, on the grounds that the cube is convex. That is a correct optimization. It is not a different rule, provided the ask inside `EdgeCurve::certify` reads the span, its ends included.
  - Inside certify, the ask is enforced by the type: certify is the only way to build an `EdgeCurve`, so "every certified edge is in the box" holds without auditing the roughly 59 `add_point` sites.
  - The vertex ask at the Euler operators does depend on that audit, since `Construct::mvfs` has "no preconditions" today.
- What A's version leaves possible: a certified line edge outside the box, wherever some point write skipped the operators. That is A's own unaudited claim.
- My position: certify asks every edge's span (the type guarantee). The point write asks each vertex. The line-edge skip is the build's call.
- This is a detail for the build, not for Ev.

**(2) `[tolerance.units]`.** I have moved to A: leave it unchanged.
- Under both reports "rejected at construction" becomes true as written. The finish is the end of construction.
- Ev asked on PR 6329 that DESIGN.md carry less and point at the README.
- Not worth Ev's attention.

**(3) The placing doors.** I have partly moved, and this one is worth Ev's attention as a secondary choice.
- A's evidence moved me. The list of doors that ask is kept by hand, it already missed the mirror once (PR 7088), and every new operation has to classify itself against it.
- My point A does not dispute: the hull argument is a theorem once the reach is true, so the list is sound. It is fragile, not wrong.
- The uniform form, stated as a type: the only way to make a `Framed` under a map is a constructor that asks. That retires the list and the hull paragraph in `framed.rs`.
- Its cost is one ask per operation under a frame. That cost is unmeasured on the box-scalar lanes, where PR 7088's ask already starved a row.
- I lean to the uniform ask (*unsure*), with the cost measured first. Reversing it is easy.

**(4) Per-kind reads.** A is right on the spline arm, and I correct my report.
- `face_cover`'s spline arm is the whole control net today. Clipping it to the face's chart window and subdividing is new code, as the torus's crest points are.
- I add a plane or cone face bounded by ellipse or spline edges to the inexact list. Its hull is only as exact as its edges' covers.
- On A's worry that the cover's trim constraints are decisions taken before the box is known: under the construction tier every edge is already in the box when those decisions run. A face can bulge past the box only by about its own size, a few km at most, which is far inside the scale (about 1e7 m) where verdicts become coin tosses.
- So a definite `Out` there is sound and the upper bound holds (*likely*). This is a build detail.

**(5) The README bullet's wording.** I adopt A's wording, anchored in "read as its cover" to match the bullet Ev approved on PR 6596, and add "from inside off points known to be on it", so the refusal side is stated rather than implied. Not a fork.

I found no point we differ on that is missing from your summary.

---

## For Ev

**Recommendation (likely).** Change three things about the session box.
1. **It is a predicate of the region a piece of geometry occupies, faces' regions included.** Today it reads a body's vertices and edges only. On PR 6596 you approved this same correction for the consumed extent ("a face's region, not its boundary"). That change left the session-box bullet, which has the same flaw.
2. **It decides on bounds from both sides.**
   - It admits only where the region's cover lies inside the cube. The cover is the one outer bound of the region an edge or face can occupy.
   - It refuses as "past the box" only where a point known to be on the body lies past it.
   - Where the two disagree, it refuses as a kernel limit, never as an admission.
   - So "admitted" means "no point past the box", which is the premise the run's admission rule on (ε, K) rests on.
3. **It is asked where a region comes to exist, before anything is decided on it.**
   - A vertex's point is asked when it is written.
   - An edge is asked inside its certification, before its residual.
   - A body's faces are asked at the finished-body gate.
   - A body stood somewhere by a map is asked at the door that stands it.

### What is wrong today, and what it is evidence of
- **The predicate is one-sided.** Reading from below makes a refusal sound but never makes "in the box" true.
  - A sphere of r = 50 centred at (0, 0, 975) reaches 1025 m.
  - It is admitted when its poles are on y and refused when they are on z, because only then is its top a vertex. *sure*
- **The split or fillet of a placed copy is the same defect.**
  - A placed copy is asked once, at the door that places it. A later split or concave fillet makes edges on its unread bulge.
  - The placing rule says a result inside its operand's hull stands where the box admitted the operand. That holds once "admitted" means the whole region is in the cube.
  - With change 2 the split and the fillet need no new door. *likely*
- **A far body is refused by something else first, which is a different defect: the order of the questions.**
  - A 1e8 m cube fails an edge's certification by one ulp before the box is asked. Which refusal it gets depends on how a line carrier is spelled.
  - A certificate means "within ε" only inside the box, so the box is the certificate's precondition and is asked first.
  - On PR 6329 you chose the finished-body gate alone, "unless we actually find it causing a problem". This is that problem. *sure*
- **Provenance of the wording I would change.**
  - The bullet's "read from below" and "one it admits may still reach past it" were written in PR 6608, a fix for red tests. It was not put to you.
  - The same edit deleted the sentence recording your PR 6329 answer.
  - What you ratified read "one predicate over a body's consumed extent".

### Reading both sides, by kind of face
- **Ruled faces.**
  - A plane, cylinder or cone face is ruled, so its farthest coordinate lies on its boundary, or at a cone's apex where the face holds the apex.
  - Its cover reads that exactly wherever its edges' covers are exact: lines and circle arcs.
  - An ellipse or spline edge is read as a sampled lower bound and a cover upper bound. *likely*
- **Sphere faces.**
  - Along each axis a sphere face's only interior extreme is that axis's pole.
  - The existing cover read along an axis (`face_cover_toward`) classifies that pole against the face: In makes the pole a point of the body, Out leaves the boundary as the bound. So it is exact. *sure*
- **Torus faces.**
  - The normal is parallel to an axis at up to four points, or along two circles when the axis is the torus's own.
  - Each is classified against the face. This is new closed-form code. *likely*
- **Spline faces.**
  - Today their cover is the whole control net, so a small face trimmed from a large net would refuse.
  - The net has to be clipped to the face's chart window and subdivided until the two sides decide. This is new code.
  - A straddle that remains refuses as a limit.
  - Every evaluation of the face passes through those control points, so a net past the box is not pure looseness.

### Who asks: the final states
- **W1 (recommended): the construction tier, the gate and the placing doors.**
  - **True at all times:** every vertex and every certified edge of every body, finished or not, is in the box. So every residual and every kernel-bug ending inside a build reads in-box geometry.
  - **Enforced by type:** certification is the only way to build an edge, so an out-of-box certified edge cannot exist.
  - **Cost:** a vertex is three comparisons; an edge is one box of its cover.
  - **What it leaves:** a curved face can bulge past the box during a build, by at most about its own size, until the finish refuses it. That is numerically harmless at that scale. *likely*
- **W2: the gate and the placing doors only** (today's choice).
  - A far body's refusal then depends on rounding.
  - That breaks "which kind a failure is is always plain" (`[loud-failure]`).
- **W3: each building door asks its output's extent up front** (declined on PR 6329).
  - It restates, once per operation, what the construction tier sees once for all of them.

**Secondary choice: which placing doors ask.**
- **Today:** a hand-kept list of doors whose result can leave its operand's hull: a placement, a sketch frame, a lift, a mirror. It missed the mirror once.
- **Alternative:** the only way to make a placed value is a constructor that asks the box. That retires the list and its argument, for one more read per operation under a frame.
- **Lean:** the uniform ask (*unsure*). Its cost on the interval lanes is unmeasured, and an ask there has already starved a test. Either way is easy to reverse.

### Worked examples
- **The sphere above.** Read along +z, its face's cover finds the pole (0, 0, 1025) In. It refuses at −25 whichever axis its poles are on.
- **A torus, r = 400, R = 450, centred at 250 m.** The gate reads about 806 m off its seams; its cover reads about 1100 m. It refuses. (The issue's figures, not run.)
- **The 1e8 m cube.** Its first vertex refuses as past the box, whatever the carrier's spelling.

### Ratified text I would change
- **The session-box bullet in `crates/geom-core/README.md`, "Float rounding",** to:
  > The session box is one predicate over the region a piece of geometry occupies, never a carrier's stored terms. The region is read from outside as its cover, and from inside off points known to lie on it. The box bounds each coordinate (`[tolerance.units]`), so it is a cube. A region is in the box only where its cover is. One with a point past it is refused as past the box. One whose cover alone reaches past it is refused as a limit. The door at which a region comes to exist asks it, before anything is decided on that region: the point write asks it of a vertex, certification of an edge, the finished-body gate of a body's faces, and the door that stands a body by a map of that body where it stands.
- **`[tolerance.units]`: no change.** "Rejected at construction" becomes true as written.

**Confidence.** Change 1: *sure*. Change 2: *likely*. W1: *likely*; I audited no door for decisions it takes before its first vertex. The uniform placing ask: *unsure*.

## For the orchestrator

- **Where I still differ from the other report.**
  - Certification asks every edge's span, its ends included. That is the type-enforced guarantee. The other report asks only curved edges and leans on the vertex asks at the operators. Its line-edge skip is a sound optimization only once every point write is audited.
  - These are build details, not for Ev. Otherwise we agree.
- **Provenance, checked.**
  - `47e9bbfde` (PR 6608) wrote "read from below" and deleted "…takes 'in the box' as its premise", Ev's 6329 answer.
  - `d0c17dee4` is PR 6596's `[ev]` consumed-extent change.
  - The checkout is shallow, so `[tolerance.units]`' "rejected at construction" resolves only to the graft (`dcd012f6b`). Taken as v0 text per `work/shell/the-session-box-is-not-enforced-at-construction.md`.
- **Not checked.**
  - Scaffold or cutter edges certified past 1 km by any construction. W1 would newly refuse them below about 1e7 m.
  - The roughly 59 `Body::add_point` sites.
  - Interval-lane cost of a face-cover read at every finish, and of the uniform placing ask (PR 7088 reported a starvation).
  - The placed read's bit-pin: `a_placements_reach_is_its_images_to_the_bit` versus reading `Cover::box_in` along the motion's axes. Decide whether the pin is on bits or on the verdict.
- **Build notes.**
  - The face ask stays after `validate_closed` and before tier 3's first geometric verdict.
  - `edge_coordinate_reach_below`, `sampled_reach` and `cover_is_exact` fold into the inner bound of the one two-sided read.
  - `boxes.rs`' door inventory line ("a looser arc arm is the session box's unsound direction") reverses under change 2.
  - Word the bullet without "world", so the far-apart-copies fork (`work/onehome/an-operation-over-far-apart-copies-has-no-conditioned-computing-frame.md`) can name the frame.
- **Defect off the question.** The header of `crates/editor-core/src/eval/framed.rs` states the hull argument as if the box admitted the operand's hull, which is false today. Fix it with this change, or delete it if the uniform ask is taken.
- **Rows that move.** The mesh probes `newell_probes.rs` `probe_d_huge_offset_tiny_face` and `r2_cert9_probes.rs` `p6_reproduce_the_huge_offset_red_digit` return to `OutsideSessionBox`.

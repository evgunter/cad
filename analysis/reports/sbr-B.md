## For Ev

**Recommendation (likely).** Change three things about the session box:
- **What it is a predicate of.** A region's reach: its largest |coordinate|, which is the region's extent along the six world directions ±x, ±y, ±z.
- **How it decides.** On both sides. It refuses only where a lower bound read from points of the body is definitely past the box. It admits only where an upper bound read from the region's cover is definitely inside it.
- **What it reads, and who asks it.** It reads the whole point set, faces' regions included. Three doors ask it:
  - each edge's certification gate, before the residual;
  - the finished-body gate, of the whole body;
  - the door that places a copy, of the whole body under its map.

Terms used below:
- *Reach*: a region's largest |coordinate|. The box is a cube, so this is the measure.
- *Cover*: the one reading of the region an edge or face can occupy (`topo::face_cover`, `topo::edge_cover`). It is a superset, so it bounds the reach from above.
- *Below / above*: a lower bound, from points that are on the body, and an upper bound, from the cover.
- *Straddle*: the lower bound is inside and the upper bound is outside. Neither side decides.

### What the evidence points at (premise check)

1. **The predicate is one-sided.** Reading only from below makes a refusal sound: anything it refuses really has a point past the box. It cannot make "this body is in the box" true. The admission rule needs that second statement. The seam-dependent sphere (evidence 1) is a symptom of this, not a missing entity in the walk. Adding faces' boundaries or samples would still read only from below. *sure*
2. **The split and the fillet of a placed copy (evidence 2) do not need a new door.** PR 7088 relies on one argument: the box is a convex cube, so a result inside its operand's hull is inside the box wherever its operand was. That argument is sound. It fails today only because the reach the box admitted was short. Once the placing door asks the true reach, every one-input operation inside its operand's hull is covered as 7088 says. *likely*
3. **The far body (evidence 3) is a problem of order, not of what is read.** A certificate decides "within ε" only where the admission rule holds, which means only inside the box. So the box is a precondition of every certificate, and the certificate's own gate should check it first. On PR 6329 you chose "the gate alone … unless we actually find it causing a problem". This is that problem: which refusal a 1e8 m body gets depends on how a carrier is spelled. *sure*

### The options, as final states

**A (recommended): two-sided reach, asked by the certification gate, the finished-body gate and the placing doors.**
- **What it reads.** One read: the body's extent along each world direction, the largest over its faces. Each face's cover already contains its boundary, so edges and vertices are included.
  - **Exact for plane, cylinder and cone faces.** A coordinate's extreme on these lies on the boundary (along a ruling), or at the apex where the face holds it. The cover's hull reads it exactly. *likely*
  - **Exact for sphere faces.** The only interior extreme along an axis is that axis's pole. `face_cover_toward` already classifies that pole against the face: if it is In, the pole is a point of the body; if it is Out, the boundary bounds the face. *sure*
  - **Tori need new closed-form code.** The normal is parallel to an axis at up to four points, or along two circles when the axis is the torus's own. Each is classified against the face. *likely*
  - **Spline faces read both sides.** The lower bound comes from points sampled inside the trim region. The upper bound comes from the control net clipped to the face's chart window, subdivided until the two sides decide. A remaining straddle is refused as undecided, never admitted (`[verdicts]`).
- **Who asks.**
  - `EdgeCurve::certify` asks first, of the edge's span: its ends and its cover's box. A 1e8 m body then refuses at its first edge, and refuses as the box, whatever the carrier's spelling.
  - The finished-body gate asks of the faces' regions. `FinishedBody::moved_by` asks the same.
  - `session_box_at` asks of the faces under the map. It reads each face cover's range along the world axes pulled back into the body's frame, so no image is built.
- **What it makes true.**
  - No finished or placed body has a point past the box.
  - Every refusal names a real point past the box.
  - A straddle is its own plain kind: "cannot show it lies in the box".
  - The hull argument in `framed.rs` becomes true as written.
- **What it leaves possible.** During a construction, before its finish, a verdict can read a curved face whose bulge passes the box while all of its edges are inside. That excess is bounded by the face's own size, a few km at most, so the numerical harm is negligible. The finish refuses the body. *likely*
- **Cost.** Cheap: six support reads per face and one box read per edge.
- **Reversible.** Removing the certification ask leaves gate-only.

**B: the same two-sided reach, but asked only at the finished-body gate and the placing doors (your 6329 split, choice B, kept).**
- It closes evidence 1 and evidence 2.
- It leaves evidence 3: a body beyond about 1e7 m (where one ulp passes ε) refuses as a certification residual, and which refusal it gets depends on rounding luck. That breaks "its kind is plain" (`[loud-failure]`).
- Defensible if you want exactly one door per body. I lean against it.

**C: keep the read from below, add faces' points to it, and reword the README to "a body it admits reaches past the box by at most its curvature's bulge".**
- It is honest, but the premise stays false. The admission rule's floor would then rest on a number nothing bounds.
- Rejected.

**Not recommended: read the cover alone, from above.**
- It is the simplest code.
- It refuses bodies that are inside the box wherever a cover is loose, as an input error, so the error claims a point that does not exist. That is the wrong kind of failure.

### Worked example

A sphere of r = 50 centred at (0, 0, 975).
- **Today:** poled on z it is refused at margin −25. Poled on y it is admitted, because its top lies inside a face.
- **Under A:** the face's cover read toward +z finds the pole (0, 0, 1025) classified In, which is a point of the body. The lower bound is 1025, so it is refused at margin −25 either way.
- **Placed copy:** placed at the same spot as a copy, the placing door refuses it the same way. A later split through its bulge never runs.

### Ratified text I would change

- **The session-box bullet in `crates/geom-core/README.md`, "Float rounding".**
  - It becomes: "one predicate over a region's largest |coordinate|, decided on bounds from both sides (from below off points of the region, from above off its cover); asked of each edge at its certification before its residual, and of each body's faces at the finished-body gate and at the door that places it".
  - The sentence "may still reach past" is deleted.
  - Provenance: the box predicate and the gate-only choice are yours (PR 6329). The words "vertices' and edges' own points" and "a cube" were written by agents (PR 6608, and its fix pass) without a question to you.
- **`[tolerance.units]`.** Its sentence "geometry outside the range is rejected at construction" has been true only at the finish. Make it precise: "rejected where it is made: an edge at its certification, a body at its finish and at its placement".

**Confidence:** *likely* on A overall; *sure* on the premise corrections 1 and 3.

## For the orchestrator

- **Provenance.**
  - PR 6329: Ev wrote, verbatim, "for split 2, B seems good unless we actually find it causing a problem, at which point we can think about our strategy for making the checks cheap".
  - Designer B's version in 6329 had a second caller: any failure on derived geometry asks the box before it ends. The implemented text dropped it. Evidence 3 is the residue of dropping it.
  - The README phrase "read from below off its vertices'" was written in 47e9bbfde ("onecopy: session-box fix pass", PR 6608 delta review).
  - The 6329 designer text said "face boundaries", approved in passing.
  - The checkout is shallow (history from about 2026-11-01), so earlier provenance of `[tolerance.units]`'s "rejected at construction" was not traceable. Row `no-door-rejects…` calls it D4 ¶4 v0 text.
- **Not checked.**
  - Whether any construction legitimately certifies scaffold or cutter edges past 1 km. If one does, A's certification ask would newly refuse it between 1 km and about 1e7 m. Grep the blend reach, the split and through-all paths before dispatch.
  - Whether `Cover::patch`, the torus cover, can serve as the upper bound as it stands.
  - How often a spline face would straddle.
  - That every `EdgeCurve::certify` caller wraps its error with the operation's name. `[tolerance.typed-failure]` says the attachment gates do.
- **Defect off the question.** The `crates/editor-core/src/eval/framed.rs` module doc states the hull argument as if the box had admitted the operand's hull, which is false today. It becomes true under A or B. Under neither of them it is wrong as written, and should cite `work/oneround/the-session-box-reach-misses-a-curved-faces-bulge.md`.
- **Neighbouring rows.**
  - `work/onehome/a-rigid-motion-rewrites-and-recertifies-every-edge.md` (filed from 6329): if its answer is "a rigid image stays certified", nothing changes here, because the placing door still asks.
  - The two mesh probes (`newell_probes.rs` `probe_d_huge_offset_tiny_face`, `r2_cert9_probes.rs` `p6_reproduce_the_huge_offset_red_digit`) return to `OutsideSessionBox` under A.
- **Layering assumption.** A's certification ask lives in `geom-brep` (it reads `edge_coordinate_reach_below` and the cover's box), and the face read lives in `topo::cover`, beside `face_cover_toward`. I assumed a two-sided face reach is a method on the face cover plus a points-of-the-face reading. If BOXES or ONEHOME read `[one-way.places]` as forbidding a lower-bound reading beside the cover, raise it: `edge_coordinate_reach_below`'s sampled branch is the existing precedent.

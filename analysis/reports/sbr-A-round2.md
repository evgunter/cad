## For Ev

**Recommendation.** Three changes to the session box. You have in effect already ruled on the first.

1. **The box is a predicate of the region a piece of geometry occupies, faces' regions included.** Its measure is the region's largest |coordinate|. Today it reads a body's vertices and edges only. On PR 6596 you approved this same correction for levers ("i agree that there's only one right solution here"): the consumed extent reads "a face's region, not its boundary". That PR left the session-box bullet, which has the same flaw, to this row. *sure*
2. **It is decided on bounds from both sides, and admits only on the outer one.** The outer bound is the region's *cover* (the one reading of the region an edge or face can occupy, `[one-way.places]`; it contains the region). The inner bound is points known to be on the body. "Admitted" then means "no point past the box", which is the premise the run's admission rule on (ε, K) rests on. Today only a point proven past the box refuses, and everything else is admitted. *likely*
3. **Three doors ask it, each before anything is decided on what it asks about.** *likely*
   - An edge's certification, of the edge's cover (its ends included), before it reads a residual.
   - The finished-body gate (the check every operation's result passes), of the body's faces.
   - The door that stands a body somewhere by a map (a *placing door*), of the body where it stands, as now.

   The first is new. On PR 6329 you chose the gate alone "unless we actually find it causing a problem, at which point we can think about our strategy for making the checks cheap". The third defect below is that problem, and the ask is one box read per edge.

**What is wrong today.**
- **A curved face's far side is not read.** A sphere of r = 50 centred at (0, 0, 975) reaches 1025 m. It is admitted when its poles are on y and refused when they are on z, because only then is its top a vertex.
- **Edges made later on that far side are never asked.** A placed copy is asked once, at its placement. A split or a concave fillet then makes edges on the unread bulge.
- **A body far past the box is refused by something else first.** A 1e8 m cube fails an edge's certification by one ulp before the box is asked. Which refusal it gets depends on how a line is spelled.

**Premise check.**
- **The bullet's operative words were never put to you.** "Read from below" and "one it admits may still reach past it" were written in PR 6608, a fix for red tests with no `[ev]` and no comment from you. What you ratified on PR 6329 read "one predicate over a body's consumed extent".
- **That pass also deleted the sentence recording your choice of who asks**: "A kernel-bug panic that reads a margin on derived geometry takes 'in the box' as its premise." No code acts on it.
- **The first two defects are one.** PR 7088's rule says a result inside its operand's hull stands where the box admitted the operand. The cube is convex, so the rule is sound, but only if "admitted" means the whole region is in the cube. With changes 1 and 2 it does, and the split and the fillet need no new door. *likely*
- **The third is a matter of order.** A certificate decides "within ε" only where the admission rule holds, which is inside the box. So the box is a precondition of every certificate, and today it is asked after them.
- **Only sphere, torus and spline faces can bulge.** A plane, cylinder or cone face is ruled, so its farthest coordinate lies on its boundary, or at a cone's apex where the face holds it. *sure*

**Worked examples.**
- **The sphere above.** Read toward +z, its pole (0, 0, 1025) is placed inside the face, so it is a point of the body. Poled on y or on z, it refuses at −25.
- **Torus, r = 400, R = 450, centred at 250 m.** The gate reads about 806 m off its seams. Its true reach is about 1100 m, so it refuses. (The open row's figures, not run.)
- **The 1e8 m cube.** Its first edge's certification asks the box and refuses in the box's sentence, however the line is spelled. Today, at the default ε, that edge refuses on a check whose ending names a kernel defect (`CertCheck::MappedSource`, read in the code, not run).

**What it reads: the final states.**
- **Both bounds (recommended).**
  - Cover inside the cube: admitted.
  - A point of the body past it: refused as past the box, the input's fault.
  - Between the two: refused as a kernel limit ("cannot show it lies in the box"), naming the face. Never admitted.
  - One recourse on both refusals: move it toward the origin by the margin. `LeverBounds` is already read this way: act only on a verdict both bounds share.
  - **Exact where it matters most.** A sphere face reads the same from both sides, and so does a plane, cylinder or cone face whose edges are lines and circle arcs. A torus needs new closed-form code for its far points. *likely*
  - **What it leaves:** a spline face whose control net reaches past the box's edge while its surface may not is refused as a limit. That needs a body within its net's slack of 1 km. Every evaluation of the face passes through those control points, so a net past the box is not pure looseness. *likely*
- **The outer bound only.** One arm fewer. Its refusal says "past the box" of a body that may be inside it, so a kernel limit is told as the input's fault. Not recommended.
- **The inner bound only, with faces' far points added** (the open row's sketch). It keeps "refused means a point past". A body it admits may still reach past wherever nothing was sampled, so the premise stays unproved. Not recommended.

**Who asks: the final states.**
- **Certification, the gate and the placing doors (recommended).**
  - **What it makes true:** every certified edge of any body, finished or not, is in the box, because certification is the only way to make one. So is every vertex the Euler operators make: each is written only as the end of an edge already certified (read in `mev`'s two forms and the ring vertex). Every finished or placed body is in the box. So a residual read during a build reads in-box geometry, and a body past the box is told so by the box.
  - **Why at the certificate:** in-box is the pose that licenses treating f64 as exact, and `[simple.conditioned]` decides a pose before the formula that leans on it.
  - **What it leaves:** inside one build, a curved face's bulge is unasked until the finish, which refuses the body. So is a body's first vertex until its first edge. A door's inputs that are no region of a body (a profile's 2-D coordinates, a far axis) can still be decided on before any edge is certified.
  - **Reversible:** remove the certification's ask and the gate remains.
- **A failure inside a build asks the box before it ends** (a form proposed on PR 6329; unbuilt). Nothing on the passing path. Every refusing or panicking site has to remember to ask.
- **The gate and the placing doors only** (your PR 6329 choice, kept). It closes the first two defects. A body past about 1e7 m still refuses as a certification residual or as the box by rounding luck, so `[loud-failure]`'s "which kind a failure is is always plain" fails for it. Defensible if you want one asker per body; I lean against it.
- **Each building door asks its output's extent up front** (declined on PR 6329). It states once per operation what certification sees once for all of them.

**Ratified text I would change.**
- **The README's session-box bullet**, to: "The session box is one predicate over the region a piece of geometry occupies, never a carrier's stored terms. The region is read from outside as its cover, and from inside off points known to lie on it. The box bounds each coordinate (`[tolerance.units]`), so it is a cube. A region is in the box only where its cover is. One with a point past it is refused as past the box. One whose cover alone reaches past it is refused as a limit. The door at which a region comes to exist asks it, before anything is decided on that region: certification asks it of an edge with its ends, the finished-body gate of a body's faces, and the door that stands a body by a map of that body where it stands."
- **`[tolerance.units]`: no change.** "Rejected at construction" becomes true as written: an edge at its certification, a body at its finish and its placement. On PR 6329 you asked that DESIGN point to the README for such specifics.

**Left as it is, with a row to file.** The placing doors are a hand-kept list (a placement, a sketch frame, a lift, a mirror: those whose result can leave its operand's hull). It missed the mirror once (PR 7088). Changes 1 and 2 make its argument sound but do not check the list. The uniform form is that the only way to make a value stood by a map is a constructor that asks, which retires the list for one more read per operation. Its cost on the interval lanes is unmeasured, and an ask there has already starved a test, so it goes to its own row to be measured first. The README sentence above holds under either form. Separately, which frame an operation over far-apart copies computes in is its own open fork, and nothing here settles it.

**Confidence.** Change 1 *sure*. Change 2 *likely*. Change 3 *likely*: I did not check whether any construction certifies helper edges past 1 km on purpose, which the new ask would refuse.

## For the orchestrator

**Your "met" list is right.** What follows is on the crossed points, as statements about the code afterwards.

**(1) A vertex's point: forward three askers, no vertex ask.**
- **"Every vertex the operators make is in the box" is already true under certification's ask.** I read the operators this round. `mev_fan_gate` and `mev_lone` certify the new edge from the old point to the new point before any mutation; `add_point(point)` comes after. `ring_vertex` runs that gate with a chord strut and then kills the strut, so a lone ring vertex was an edge's end when it was asked. `mev_null` puts the new vertex on the old one's point (from its test's doc; I did not read its body). No geometric decision precedes the certification in any of them: the plan before it is topological.
- **What stays unasked:** `mvfs`'s seed vertex until its first `mev`, which then asks it as `p_old`. Nothing is decided on it in between.
- **Points written outside the operators** (import adoption, `map_isometry`, `move_points_then_rechart`) reach a certification or the move's own ask before anything leans on them. A fourth asker there would have to sit in `Body::add_point`, the raw builder whose doc says "no validity promises", and no type would hold it.
- **So the vertex ask states a second time what the edge ask already makes true**, in a place that needs an audit to trust. I would not forward it. B's reason for adding it ("a construction can decide on points before any edge exists") is the claim I made in my first report and could not support once I read `mev`. If the build finds such a decision, that is a finding to bring back. This is a build detail, not Ev's.

**(2) The placing doors: its own row, named to Ev in one paragraph.**
- **No ratified text turns on it.** The README sentence "the door that stands a body by a map" is true under the hand-kept list and under the uniform constructor alike. The list lives in `framed.rs`'s module doc, which is not ratified.
- **Ev cannot choose yet.** Both leans are *unsure* for the same reason, an unmeasured cost on the box-scalar lanes where PR 7088's ask already starved a row. A question whose answer waits on a measurement is not ready for Ev.
- **Both packagings are defensible.** I kept the alternative and its cost in one paragraph for Ev, so nothing is hidden, and did not ask for a choice. If you prefer B's secondary-choice block, the content is the same. That is your call, not a design difference.
- **Row to file:** "the doors that ask where a body stands are a hand-kept class", with the mirror miss as evidence, the constructor form as the candidate and the lane cost as its first step.

**(3) Smaller.**
- **Spline subdivision:** refuse the straddle as a limit on the clipped net; add a refining stage only if a row shows the straddle. Build detail.
- **"A few km at most":** still wrong in B's round-1 text. A small in-box loop on a very large sphere bounds the sphere's far side, so an unasked bulge is as large as the carrier. The trim constraints' soundness there rests on their margins being as large as the carrier, not on the bulge being small. The finish still refuses the body.

**Taken from B this round.** A plane, cylinder or cone face is exact only where its edges' covers are (lines and circle arcs), and the README bullet's "read from outside as its cover, and from inside off points known to lie on it".

**Provenance.**
- `47e9bbfde` (PR 6608) wrote "read from below" and removed the "takes 'in the box' as its premise" sentence; PR 6608 has no issue comments. The PR 6329 text is in `31576a6c6`. `d0c17dee4` is PR 6596's `[ev]` change.
- `[tolerance.units]`' "rejected at construction" resolves only to the shallow graft (`dcd012f6b`, 2026-11-01). `work/shell/the-session-box-is-not-enforced-at-construction.md` calls it ratified since v0; I took that as given.

**Not verified.**
- Nothing was built or run. The sphere figures are the row's measurement; the torus figures are the row's "likely, not run".
- That the 1e8 m body is told a kernel defect: read from `CertCheck::MappedSource => Ending::Residual(Unsized::Defect)` in `crates/geom-brep/src/certify.rs`, not from a rendered refusal.
- Whether a construction certifies cutter or scaffold edges past 1 km on purpose. Such an edge is a certificate read outside the premise, so a hit is a finding, not a reason to drop the ask.
- That every finished face has a cover. A granting door refuses an uncovered entity (`crates/topo/src/boxes.rs`), and under change 2 the box is one.
- Cost on the box-scalar lanes of a face-cover read at every finish.

**What the build would meet.**
- **The gate's order.** The face ask stays after `validate_closed` and before tier 3's first geometric verdict.
- **The placed read.** `a_placements_reach_is_its_images_to_the_bit` pins bits; `Cover` has no moved form, and a face cover read along the motion's axes rounds differently. Decide whether the pin is on bits or on the verdict.
- **New code:** the spline arm's clip to the face's chart window (it is the whole net today), and a torus's far points.
- **Second readings that fold away:** `edge_coordinate_reach_below`, `sampled_reach` with a measure and `cover_is_exact` become the inner bound of one two-sided read.
- **Layering.** The certification's ask sits in `geom-brep` and reads `geom`'s `Curve3::cover_over` hulled with the two ends.
- **`boxes.rs`' door inventory** line for the session box reverses, with its `cover-kind-match.readers` entry.
- **`EdgeCurve::carried`** (the isometry's path) makes an edge without certifying; the move's own ask covers it.

**Rows this touches.**
- The mesh probes `newell_probes.rs` `probe_d_huge_offset_tiny_face` and `r2_cert9_probes.rs` `p6_reproduce_the_huge_offset_red_digit` return to the box's refusal.
- `crates/editor-core/src/eval/framed.rs`'s header states the hull argument as if the box had admitted the operand's hull. It becomes true under the recommendation.
- The README bullet avoids "world", so the far-apart-copies fork can name the frame.

**Housekeeping.** I changed nothing in the repository. My only files are scratch copies under `~/.local/share/cad-work/designer-sbr-a/`.
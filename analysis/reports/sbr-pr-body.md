**The decision: the session box becomes a two-sided predicate over regions, and an edge's certification asks it before reading a residual.** Both designers agree after two rounds; nothing is split. The diff is the session-box bullet of `crates/geom-core/README.md` ("Float rounding").

**Three changes.**
1. **It reads the region a piece of geometry occupies, faces included**, through its cover (the one outer bound of what an edge or face can occupy). Today it reads vertices and edges only, so a sphere of r = 50 at (0, 0, 975) is admitted or refused by where its seams fall. You approved the same correction for the consumed extent on PR 6596.
2. **It decides on both bounds.** Admitted only where the cover lies in the cube; a point of the body past it refuses as past the box; a cover alone past it refuses as a kernel limit. "Admitted" then means "no point past the box", which is what the admission rule on (ε, K) assumes.
   - Rejected: from below only (what the tree does; the premise stays unproved), and the cover only (it tells a kernel limit as the input's fault).
3. **Three doors ask it: an edge's certification (new), the finished-body gate of the faces, and the placing door.** On PR 6329 you chose the gate alone "unless we actually find it causing a problem". A body at 1e8 m is that problem: it refuses at a certification residual or at the box by how a line carrier rounds. With certification asking, every certified edge is in the box by type.
   - Rejected: the gate alone; every failing site asking before it ends; each building door asking its output's extent.

**What I am asking.** Approve the three changes and the bullet's text in the diff? `[tolerance.units]` does not change.

**One thing you should know.** The bullet's current words "read from below" and "one it admits may still reach past it" were never put to you. PR 6608's fix pass (`47e9bbfde`) wrote them over "one predicate over a body's consumed extent", and removed the sentence recording your PR 6329 answer. I checked the commit and that the PR has no comments.

**Left out on purpose.** The doors that ask where a body stands are a list kept by hand in a module doc. Both designers lean to one constructor that asks, and both are unsure because its cost on the interval lanes is unmeasured. It is filed as `work/onehome/the-doors-that-ask-where-a-body-stands-are-a-hand-kept-class.md`, to be measured first.

**Not yet known.** Whether any construction certifies helper edges past 1 km on purpose (the new ask would refuse them), and what a face-cover read at every finish costs on the interval lanes. A torus's far points and a spline net clipped to its face are new code.

Rounds, provenance and what the build would meet: `work/oneround/log.md` (2026-11-19) and `docs/DESIGN-FORK-LOG.md` row 289.

---

## A

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

---

## B

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

---

🤖 Generated with [Claude Code](https://claude.com/claude-code)

https://claude.ai/code/session_01EdpJs4dRmhgaKbdqQzfNmw

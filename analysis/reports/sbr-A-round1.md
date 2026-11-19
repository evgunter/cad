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
  - **Exact where it matters most.** Plane, cylinder, cone and sphere faces read the same from both sides. A torus needs new closed-form code for its far points. *likely*
  - **What it leaves:** a spline face whose control net reaches past the box's edge while its surface may not is refused as a limit. That needs a body within its net's slack of 1 km. *likely*
- **The outer bound only.** One arm fewer. Its refusal says "past the box" of a body that may be inside it, so a kernel limit is told as the input's fault. Not recommended.
- **The inner bound only, with faces' far points added** (the open row's sketch). It keeps "refused means a point past". A body it admits may still reach past wherever nothing was sampled, so the premise stays unproved. Not recommended.

**Who asks: the final states.**
- **Certification, the gate and the placing doors (recommended).**
  - **What it makes true:** every certified edge of any body, finished or not, is in the box, because certification is the only way to make one. Every finished or placed body is in the box. So a residual read during a build reads in-box geometry, and a body past the box is told so by the box.
  - **Why at the certificate:** in-box is the pose that licenses treating f64 as exact, and `[simple.conditioned]` decides a pose before the formula that leans on it.
  - **What it leaves:** inside one build, a curved face's bulge and a vertex on no certified edge are unasked until the finish, which refuses the body. A door's inputs that are no region of a body (a profile's 2-D coordinates, a far axis) can still be decided on before any edge is certified.
  - **Reversible:** remove the certification's ask and the gate remains.
- **A failure inside a build asks the box before it ends** (a form proposed on PR 6329; unbuilt). Nothing on the passing path. Every refusing or panicking site has to remember to ask.
- **The gate and the placing doors only** (your PR 6329 choice, kept). It closes the first two defects. A body past about 1e7 m still refuses as a certification residual or as the box by rounding luck, so `[loud-failure]`'s "which kind a failure is is always plain" fails for it. Defensible if you want one asker per body; I lean against it.
- **Each building door asks its output's extent up front** (declined on PR 6329). It states once per operation what certification sees once for all of them.

**Ratified text I would change.**
- **The README's session-box bullet**, to: "The session box is one predicate over a region's largest `|coordinate|`, never a carrier's stored terms: the box bounds each coordinate (`[tolerance.units]`), so it is a cube. It is decided on bounds from both sides, from below off points of the region and from above off its cover. A region is in the box only where its cover is. One with a point past it is refused as past the box, and one whose cover alone reaches past it as a limit. An edge's certification asks it of the edge before its residual, the finished-body gate of a body's faces, and the door that stands a body by a map of that body where it stands."
- **`[tolerance.units]`: I would leave it.** "Rejected at construction" becomes true of an edge at its certification and of a body at its finish and its placement. Naming those three doors there as well would state them in two places, and on PR 6329 you asked that DESIGN point to the README for such specifics. This is a wording detail either way.

**Left as it is.** The placing doors are a hand-kept list (a placement, a sketch frame, a lift, a mirror: those whose result can leave its operand's hull). It missed the mirror once (PR 7088). Changes 1 and 2 make its argument sound but do not check the list. Which frame an operation over far-apart copies computes in is its own open fork, and nothing here settles it.

**Confidence.** Change 1 *sure*. Change 2 *likely*. Change 3 *likely*: I did not check whether any construction certifies helper edges past 1 km on purpose, which the new ask would refuse.

## For the orchestrator

**Your read of the two reports is right**, with one addition: B does not cite PR 6596 (`d0c17dee4`, Ev: "only one right solution here"), which settles change 1 more firmly than either report's argument.

**Point by point.**
1. **The construction tier: I move to B.** `EdgeCurve::certify` already takes the edge's two ends, and certification is the only constructor of a certified edge, so one ask there holds by type. My vertex ask needed every Euler operator to gain a band, and the raw builder bypassed it anyway. What B's form leaves that mine did not: a vertex on no certified edge (a lone-vertex loop, null scaffolding) is unasked until the finish. I know of no decision that leans on one before then. I also drop "a line edge is asked nothing": a line's cover is its segment, so the uniform ask of the edge's cover is the same read with no kind match.
2. **`[tolerance.units]`: I hold, and it is a detail.** B's sentence repeats the README's list of doors in DESIGN. `[one-way.places]` makes the README the one place, and Ev's PR 6329 comment (14:31Z) asked for DESIGN to "refer to a readme … instead of spelling them all out". Ev sees the diff either way; it does not need a question.
3. **The placing doors: I move to B for this fork.** My lean was *unsure* and unmeasured, and B's answer closes the defect without it. My argument is not answered: the list is kept by hand and missed the mirror. That is a row to file (a gate on the class, or the uniform ask), not part of this question. I kept one sentence for Ev under "Left as it is".
4. **Per-kind reads: agreed, except the spline's subdivision.** I agree on plane, cylinder, cone, sphere and that a torus needs new code. B subdivides a spline's clipped net "until the two sides decide". I would not build that loop into the predicate now: the case is a spline face within its net's slack of 1 km, and a limit refusal with its margin serves it. The build can add a stage if a row shows the straddle. Not for Ev. My two flags stand as build notes and B's report does not contradict them: the spline arm of `face_cover` is the whole net today, and the trim constraints are decisions taken before the box is known.
5. **The README wording: merged.** I took B's measure ("largest |coordinate|") and "from both sides", and kept three things B's sentence drops: "never a carrier's stored terms" (Ev's own phrase on PR 6329), the cube sentence, and what each arm does.

**One claim of B's I would not forward.** B bounds the unasked bulge inside a build at "a few km at most". A small loop on a very large sphere bounds the sphere's far side too, so the bulge is as large as the carrier. The conclusion survives: the finish refuses the body.

**Provenance.**
- `git log --all -S'read from below off' -- crates/geom-core/README.md` gives `47e9bbfde` (PR 6608, 2026-11-09). PR 6608 has no issue comments and its body does not ask Ev. The same commit removed the "takes 'in the box' as its premise" sentence. The PR 6329 text is in `31576a6c6`.
- `[tolerance.units]`' "rejected at construction" resolves only to the shallow graft (`dcd012f6b`, 2026-11-01). `work/shell/the-session-box-is-not-enforced-at-construction.md` calls it ratified since v0; I took that as given.

**Not verified.**
- Nothing was built or run. The sphere figures are the row's measurement; the torus figures are the row's "likely, not run".
- That the 1e8 m body is told a kernel defect: read from `CertCheck::MappedSource => Ending::Residual(Unsized::Defect)` in `crates/geom-brep/src/certify.rs`, not from a rendered refusal.
- Whether a construction certifies cutter or scaffold edges past 1 km on purpose (B's flag; mine too). Such an edge is a certificate read outside the premise, so I would treat a hit as a finding, not as a reason to drop the ask.
- That every finished face has a cover. A granting door refuses an uncovered entity (`crates/topo/src/boxes.rs`), and under change 2 the box is one. Its standing by cause is `work/boxes/face-uncovered-stands-as-a-limit-whatever-its-cause.md`.
- Cost on the box-scalar lanes. PR 7088 reports the placing door's ask starving `m10_3::a_terminal_sliver_refuses_naming_its_predicate_and_is_not_refined`. A face-cover read at every finish wants measuring.

**What the build would meet.**
- **The gate's order.** `structural_via` runs `validate_closed` before the box, so links are proven before a face is walked. The face ask stays after it and before tier 3's first geometric verdict.
- **The placed read.** `body_reach_at` maps each carrier so that it matches the image "to the bit" (`a_placements_reach_is_its_images_to_the_bit`). `Cover` has no moved form, and reading a face cover along the motion's axes rounds differently from the image's own ask. Decide whether the pin is on bits or on the verdict.
- **Second readings that fold away.** `edge_coordinate_reach_below`, `sampled_reach` with a measure and `cover_is_exact` become the inner bound of one two-sided read.
- **Layering.** The certification's ask sits in `geom-brep` and can read `geom`'s `Curve3::cover_over` hulled with the two ends; `topo`'s `edge_cover_over` is the same reading one crate up.
- **`boxes.rs`' door inventory** says a looser arc arm is the session box's unsound direction. Under change 2 that line reverses, with its `cover-kind-match.readers` entry.
- **`EdgeCurve::carried`** (the isometry's path) makes an edge without certifying; the move's own ask covers it.

**Rows this touches.**
- `crates/mesh/tests/newell_probes.rs` `probe_d_huge_offset_tiny_face` and `r2_cert9_probes.rs` `p6_reproduce_the_huge_offset_red_digit` return to the box's refusal.
- `crates/editor-core/src/eval/framed.rs`'s header states the hull argument as if the box had admitted the operand's hull. It becomes true under the recommendation.
- `work/onehome/a-gathers-box-ask-reads-each-image-in-its-computing-frame.md` and the far-apart-copies fork read the same predicate under a map. The README bullet avoids "world" so that fork can name the frame.

**Housekeeping.** I changed nothing in the repository. My only files are scratch copies under `~/.local/share/cad-work/designer-sbr-a/`.
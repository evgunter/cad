## For Ev

**Recommendation.** Three changes to the session box. You have in effect already ruled on the first.

1. **The box is a predicate of the region a piece of geometry occupies, read through its cover**, faces' regions included. (A *cover* is the one outer bound of the region an edge or face can occupy, `[one-way.places]`.) Today it reads a body's vertices and edges only. On PR 6596 you approved this same correction for levers ("i agree that there's only one right solution here"): the consumed extent reads "a face's region, not its boundary". That PR left the session-box bullet, which has the same flaw, to this row. *sure*
2. **Its sound direction is admission.** A body is admitted only where its cover lies inside the cube, so "admitted" means "no point past the box". That is the premise the run's admission rule on (ε, K) rests on. Today it is the reverse: only a point proven past the box refuses, and everything else is admitted. *likely*
3. **It is asked where a region comes to exist, before anything is decided on it.** *likely*
   - A vertex's point and a curved edge: at the construction tier (the Euler operators, and an edge's certification before it reads a residual).
   - A body's faces: at the finished-body gate (the check every operation's result passes).
   - A body stood somewhere by a map: at the door that stands it (a *placing door*), as now.

   This is `[tolerance.units]`' "rejected at construction". It is also the cheap check you deferred on PR 6329: "B seems good unless we actually find it causing a problem, at which point we can think about our strategy for making the checks cheap". The third defect below is that problem.

**What is wrong today.**
- **A curved face's far side is not read.** A sphere of r = 50 centred at (0, 0, 975) reaches 1025 m. It is admitted when its poles are on y and refused when they are on z, because only then is its top a vertex.
- **Edges made later on that far side are never asked.** A placed copy is asked once, at its placement. A split or a concave fillet then makes edges on the unread bulge.
- **A body far past the box is refused by something else first.** A 1e8 m cube fails an edge's certification by one ulp before the box is asked, and which refusal it gets depends on how a line is spelled.

**Premise check.**
- **The bullet's operative words were never put to you.** "Read from below" and "one it admits may still reach past it" were written in PR 6608, a fix for red tests with no `[ev]` and no comment from you. What you ratified on PR 6329 read "one predicate over a body's consumed extent".
- **The same PR 6608 pass deleted the sentence recording your choice of who asks**: "A kernel-bug panic that reads a margin on derived geometry takes 'in the box' as its premise." No code acts on it.
- **The first two defects are one.** PR 7088's rule says a result inside its operand's hull stands where the box admitted the operand. That holds only if "admitted" means the whole region is in the cube. With changes 1 and 2 it does, and the split and the concave fillet need no new door. *likely*
- **The third is a different one.** The box is asked at the end of a build, but the build's own certificates lean on it from the first edge.
- **Only sphere, torus and spline faces can bulge.** A plane, cylinder or cone face is ruled, so its farthest coordinate lies on its boundary, or at a cone's apex where the face holds it. *sure*

**Worked examples.**
- **The sphere above.** Its cover is the ball, top at 1025 m. Poled on y or on z, it refuses at −25.
- **Torus, r = 400, R = 450, centred at 250 m.** The gate reads about 806 m off its seams; its cover reads about 1100 m, so it refuses. (The row's own figures, not run.)
- **The 1e8 m cube.** Its first vertex refuses in the box's sentence. Today, at the default ε, its first edge refuses on a check whose ending names a kernel defect (`CertCheck::MappedSource`, read in the code, not run).

**What it reads: the final states.**
- **A (recommended): both bounds, admitted on the outer one.** The cover bounds the region from outside. Points known to be on the body (a vertex, a point of an edge, a sphere's pole its face holds) bound it from inside.
  - Cover inside the cube: admitted.
  - A point past it: refused as past the box, the input's fault.
  - Between the two: refused as a kernel limit (`Refusal::Limit`), naming the face.
  - One recourse on both refusals: move it toward the origin by the margin. This is how `LeverBounds` is already read: act only on a verdict both bounds share.
  - **What it leaves:** a spline face within its control net's slack of the box's edge refuses though it may lie inside. That is not pure looseness. Every evaluation of the face passes through those control points, so they are coordinates the rounding premise is about. *likely*
- **B: the outer bound only.** One arm fewer. Its refusal says "past the box" of a body that may be inside it, so a kernel limit is told as the input's fault. A is one step from B, and back.
- **C: the inner bound, with faces' far points added** (the open row's sketch). It keeps "refused means a point past". A body it admits may still reach past wherever a cover is loose, so the premise stays unproved. Not recommended.

**Who asks: the final states.**
- **W1 (recommended): the construction tier, the gate, the placing doors.**
  - **True at all times:** every vertex and every certified edge of any body, finished or not, is in the box. So every residual and every kernel-bug panic inside a build reads in-box geometry, and its story is true.
  - **Why there:** in-box is the pose that licenses treating f64 as exact, and `[simple.conditioned]` decides a pose before the formula that leans on it.
  - **Why cheap:** the cube is convex. A line edge between admitted vertices is asked nothing; a vertex is three comparisons; a curved edge is one box of its cover.
  - **What it leaves:** a door's inputs that are no region of a body (a profile's 2-D coordinates, a far axis) can be decided on before any vertex is made. A profile is a region too and should be asked at its own door; I did not check that it has one.
  - **Reversible:** remove the tier's ask and the gate remains.
- **W2: a failure inside a build asks the box before it ends** (one designer's form of that deleted sentence on PR 6329; unbuilt). It costs nothing on the passing path. Every refusing or panicking site has to remember to ask, and until one does a far body passes or fails its certificates by rounding.
- **W3: the gate and the placing doors only** (today, and the "B" you chose on PR 6329 until it caused a problem). A far body's refusal depends on how a carrier is spelled, and `[loud-failure]`'s "which kind a failure is is always plain" fails for it.
- **W4: each building door asks its output's extent up front** (declined on PR 6329). It states once per operation what the construction tier sees once for all of them.

**Ratified text I would change.**
- **The README's session-box bullet**, to: "The session box is one predicate over the region a piece of geometry occupies, read as its cover, never a carrier's stored terms: the box bounds each coordinate (`[tolerance.units]`), so it is a cube. A region is in the box only where its cover is. One with a point past it is refused as past the box, and one whose cover alone reaches past it as a limit. The door at which a region comes to exist asks it, before anything is decided on that region: the construction tier asks it of a vertex and of a curved edge, the finished-body gate of a body's faces, and the door that stands a body by a map of that body where it stands."
- **`[tolerance.units]`: no change.** "Rejected at construction" becomes true as written.

**Two smaller points.**
- **The doors that ask where a body stands are a hand-kept list** (a placement, a sketch frame, a lift, a mirror: those whose result can leave its operand's hull). It missed the mirror once (PR 7088). Changes 1 and 2 make its argument sound. Asking every value stood by a map when it is made would retire the list for one more read per operation. I lean to the uniform ask. *unsure*: its cost on the interval lanes is not measured.
- **The predicate is of a region in a frame.** Which frame an operation over far-apart copies computes in is its own open fork (`work/onehome/an-operation-over-far-apart-copies-has-no-conditioned-computing-frame.md`). Nothing here settles it.

**Confidence.** Change 1 *sure*. Change 2 and state A *likely*. W1 *likely*: its cost on the interval lanes is unmeasured, and I audited no door for decisions it takes before it makes its first vertex.

## For the orchestrator

**Errors in the brief.**
- The brief says the README "ratifies" the from-below predicate. `git log --all -S'read from below off' -- crates/geom-core/README.md` gives `47e9bbfde` (PR 6608, 2026-11-09), an implementer's fix pass. PR 6608 has no issue comments, and its body does not ask Ev. The PR 6329 text is in `31576a6c6`.
- `47e9bbfde` also removed "A kernel-bug panic that reads a margin on derived geometry takes 'in the box' as its premise", Ev's split-2 answer (PR 6329, 14:31Z comment). CLAUDE.md has a design change to a ratified README page wait for Ev; this one did not. I treated both edits as unratified.
- The neighbouring bullet (the consumed extent) was changed to "a face's region ... each read as its cover" by `d0c17dee4` on PR 6596, which Ev approved in words. The brief does not mention it, and it is the strongest evidence for change 1.

**Provenance I could not check.** `[tolerance.units]`' "rejected at construction" and "no coordinate larger in magnitude" resolve only to the shallow graft (`dcd012f6b`, 2026-11-01). `work/shell/the-session-box-is-not-enforced-at-construction.md` calls them ratified since v0; I took that as given.

**Claims I did not verify.**
- Nothing was built or run. The sphere figures are the row's measurement; the torus figures are the row's "likely, not run".
- That the 1e8 m body is told a kernel defect: read from `CertCheck::MappedSource => Ending::Residual(Unsized::Defect)` in `crates/geom-brep/src/certify.rs` and the README's at-a-build column, not from a rendered refusal.
- That every building door mints its vertices through `topo::construction::Construct`. `Body::add_point` has about 59 references in `crates/topo/src` and I audited none. The raw builder, import adoption and `map_isometry` write points outside the Euler operators; under W1 they still meet the gate or the move's own ask.
- That a finished body's every face has a cover. `FaceUncovered` exists (`crates/topo/src/cover.rs`). By `boxes.rs`'s own rule a granting door refuses an uncovered entity, and under change 2 the box is a granting door. Its standing by cause is `work/boxes/face-uncovered-stands-as-a-limit-whatever-its-cause.md`.
- Cost on the box-scalar lanes. PR 7088 reports the placing door's ask starving `m10_3::a_terminal_sliver_refuses_naming_its_predicate_and_is_not_refined`. A face-cover read at every finish, and an ask per mint, want measuring before W1 is specced. If that cost changes what is feasible, it is a separate statement from the ranking.

**What the build would meet.**
- **The spline arm of `face_cover` is the whole control net.** A small face trimmed from a large net would refuse. The row's "control net clipped to the face" is needed first, or state A over-refuses.
- **The cover's trim constraints are decisions** (`curved_face_placement`, the chart windows), taken before the box is known. They narrow only on a definite `Out`; check that holds in the admitting direction for geometry past the box.
- **The inner bound on a face** needs a point proved on it. `face_cover_toward` already places a sphere's poles; a definite `In` there is such a point. A torus's crest would be new.
- **The gate's order.** `structural_via` runs `validate_closed` before the box, so links are proven before a face is walked. The face ask must stay after it and before tier 3's first geometric verdict.
- **The placed read.** `body_reach_at` maps each carrier and re-takes its cover so that it matches the image "to the bit" (`a_placements_reach_is_its_images_to_the_bit`). `Cover` has no moved form. Reading `Cover::box_in` along the motion's axes rounds differently from the image's own ask, so decide whether the pin is on bits or on the verdict.
- **Second readings that fold away:** `edge_coordinate_reach_below`, `sampled_reach` with a measure, and `cover_is_exact` are a reading of an edge's region beside its cover. Under state A they become the inner bound of one two-sided read.
- **`crates/topo/src/boxes.rs`' door inventory** says a looser arc arm is the session box's unsound direction. Under change 2 that line reverses, with its `cover-kind-match.readers` entry.
- **W1 by type where it can be:** certification is already the only way to build an `EdgeCurve`, so an ask inside `EdgeCurve::certify` makes an out-of-box certified edge unrepresentable. `EdgeCurve::carried` (the isometry's path) bypasses it and is covered by the move's ask. `Construct::mvfs` documents "no preconditions" today.

**Rows this touches.**
- `crates/mesh/tests/newell_probes.rs` `probe_d_huge_offset_tiny_face` and `r2_cert9_probes.rs` `p6_reproduce_the_huge_offset_red_digit` return to the box's refusal under W1.
- `work/onehome/a-gathers-box-ask-reads-each-image-in-its-computing-frame.md` and the far-apart-copies fork read the same predicate under a map. Word the README bullet without "world" so that fork can name the frame.
- `crates/editor-core/src/eval/framed.rs`'s header carries the hull class. If the uniform ask is taken, that paragraph goes.

**Housekeeping.** I changed nothing in the repository. My only files are scratch copies of PR bodies and this report under `~/.local/share/cad-work/designer-sbr-a/`.
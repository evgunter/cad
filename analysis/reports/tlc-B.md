## For Ev

**Recommendation (likely): nobody chooses a bound.** Three things become true of the code.

1. **The turn-lever doors return the region, not a lever.** `wedge_reach` and `edge_reach` hand back the consumed extent known from both sides, as the README's one extent type (`geom_brep::Reach`): points known to lie on it (the inside read) and its cover (the outside read). Each decision reads its lever from that region at the place its own datum is read.
2. **The lever stays two-sided until the decision funnel.** `Margin::levered` and `Margin::sagitta` take both bounds. The funnel serves a verdict only if it holds at every length between them. For a margin that grows with its arm (all of these), that means a definite sign is proved at the bound from below and a Zero at the bound from above. Anything else is refused.
3. **"The bounds disagree" is an in-band reading of one scalar decision.** The existing straddle door (`geom_core::k_stats::straddle`) mints one `Indeterminate` whose margin is the hull of the two readings. It leaves each composite verdict through the in-band arm that verdict already has, tagged with rung and station. No verdict gains a variant.

Terms: *bound from below* is a length the region certainly reaches (read off its points); *bound from above* is a length it certainly does not pass (read off its cover). A *composite verdict* is one answer folded from several scalar decisions (the wedge fold: arm gate, class, sign, pairing, bend, end). *Site* is the point or line a lever's distance is measured from.

### Premise check

- **"Which bound does this caller need" is the wrong question.** The ratified sentence's three cases are one rule seen from the consumer's side. A consumer that acts only on definite signs accepts exactly the same readings whether it reads both bounds or only the one from below, and likewise for Zero from above (sure: the margin is `x·L` with `below ≤ L ≤ above`). So the 41 callers need no classification, which is the step four reviews found wrong. The classification only matters when one bound is missing.
- **The composite verdicts do have a home for a disagreement.** Each already carries the funnel's refusal: `MustCarryVerdict::InBand`, `PairRefusal::InBand`, `WedgeVerdict::InBand`, `classify_shared_rim`'s `Err(Indeterminate)`, `SeamUnread::Lever`, `PinMiss::Escalated`. A new "bounds disagree" variant would say "in band" twice (sure, read in the tree).
- **"NURBS faces have no cover" contradicts the tree.** `Surface::cover` gives a spline its control net and refuses only a poisoned net (the placeholder) or non-finite data. The 1.24 million poison reads need their refusal reason read before anything rests on them.
- **The rule is already spelled three ways for a scalar**: `topo::validate::decide_agreed` (its own straddle row), `ClassRead::shown_at_both` in `intersect.rs` (a straddle under the predicate's own name), and `classify_seam`'s bracketed spline arm (escalates as `dihedral_arm`). The two-sided region also exists twice, privately (`ScopeRegion` in `offset_axial`, `FaceRead` in `wedge_reach`). The missing part is one home.

### The answers as final states

- **A, recommended: the funnel serves each sign from the bound that proves it.**
  - A sign compared and returned as a bool (`stands_on`, `sheets_part`) is right with no change of its own, because the `Sign` it compares was only served where both bounds agree.
  - The class tokens (`Corner`, `Tangency`) carry the two-sided folded arm, and `classify_seam`'s spline bracket becomes the general case.
  - `LeverBounds::below()` and `above()` are readable only in the extent home and the funnel, held by a gate whose pins only fall.
  - Still possible: a door building its region from too little geometry. Review holds that, as today.
  - Reversible by re-exposing the two accessors.
- **B: consumers take both bounds and each row inside picks by hand**, as `cone_cylinder_section` does today. Fewer types change. A row that picks the wrong side still compiles, so the reviewed failure moves from 41 callers to about 15 rows.
- **C: each caller picks one bound** (the row as filed). Not coherent: one must-carry call needs its Smooth premise proved from above and its determinate bend from below.
- **D: a "bounds disagree" variant on each of the seven verdicts.** It states one fact twice.

**Worked example** (ε = 1e-9, K = 10; an imported face bounded by one full circle, below 1.4e-3, above 2):
- `sin θ = 4e-10`: 5.6e-13 at below, 8e-10 at above. Both Zero, so Smooth is served.
- `sin θ = 4e-9`: Zero at below, 8e-9 at above, which is in band. Refused once, as `dihedral_wedge`'s straddle at that station.
- Today the second case stores a seam on a join that opens 8ε across the face.

### What is read where (item 3)

- **Faithful to the README:** the wedge and its bend read the region from each station, where their angle and curvature are read. The joint turn reads it from the joint. Today both read from the middle of the edge's span, and the turn takes the shorter of two edges' reaches, each from a different middle.
- **A plain point at the station is not enough.** The lever counts the faces' run along the edge, and from an end station that run is about twice what it is from the middle. That doubles the over-read the open row `the-wedge-reach-counts-a-faces-run-along-its-edge` reports, on exactly the definite-sign claims.
- **Ev's to decide; I recommend the site be the line the datum turns about** (likely). A dihedral at a station turns about the edge's tangent line there, so a face point moves by `sin θ` times its distance from that line. An azimuth at a pole turns about the chart's axis, as `joint_arm` already reads away from the pole. `Cover::distance_range` already takes a line.
  - Example: a strip 4 long and 0.05 wide at `φ = 15ε` to its neighbour reads 30ε from the edge's middle and about 60ε from an end station, both a sharp corner. From the hinge it reads 0.75ε, the tangent joint the profile declared.
- A datum with no position (two plane normals' cosine in `stands_on`) keeps the edge's middle.

### The other items

- **No cover (item 2).** With no bound from above, nothing proves a Zero, so a decision whose reading from below is not definite refuses. Definite signs are still served from below, so certification is unchanged. This follows from the rule, not from a caller's choice.
- **Site mint (item 4).** The door holds the surface and every half's carrier and span, so `Surface::face_region` over those edges' covers is the cover of the face as the door leaves it. That function was written for a face not yet built.
- **Items 5 and 6.** The two bounds are built through one constructor that keeps below no greater than above. The lift-free cover is a looser cover, and any cover is a sound outside read, so it stays.
- **Bound from below (item 1).** It should read each boundary edge's own reach (exact on a line and a circle arc, sampled otherwise; `edge_reach_bounds` does this already), not only vertices. A sphere face's bulge past a small rim stays between the bounds, and a decision that straddles there refuses. This moves definite verdicts toward deciding, which PR 7141's acceptance does not cover.

### Faithful, or yours

- **Faithful:** composites take the extent; each scalar is served from the bound that proves it; a disagreement goes out through the in-band arm; the door holds the region; the site mint's cover; below no greater than above.
- **Yours:**
  - The lever sentence restated as the proof rule: "a definite sign is read from below and a Zero from above, whoever acts on it; where the two disagree the reading is in band".
  - The site as a line: "far distance from that point" becomes "from the point or line its datum turns about".
  - Edges in the bound from below.
  - The joint turn's region: I lean to the walked face alone rather than both edges' neighbours (unsure).
  - Any class of valid bodies the measurements show newly refusing.
- **Provenance of the text I would change:** both sentences are agent-written and were approved on PR 7141 ("sounds great! i am less sure about the records thing").

### Measure first, and judge against

1. **Why the 1.24M reads have no cover**, by refusal reason. Judged against `Surface::cover`'s contract.
2. **A alone, same site and same below.** Per predicate and door, count the Zeros served today that refuse, and the bodies that newly refuse. Definite verdicts must be bit-identical, and any difference is a conversion bug. Every newly refusing class is listed for you (PR 7140).
3. **Site**: middle against station point against hinge, on the same reads, with verdict flips each way. Judged against that row's table: the hinge must agree with the profile.
4. **Edges in the bound from below**: the above/below ratio again, the definite verdicts newly served, and the cost on the symbolic lane.
5. **Door against rest**: edges a door decided that tier 3 decides otherwise. Judged against zero.
6. All of it on the whole corpus, at 1e-6 and 1e-12, and on the interval lane. The figures so far are a partial f64 run at one ε.

Confidence: A likely; the equivalence of one-sided and both-bounds reads sure; the in-band arms sure; the hinge likely; the turn's region unsure.

## For the orchestrator

**Not run.** I executed nothing and every figure is the lane's. I read `wedge_reach.rs`, `pcurves/decision.rs` (the turn), `dihedral.rs`, `edge_wedge.rs`, `extent.rs`, `cover.rs`, `period.rs`, `stands_on`, the head of `sheets_part` and the `LeverBounds` consumers in `intersect.rs`. I did not read `run_checks` whole, `must_carry_description`, `classify_shared_rim`'s body or `walls_inside`'s decisions, so the lane's classification of those stands unchecked. Under A it no longer matters.

**Errors or gaps in the brief**
- Item 2: see the premise check. `face_cover_in`'s `Nurbs | Approx` arm returns the carrier's net cover. My guess is placeholder nets mid-construction, which I could not check.
- "Nothing says the two bounds disagree" is true of the variants and false of the types.
- The brief does not mention that `classify_seam` already brackets an arm from both sides, or that `cone_cylinder_section` already takes `LeverBounds` and picks per row. Those are answers A and B in miniature.
- `JointSides::lever` takes the lesser of two reaches read from two different edge middles, so the turn's lever is read at neither its datum's point nor one point.

**Claims I could not check**
- That `Cover::distance_range` from a line is tight on every piece kind.
- That a line site is affordable at the symbolic tier. The module doc of `wedge_reach` gives that tier's cost as the reason for vertices only.
- That every margin built on a turn lever grows with its arm. I checked `levered`, `sagitta` and the `Period::decide` meter. A closure handed to `decide_at_lever` can be non-monotone, and then its two ends do not bound it. Under A the two-sided lever should enter only through the `Margin` doors.
- The sum rows (`decide_across`: datum plus swing) prove both verdicts from above. A's funnel has to evaluate both ends rather than assume growth there.

**Shape notes for whoever builds A**
- `LeverBounds` belongs in `geom-core` beside `Lever`, since the `Margin` doors and `k_stats` read it. That is a move with no design content.
- One straddle spelling: the funnel mints it, under the `<predicate>_straddle` row convention the tree already uses. `shown_at_both`'s same-name straddle and `classify_seam`'s `dihedral_arm` spelling fold into it.
- `MATERIAL_PAIRING`'s doc argument (the margin lies between `√(arm² − ε²)` and the arm) assumes one arm, so re-derive it for a Zero proved from above and a pairing proved from below.
- "The demanded set and the stored set are one set" survives: the constructor and tier 3 read the same region at the same stations.
- A missing bound from above reaches the user as an anonymous invalid margin. `FaceUncovered` holds the reason at the door and the `Indeterminate` cannot carry it. I left this alone because D4 poison is the ratified spelling, but the recourse text will name no lever that reaches it.

**Off the question**
- `LeverBounds::Between` has no below-no-greater-than-above invariant. Only `WedgeReach::reach` folds it by hand; `edge_reach` and `ScopeRegion::reach_from` do not.
- In the PR tree, `edge_wedge.rs` (both doors) and `singular_turn`'s `Period::decide` still read `.below()` for every-sign decisions. This is the stopped work, noted so it is not merged as is.
- The honeone row and this one should be one design. Answering item 3 with a point at the station while that row is open makes its defect worse, so I would not dispatch them separately.
- Moving the site or the bound from below ends "bit-identical to the old lever" for certification, and that differential is not the one PR 7141 pre-accepted.

**Sequencing, not ranking:** A with the site and the bound from below unchanged is the smallest unit, and its differential moves only toward refusing. The site and the edges-aware bound from below are separate units, each with its own differential.
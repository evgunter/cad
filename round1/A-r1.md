## For Ev

**Recommendation (likely).** Each local reading should be decided once, as a fact of the entity it belongs to. Tier 3 decides it and the verdict keeps it. The embedding question reads it and decides only lengths. Concretely:
- **Check 4 decides an edge's wedge over the whole edge, not at stations.** It reads enclosures over pieces of the edge with the folded lever: the least of the faces' reach and their curvature radii. The verdict keeps, per edge, its class (crease of a given turn, seam, end) and the lever the class was decided at.
- **What has no edge gets its own per-entity fact.**
  - A vertex where faces meet only at that vertex: how they leave it, levered at their reach from it.
  - A face at its chart singularity (pole or apex): check 6 already places the singularity at a vertex, and that vertex is the reading's subject.
  - A face's own turning: a per-face fact.
- **The question takes the verdict as its input, never re-classes an entity, and picks its argument by the class.** A crease gets the first-order separation; a seam or an end gets the second-order arm; an edge tier 3 refused never reaches it.
- **Near a shared entity, every cell reading is one length**: how far the face has got from the separator at that entity's lever, checked against K·ε. The lever is read from the verdict, never computed per cell.

This is the literal form of your PR 7095 ruling ("one reading for both"), and it extends what you approved on PR 6893: the strict question takes tier 3's verdict, and `AtRestBody` keeps tier 3's per-entity facts.

**Premise check: four corrections.**
1. **There is nothing for the question to read yet** (sure). `clearance::embedded` takes a bare `Side` (body plus faces) on main, on the PR and on its measure branch. `AtRestBody` keeps only check 7's volume certificate (`KeptSign`), not a single wedge. The at-rest gate does not yet ask the embedding question on main (the P0 row is open). So "reads tier 3's facts from its verdict" is unbuilt on both sides. PR 7329 re-decides because no fact exists to read, not by choice.
2. **The departure is the wedge on only one of its arms** (sure).
   - With the other face's carrier as separator, its sine is check 4's.
   - With a plane through the edge, or the graph separator, the sine is the face's angle to a surface the engine built: a proof device, not a wedge.
   - On every arm, what `sign_on` actually establishes is a separation. The separator value ψ at a point is at least sine × (distance from the edge), so sine × arm is a lower bound on how far the face has moved off the separator at distance `arm`. That is a length, which answers your first question on 7095.
3. **The apex refusal under the fold is the per-cell pattern again** (likely). The fold is read as an enclosure over the cell, and a cell holding the apex reaches 0. Tier 3 escapes only because a station is a point. A lever per entity escapes for the right reason: near the apex, the apex vertex's reading governs.
4. **Item 4 of the brief is half right** (sure). Poles and apexes do have a tier 3 fact: check 6 places every chart singularity at a vertex. Until 2026-11-13 the DESIGN text named it among the facts "never re-decided over a cell". PR 7329 instead reads it off a struct's shape (`Along::collapsed`). Vertex-only meetings and a face's own turning have no tier 3 fact.

**Why the lever must be the fold, not the reach** (sure). Within a cell, the strip where the two faces are within the band has width K·ε/sine. The test sine × L ≥ K·ε says that strip is narrower than L. L must be the scale over which the first-order angle actually governs the gap, and on a curved face that scale is its radius.
- Worked case: a plane through a cylinder's ruling, radius r = 1e-3, angle θ = 5e-7.
- The plane cuts the cylinder a second time 2rθ = 1e-9 = ε from the edge.
- The PR reads a departure of 5e-7 m; check 4 reads tangent. The geometry really is ambiguous, and check 4 is right.

**The options, each as a final state:**

**A (recommended): per-entity facts in tier 3, which the question reads.**
- What becomes true:
  - Each entity's angle is decided at one site, with one lever.
  - The question cannot class an edge differently from check 4, because it never classes one.
  - It can still refuse where tier 3 passed: a cell's sine covers points off the edge, so its lower end can only be lower. Splitting resolves that. It can never accept where tier 3 refused.
  - Every band decision in the question is a length against K·ε, which matches "ε is a maximum deviation from true geometry".
  - The reach's site (ONEROUND's fork) and PR 7262's two-sided lever bounds land once, in tier 3. The verdict keeps the lower bound, and the question inherits whatever is settled. My answer does not depend on how the site is settled.
- The five other cell-levered margins split two ways:
  - `_corner_departs` (a corner's turn) and `_monotone` (a face's own turning) are angles. They read the per-entity lever.
  - `_column` and `_graph` are proof devices (is this curve a graph over an axis?). They want a sign, not a margin.
  - `_cusp` reads tier 3's `End` from the verdict, which 6893's plan already says.
- What it costs:
  - Check 4 over pieces of the edge needs an enclosing scalar at `f64`. The engine already lifts the body this way.
  - A new tier 3 vertex check.
  - `embedded` takes a verdict, and the cavity door changes with it.
  - Constructors that read an edge's wedge (the must-carry rule, the blend's convexity) get the whole-edge reading too. That is a gain: no pinch between stations.
- Reversible: moderately. The verdict's fields are additive.

**B: the same reading at two sites.** PR 7329 plus the fold: the engine calls `folded_lever_arm` per cell.
- Pros: a smaller change, and no new tier 3 checks.
- Cons:
  - One fact is decided twice, kept in step by a shared function.
  - The split between stations and the whole edge stays, so a body can pass tier 3 and refuse at the gate on tier 3's own margin.
  - Every cone that owns its apex refuses unless a new ε concession is made within the apex's band. That would be yours to rule, and it is the sliver's shape again.
  - Vertex-only meetings and a face's own turning keep angle decisions that live only in the engine.
  - "Never re-deciding one over a cell" would have to be retired.

**C: the engine owns the reading and check 4 reads it.** Rejected. Constructors need an edge's wedge without asking the global question, and tier 3 runs first.

**Ratified text I would change.** `[topology.tiers]`, the finished-body paragraph:
- Keep "never re-deciding one over a cell" and "one reading for both". Option A makes both true.
- Replace "an angle metered at the reach of the faces it governs" with: the lever tier 3 decided the entity's fact at, which is the faces' reach folded with their curvature.
- Add that the question's reading near a shared entity is a length: the separation a face reaches at that lever.
- Add the vertex fact to tier 3's list.

Provenance:
- "one reading for both" is your ruling on 7095.
- "never re-deciding one over a cell" is agent wording that tightened a list of named facts (commit `482212540f`, your "approved, broadened" on 6893). The list it replaced named the chart singularity and the wedge ends.

**Confidence:** recommendation likely; premise points 1, 2 and 4 sure; point 3 likely.

## For the orchestrator

- **Already-approved ground.** `work/embed/program.md` and `the-at-rest-gate-certifies-embedding.md` (around line 395) record Ev's 6893 approval:
  - `embedded` takes a tier-3-built value in place of `Side`;
  - that value carries each edge's `WedgeVerdict` and each face's singular vertex (today dropped after `place_on_boundary`).

  Option A adds to it: the lever per edge and per vertex, check 4 over the whole edge, and the vertex fact. Lane B/C should be told this fork touches its spec.
- **Errors in the brief.**
  - "A finished solid passes tier 3 and then the embedding question" is not true on main. The gate does not ask the question yet.
  - Item 4 overlooks check 6's chart-singularity fact.
  - "Re-decides the wedge" is true of the carrier arm only.
- **Assumed, not checked:**
  - that every `WedgeVerdict::InBand`, `Mixed` or `Lamina` edge produces a tier 3 finding, so it never reaches the question;
  - the cost of a whole-edge check 4 at `f64` through the lift;
  - whether census/link code (`crates/topo/src/link.rs`, vertex links) already holds a reading the new vertex fact could reuse.
- **Not checked by measurement:** the claim that under A the question can refuse where tier 3 passed but never accept where tier 3 refused, beyond the argument given. The interval lower end of a cell's sine is at most the edge's sine over the cell's stretch of the edge.
- **Off-question.** `wedge_reach.rs`'s module head on the PR says the curvature arm caps the lever "in any case", which is false for the engine. `folded_lever_arm`'s "route new callers through this function" is left standing. Both reviews found these already, and under A both become true again.
- I read no other designer's work and nothing under `/root/.embed-private`.

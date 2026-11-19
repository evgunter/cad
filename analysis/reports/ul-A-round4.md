**Reconciliation round 4: closing answers.**

**1. The struct-level state is met, stated as the three-way test.**
- A record that holds what the lever is read from derives it and never stores it beside.
- Otherwise, a record with one fixed pivot may hold the lever.
- Otherwise, it holds its region.

B's five records come out the same under it: `SsiDomain` holds its ball, the sector's `arm` and `SectionOval::extent` are derived, `Piece::Segment` loses its field, and editor-core's `Arm` goes.

B's test is a two-way cut (a reading record keeps its arm; "any other record holds its region and no arm"). It has no middle case, so it could change structs the three-way test leaves alone:
- **`topo::bound_side::BoundSupport::Bisector { arm }`**: a bisector bound is "a direction with no point behind it". It has no region to hold, and its `arm` is where a sub-sector's corner arm lives. Under the three-way test it stands (no geometry held, one fixed vertex).
- **`BoundSupport::Conic { extent }`**: the edge's `edge_extent_inf`, held on a bound whose carrier the bound does not hold (`arc` is `None` for a spiric or a spline). It stands for the same reason.
- **`topo::star::walk::Sector.arm`**: this struct holds no bounds itself (the `Star` does), so it is a method on the star or it stands under the middle case.

Whether B's "reading record" already covers a bound read at its vertex is ambiguous in B's wording. If it does, there is no difference at all. Either way a unit settles these three; none is Ev's.

**2. The README: forward the appended clause. The round-2 sentence as worded is not acceptable to me, and that is more than wording.**
- The sentence's second half ("A record that describes geometry for verdicts still to come holds its region, never a lever") is wider than the state we met. Read literally it outlaws the `BoundSupport` fields above, since a sector bound describes geometry for verdicts still to come and holds a lever. I wrote that sentence before reading those structs.
- The clause says exactly the part that was once misapplied: "…read at the same one, so a record that verdicts read from different points holds its region, not a lever". The derive-don't-duplicate half is already `[one-way.places]`' "a method on it, never a second reading".
- A new sentence is acceptable if it is narrowed to the three-way test. Between that and the clause the difference is only wording and placement. Acceptable alternative text:
  > "A lever belongs to a reading, read from the point that reading's datum is pinned at: a record that holds the geometry derives its lever, and one that verdicts read from different points holds its region, not a lever."

## For Ev

Unchanged from round 3.

## For the orchestrator

**Remaining differences from B, as of B's round-3 report.**
- **The records text.** B forwards the round-2 sentence as a third sentence. I forward the separable clause on the existing same-point sentence, with the narrowed sentence above as an acceptable alternative. The un-narrowed sentence over-reaches (`BoundSupport`).
- **Three structs the two tests may read differently:** `BoundSupport::Bisector.arm`, `BoundSupport::Conic.extent`, `star::walk::Sector.arm`. A unit's call.
- **Nothing else.** B's "the sector's `arm` and `SectionOval::extent` become regions" and my "each is a method on what the struct holds" are the same state: the sector derives from its bounds, the oval from its carrier's cover.

**If you forward B's `## For Ev` instead of mine,** two lines differ in substance:
- Its sentence 2 should be replaced by the clause or the narrowed sentence.
- Its `SectionOval` bullet ("becomes the oval's ball") should say the lever is read off the carrier the oval already holds, as the conic beside it is, so no field is added.

**What I read for this round.** B's round-3 answers and records paragraph. For the `BoundSupport` point I used the enum as read in round 3 (`Chord`, `Extent`, `Conic`, `Bisector`); I did not trace who constructs `Bisector.arm` or `Conic.extent`.

**Carried, unchanged from round 3.**
- **Not checked:**
  - no builds were run and nothing was measured;
  - the sector's `min` over its bounds is not traced through `Star::refine`;
  - a spiric's `cover()` per `lever()` call is unmeasured;
  - the scan of about 34 lever fields is a regex heuristic;
  - about 40 of the 66 sites are known only by their comments;
  - `pcurve_cache`'s focal β, `unit_sign` and `coincide::replay` are unsure.
- **To file or route:**
  - `Lever::curvature` has no callers (grep);
  - the `Lever::max` doc fix;
  - `plane_misses`' stored-origin pivot;
  - `sagitta`'s and `over_lever`'s bare arms and `metering_arm`, with "Hidden levers";
  - whether `intersect`'s apex rounding charges are on the site-rounding row;
  - one pass over the lever fields against the three-way test once Ev answers.
- **The filing row is stale:** title 74, body 63, tree 66; sites gone and new as listed in round 3; no fixture family.
- **Sequencing:**
  - the fixtures and the pure bit-movers can go before Ev answers;
  - the `geom-core` point reads go with `oneround/lever-doors`;
  - the verdict-moving sites and the records wait for the answer.
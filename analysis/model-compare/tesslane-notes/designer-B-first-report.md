## For Ev

**Recommendation (likely).** Do not build an arm per (chart, boundary) pair. One lane meshes every non-planar face: today's `trimmed` lane, with three things it now holds per arm factored out.

1. **The face's domain is read from the body, every loop.** A loop's *lift* is its stored chart curves placed end to end in the face's chart (`Body::loop_lift`). The *domain* is the outer loop's lift minus the rings'. The lane triangulates that region: boundary points are the shared chord points, interior points are a lattice, inside/outside is the planar lane's even-odd count over all loops.
2. **A chart contributes one table row** (cylinder, cone, sphere, torus, spindle sheet, spline): lattice steps for a target, the per-triangle certificate, the metric the triangulation runs in, its singular rows (pole, apex), its outward normal.
3. **A boundary curve contributes three facts about its stored image**: its value at a parameter, a box over a span, and a speed bound per chart axis over a span. The chord pass then has one rule for every face: chord count ≥ speed × span ÷ the face's lattice step, per axis.

That is charts + images, not charts × images. A face is then never routed by what kind of curve bounds it.

**Premise check.**
- The question divides faces into "iso-rectangle" and "not". The code does not: `lane_of` divides on the carrier kinds of the outer loop, and shape appears only as a refusal afterwards. Its own doc says so. The notch and all the ring refusals are that mismatch.
- "An arm per pair" overstates what is missing (sure). Certificates exist for all five analytic charts and take any triangle (`cert.rs`). `trim_polygon` already accepts every exact image form on every chart. For cone × ellipse and torus × spiric, what is absent is a roster entry in `trimmed::Lane`, two closed-form speed bounds, and the sphere arm's pole-side rule stated for any singular row.
- The row lists "a seam rule for a polygon crossing a period" as absent on the torus. By reading, the lift already supplies it: it carries the whole-period shifts along the loop (likely; not run).
- What is absent in every row is the domain as one value with its rings, and one theorem (below).
- The boundary is derived twice today. The rectangle lane re-derives a loop's place in the chart from carriers (`walk::loop_polygon`, with tie rules at poles); the trimmed lane reads the stored lift, whose joints were decided at mint. They are two descriptions of one thing.
- Per-chart knowledge sits in five places with different rosters (`sizing.rs`, `cert.rs`, `curved::grid_counts`, `trimmed::Lane`, three chord tightenings). The `trim_frontier` refusal is two of those rosters disagreeing.

**How it meets the two promises.**
- `[tolerance.chordal]` (sure): unchanged in kind. Every emitted triangle passes its chart's certificate or the face refuses typed. Boundary vertices are the shared carrier points, so watertightness stays by id. A ring placed a period off cannot yield a wrong mesh: its triangles span a period and fail the certificate.
- `[mesh.sizing]`: the theorem factors as the code does.
  - **Cell bound, per chart**: any triangle inside a chart box one lattice step wide deviates at most δ. Proved and pinned for every chart today (sure).
  - **Containment, per triangulation**: how far past one cell a triangle can reach.
    - *Aligned run* (boundary on a chart iso line, which the lattice takes a line through): proved today, by the strip argument at `sagitta_step` and the aspect lemma at `sphere_grid_steps`. A face with only aligned runs sizes at δ. That covers every rectangle and, with a lattice row at the notch floor, the notched wall (likely).
    - *Oblique run* (boundary crossing lattice lines): missing. The three open debts (trimmed cylinder, trimmed sphere, cone with a split side) are this one lemma seen from three charts (sure).
  - So the lane carries **one** named `UNPROVED_HEADROOM`, read where it picks the target for a face with an oblique run, and one debt row replaces three. Cone and torus join an existing debt; no new one opens.
- **Candidate lemma for that debt (unsure; a sketch).** Work in the triangulation's metric: `a` the lattice cell side, `s` the longest boundary chord, `M` the chart's second-derivative bound. If no lattice point is placed within `s/2` of a boundary chord, every triangle fits in a circle of radius `a/√2 + s`, and its deviation is at most `½·M·radius²`. At `s → 0` this is exactly the cell bound the steps already invert.
  - It also makes two of the four retry kinds unreachable: a grid point on a constraint, and a fold over a chord (likely).
  - The constant is crude. Boundary chords 4× finer than the lattice give about today's density, with a proof. I believe the sharp constant is `a/√2` once `s ≲ 0.4a`; that would drop the interior to the rectangle lane's density, half today's cells.

**Worked example.** Pipe ∪ a bar through its wall. The wall's outer lift is the rectangle one period wide. The hole is two rim arcs and two rulings, all aligned, so the lattice takes a row and a column through each and the face sizes at δ with no headroom. Turn the bar 0.3 rad: the hole is ellipse arcs, oblique, and the same code sizes that face at the headroom. Only the target differs.

**The final states.**
- **A. One lane; the rectangle is a case of it (my lean, likely).** `curved.rs`, `walk::loop_polygon` and its five doors retire, and `lane_of` reads the surface kind only.
  - True afterwards: one reader of a loop's place in the chart, one roster of charts, and the rectangle's tight theorems kept as the aligned case.
  - Still possible and refused typed: a loop whose lift does not close in the chart (a pole inside the face, a seamless band).
  - Consequence: swept faces' meshes must be shown byte-identical or re-baselined (D9 replay, TESS-BUDGET).
  - Reversible until the last step.
- **B. Two lanes, routed by the domain's shape.** Everything in A except that last step: the rectangle lane stays for a face with no ring whose outer loop is a rectangle.
  - Nothing emitted today moves.
  - It leaves two derivations of where a loop sits, and a router predicate that the rectangle lane's walk can contradict.
  - B is A stopped one unit early, so the choice need not be made now.
- **C. An arm per pair, as rows arrive.** Every arm repeats the roster, opens its own headroom debt, and still leaves rings and the notch unrouted. Not recommended (sure).

**Order.** Units 1–4 move no mesh emitted today; they turn refusals into meshes.
1. The chart table, the three image facts, and the one chord rule. A pure re-cut, pinned byte-identical.
2. The lane takes cone, torus and spindle. Closes cone × ellipse and torus × spiric.
3. The face's lift with rings, and the domain of several loops. The ring guard goes.
4. `lane_of` routes on the domain. Closes the notch and the rings on cylinder, cone, sphere and torus.
5. The oblique lemma, and the headroom deleted. Independent of 2–4; it starts with the measurement the debt rows ask for.
6. Rectangle faces move over. This is the A/B fork.

**Ratified text I would change.**
- `crates/mesh/README.md` `[mesh.sizing]`: "Each lane (curved, trimmed, NURBS)" and the bullet naming three rim mechanisms. Provenance: `[ev]` PR 5708, fork-log row 195. Your own words there were "as big as possible without any extra work"; the per-lane list was designer text you adopted. The rule stands. The list is re-cut to cell bound per chart, containment per boundary class, and one rim rule.
- Not changed: the tabled interior-pole question (your 2026-09-18 ruling, kept out of DESIGN.md). This design does not need it. A loop that does not close refuses in one place, which is where an interior pole would be taught if you reopen it.

## For the orchestrator

**Brief corrections.**
- The checkout is not shallow (about 60k commits, back to July). The provenance checks above were made; an unbounded `-S` over all paths is slow, so limit it to a path.
- All of the report is from reading; nothing was run.

**Claims I could not check; measure before speccing.**
- Whether a ringed face's ring lift sits in the outer loop's sheet. `pcurves::Lift`'s doc says the chain's start is gauge, and no API places one loop relative to another (sure). So unit 3 needs a per-ring deck element (whole periods; the involution on a sphere), decided with a band as joints are. That is `crates/topo/src/pcurves.rs`, not mesh ground. `chart_region`'s `SeamBranch` refusal is the same gap.
- Whether any native door mints a loop whose lift does not close. DESIGN `[topology]` ("a wall keeps the cuts its author wrote") and `one_region.rs`'s essential-cycle pairs suggest a seamless band may be legal at rest. A probe of `Lift::closes_in_chart` over the corpus's curved loops would answer it.
- Whether a spiric-bounded torus loop's lift closes and gives a simple polygon.
- Whether rectangle faces through the region lane reproduce today's bytes (unit 6).
- The lemma sketch. Hidden-circumcentre triangles and the sphere's one-latitude metric are not worked through.
- Not read: `nurbs_cert.rs`, `memo.rs`, `tools/tess-meter`.

**Unit notes.**
- Image facts belong beside `Pcurve::eval` in `crates/geom-brep/src/pcurve_cache.rs`.
  - `closed_form_span_box` answers only `Harmonic` and `FocalSection` today.
  - The speed bounds are in `mesh/chords.rs` for four forms.
  - Missing, all closed form: the spiric wall (`|u′| ≤ |d|·r / (f_min·(R − r))`, `v′ = 1`), the focal section (`|u′| ≤ (1+|β|)/(1−|β|)`), and a projected image off the sphere (`sup|C′|` over the chart's least lever).
- `sizing::cone_face_rho_max` and `sphere_face_widest_cos` read circle carriers on the outer loop only. A cone face bounded only by ellipses has no answer, and rings are unread. Replace both with the window from image boxes over all loops.
- `memo::FaceInputs::gather` must fold the rings' rows into the key.
- `sphere_polygon_shape`, `at_pole` and `pole_side_columns` generalise to the chart's singular rows (cone `v = 0`, spindle `±v*`). `cert_cone` and `cert_spindle` need the per-corner singular flag the rectangle lane's `UvPoint.pole` carries.
- `TrimmedFace::outward` needs the chart normal for cone and torus.
- `RingOnCurvedFace` and its "kernel bug" Display retire at unit 3. A non-closing loop gets its own capability refusal.
- The region lane needs stored rows. Mesh tests that hand-build bodies through the Euler doors without a mint will refuse as a lift gap after unit 6.
- `chords.rs`'s header admits its four tightening doors are unenforced. Unit 1 removes the list.
- Before unit 6, unit 4's router is `props::require_iso_rectangle` used as a route, not a refusal. Issue 727's note at `require_iso_rectangle_face` anticipates this.
- Your log says a new arm owes its theorem in the same change. Under this design there is no new arm and one theorem owed once (unit 5). Units 2–4 extend the existing named headroom, which the README permits. Whether unit 5 goes first is sequencing only.

**The two excluded defects.**
- The C⁰ section spline: agreed, not this problem. The carrier is the defect. A projected-spline speed bound would rest on the same C¹ premise.
- The turned ball: not this problem, but its site is one this design retires, and its open question has an answer by reading.
  - `u = −4.838e−49` lies in spade's refused band `(0, 2⁻¹⁴² ≈ 1.8e−43)`.
  - `planar.rs` passes every chart coordinate through `spade::mitigate_underflow`, and its module doc argues why that is not snapping.
  - `curved`'s `insert` closure and `trimmed::cdt_point` do not.
  - The fix is that floor at the curved CDT entries, independent of this design. Worth adding to that row.
DELTA REPORT — PR 7517 @ 390036911f (against my first pass on a87d047b5a).

**Verdict: APPROVE-WITH-FIXES.** Nothing blocking. The fixes are tracker and doc items (e1–e3 below), none in production code.

## (a) My first findings
- **MAJOR, the bit-identity gate: FIXED.**
  - `bash scripts/run-gates.sh` returns rc 0.
  - `rg 'bit_identity|one_stored_point|eq_bits' crates/mesh` is empty, test code included.
  - The decision is now `CollapsedSides::holds` → `chart_table::at_a_singular_point` over the home's side points, at `tol.band`.
- **MINOR, bytes across ε: FIXED.**
  - Whole `mesh` with `--ignore-default-filter` passed 556/556 at each of ε 1e-9, 1e-6 and 1e-12. That run includes `the_class_meshes_to_one_digest_at_every_eps` and the two new `d9_mesh_goldens` entries.
  - My first-pass sweep (14 bodies × 4 δ × 3 ε, one digest each) agrees with that.
- **MINOR, the near-side witness and its filing: SUPERSEDED.** The band read makes the in-band vertex walked. The new row `a_vertex_in_a_sides_band_is_on_it_and_meshes_as_the_vertex_on_its_point` passed in that run, and the ε-split case now lives on the pole-identification row (see (c)).
- **NOTE, uncertified dropped slivers: ADDRESSED IN DOC.** `SidePoints`' doc says what is proved (the segment is an edge of kept triangles) and that the sliver figure is measured, not proved. That is accurate.
- **NOTE, the class row does not pin the count: UNCHANGED.** The relation, pieces and lune rows pin it. Acceptable.
- **NOTE, tracker:**
  - The corner row is now P3/M.
  - The P4 row for the twelve pasted chord-pass setups is filed, and `witness_pass` is the shared helper.
  - `work.py lint` is ok.
  - Still not fixed: the source issue stays `status: dispatched` while the PR "Closes" it.
- **Style:**
  - FIXED:
    - `SideWalk` → `SidePoints`, which ends the name collision with topo's enum;
    - `Along { fixed: ChartAxis, held, step }`, so the axis is a type;
    - `PolygonOf` replaces the nine-parameter `face_polygon`;
    - the "another `u`" docs;
    - a missing `shared[id]` is now `unreachable!` with a reason;
    - inserted points hold the fixed axis at `held` exactly (the knot domain's end), so "on the side" is true by construction;
    - the emit comment no longer claims a spline shape check.
  - REMAINING (style, not gating):
    - `trim_polygon` still takes a `(singular, walk)` tuple (likely);
    - `CollapsedSides::points` skips an undecided or two-sided vertex with `continue`, a documented quiet skip (unsure);
    - `holds` reports `SphereTrimShape { cause: SingularUndecided }` for a spline side, a wrong name; you say a rename row is being filed (sure).

## (b) What the fix pass and the two merges of main broke or newly made
- No red found. Whole `mesh` is 556/556 at three ε.
- I did NOT re-run the `sweep` digest differential on this head. That is the one pass I skipped, so the 611-test equality is proved for a87d047 only.
- Main's merged `trimmed.rs` changes are the walked lane (`walkable`, `walk_face`, `may_make`, `faces_holding_side`), RouterDefect and the apex-read unification. By inspection, the walked lane runs only on analytic (cone) rows, and `walk_face` refuses a spline row typed, so it does not meet a collapsed side.
- What I did not exercise: the shared-side rule's `faces_holding_side` reading a polygon point whose id is P at several chart locations, beside `edges[i]`. It is covered only by the class rows' check_mesh and winding passing. Note, no row targets it.

## (c) The ε question: DEMONSTRATED (sent in the interim)
- The wedge with its vertex points restated +1e-8 (`restate_geometry`), meshed at δ 0.1:
  - ε 1e-6: 248 tris, side walked, digest 25e3945334089fe8;
  - ε 1e-9 and ε 1e-12: 240 tris, one copy, digest 53228aa5bd157cc1.
- Both meshes are closed. The same split shows at δ 3e-2 (656 against 648), at off 1e-7, and at off 1e-10 and 1e-11 (where ε 1e-12 breaks from the other two).
- The undecided band refuses `SphereTrimShape`, typed. No lift/mesh disagreement, no panic.
- On main this input was ε-invariant, because the spline arm read no side. So this is a fresh instance of the debt, which the row now names in prose only.
- It belongs on `pole-identification-lets-eps-choose-between-two-meshes-of-a-cone-face.md` as its spline witness. I judge it NOT blocking: no door builds such a vertex, and the class row holds the loft to one byte stream. The row's P3 should be reconsidered now that it has two readers and a concrete witness.

## (d) Attack on your reading
Mostly it survives.
- `[extensibility]` (~1179–1186) fixes the singular POINT on the stored values. It says nothing about a vertex's incidence on that point. Incidence is a coincidence between two cells, and the bit-identity gate header and `bit_identity.rs` docs retire the bit channel for exactly that.
- `[tolerance.chordal]` (DESIGN 650–653) names "the one ε read that is a classification — pole identification", owing a guard (#896). A collapsed side is that same classification on another chart.
- So the banded read conforms with no revision, and the exact read was the one needing one. I withdraw my first-pass "design question for Ev".

Three weak points:
1. The same `[extensibility]` paragraph says "a constructor that decides such a side collapsed at its band builds it exactly one point". That is a constructor discipline, and the analogous discipline for the VERTEX (write it as the side's point) is unwritten. The import cure from the deleted "hair off" row is therefore still the right long-term home, and nothing now tracks it except the pole row's prose.
2. `crates/mesh/README.md` `[mesh.winding]`, lines 29–30: "No ε is read. The mesh stays a function of (body, δ), as `[tolerance.chordal]` rules." The bullet is scoped to the winding verdict, but the second sentence is stated generally. It was already false at analytic poles, and my witness makes it false for spline sides too. It is a doc/ruling mismatch that rides the guard debt. Not blocking, but worth a line on the pole row.
3. A vertex decided ON a side while up to the band away from the side point gets copies whose emitted position is the vertex, not P. The corner error is then up to the band's reach, which is K·ε, not ε, against a promise of "honestly δ + ε". This is pre-existing at analytic poles, and new for spline sides. NOTE, by inspection.

Should the guard debt block this PR? No, by the clause as written: it names the debt, and it is tracked. I would block only if a door could build the vertex, and none does.

## (e) Remaining items
- e1: Pin the (c) witness on the pole-identification row, and revisit its P3. Not blocking.
- e2: Set the source issue's status to match "Closes" (or have the orchestrator close it at merge). Not blocking.
- e3: Note the `[mesh.winding]` 29–30 general sentence against pole identification, on the pole row. Not blocking.
- e4: `SphereTrimShape` naming for a spline side. A rename row is being filed. Not blocking.
- e5: Hosted CI on 390036911f: run 45133393167 has "change filter" queued; "prime the build cache" and "prime the lint cache" were skipped. NO test or lint job has run. The verdict is conditional on that gate going green. The local equivalents are green: gates rc 0, and `mesh` whole at three ε.

## Executed / read
- Executed at 390036911f:
  - `scripts/run-gates.sh`;
  - `work.py lint`;
  - the ε probe (wedge, 9 offsets × 2 δ × 3 ε);
  - `mesh` whole with the slow set at three ε;
  - the rg sweep for the bit channel.
- Read only: the production diff against main-at-merge (`trimmed.rs`, `chart_table.rs`, `chords.rs`); main's merged `trimmed.rs` log and diff; DESIGN 642–660 and 1170–1186; the mesh README `[mesh.winding]`; the tracker rows; the d9 golden diff; the check runs.
- Not re-run: the `sweep` digest differential and the budget deviation sweep. The tree is clean; nothing committed or pushed.

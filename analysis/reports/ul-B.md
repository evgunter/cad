## For Ev

**Recommendation (likely): keep the three sources as you ruled on PR 6380, add no fourth, and delete the `unsourced` doors.** That means `Lever::unsourced`, `ExtentBall::unsourced` and their per-file pin all go. Each of the 66 sites is re-posed by **the role its length plays in the reading**, not by which source it lacks. Six roles cover every site I read (about 30 of the 66). In none of them was the length an arm that needed a fourth source. One clarifying sentence goes into the ratified text, and one point-level extent read moves down a crate so that `profile` can reach it.

**What has to change in the premise (sure).**
1. **The five families sort the sites by why bit-identical typing failed, not by what each arm is.** PR 6391's acceptance test was "wrapping is bit-identical", so any arm whose honest read rounds differently got parked. "It would move verdicts in the last bits" is a cost, not a reason. Where I checked, the moved verdict is the one that was wrong. `rim_wedge::same_circle` levers an axis tilt at `r_a + r_b`. The two circles share a centre (decided first) and have equal radii (decided first), so a tilt moves each rim point by `sinθ·r`. The arm is the circle's own radius, and the sum doubles it.
2. **Many of the 66 are not arms at all.** In `x · L`, the length is sometimes the thing being measured, or a tolerance. `Margin::levered` has one typed slot, so these landed in it as the "arm". DESIGN's margins clause (i) already says "a site where no door honestly fits is a finding, not a cast", and the README says "no `Margin` door takes hand-written slack". The `unsourced` door is that cast. `boolean::join::off_conic_slack`, for example, is a hand-written slack (ε·2cot ψ) routed through `levered`.
3. **7 of the 66 are in `#[cfg(test)]` code**: `star/cut`, `star/order` (the probe), `boolean/fan` (2), `link/chord_rows`, `boolean/join`'s apex fixture, and the `ssi/certify` twin. These belong at the fixture doors that already exist.
4. **`profile`'s six arms are a layering question, not a sourcing one.** Each is a chord between two points or an arc's radius, and both are extent reads in kind. They have no door only because the point-level reads live in `geom_brep`, which sits above `profile`. `geom`'s spiric torus ball (the one item the gate exempts below the extent home) is the same problem.

**The six roles, and what each site becomes.**
- **R1. A read of consumed geometry that the extent home lacks, or has with other bits.** Add the read to the home, or adopt the one there, and accept the moved verdicts.
  - Sector/germ arms: `sector_shape::metering_arm` (one function that serves 9 callers) and `census`' wedge arm, both read through `Lever::min` of the edges' extents.
  - Sums of radii or reaches:
    - where the pivots coincide, the union's `max`;
    - where one pivot stands off another, the existing `ExtentBall::lever_from_within`. This covers the mate bundle's `Arm` (the copy's reach plus its slides and poses) and `eval::measure`.
  - Arc lengths, and lengths read off a line's parameter span: the edge's extent read.
  - `chart_region`'s `hyp`, a cylinder piece's reach: a new home read.
  - `SsiDomain::extent`: the field becomes a `Lever` built by its producers in `pair_section`, which already read it off a ball (`radius·2`). Its ~80 test literals become fixtures.
  - The marched polyline: its points' diameter.
  - A ray's reach: the traced segment's own length.
  - `solid_contain`'s ruled-face reach and the corner and mitre balls: read off the faces and edges they consume.
  - Villarceau `tilt`: the circle's own radius (`curvature_radius`).
- **R2. An arm derived from a sourced arm by a pure number or an offset.** The number moves into the reading; the offset moves into the band.
  - `star::order::leaned_lever` is `unsourced(L·min(s₀,s₁))`. It becomes `levered(x·min(s₀,s₁), L)`: the same product, with the lean on the reading.
  - `pcurves` joint room (`d − 2ε`): lever at `d`, and read the ends' ε through `Band::padded`.
  - `curve_pair::pose` route (`length + e + e`): `ExtentBall::of_cover_within`.
- **R3. The length is the measurand.** It becomes `Margin::of` (a projected length) or `levered_inv`.
  - `props::curved` `off_axis` (the offset projected by `cos α`): `Margin::of`.
  - `intersect`'s apex reaches (`levered(1/d, size)`): `levered_inv(size, d)`, whose different rounding is the only reason it was parked.
- **R4. The length is a tolerance.** The question being asked is about the band, so it is answered with a band, never a lever. Worked example, `validate::resolved_as`:
  - Today: `levered(K, unsourced(rounding))` in the band `(ε, Kε)`. It decides Zero when `rounding ≤ ε/K` and apart when `rounding ≥ ε`.
  - Re-posed: `Margin::of(rounding)` in the band `(ε/K, ε)`. The verdict is identical, and the band now states that `rounding` must fit a share of ε.
  - `dihedral` (both sites): `sinθ` levered at the true arm (which every caller holds), in the band that escalates at the arm's tolerance.
  - `join::off_conic_slack`: `Band::padded`.
- **R5. A unit or zero standing in for a missing arm.**
  - `overlap::one_sense` (a cosine at 1 m): the faces' extent.
  - `fate::sign` (always 1, on a unit vector's length or a count of half-turns): the corner's edges, or `sign_of` where the reading is exact.
  - `ring_path`'s lead `Piece::Segment` (an arm of zero meaning "no arm"): make "no arm" unrepresentable by giving that piece no lever field.
- **R6. Test code.** Use the fixture doors.

Confidence: the cut into roles is sure for every site named above. That every one of the 66 fits a role is likely; I did not read every site. The least certain are `pcurve_cache`'s focal β and `unit_sign` (each looks like a rate, so the metric door), and `coincide::replay`'s rounding of an arm up to a power of two (a symbolic-lane device).

**The final state this makes true.** Every `Lever` is built from geometry, a curvature or the session box. There is no door that takes a length unnamed, so the ratified sentence is true of the tree rather than of a pinned list. The gate shrinks to its two other holds: the extent's own door stays in its home, and the fixtures are counted. What remains possible: a geometric constructor can still be handed a false number. That is the constructor-doors row your PR 6380 ruling opened, already in flight, and the two pieces of work end in the same place. Reversible: if a genuinely sourceless arm ever appears, a fourth source can be ratified then. I found no such arm.

**The alternatives.**
- **B. A fourth source: "a length the verdict's own code measures of the geometry it consumes, outside the home."** It would absorb `profile`, `hyp`, the ruled-face reach and `SsiDomain` at no cost. But it is exactly the vouched length that the one-home rule exists to stop: any site could call its length that. Rejected (likely).
- **C. Restate the sentence as "named by its source, or pinned `unsourced` with a reason."** This is what you declined on PR 6380 ("keep the text and fix the violations"). It leaves a standing cast door. Rejected (sure).
- **D. `profile` depends on `geom_brep` to reach the home.** This inverts the layering: a 2-D sketch crate would depend on the B-rep crate for a chord. Rejected in favour of moving the point-level read down (likely).

**Ratified text I would change** (`crates/geom-core/README.md`, "Float rounding", the lever bullet, which your `[ev]` commit wrote). The three sources and "rates are not levers" stay. Add:
> "A length that is the reading itself (a residual, an offset, a datum's stand-off) is the margin, and a tolerance is the band's: neither is an arm. A dimensionless factor (a lean, a gain) belongs to the reading, not the arm. The point-level reads (two points' distance, a point set's reach from a pivot) live beside `Lever` in `geom-core`, so a crate below `geom_brep` reads them; covers and `Reach` stay in `geom_brep`."

This states the principle behind your PR 6329 question ("how does this relate to the levering for other things that aren't from a measurement"), which was answered then only for rates. No `docs/DESIGN.md` text changes. Its `[tolerance.epsilon]` sentence ("the reach at which the angle displaces the geometry it governs") already implies all of the above.

## For the orchestrator

- **The counts disagree.** The row's title says 74, its body says 63, and the brief, the counts file and `rg 'Unsourced lever'` all say 66. Reconcile the row's title when it is next touched.
- **The per-file pin understates the exposure.**
  - `geom_brep::sector_shape::metering_arm(arm: T) -> Lever<T>` is a `pub` function that turns a bare `T` into a lever through one pinned `unsourced`. Its 9 callers are invisible to the gate, so it is a second vouch door. Any R1 fix must delete the function, not re-source its body.
  - The same shape exists, unpinned, at `Margin::sagitta(curvature_rel, arm: T)` (12 production calls) and `Margin::over_lever(measure, lever: T)` (44 calls). Both take a bare `T` arm. These look like more "hidden levers" for `work/oneround/extent-reads-made-outside-the-extent-home.md`. File them or append them there.
- **The family-1 sites need a per-site check.** "The union's read is a max" holds only where the pivots coincide (true at `rim_wedge`). Where one pivot stands off the other, the sum is honest via `lever_from_within`, and a max would under-state the arm, which is the wrong-answer direction. The mate `Arm` and `eval::measure` look like the second kind.
- **Overlap with the in-flight `oneround/lever-doors` unit** (the constructor-doors row): its final state, constructors that take geometry, is the same as this one's. Sequence them so that R1's new home reads take geometry from the start.
- **Provenance.** The README sentence came in at `31576a6c6` ("[ev] margin scale per Ev"), and PR 6329's body and Ev's comments confirm the ratification. Ev's PR 6380 ruling (comment 6594257162) is in Ev's own words. The checkout is shallow (from about 2026-11-01), but I needed nothing earlier than that.
- **Assumption.** That the re-baselining cost is out of scope, per designer.md §4. If Ev wants verdict moves landed separately, sequence R1 behind a differential per family, as PR 6329's "differential per door" note suggested.
- **Not verified.** About 35 of the 66 sites are classified from their `// Unsourced lever:` comments rather than read: the rest of `pcurve_cache`, `census` `parallel_reading`, `editor-core::trace`, `vtxfac`, `chord_join`, and `sweep` planar. A lane taking this row should classify each one by role first, and stop on any that fits none of them.

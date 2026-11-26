# GERM `[sizes]` build — staging spec

This spec stages the build of `docs/DESIGN.md` `[sizes]` ("A size is a
type", ratified in PR 7645, whose body holds the decision document; fork
log row 307). It is the plan for the row
`work/germ/no-sign-decision-gives-a-conic-a-finite-size.md`, and its
retirement list is that row's. It holds no code. It was read on main at
`74abdd9d9`, after PR 7658 (the audit of the 67 production conic-size
sites, whose table this spec cites as "audit #n"). Names are cited with
their line on that commit; the line may go stale, the name may not. This
spec is deleted when the last stage merges.

Section 3 lists the questions this spec does not answer. A stage that
depends on one does not dispatch before it is answered.

## 0. What the clause asks, in build terms

- **One type, `Size`, with one fallible mint.** Finite and strictly
  positive. At `Interval` an enclosure that straddles zero is a `Size`,
  and one at or below zero at every point is not. A zero, a negative, a
  NaN or an unbounded value refuses typed and never panics. Finiteness
  is decided at that mint and nowhere else. `decide` stays a sign
  verdict.
- **Conic sizes are exact `Size`s.** There is no band at the mint. A
  conic's existence is its producer's pose decision, or is implied by
  the surface it lies on.
- **A surface kind's convention is a band-decided witness**, minted
  where the surface comes to exist, on the run's tolerance. The
  witnesses are a cylinder's and a sphere's radius, the torus's
  `RingTorus` and the spindle's sheet verdict. A witness hands out
  `Size`s. Check 1's radius half and the section arms' radius guards
  retire.
- **The cone's half-angle is an exact type** strictly inside
  `(0, π/2)`. Its band fact and its lever are
  `work/germ/a-cones-half-angle-band-has-no-lever-where-the-surface-is-minted.md`,
  which this build does not answer.
- **A builder that computes a size returns `Result`.** Evaluators stay
  total. A builder below a predicate battery hands back numbers, and the
  entity is minted after the battery passes. The blend torus's ring
  witness is minted through a token that only P3's
  (`fillet3_spine_regularity`) passed decision mints.

**Difficulty letters and tiers.** The letters are the review protocols'
H / M / L / S (`docs/MODEL-COMPARE-PROTOCOL.md` step 2,
`docs/DUAL-REVIEW-PROTOCOL.md` rule 1). This spec reads them as follows:

- **S**: small and local.
- **M**: moderate, one mechanism.
- **H**: hard. The change carries soundness: a wrong answer ships
  silently.
- **L**: large. Many sites, and the whole change is wrong if one
  posture is wrong.

The tier for each stage is this spec's recommendation. The tier is the
orchestrator's call at dispatch (`memories/orchestration-model.md`,
"Review tiers"), and the letter fixes the arm only once the unit is
dual.

| order | stage | scope (one line) | depends on | letter | tier |
|---|---|---|---|---|---|
| 1 | S1 `size-type-and-mint` | `Size`, its one mint, its error | — | M | single, FULL |
| 2 | S2 `conic-sizes` | `Circle`/`Ellipse` store `Size`; the conic doors are fallible; `conic_size_read` retires | S1 | L | dual |
| 3 | S3a `cylinder-sphere-radius-witness` | the cylinder and sphere radius witness | S1 | L | dual |
| 4 | S3b `torus-holds-ring-torus` | `Surface::Torus` holds `RingTorus` | S1, S3a's pattern | H | dual |
| 5 | S3c `spindle-holds-its-sheet` | `Surface::Spindle` holds its sheet verdict | S3b | M | single, FULL |
| 6 | S4 `cone-half-angle-type` | the exact half-angle type | S1 | M | single, FULL |
| 7 | S5 `fallible-builders` | the remaining size-computing builders return `Result`; the blend arms hand back numbers | S2, S3a–S4 | H | dual |
| 8 | S6 `fillet-headroom-and-ring-token` | headroom as a request-level pass; the blend torus minted from P3's token | S5 | H | dual |
| 9 | S7 `retire-rechecks` | check 1's radius half, the section arms' radius guards, the interim guards still standing | S3a–S6 | M | single, FULL |

**Order.** S1 comes first. S2, S3a and S4 each need only S1 and can run
in parallel lanes, but they share files (`geom`, `geom-brep`
`intersect.rs`), so run them stacked rather than concurrently unless the
orchestrator splits territory. S3b follows S3a so the witness posture is
set once. S5 needs every type in place. S6 is the fillet alone. S7
deletes what has become a second place.

**Every stage carries a deletion.** A guard retires in the same stage
that makes it redundant, so there are never two places. S7 holds only
what cannot go until several stages have landed.

## 1. The stages

### S1 — `size-type-and-mint` (M, single FULL)

**Scope.**
- A `Size<T>` newtype over the scalar. It has one fallible mint
  (`Size::new(x) -> Result<Size<T>, SizeRefusal>`, or a name of the
  implementer's choosing), a read-out (`get`), and no other
  constructor: no `From`, no `new_unchecked`, no `Default`.
- `SizeRefusal` names which of the four failed: zero, negative, NaN, or
  unbounded. It carries the value as `f64` bounds, the way
  `BlendError::NonpositiveSize` and `SizeBeyondRange` carry theirs
  (`crates/sweep/src/blend/mod.rs:1821`, `:1834`).
- The mint's predicate is today's `stored_size` predicate with the zero
  end closed, so that it refuses **zero** as well. Today's predicate is
  `crates/geom/src/curves/conic.rs:580`: "bounded and not definitely
  negative", whose second read tells a decided negative from a
  straddle. `[sizes]` reads "not definitely at or below zero", so at
  `f64` the mint refuses `0.0`, and at `Interval` it refuses `[−a, 0]`
  and admits `[−a, b]` with `b > 0`.
- **Home**: `geom_core`, beside `Real`. `geom`, `geom-brep`, `topo`
  and `sweep` all mint sizes, and `quantity::Length`
  (`crates/quantity/src/lib.rs:96`) is the user-facing type that
  `[sizes]` keeps apart.

**Moves.**
- `bounded_size` (`conic.rs:562`) becomes the mint's finiteness read.
  It keeps the same predicate, `Real::is_bounded`, so the `Interval`
  blind spot of `is_finite_length` (an upper end at `∞` whose width is
  not poison) cannot come back.
- Nothing else moves. S1 adds no reader.

**Claims for the reviewer to falsify.**
1. The mint never panics, at `f64`, `Interval`, `Dual` and `Sym`, on
   any bit pattern, including `±0`, subnormals, `±∞`, NaN, `[MAX, ∞]`
   and `[−∞, x]`.
2. At `Interval` the mint admits exactly the enclosures that are
   bounded and not at or below zero at every point. A straddle is
   admitted. `[0, 0]` and `[−1, 0]` refuse.
3. No path constructs a `Size` except the mint. Check the derives and
   the trait impls, including `serde`, `map_scalar`-style lifts, and
   `Default`.
4. `decide` and `predicate.rs` are untouched, so `decide(+∞)` is still
   `Positive`.

**Rows owed.** A mint table at each scalar, with one row per refusal
kind and one per admitted edge case: a straddle, `MIN_POSITIVE`, and
`MAX`. Each row needs a mutant that goes red: the zero end dropped, the
finiteness read dropped, and the finiteness read switched to
`is_finite_length`.

### S2 — `conic-sizes` (L, dual)

**Scope.**
- `Circle`'s `radius` and `Ellipse`'s semi-axes are stored as `Size`
  (`conic.rs:156`, `:185`).
- `Circle::new(frame, Size)` (`conic.rs:701`), `Ellipse::new`
  (`conic.rs:951`) and `Curve3::circle` (`conic.rs:1467`) take a
  `Size` and are infallible. Their `# Panics` sections go.
- `stored_size` (`conic.rs:580`) is deleted: the type now holds what it
  asserted.
- Every production site in the audit's table mints its `Size` and
  returns `Result`. That is 67 sites, less the 3 literals and the door
  itself.

**Moves, by audit disposition.**
- **stored** (25 sites, e.g. audit #3, #23, #37, #46–48, #61–67): the
  site reads `c.radius()`, which is already a `Size`. No mint, no
  `Result`.
- **literal** (#1, #41, #50): a named constant (`Size::ONE`) minted
  once, under S1's rows, with no `expect` at the site.
- **decided** and **proven** (24 + 9 sites): the producer keeps its
  pose decision, decides it first, and then mints. The mint can now
  refuse only on finiteness, and on zero at a decided-positive size it
  cannot. That refusal maps into the producer's existing error under
  the variant this stage adds (see the error table below).
- **The retirements this stage owns** (the build row's first bullet):
  - `geom::conic_size_read` (`conic.rs:552`, re-exported at
    `crates/geom/src/lib.rs:175` and `crates/geom/src/curves.rs:69`) is
    deleted with its three readers' finiteness halves:
    - `plane_sphere_section_read`
      (`crates/geom-brep/src/intersect.rs:1540`, the read at `:1605`)
      decides `ps_center_gap` on the raw root and mints after it. A
      root with no finite value refuses from the mint, not from a
      poisoned row read.
    - `topo::replace_face::moved_radius`
      (`crates/topo/src/replace_face.rs:1263`, the read at `:1264`):
      its sign half is a pose decision and stays, deciding
      `offset_moved_radius` on the raw sum. `MovedCircleCollapses`
      stays. Only the finiteness half retires, into the mint. Its
      readers are `transport_curve` (`replace_face.rs:1304`, every
      arm) and `topo::offset_axial::moved_seam`
      (`crates/topo/src/offset_axial.rs:2868`).
    - `topo::offset_axial::latitude_circle` (`offset_axial.rs:2844`,
      the read at `:2852`): `offset_axial_latitude_radius` is an
      escalation only, and its finiteness read goes. Whether the row
      itself survives is for the lane to read: if its only question was
      finiteness, it retires whole, with its roster entry.
  - `Curve3::representability_margins`'s circle and ellipse arms
    (`crates/geom/src/curves.rs:1308`–`1312`) return nothing, since a
    `Size` cannot be outside them. The spiric's arm (`:1313`) is
    Question 4.
- **Kept**: the five speed-floor readers decide span questions under
  either design (PR 7645, the further designer, point 1):
  - `certify.rs` check 2;
  - `pcurve_cache.rs`;
  - `props/curved.rs`;
  - `end_face.rs`;
  - `containment.rs`.

  `param_rate_gate`'s conic arm stays as well: it is incidental and not
  a duplicate under an exact `Size`.

**The error each producer's mint refusal becomes.** A finite size
already passed by a decided-positive row can fail only as unbounded, so
each producer gains one arm:

| producer | today on a non-finite size | after S2 |
|---|---|---|
| `geom-brep` section reads (`intersect.rs`, `pair_section/`) | a poisoned row read: `Escalated` | `SectionError` arm carrying `SizeRefusal` |
| `replace_face` transports, `offset_axial` seams | `Unmoved::Escalated` (since PR 7658) | Question 6 |
| the blend arms | panics past ~1.5e308 (audit #35, #36) | S5 |
| STEP import | refused before construction (`positive_length`, `crates/step-import/src/interpret.rs:282`; `root_length`, `crates/step-import/src/entities.rs:530`) | the import's own refusal, its mint the one mint |

**Sites the audit left "open" for finiteness** (the build row, "Open"),
each of which S2 closes by type:
- `sphere_sphere_section_read` (`intersect.rs:1713`, audit #6);
- `plane_cone_section` (`:2472`, #7);
- `plane_torus_read` (`:3119`, #9, #10);
- `coaxial_cone_sphere` (`:3373`, #11);
- `coaxial_cone_torus` (`:3537`, #12);
- `cone_cylinder_section` (`:3735`, #13);
- `pair_section/gap.rs:150` `Core::of` (#16);
- `pair_section/torus.rs:348` `Ring::parallel` (#17; its sign stays
  SECTTORUS's);
- `pair_section/winds.rs` `channels` (#18).

`Ring::of`'s `unreachable!("Ring::of mints a ring of finite data
only")` (`pair_section/torus.rs:336`), reached at `R = ∞` in
`CylinderTorusPose::decide` and `TorusTorusPose::decide`, is S3b's.

**Claims for the reviewers to falsify.**
1. No production path constructs a conic without a `Size` mint or a
   stored `Size`. Re-run the audit's grep and its second pass, which
   covers turbofish, constructors as values, `map_scalar` and
   `quarter_turned_where`, and report the count.
2. Every site the audit marked decided or proven still decides its pose
   **before** the mint, with no new band decision at the mint. In
   particular, no site replaces a pose row by the mint's refusal.
3. A size that is exactly zero today and was stored is now refused, and
   each such production path is either unreachable (an argument at the
   site) or refuses typed. Read in particular:
   - `AxisFrame::latitude` (`crates/sweep/src/revolve/axis.rs:87`, a
     pinned vertex guarded at each caller);
   - the cone transport arm (`replace_face.rs`, a parallel moved onto
     the apex);
   - `plane_cylinder_classified` at `r = 0`, which `pc_cylinder_radius`
     refuses today.
4. `Curve3::map_scalar` (`scalar_lift.rs`) keeps a `Size` a `Size` for
   every production closure, or re-mints fallibly (Question 5).
5. `moved_radius` still refuses `MovedCircleCollapses` on a decided
   non-positive sum, and the torus parallel keeps its `ρ + d·cos v`
   pose (PR 7658 m5's row stays green).

**Rows owed.**
- PR 7658's `conic_sizes_are_decided` rows (`geom-brep` and `sweep`),
  `moved_radius_rows` and `latitude_radius_rows` are re-pointed. Each
  pins the new refusal variant, not `is_err()`, and its mutant is now
  "the site skips the mint's `?`", which cannot compile. So the row's
  mutant becomes "the site decides its pose after the mint", red where
  the order matters.
- One row per audit-"open" site above at `f64::MAX`-scale data: a
  refusal, not a panic.
- A zero-radius row for each claim-3 path.

### S3a — `cylinder-sphere-radius-witness` (L, dual)

**Scope.**
- `Surface::Cylinder { radius }` (`crates/geom/src/surfaces.rs:121`)
  and `Surface::Sphere { radius }` (`:191`) hold a witness type whose
  mint decides `radius` positive on the run's band and hands out a
  `Size`. Its mint takes the band, so it lives in `geom` beside
  `RingTorus` (`surfaces.rs:656`).
- The field stops being a raw `T`. The struct-literal construction the
  enum docs promise (`surfaces.rs:86`–`87`, "construction is by
  struct-literal variant syntax") then needs a minted witness, and that
  doc and the crate rule at `crates/geom/src/lib.rs:79`–`84` ("At rest,
  tier-3 check 1 certifies … a radius above zero") are re-worded with
  the change. That is not a second decision: `[sizes]` decides it, and
  fork 274 decision 1 did the same for the frame.

**The witness's producers** (where a cylinder or sphere comes to
exist). Each producer mints on its run's band:
- `sweep` revolve (`crates/sweep/src/revolve/surfaces.rs:33`, `:44`)
  and extrude (`crates/sweep/src/extrude.rs`);
- the blend arms (`crates/sweep/src/blend/arms.rs:473`, `:1304`,
  `:1406`), which are S5's;
- `geom_brep::offset_surface` (`crates/geom-brep/src/offset.rs:333`).
  Its `offset_radius_floor` (`:344`) **is** the witness's mint for an
  offset, so it becomes the mint and not a second check;
- STEP import (`crates/step-import/src/entities.rs`, `snap.rs`,
  `normalize.rs`);
- `topo::replace_face` (`crates/topo/src/replace_face.rs`, eight
  literals), `props/quad_lane.rs`, `sector_face.rs`;
- `Surface::map_scalar` (`crates/geom/src/scalar_lift.rs:226`), which
  is Question 5;
- every test fixture. There are about 1,100 `Surface::Cylinder {` and
  860 `Surface::Sphere {` sites, about half in test files. A
  `test-support` mint that panics on a refusal is the fixture door,
  and a test-only panic is not a production one.

**The census this stage owes first.** Run
`rg -U 'Surface::(Cylinder|Sphere) \{' crates`, split into
constructions and patterns, and split the constructions into production
and test. Commit the counts in the PR body. A pattern site that only
reads `radius` reads `witness.size().get()`, or the reader takes the
`Size`.

**Retirements this stage owns** (the clause's "the section arms' radius
guards re-decide this fact, and retire"). Each decides
`Margin::of(radius)` on a cylinder's or sphere's stored radius:
- `cylinder_radius_decided` (`intersect.rs:1180`), with its two K
  names `pc_cylinder_radius` (`:1206`) and `coc_cylinder_radius`
  (`:3778`). This is the build row's second bullet;
- `cs_cylinder_radius` / `cs_sphere_radius`
  (`crates/geom-brep/src/pair_section.rs:375`–`376`);
- `cc_cylinder_radius` (`pair_section/cylinders.rs:67`,
  `CcDecision::Radius`);
- `ct_cylinder_radius` (`pair_section/torus.rs:898`) and
  `st_sphere_radius` (`:613`), both through `radius_guard` (`:359`);
- `kc_cylinder_radius`, `kc_sphere_radius` and their `_floor` rows
  (`pair_section/cones.rs:132`, `:140`, `:154`, `:162`);
- `kos_sphere_radius` (`intersect.rs:3380`);
- `carrier_sphere_radius` (`crates/geom-brep/src/carrier_eq.rs:626`)
  **only if** it decides the radius's sign and not the equality of two
  radii. The lane reads it and says which.

The public enums go with them, where nothing else reaches them:
- `SectionRadius` (`intersect.rs:999`);
- `SectionError::RadiusEscalated` (`:704`);
- `BooleanDecision::Radius` (`crates/topo/src/boolean/refusal_routes.rs:157`);
- their `pncad-py` tags and `editor-core` roster entries;
- `docs/predicate-dimension-audit.md`'s rows.

Each retired K name leaves the recourse roster in the same commit.

**Claims for the reviewers to falsify.**
1. No production path holds a cylinder or sphere surface whose radius
   was not decided positive on the run's band, and no witness is minted
   on a caller's or an offered band. Name each producer's band source.
2. Each retired guard decided exactly `Margin::of(r)` on the stored
   radius, on the same band the witness was minted on, so the
   witness's `Positive` implies the guard's. A guard on a different
   margin, a floor row, or a wider band is not this fact, and stays.
   Read the `kc_*_floor` rows especially.
3. The witness is not lost in any surface transform: a move, a
   reflection, `map_scalar`, or offset by `d`. A scaled or offset
   surface re-mints.
4. Check 1 still refuses an imported body whose radius misses the
   convention. It refuses at import, at the witness mint, with the
   import's typed error, and never reaches tier 3 holding one.

**Rows owed.**
- For each retired guard, a row that once reached it, re-pointed to
  the producer's refusal at the mint.
- `plane_cylinder_section` on `radius: −1.0` (PR 7658's
  `a_plane_cylinder_rim_decides_the_cylinders_radius`) cannot now be
  built, so the row becomes "the witness mint refuses `−1.0`, zero, in
  band, NaN and `∞`", with its mutant on the mint.

### S3b — `torus-holds-ring-torus` (H, dual)

**Scope.**
- `Surface::Torus { major_radius, minor_radius }` (`surfaces.rs:221`)
  holds a `RingTorus` (`surfaces.rs:656`), whose fields are already
  private and whose mint is `RingTorus::of` / `of_at` (`:720`, `:737`).
- `RingTorus` hands out its radii as `Size`s.
- `RingTorus::decided` (`surfaces.rs:767`), the crate-private
  unchecked mint, is used by `Curve3` at `crates/geom/src/curves.rs:1522`
  and `:1567` to read a spiric's radii as a ring. It retires: those
  sites read the torus's witness (Question 4).

**Producers**:
- revolve (`revolve/surfaces.rs:52`);
- the blend arms (`arms.rs:610`, `:1207`), which are S6's, through the
  P3 token;
- `offset_surface` (`offset.rs:379`–`397`), which re-mints the ring at
  `r + d`;
- STEP import;
- `map_scalar` (`scalar_lift.rs:238`);
- fixtures (about 780 sites).

**Readers that re-decide the ring today.** The convention's own rows:
- `torus_tube_positive` / `ring_torus_convention`
  (`crates/geom/src/rows.rs:98`);
- `require_ring_torus` (`surfaces.rs:632`) and `is_ring_torus`
  (`:648`);
- `TorusRefusal::require` / `require_at`
  (`crates/geom-brep/src/torus_convention.rs:202`, `:222`), read by
  `plane_torus_read` and the pierce normal.

Their callers:
- `topo::torus_region` (`crates/topo/src/torus_region.rs:306`);
- `boolean/section_cert.rs:901`;
- `boxes.rs:2740`;
- `cover.rs:399`;
- `geom_brep::variety` (`crates/geom-brep/src/variety.rs:124`);
- `pair_section/torus.rs:248` `Ring::of`;
- `curve_face/ellipse_torus.rs:308`, which calls
  `RingTorus::of(..).unwrap()`, a panic on data if the torus is not a
  ring.

The registry of readers with their own rows is
`RingReader` (`rows.rs:78`): `SelfIntersection`, `BooleanSheet`,
`BooleanRider`.

**What retires.**
- The section arms' ring guards (`Ring::of`'s two rows,
  `TorusRefusal::require` at the section arms) retire with the
  witness: the arm reads `ring` from the surface.
- `Ring::of`'s `unreachable!` (`pair_section/torus.rs:336`) and the
  `Circle::new(frame, ∞)` sites in `gap::Core::of` and `winds.rs` close
  by type.
- Whether the three `RingReader` variants other than `Convention`, and
  the topo readers above, also retire is **Question 2**. This stage
  does not decide it.

**Claims for the reviewers to falsify.**
1. Every torus surface in production holds a ring decided on the run's
   band. No path builds one from `RingTorus::decided` or any other
   unchecked door.
2. The tube half is decided first, as today (`of_at`'s order,
   `surfaces.rs:744`–`750`), so a negative `r` never reaches `R − r`.
3. The torus with `major_radius = ∞` (the build row's
   `Ring::of` repro) cannot be built, and its former panic sites are
   unreachable by type, not by guard.
4. `ellipse_torus.rs:308`'s `unwrap` is gone, or is on a ring read from
   the witness.

**Rows owed.**
- The ring witness mint table: tube in band, ring in band, the tube
  negative with the ring "positive" (`r = −R`), `R = ∞`, and NaN.
- A row per retired ring guard re-pointed to the producer.
- The `Ring::of` repro as a refusal at the mint.

### S3c — `spindle-holds-its-sheet` (M, single FULL)

**Scope.**
- `Surface::Spindle` (`surfaces.rs:258`) holds a sheet witness minted
  by `spindle_sheet` (`surfaces.rs:834`). Today that returns
  `Result<(), SpindleMiss>`. It mints the witness instead, decides
  `r > 0` first, and hands out `r` and `|R|` as `Size`s with the
  signed `R`'s side kept.
- Producers: revolve (`revolve/surfaces.rs:61`), STEP import,
  `map_scalar` (`scalar_lift.rs:247`), and fixtures (about 190
  sites). `offset_surface` refuses a spindle today
  (`offset.rs:403`, `OffsetError::SpindleSheet`), and that refusal
  stays.

**Retires.**
- The re-decisions `topo::injective` (`crates/topo/src/injective.rs:660`)
  and check 1's spindle arm (`crates/topo/src/validate.rs:7743`) are
  S7's, unless they move here whole.
- `ValidationError::DegenerateSpindle` goes with check 1's arm.

**Claims.**
1. Every spindle in production holds a decided sheet.
2. The signed major radius's side survives every transform: a
   reflection and `map_scalar`.
3. `injective.rs:660` decided the same margins on the same band.

**Rows.** The mint table at each of the three ends
(`SpindleEnd::Far`, `Sphere`, `Horn`, `surfaces.rs:790`), and an
in-band row for each.

### S4 — `cone-half-angle-type` (M, single FULL)

**Scope.**
- `Surface::Cone { half_angle }` (`surfaces.rs:160`) holds an exact
  type, strictly inside `(0, π/2)`, with one fallible mint and no band.
  This spec calls it `HalfAngle`; the name is the lane's.
- Its band fact (`pn_aperture_sin/cos` at `intersect.rs:2514`,
  `kos_aperture_*` at `:3396`, `kot_aperture_*` at `:3548`,
  `coc_aperture_*` at `:3793`, `kt_aperture_*` at
  `pair_section/cone_torus.rs:359`, `kc_cone_aperture_*` at
  `pair_section/cones.rs:130`, `section_cone_*_aperture` at
  `topo/src/boolean/section_cert.rs:1245`, `:1366`, and `topo/src/chord_join.rs:8082`)
  **stays**. It is the lever question's, not this stage's.

**Producers**:
- revolve (`revolve/surfaces.rs:40`);
- `chamfer_frustum` (`arms.rs:1562`), whose `asin(|g_r|/|g|)` can
  reach `0` or `π/2`, so the chamfer arm's mint can refuse. That is S5;
- `offset_surface` (`offset.rs:361`, which keeps α);
- STEP import;
- `map_scalar` (`scalar_lift.rs:230`);
- fixtures (about 800 sites).

**Claims.**
1. The mint refuses `0`, `π/2`, anything outside the range, NaN and
   `∞`, and admits a straddle at `Interval` only where the enclosure is
   not wholly outside the range. The exact semantics at the two ends is
   the lane's to state and pin, mirroring `Size`'s.
2. No aperture band row is removed.
3. Check 1's two half-angle margins (`surfaces.rs:1041`–`1050`) are
   vacuous after this stage. Whether they retire here is Question 3.

**Rows.** The mint table, and the chamfer arm's degenerate frustum
(a foot pair along the axis), which refuses typed and does not reach
the mint.

### S5 — `fallible-builders` (H, dual)

**Scope.** Every builder that computes a size and is not yet fallible
after S2–S4 returns `Result`. A builder below a predicate battery hands
back numbers, and its entity is minted after the battery.
- `curve_pair::on_surface` doors (`crates/geom-brep/src/curve_pair/on_surface.rs`):
  `circle` (`:413`), `parallel` (`:443`), `meridian` (`:496`),
  `great_circle` (`:704`), `about` (`:741`). This is CPAIR's
  `on-surface-curve-doors-state-a-raw-surfaces-radius-undecided`.
  Once the surface holds a witness, the cylinder's and sphere's radius
  and the torus's `R + r·cos v` are `Size`s or implied ones. The cone
  parallel's `|v·sin α|` mints exactly, and its band is the cone lever
  question's.
- `topo::boolean::trace` `circle_crossings_to` / `circle_crossings`
  (`crates/topo/src/boolean/trace.rs:436`, `:479`). This is TRACE's
  `circle-crossings-take-a-callers-radius-undecided`: the caller's
  radius becomes a `Size` parameter.
- **The blend arms hand back numbers.** `plane_sphere_blend`
  (`arms.rs:565`), `Meridian::blend` (`:1184`), `Ruling::blend`
  (`:1294`), `chamfer_frustum` (`:1536`) and `plane_plane_blend` (`:452`) return
  an arm record of numbers: frame, spine, tube, trims. They mint no
  `Surface` and no conic, and are fallible where an exact `Size` or
  `HalfAngle` mint fails. The mint error is the arm's own "radius
  beyond representable range" refusal (PR 7645: needed even with
  headroom first, since headroom saturates at `r`).
  `EdgeBlend.surface` (`arms.rs:610`, `:1207`) is minted later, in S6.
  This is BAND's
  `the-blend-arms-mint-conics-before-a-predicate-reads-an-oversized-radius`.
  - `BallRoom`'s methods (`crates/sweep/src/blend/battery.rs:909`–`957`)
    return the arm's `Result`.
- **Evaluators stay total.** No `eval`, `eval_d`, chart or residual
  gains a `Result`. Poison still flows through numbers. Claim 3 checks
  this.

**Claims.**
1. Every arm's sizes go through the one mint. The plane–sphere contact
   radius `sphere_trim_r = s·R/o` (`arms.rs:601`) is decided by no
   predicate (PR 7645, the further designer). Under an exact `Size`
   it passes the mint, and the battery's missing decision for it stays
   BAND's, filed and not built here.
2. The arms still never refuse anything a battery predicate refuses
   first. The only new arm refusal is the mint's overflow, and every
   arm refusal reaches the request as a typed `BlendError`.
3. No evaluator signature changed. Diff `eval*` across `geom` and
   `geom-brep`.
4. `fillet_edges(lantern, lip rim, 1.5e308)` and `f64::MAX`, which
   panic on main (audit #35, #36), refuse typed.

**Rows.**
- The lantern rows at `1.5e308` and `f64::MAX`.
- A chamfer frustum whose cylinder foot falls on the axis.
- One row per CPAIR and TRACE door at an over-range size.

### S6 — `fillet-headroom-and-ring-token` (H, dual)

**Scope.**
1. **Headroom as one request-level pass.** Today `run_battery_for`
   (`battery.rs:2239`) runs `resolve_link` (`:1095`) on every edge.
   That classifies, decides the ball room and computes the arm. P1
   (`radius_headroom`, `:644`) runs afterwards (`:2268`–`2280`).
   - Split `resolve_link` into a **classify** pass (everything up to
     the `BallRoom` token, `:876`–`883`) and a **compute** pass (the
     arm).
   - Run P1 over every classified link between the two passes.
   - This reorders no existing refusal, because today's arms never
     refuse (PR 7645, point 2).
   - The run comment "an out-of-scope pair refuses before any margin is
     taken" (`:2247`–`2248`) is already false and is re-worded with the
     change.
2. **The blend torus minted from P3's token.**
   - `spine_regularity` (`battery.rs:818`) returns, on `Positive`, a
     token type that only it constructs, the way `BallRoom` is
     constructed only by the room decision (`:909`).
   - `RingTorus` gains one door taking that token and the arm's
     numbers `(s, r)`. P3's margin `(s − r)·r/s` positive with
     `0 < r < s` implies both ring halves positive on the band (PR 7645,
     designer A's condition).
   - The torus `Surface` of a coaxial or plane–sphere link is minted
     **after** the battery, from the token.
   - P3 runs on `link.blend.spine_curvature` (`:2294`) today, read off
     the minted torus (`arms.rs:615`, `:1212`). After S5 it reads the
     arm record's spine.
   - The door must not record a decision under P3's row that P3 did
     not take. `RingTorus::of_at`'s `R − r` and P3's `(s − r)·r/s`
     share a zero but not a value (PR 7645, designer B), so the token
     door decides nothing and records nothing.

**Claims.**
1. No refusal that a request reached before this stage changes name or
   order. Diff every `fillet_edges` and `chamfer_edges` row's refusal
   variant, base against head. The only changes are at radii where P1
   now runs before an arm that used to panic or overflow.
2. `RadiusHeadroom` is the refusal at `1.5e308` as at `1e200` on the
   lantern ("same situation, same name").
3. Only `spine_regularity`'s `Positive` arm can construct the token.
   The token is not `Clone` or `Copy` across links, or it carries the
   link it was decided for. A token from link 1 cannot mint link 2's
   torus.
4. "No surface is minted before the battery passes" is true by type:
   no `Surface` is in an arm record.
5. A chamfer (no P3, `battery.rs:2226`) mints no torus, so it
   needs no token.

**Rows.**
- The lantern at `1.5e308`, which refuses `RadiusHeadroom`.
- A two-link request whose link 2 is out of scope and whose link 1 is
  headroom-short, refusing link 2's classification first, as today.
- A token-forgery compile-fail doc test, or an argument at the type.
- The pip rims' torus (spine `s`, tube `r`) at `r → s`, refusing
  `SpineIrregular` before any torus exists.

### S7 — `retire-rechecks` (M, single FULL)

**Scope.** What remains a second place once S3a–S6 have landed.
- **Check 1's radius half.**
  `Surface::representability_margins` (`surfaces.rs:1035`) loses its
  cylinder and sphere arm (`:1038`) and the torus and spindle
  minor-radius arm (`:1052`–`1061`). The cone arm is Question 3.
  Check 1's ring half at its call site (`validate.rs:7715`–`7736`,
  `ValidationError::DegenerateTorus` / `DegenerateTorusEscalated`) and
  its spindle arm (`:7738`–`7750`) go. So do `poisoned_datums`' entries
  for those datums, because a witness cannot hold NaN.
  `analytic_datum_verdicts` (`validate.rs:7308`) stays for what still
  has margins: the spiric, unless Question 4 moves it, and the cone,
  unless Question 3 does.
- **The section arms' radius guards** S3a and S3b did not reach. The
  lane re-greps for `Margin::of(radius)`-shaped rows on a surface's
  stored datum and lists each with its disposition.
- **The interim guards** (the build row's list). Each retires in the
  stage named, or here:
  - `conic_size_read` and its readers: S2;
  - `cylinder_radius_decided`: S3a;
  - `nonpositive_size_gate`'s finite end
    (`crates/sweep/src/blend/build.rs:271`–`276`,
    `BlendError::SizeBeyondRange`): Question 1;
  - `shell`'s `is_finite_length(thickness)`
    (`crates/topo/src/shell.rs:1587`, and its Display read at `:666`):
    Question 1. It has the `Interval` blind spot PR 7658's M1 named:
    `[MAX, ∞]` passes `x − x`.
- **Docs.** Re-word the two at-rest prose sites:
  `crates/geom/src/lib.rs:79`–`84`, and the `Cone` / `Cylinder` field
  docs ("positive by convention", "evaluated as-is … the net at rest
  is check 1"). Re-wording follows the change; it decides nothing new.

**Claims.**
1. Every retired check was a re-decision of a witness's fact on the
   same band. A body imported with a bad datum is refused at import,
   with the import's typed error. Run a STEP fixture with
   `radius = −1`, `0`, an in-band torus, and a spindle on its horn.
2. `validate`'s tier-3 report on every finished body in the workspace's
   rows is unchanged, except the removed variants, which no finished
   body can now produce.
3. No `pncad-py` tag, `editor-core` roster entry or recourse-roster K
   name dangles.

**Rows.** The STEP fixtures of claim 1. A tier-3 row that once read
`DegenerateTorus` is re-pointed to the import refusal.

## 2. Territory

These stages edit other programs' files. Run
`python3 scripts/work.py territory` at each dispatch.

| crate / file | stages | program's territory, as PR 7658 found it |
|---|---|---|
| `geom/src/curves/conic.rs` | S1, S2 | CONIC / ONECOPY |
| `geom/src/surfaces.rs`, `rows.rs`, `scalar_lift.rs` | S3a–S4, S7 | shared |
| `geom-brep/src/intersect.rs`, `pair_section/` | S2, S3a, S3b | shared, SECT, SECTTORUS |
| `sweep/src/blend/` | S5, S6, Q1 | BAND |
| `topo/src/replace_face.rs`, `offset_axial.rs`, `shell.rs` | S2, Q1 | SHELL |
| `topo/src/validate.rs` | S7 | the at-rest gate's owner |

The rows S5 closes are other programs' files: CPAIR, TRACE and BAND.
They are moved to GERM or closed by the stage's PR with a closing note,
as PR 7658 closed SHELL's.

## 3. Questions for the orchestrator

Each has a recommendation. None is built around: the stage named waits
for the answer.

**Q1. The user-facing sizes at the door: the blend size and the shell
thickness.** (Blocks S7's interim-guard half.)

Today `nonpositive_size_gate` (`build.rs:271`) refuses
`NonpositiveSize` / `SizeBeyondRange` by a bracket read (`lo() > 0`,
`hi()` finite). `shell` decides `shell_thickness` on the band and
checks `is_finite_length` (`shell.rs:1587`–`1593`). `[sizes]` says
"one mint" and "finiteness is decided at the mint and nowhere else". It
is silent on whether a user's size is "a length the kernel stores as a
size". The fillet radius is: it becomes the tube radius. The shell
thickness becomes an offset distance.

There is a tension the clause does not resolve. At `Interval` the
door's bracket read refuses a straddle (`[−1, 1]`), while the mint
admits it. So "the door's refusals become the mint's error" changes
behaviour at `Interval`, though not at `f64`.

*Recommendation:*
- Both doors mint a `Size` from the request through the one mint, and
  carry its `SizeRefusal` inside the verb's own variant, which renders
  the verb and the recourse. For example `BlendError::Size { refusal }`
  replaces `NonpositiveSize` and `SizeBeyondRange`, and
  `ShellError::Thickness` keeps its name with the refusal inside. That
  is one mint, and the door adds words, not a decision.
- The blend door's straddle refusal at `Interval` goes. A straddling
  radius meets P1/P3 downstream, which escalate in band, and that is
  sound.
- The shell's `shell_thickness` band decision is a band question (as
  `moved_radius`'s sign half is a pose question) and stays. Only its
  finiteness read retires into the mint.
- `work/verdict/blend-size-gate-unmetered-under-epsilon.md` keeps its
  question unchanged.

**Q2. Do the ring readers outside the section arms retire?** (Blocks
S3b's full retirement list.)

`[sizes]` names "the at-rest check's radius half and the section arms'
radius guards". Several other readers decide the same ring convention
on their own rows:
- `RingReader::SelfIntersection`, `BooleanSheet` and `BooleanRider`
  (`rows.rs:78`);
- `torus_region.rs:306`, `section_cert.rs:901`, `boxes.rs:2740`,
  `cover.rs:399` and `variety.rs:124`.

Under `[one-way.places]` they are second places once the surface holds
a `RingTorus`, provided they decide on the run's band. The clause also
says a witness is "never on a caller's band", and there is an offered
band: the tests read `require_ring_torus` on `offered`
(`surfaces.rs:2569`).

*Recommendation:*
- Retire every reader that decides on the run's band. `RingReader`
  then shrinks to nothing and goes, with its roster.
- A reader that decides on a wider, offered band is asking a different
  question (does the ring survive the offer). It stays, and is listed
  by name in S3b's PR.
- S3b's lane classifies each reader by its band source before deleting
  any.

**Q3. Check 1's cone half, and the poison half.** (S4/S7.)

The clause retires check 1's "radius half". Once S4 lands, the cone's
two half-angle margins (`surfaces.rs:1041`–`1050`) and the poison read
of a witness datum are vacuous by type too.

*Recommendation:* they retire with the type that makes them vacuous
(S4 for the cone, S3a–S3c for poison), on the same argument as fork 274
decision 1's frame. This is not a new decision.

**Q4. The spiric's radii.**

`Curve3::Spiric { major_radius, minor_radius, offset }` stores raw
`T`s, and `Curve3` reads them as a ring through the unchecked
`RingTorus::decided` (`curves.rs:1522`, `:1567`). Check 1's curve half
reads its minor radius (`curves.rs:1318`). `[sizes]`' conic bullet
names circles and ellipses only.

*Recommendation:* a spiric lies on its torus, so its radii are "implied
by the surface it lies on". It holds the torus's `RingTorus` witness,
copied from the surface it was sectioned from (215
`Curve3::Spiric {` sites). That retires `RingTorus::decided` and the
curve half's spiric arm in S3b. If the orchestrator judges it outside
`[sizes]`' text, it becomes a filed row and `RingTorus::decided` stays
until then.

**Q5. `map_scalar` on a witness and on a `Size`.**

`Surface::map_scalar` and `Curve3::map_scalar` (`scalar_lift.rs:226`,
and the conic arms) take an arbitrary closure. A lift (`f64` to
`Interval` at a point, `Dual::constant`, `Sym::from_f64`) keeps a band
decision, but an arbitrary map does not.

*Recommendation:* the scalar maps that touch a `Size` or a witness take
a sealed lift trait, whose implementations are the existing production
lifts PR 7658 enumerated. That keeps the decision true by type. The
alternative, re-minting fallibly inside `map_scalar`, makes a total
function fallible and needs a band it does not have.

**Q6. The refusal an over-range offset becomes.** (S2.)

Since PR 7658, a moved circle or seam whose size has no finite value
escalates (`Unmoved::Escalated`, `ReplaceFaceError::Escalated`), and a
decided non-positive one refuses `MovedCircleCollapses`
(`Refusal::Limit`, "use a smaller offset"). After S2 the unbounded case
is the mint's typed refusal. That is a limit on the distance, not an
undecided margin.

*Recommendation:* map it to a `Refusal::Limit` variant ending on the
distance's recourse, beside `MovedCircleCollapses`. Re-point PR 7658's
rows that pin `Escalated` (`a_ball_moved_by_an_infinite_distance_refuses`,
`a_sphere_face_moved_past_every_finite_radius_refuses`). A NaN distance
still escalates at the pose decision, before the mint.

**Not a question, noted.** `[surfaces]` names a spheroid, which has no
`Surface` variant on main. When it is built, its two radii are
witnesses under `[sizes]`' surface bullet. No stage here owes it.

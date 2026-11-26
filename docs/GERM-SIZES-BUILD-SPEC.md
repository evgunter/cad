# GERM `[sizes]` build — staging spec

This spec stages the build of `docs/DESIGN.md` `[sizes]` ("A size is a
type", ratified in PR 7645, whose body holds the decision document; fork
log row 307). It is the plan for the row
`work/germ/no-sign-decision-gives-a-conic-a-finite-size.md`, and its
retirement list is that row's. It holds no code. It was read on main at
`74abdd9d9`, after PR 7658, which audited the 67 production conic-size
sites. This spec cites that audit's table as "audit #n". Names are cited
with their line on that commit; the line may go stale, the name may not.
This spec is deleted when the last stage merges.

Section 3 records the orchestrator's rulings and lists the questions
still open. A stage that depends on an open question does not dispatch
before it is answered.

## 0. What the clause asks, in build terms

- **One type, `Size`, with one fallible mint.**
  - A `Size` is finite and strictly positive.
  - At `Interval`, an enclosure that straddles zero is a `Size`. One at
    or below zero at every point is not.
  - A zero, a negative, a NaN or an unbounded value refuses typed and
    never panics.
  - Finiteness is decided at that mint and nowhere else.
  - `decide` stays a sign verdict.
- **Conic sizes are exact `Size`s.** There is no band at the mint. A
  conic's existence is its producer's pose decision, or is implied by
  the surface it lies on.
- **A surface kind's convention is a band-decided witness.**
  - It is minted where the surface comes to exist, on the run's
    tolerance.
  - The witnesses are a cylinder's and a sphere's radius, the torus's
    `RingTorus` and the spindle's sheet verdict.
  - A witness hands out `Size`s.
  - Check 1's radius half and the section arms' radius guards retire.
- **The cone's half-angle is an exact type**, strictly inside
  `(0, π/2)`. Its band fact and its lever are
  `work/germ/a-cones-half-angle-band-has-no-lever-where-the-surface-is-minted.md`,
  which this build does not answer.
- **A builder that computes a size returns `Result`.**
  - Evaluators stay total.
  - A builder below a predicate battery hands back numbers, and the
    entity is minted after the battery passes.
  - The blend torus's ring witness is minted through a token that only
    P3's (`fillet3_spine_regularity`) passed decision mints.

**Difficulty letters and tiers.** The letters are the review protocols'
H / M / L / S (`docs/MODEL-COMPARE-PROTOCOL.md` step 2,
`docs/DUAL-REVIEW-PROTOCOL.md` rule 1). The protocols fix only the arm
each letter takes, so this spec reads them as follows:

| letter | reading |
|---|---|
| S | small and local |
| M | moderate, one mechanism |
| H | hard: the stage carries soundness, and a wrong answer ships silently |
| L | large: many sites, and the whole stage is wrong if one posture is wrong |

The tier given for each stage is this spec's recommendation. The tier is
the orchestrator's call at dispatch (`memories/orchestration-model.md`,
"Review tiers"). The letter fixes the arm only once the unit is dual.

| order | stage | scope (one line) | depends on | letter | tier |
|---|---|---|---|---|---|
| 1 | S1 `size-type-and-mint` | `Size`, its one mint, its error | — | M | single, FULL |
| 2 | S2 `blend-arms-hand-back-numbers` | the blend arms return numbers in a `Result`; the entities are minted after the battery; the blend torus is minted through P3's token | S1 | H | dual |
| 3 | S3a `cylinder-sphere-radius-witness` | the cylinder and sphere radius witness; the section radius guards on it retire | S1, S2 | L | dual |
| 4 | S3b `torus-holds-ring-torus` | `Surface::Torus` holds `RingTorus`; the spiric holds it too | S3a | H | dual |
| 5 | S3c `spindle-holds-its-sheet` | `Surface::Spindle` holds its sheet verdict; its re-decisions retire | S3b | M | single, FULL |
| 6 | S4 `cone-half-angle-type` | the exact half-angle type | S1, S2 | M | single, FULL |
| 7 | S5 `conic-sizes` | `Circle`/`Ellipse` store `Size`; every conic builder is fallible; `conic_size_read` and the ellipse door's own decisions retire | S3a–S4 | L | dual |
| 8 | S6 `fillet-headroom-pass` | headroom as a request-level pass | S2 | M | single, FULL |
| 9 | S7 `retire-rechecks` | check 1's radius half, and every remaining second place | S3a–S6 | H | dual |

**Why this order.**
- **The blend arms go first.** The arms (`arms.rs:452`, `:565`,
  `:1184`, `:1294`, `:1377`, `:1536`) are infallible `T: Real`
  functions. They hold conic constructions (`:617`, `:626`, `:1201`,
  `:1552`) and surface literals (`:473`, `:610`, `:1207`, `:1304`,
  `:1406`, `:1562`). S3a, S3b, S4 and S5 would each break them.
  - S2 therefore makes the arms return numbers before any type changes.
  - After S2, the entities are minted in one post-battery place. Each
    later stage changes only that place.
  - There is no interim posture, and no unchecked witness door.
- **Surfaces go before conics.** The audit's "stored" conic sites
  #24, #25, #27, #31, #38, #40, #49 and #63 read a surface's radius.
  Once S3a and S3b land, those radii are already witness `Size`s. So S5
  reads them without a mint, and nothing is minted twice.
- **Sharing files.** S3a and S4 share files (`geom/src/surfaces.rs`,
  `geom-brep/src/intersect.rs`). Run them stacked, not concurrently,
  unless the orchestrator splits territory.
- **S6** can run any time after S2.
- **S7** deletes what is still a second place once its replacements have
  landed.

**Every stage carries a deletion.** A guard retires in the same stage
that makes it redundant, so there are never two places. S7 holds only
what cannot go until several stages have landed.

## 1. The stages

### S1 — `size-type-and-mint` (M, single FULL)

**Scope.**
- **The type.** A `Size<T>` newtype over the scalar.
  - It has one fallible mint
    (`Size::new(x) -> Result<Size<T>, SizeRefusal>`, or a name of the
    implementer's choosing).
  - It has a read-out (`get`) and named constants such as `Size::ONE`.
    Each constant is minted once, under this stage's rows.
  - It has no other constructor: no `From`, no `new_unchecked`, no
    `Default`.
- **The error.** `SizeRefusal` names which of the four failed: zero,
  negative, NaN or unbounded. It carries the value as `f64` bounds, as
  `BlendError::NonpositiveSize` and `SizeBeyondRange` carry theirs
  (`crates/sweep/src/blend/mod.rs:1821`, `:1834`).
- **The predicate.** The mint's predicate is today's `stored_size`
  predicate with the zero end closed.
  - Today's predicate is at `crates/geom/src/curves/conic.rs:580`:
    "bounded and not definitely negative". Its second read tells a
    decided negative from a straddle.
  - `[sizes]` reads "not definitely at or below zero". So at `f64` the
    mint refuses `0.0`.
  - At `Interval` it refuses `[−a, 0]` and admits `[−a, b]` with
    `b > 0`.
- **Home.** `geom_core`, beside `Real`.
  - `geom`, `geom-brep`, `topo` and `sweep` all mint sizes.
  - `quantity::Length` (`crates/quantity/src/lib.rs:96`) is the
    user-facing type, which `[sizes]` keeps apart.

**Moves.**
- `bounded_size` (`conic.rs:562`) becomes the mint's finiteness read.
  - It keeps the same predicate, `Real::is_bounded`.
  - So the `Interval` blind spot of `is_finite_length` cannot come back.
    That blind spot is an upper end at `∞` whose width is not poison.
- Nothing else moves. S1 adds no reader.

**Claims for the reviewer to falsify.**
1. The mint never panics, at `f64`, `Interval`, `Dual` or `Sym`, on any
   bit pattern. This includes `±0`, subnormals, `±∞`, NaN, `[MAX, ∞]`
   and `[−∞, x]`.
2. At `Interval` the mint admits exactly the enclosures that are bounded
   and not at or below zero at every point. A straddle is admitted.
   `[0, 0]` and `[−1, 0]` refuse.
3. No path constructs a `Size` except the mint and its constants. Check
   the derives and the trait impls: `serde`, scalar lifts and `Default`.
4. `decide` and `predicate.rs` are untouched, so `decide(+∞)` is still
   `Positive`.

**Rows owed.**
- A mint table at each scalar: one row per refusal kind, and one per
  admitted edge case (a straddle, `MIN_POSITIVE`, `MAX`).
- Each with a mutant that reads red:
  - the zero end dropped;
  - the finiteness read dropped;
  - the finiteness read switched to `is_finite_length`.

### S2 — `blend-arms-hand-back-numbers` (H, dual)

This is BAND's
`the-blend-arms-mint-conics-before-a-predicate-reads-an-oversized-radius`,
and the clause's "a builder below a predicate battery hands back
numbers". It is moved to GERM, or closed by this PR with a note.

**Scope.**
- **The arms return arm records of numbers in a `Result`.** The arms
  are:
  - `plane_plane_blend` (`arms.rs:452`);
  - `plane_sphere_blend` (`:565`);
  - `Meridian::blend` (`:1184`);
  - `Ruling::blend` (`:1294`);
  - `corner_ball` (`:1377`);
  - `chamfer_frustum` (`:1536`).

  Each record holds a frame, a spine, a tube, and the trims' centres
  and radii. It holds no `Surface` and no `Curve3`.
- **Overflow is the arm's own refusal.** Each size the arm computes
  goes through S1's mint, so an overflow refuses there. Examples are
  the plane–sphere `√((o−h)(o+h))` and the corner's `r + r`. The refusal
  is the arm's own "radius beyond representable range". PR 7645 says it
  is needed even with headroom first, since headroom saturates at `r`.
- **`BallRoom`'s methods** (`crates/sweep/src/blend/battery.rs:914`–`957`)
  return the arm's `Result`.
- **One post-battery mint** turns a passed battery's arm records into
  `EdgeBlend`'s surface and trims.
  - Until S3a–S5 land, it states the surface and conic literals exactly
    as today, from the record's numbers.
  - Each later stage changes only this place.
  - `EdgeBlend.surface` (`arms.rs:610`, `:1207`, …) and its trims are
    filled here.
- **The blend torus is minted through P3's token in this stage.**
  - `spine_regularity` (`battery.rs:818`) returns, on `Positive` only, a
    token that nothing else constructs.
  - `RingTorus` gains one door that takes the token and the record's
    `(s, r)`.
  - P3's margin is `(s − r)·r/s`. With `0 < r < s`, a positive margin
    implies both ring halves positive on the band (PR 7645, designer A's
    condition).
  - The door decides nothing and records nothing under P3's row.
    `RingTorus::of_at`'s `R − r` and P3's margin share a zero but not a
    value (PR 7645, designer B).
  - Until S3b, the torus literal reads `ring.major_radius()` and
    `ring.minor_radius()`, so there is no unchecked door.
  - P3 today reads `link.blend.spine_curvature` (`battery.rs:2294`) off
    the minted torus (`arms.rs:615`, `:1212`). After this stage it
    reads the record's spine.
  - **Do not copy `BallRoom`'s whole pattern.** `BallRoom(())` is built
    at `battery.rs:878` with no margin decided (the rim branch with no
    room) and at `:883` on `Positive`. The P3 token's only mint is P3's
    `Positive` arm.
- **Readers of `link.blend.surface`** read the post-battery entity, or
  the record's numbers where they run before the battery passes:
  - `blend/junction.rs:317`, `:334`, `:353`;
  - `blend/surgery.rs:4508`, `:4526`;
  - `blend/open/planar.rs:943`;
  - `blend/reach.rs:1780` (the corner sphere's reach).

  The lane states which side of the battery each one runs on.
- **The corner entities** (`blend/open/planar.rs:249`, the corner
  sphere; `:430`, the corner torus with `major_radius: two_r`) take
  their numbers from `corner_ball`'s record.
  - `two_r = r + r` overflows above `MAX/2`. It goes through S1's mint
    here.
  - Which decision mints the corner torus's ring is Q8. Until it is
    answered, the corner torus literal stays as today, with its
    radii already minted `Size`s.
- **Evaluators stay total.** No `eval`, `eval_d`, chart or residual
  gains a `Result`.

**Claims for the reviewers to falsify.**
1. Every size an arm computes goes through the one mint.
   - The plane–sphere contact radius `sphere_trim_r = s·R/o`
     (`arms.rs:601`) is decided by no predicate (PR 7645, the further
     designer).
   - It passes an exact mint. The missing battery decision for it stays
     BAND's, filed and not built here.
2. The arms still never refuse anything a battery predicate refuses
   first. The only new arm refusal is the mint's overflow, and every arm
   refusal reaches the request as a typed `BlendError`.
3. No `Surface` or `Curve3` is constructed before the battery passes.
   "Run over the inputs before a single surface is minted" is true by
   type.
4. Only `spine_regularity`'s `Positive` arm can construct the token.
   - The token is not `Clone` or `Copy` across links, or it carries the
     link it was decided for.
   - A token from link 1 cannot mint link 2's torus.
5. No evaluator signature changed. Diff `eval*` across `geom` and
   `geom-brep`.
6. `fillet_edges(lantern, lip rim, 1.5e308)` and `f64::MAX` refuse
   typed. They panic on main (audit #35, #36).

**Rows owed.**
- The lantern at `1.5e308` and `f64::MAX`.
- A chamfer frustum whose cylinder foot lies on the axis.
- A corner at `r` above `MAX/2`.
- The pip rims' torus (spine `s`, tube `r`) at `r → s`, refusing
  `SpineIrregular` before any torus exists.
- A token-forgery compile-fail doc test, or an argument at the type.

### S3a — `cylinder-sphere-radius-witness` (L, dual)

**Gated by Q7.**

**Scope.**
- **The witness type.** `Surface::Cylinder { radius }`
  (`crates/geom/src/surfaces.rs:121`) and `Surface::Sphere { radius }`
  (`:191`) hold a witness.
  - Its mint decides `radius` positive on the run's band.
  - It hands out a `Size`.
  - Its mint takes the band, so it lives in `geom`, beside `RingTorus`
    (`surfaces.rs:656`).
- **The docs this changes.** The field stops being a raw `T`.
  - The enum docs promise struct-literal construction
    (`surfaces.rs:86`, "construction is by struct-literal variant
    syntax"). That now needs a minted witness.
  - The crate rule at `crates/geom/src/lib.rs:79`–`84` says "At rest,
    tier-3 check 1 certifies … a radius above zero".
  - Both are re-worded with the change. That is not a second decision:
    `[sizes]` decides it, as fork 274 decision 1 decided the frame.

**The census is the authority.** This stage first runs
`rg -U 'Surface::(Cylinder|Sphere) \{' crates`. It splits the results
into constructions and patterns, and the constructions into production
and test, and commits the counts in the PR body. There are about 1,100
`Surface::Cylinder {` sites and 860 `Surface::Sphere {` sites, about
half in test files.

The lists below are examples, not the census:
- **Fresh producers.** Each mints on its run's band.
  - `sweep` revolve (`crates/sweep/src/revolve/surfaces.rs:33`, `:44`)
    and extrude (`crates/sweep/src/extrude.rs`).
  - The blend's post-battery mint (S2): the arm cylinders and spheres,
    the corner sphere (`blend/open/planar.rs:249`), the junction
    cylinder (`blend/junction.rs:353`) and `planar.rs:943`.
  - `geom_brep::offset_surface` (`crates/geom-brep/src/offset.rs:333`).
    Its `offset_radius_floor` (`:344`) is the witness's mint for an
    offset, so it becomes the mint and not a second check.
  - STEP import:
    - `crates/step-import/src/entities.rs`;
    - `recognize.rs:490` (cylinder) and `:851` (sphere);
    - `normalize/singularity.rs:130`;
    - `normalize.rs` and `snap.rs`.
  - `topo::replace_face` (`crates/topo/src/replace_face.rs`, eight
    literals), `props/quad_lane.rs` and `sector_face.rs`.
- **Re-frames** carry the witness unchanged and mint nothing. A re-frame
  moves the surface and keeps its radius:
  - `topo/src/transform.rs:286`–`320`;
  - `geom-brep/src/curve_pair/on_surface.rs:308`–`346`;
  - `Surface::map_scalar` (`crates/geom/src/scalar_lift.rs:226`),
    which takes a sealed lift (ruled Q5).
- **Fixtures.** A `test-support` mint that panics on a refusal is the
  fixture door. A test-only panic is not a production one.

A pattern site that only reads `radius` reads the witness's `Size`.

**What retires in this stage.** These are the clause's "the section
arms' radius guards re-decide this fact, and retire". Each decides
`Margin::of(radius)` on a cylinder's or sphere's stored radius.
- `cylinder_radius_decided` (`intersect.rs:1180`), with its K names
  `pc_cylinder_radius` (`:1206`) and `coc_cylinder_radius` (`:3778`).
  This is the build row's second bullet.
- `cs_cylinder_radius` and `cs_sphere_radius`
  (`crates/geom-brep/src/pair_section.rs:375`–`376`).
- `cc_cylinder_radius` (`pair_section/cylinders.rs:67`,
  `CcDecision::Radius`).
- `ct_cylinder_radius` (`pair_section/torus.rs:898`) and
  `st_sphere_radius` (`:613`), both through `radius_guard` (`:359`).
- `kc_cylinder_radius`, `kc_sphere_radius` and their `_floor` rows
  (`pair_section/cones.rs:132`, `:140`, `:154`, `:162`).
- `kos_sphere_radius` (`intersect.rs:3380`).
- `carrier_sphere_radius` (`crates/geom-brep/src/carrier_eq.rs:626`),
  **only if** it decides the radius's sign and not the equality of two
  radii. The lane reads it and says which.

**The public enums that go with them**, where nothing else reaches them:
- `SectionRadius` (`intersect.rs:999`);
- `SectionError::RadiusEscalated` (`:704`);
- `BooleanDecision::Radius` (`crates/topo/src/boolean/refusal_routes.rs:157`);
- their `pncad-py` tags, their `editor-core` roster entries, and
  `docs/predicate-dimension-audit.md`'s rows.

Each retired K name leaves the recourse roster in the same commit.

**Claims for the reviewers to falsify.**
1. No production path holds a cylinder or sphere whose radius was not
   decided positive on the run's band.
   - No witness is minted on a caller's band or an offered band.
   - Name each producer's band source.
2. Each retired guard decided exactly `Margin::of(r)` on the stored
   radius, on the band the witness was minted on, so the witness's
   `Positive` implies the guard's.
   - A guard on a different margin, a floor row, or a wider band is not
     this fact, and it stays.
   - Read the `kc_*_floor` rows especially.
3. No surface transform loses the witness: a move, a reflection, a
   re-frame or a scalar lift. A scaled or offset surface re-mints.
4. An imported body whose radius misses the convention is refused at
   import. The refusal comes at the witness mint, with the import's
   typed error, and tier 3 never sees such a body.

**Rows owed.**
- For each retired guard, a row that once reached it, re-pointed to the
  producer's refusal at the mint.
- PR 7658's `a_plane_cylinder_rim_decides_the_cylinders_radius` built
  `plane_cylinder_section` on `radius: −1.0`, which can no longer be
  built.
  - It becomes "the witness mint refuses `−1.0`, zero, in band, NaN and
    `∞`".
  - Its mutant is on the mint.

### S3b — `torus-holds-ring-torus` (H, dual)

**Gated by Q7.**

**Scope.**
- **The torus.** `Surface::Torus { major_radius, minor_radius }`
  (`surfaces.rs:221`) holds a `RingTorus` (`surfaces.rs:656`).
  - `RingTorus`'s fields are already private, and its mint is
    `RingTorus::of` / `of_at` (`:720`, `:737`), plus S2's P3-token door.
  - It hands out its radii as `Size`s.
- **The spiric** (ruled Q4). `Curve3::Spiric` holds its torus's
  `RingTorus`, taken from the surface it was sectioned from (215
  `Curve3::Spiric {` sites).
  - `Curve3::spiric`'s own two decisions, `spiric_minor_positive` and
    `ring_torus_convention` (`crates/geom/src/curves.rs:1502`, the
    decisions at `:1512` and `:1517`), retire with it.
  - `RingTorus::decided` (`surfaces.rs:767`), the crate-private
    unchecked mint, retires. It is used at `curves.rs:1522`, after the
    spiric's own decisions, and at `:1567` (`spiric_torus`), which rests
    on the premise that the spiric was minted through the deciding door.
  - The spiric's arm of check 1's curve half (`curves.rs:1313`) goes.
  - So do `SpiricInvalid::MinorNotPositive` and `NotARing`, where
    nothing else reaches them.

**The census is the authority.** As S3a. There are about 780 torus
sites. Examples:
- revolve (`revolve/surfaces.rs:52`);
- the blend's post-battery mint (S2's token);
- the junction torus (`blend/junction.rs:317`) and the surgery torus
  (`blend/surgery.rs:4508`);
- the corner torus (`blend/open/planar.rs:430`, Q8);
- `offset_surface` (`offset.rs:379`–`397`), which re-mints the ring at
  `r + d`;
- STEP import (`recognize.rs:893` and the entity resolution);
- re-frames (`transform.rs`, `on_surface.rs`, as S3a);
- `map_scalar` (`scalar_lift.rs:238`).

**Readers that re-decide the ring today.**
- The convention's own rows: `torus_tube_positive` and
  `ring_torus_convention` (`crates/geom/src/rows.rs:98`).
- `require_ring_torus` (`surfaces.rs:632`) and `is_ring_torus`
  (`:648`).
- `TorusRefusal::require` / `require_at`
  (`crates/geom-brep/src/torus_convention.rs:202`, `:222`), read by
  `plane_torus_read` and the pierce normal.
- Production callers:
  - `topo/src/torus_region.rs:306`;
  - `topo/src/boolean/section_cert.rs:901`;
  - `topo/src/cover.rs:399`;
  - `geom-brep/src/variety.rs:124`;
  - `geom-brep/src/pair_section/torus.rs:248` (`Ring::of`).
- The registry of readers with their own rows is `RingReader`
  (`rows.rs:78`): `SelfIntersection`, `BooleanSheet`, `BooleanRider`.

**What retires.** Under ruled Q2, every reader above decides on the run's
band. No production reader on an offered band was found (§3, Q2). So
all of them retire:
- `Ring::of`'s two rows and `TorusRefusal::require` at the section arms;
- the topo readers;
- `RingReader`'s three non-`Convention` variants, then `RingReader`
  itself, with its roster;
- `Ring::of`'s `unreachable!("Ring::of mints a ring of finite data
  only")` (`pair_section/torus.rs:336`), which closes by type. So do the
  `Circle::new(frame, ∞)` sites in `gap::Core::of` and `winds.rs`.

The lane classifies each reader's band source before deleting it, and
keeps any it finds on an offered band, by name in the PR body.

**Claims for the reviewers to falsify.**
1. Every torus in production holds a ring decided on the run's band, or
   minted from P3's token. No path builds one from `RingTorus::decided`
   or any other unchecked door.
2. The tube half is decided first, as today (`of_at`'s order,
   `surfaces.rs:744`–`750`). So a negative `r` never reaches `R − r`.
3. The torus with `major_radius = ∞` (the build row's `Ring::of`
   reproduction) cannot be built. Its former panic sites are unreachable
   by type, not by guard.
4. Each retired reader decided on the run's band. Trace its `band`
   argument to `Band::linear` of the run's `Tol`.
5. Every spiric's `RingTorus` is its torus's own, and none is re-minted
   from the spiric's numbers.

**Rows owed.**
- The ring mint table: the tube in band, the ring in band, a negative
  tube with a "positive" ring (`r = −R`), `R = ∞`, and NaN.
- A row per retired reader, re-pointed to the producer.
- The `Ring::of` reproduction as a refusal at the mint.

### S3c — `spindle-holds-its-sheet` (M, single FULL)

**Scope.**
- `Surface::Spindle` (`surfaces.rs:258`) holds a sheet witness minted by
  `spindle_sheet` (`surfaces.rs:834`).
  - Today `spindle_sheet` returns `Result<(), SpindleMiss>`. It mints
    the witness instead.
  - It decides `r > 0` first.
  - It hands out `r` and `|R|` as `Size`s, and keeps the side of the
    signed `R`.
- Producers, as examples (the census is the authority):
  - revolve (`revolve/surfaces.rs:61`);
  - STEP import;
  - re-frames;
  - `map_scalar` (`scalar_lift.rs:247`);
  - fixtures (about 190 sites).

  `offset_surface` refuses a spindle today (`offset.rs:403`,
  `OffsetError::SpindleSheet`), and that refusal stays.

**What retires in this stage** (its own deletion):
- `topo::injective`'s re-decision (`crates/topo/src/injective.rs:660`);
- check 1's spindle arm (`crates/topo/src/validate.rs:7738`–`7750`),
  with `ValidationError::DegenerateSpindle`;
- the spindle's half of `Surface::representability_margins`'s
  minor-radius arm (`surfaces.rs:1057`–`1061`);
- the variant's `pncad-py` tag and roster entry.

**Claims.**
1. Every spindle in production holds a decided sheet.
2. The signed major radius's side survives every transform: a reflection
   and a scalar lift.
3. `injective.rs:660` and check 1 decided the same margins on the same
   band.

**Rows.**
- The mint table at each of the three ends (`SpindleEnd::Far`, `Sphere`
  and `Horn`, `surfaces.rs:790`), and an in-band row for each.
- A STEP spindle on its horn, refused at import.

### S4 — `cone-half-angle-type` (M, single FULL)

**Scope.**
- `Surface::Cone { half_angle }` (`surfaces.rs:160`) holds an exact
  type, strictly inside `(0, π/2)`, with one fallible mint and no band.
  This spec calls it `HalfAngle`; the name is the lane's.
- Its band fact **stays**: it is the lever question's, not this
  stage's. The aperture rows are:
  - `pn_aperture_sin/cos` (`intersect.rs:2514`);
  - `kos_aperture_*` (`:3396`), `kot_aperture_*` (`:3548`) and
    `coc_aperture_*` (`:3793`);
  - `kt_aperture_*` (`pair_section/cone_torus.rs:359`);
  - `kc_cone_aperture_*` (`pair_section/cones.rs:130`);
  - `section_cone_*_aperture`
    (`topo/src/boolean/section_cert.rs:1245`, `:1366`).
- Check 1's two half-angle margins (`surfaces.rs:1041`–`1050`) and the
  poison read of the half-angle retire here (ruled Q3).

**Producers**, as examples (the census is the authority):
- revolve (`revolve/surfaces.rs:40`).
- The chamfer's post-battery mint (S2), from `chamfer_frustum`'s
  `asin(|g_r|/|g|)` (`arms.rs:1562`). That value can reach `0` or `π/2`,
  so the arm's record mint can refuse there.
- The junction and surgery cones (`blend/junction.rs:334`,
  `blend/surgery.rs:4526`).
- `offset_surface` (`offset.rs:361`, which keeps α).
- STEP import (`recognize.rs:666`, `normalize/singularity.rs:133`).
- Re-frames, `map_scalar` (`scalar_lift.rs:230`) and fixtures (about
  800 sites).

**Claims.**
1. The mint refuses `0`, `π/2`, any value outside the range, NaN and
   `∞`. At `Interval` it admits an enclosure only where the enclosure is
   not wholly outside the range. The exact semantics at the two ends is
   the lane's to state and pin, mirroring `Size`'s.
2. No aperture band row is removed.
3. A STEP cone outside the range is refused at import.

**Rows.**
- The mint table.
- The chamfer arm's degenerate frustum, with its feet along the axis,
  refusing typed.

### S5 — `conic-sizes` (L, dual)

**Scope.**
- **Storage.** `Circle`'s `radius` and `Ellipse`'s semi-axes are stored
  as `Size` (`conic.rs:156`, `:185`).
- **The doors.** `Circle::new(frame, Size)` (`conic.rs:701`),
  `Ellipse::new` (`conic.rs:951`) and `Curve3::circle` (`conic.rs:1467`)
  take a `Size` and are infallible.
  - Their `# Panics` sections go.
  - `stored_size` (`conic.rs:580`) is deleted.
- **The audit's sites.** Every production site in the audit's table
  either reads a stored `Size`, or mints one and returns `Result`.
  - The audit counts 3 literals, 24 decided, 9 proven, 25 stored, 1
    door (#22), and 5 open in part (#2, #17, #36, #51, #52).
  - The first five counts sum to 62. The five open-in-part sites make
    67.
- **The builders that become fallible here:**
  - `curve_pair::on_surface`'s doors
    (`crates/geom-brep/src/curve_pair/on_surface.rs`): `circle`
    (`:413`), `parallel` (`:443`), `meridian` (`:496`), `great_circle`
    (`:704`) and `about` (`:741`). This is CPAIR's
    `on-surface-curve-doors-state-a-raw-surfaces-radius-undecided`.
    - After S3a and S3b, the cylinder's and sphere's radius and the
      torus's `R + r·cos v` are `Size`s, or implied by the witness.
    - The cone parallel's `|v·sin α|` mints exactly. Its band belongs
      to the cone lever question.
  - `topo::boolean::trace`'s `circle_crossings_to` and
    `circle_crossings` (`crates/topo/src/boolean/trace.rs:436`, `:479`).
    This is TRACE's `circle-crossings-take-a-callers-radius-undecided`.
    The caller's radius becomes a `Size` parameter.

  Both rows are moved to GERM, or closed by this PR with a note.

**What each audit disposition becomes.**

| disposition | sites | after S5 |
|---|---|---|
| stored | 25, e.g. #3, #23, #37, #46–48, #61–62, #65–67 | reads `c.radius()` or a surface witness's `Size`: no mint, no `Result`. The surface-radius readers (#24, #25, #27, #31, #38, #40, #49, #63) read S3a's or S3b's witness |
| literal | #1, #41, #50 | a named constant (`Size::ONE`) |
| decided | 24 | the producer keeps its pose decision and decides it first, then mints. A decided-positive size can fail the mint only as unbounded |
| proven | 9, magnitudes ≥ 0, e.g. #7, #11, #26, #28, #32, #55, #57, #64 | **can fail the mint at zero**, now that S1 closes the zero end. Each site either argues that its pose excludes exactly zero, or maps the zero to its producer's degenerate variant (claim 3) |
| open in part | #2, #17, #36, #51, #52 | #2: the CPAIR doors above. #17: its sign stays SECTTORUS's, and its finiteness closes by type. #36: S2. #51–52: the TRACE doors above |

Audit #64 (`ring_path.rs:541`, `parallel`) is computed from norms of
vertex offsets, which the audit's table records. It is listed under
proven, not stored.

**The retirements this stage owns.**
- **`geom::conic_size_read`** (`conic.rs:552`, re-exported at
  `crates/geom/src/lib.rs:175` and `crates/geom/src/curves.rs:69`) is
  deleted, with its three readers' finiteness halves:
  - **`plane_sphere_section_read`** (`intersect.rs:1540`, read at
    `:1605`). It decides `ps_center_gap` on the raw root, then mints. A
    root with no finite value refuses from the mint.
  - **`topo::replace_face::moved_radius`** (`replace_face.rs:1263`, read
    at `:1264`).
    - Its sign half is a pose decision, and it stays: it decides
      `offset_moved_radius` on the raw sum. `MovedCircleCollapses`
      stays.
    - Only the finiteness half retires, into the mint (ruled Q6).
    - Its readers are `transport_curve` (`replace_face.rs:1304`, every
      arm) and `topo::offset_axial::moved_seam` (`offset_axial.rs:2868`).
  - **`topo::offset_axial::latitude_circle`** (`offset_axial.rs:2844`,
    read at `:2852`). Its finiteness read goes. If
    `offset_axial_latitude_radius` asked only finiteness, the row
    retires whole, with its roster entry.
- **The ellipse door's own decisions.** `Curve3::ellipse` and
  `ellipse_anchored` (`curves.rs:1422`, `:1446`) decide through
  `ellipse_decided` (`:1462`). Both of its halves are second places:
  - `SemiAxisNotFinite`, a finiteness read, retires into the mint.
  - `ellipse_minor_positive` band-decides the conic's own size at the
    conic door. `[sizes]` puts no band there: a conic's existence is
    its producer's pose decision, or is implied by its surface.
    - It retires.
    - Each caller states the pose decision or witness that implies its
      minor axis positive:
      - `cylinders.rs` `bisector_ellipses` and `intersect.rs`
        plane×cylinder: the cylinder radius witness;
      - `intersect.rs` plane×cone: its section rows;
      - `sweep` `cap_transverse`: its own decision, which the lane
        reads;
      - the STEP `ELLIPSE` entity: the import's `positive_length`.
    - A caller with no such decision is a finding to file, and is not a
      reason to keep the door.
  - `EllipseInvalid`'s variants go where nothing else reaches them.
- **Check 1's curve half.** `Curve3::representability_margins`'s circle
  and ellipse arms (`curves.rs:1308`–`1312`) go. The spiric's arm went
  in S3b.

**Kept.** The five speed-floor readers decide span questions under
either design (PR 7645, the further designer, point 1). They stay:
- `certify.rs` check 2;
- `pcurve_cache.rs`;
- `props/curved.rs`;
- `end_face.rs`;
- `containment.rs`.

`param_rate_gate`'s conic arm also stays.

**The error each producer's mint refusal becomes.**

| producer | today, on a non-finite or zero size | after S5 |
|---|---|---|
| `geom-brep` section reads (`intersect.rs`, `pair_section/`) | a poisoned row read: `Escalated`. A proven zero is stored | a `SectionError` arm carrying `SizeRefusal`. A proven zero refuses there, or is excluded by the site's argument |
| `replace_face` transports, `offset_axial` seams | `Unmoved::Escalated` (since PR 7658) | a `Refusal::Limit` beside `MovedCircleCollapses` (ruled Q6). A NaN distance still escalates at the pose decision |
| the blend's post-battery mint | S2's arm refusal | unchanged |
| the CPAIR doors | panic on data | a typed refusal, which the lane names |
| the TRACE doors | panic on data | the caller passes a `Size` |
| STEP import | refused before construction (`positive_length`, `crates/step-import/src/interpret.rs:282`; `root_length`, `entities.rs:530`) | the import's own refusal, through the one mint |

**Sites the audit left open for finiteness** (the build row's "Open"
section). S5 closes each by type:
- `sphere_sphere_section_read` (`intersect.rs:1713`, #6);
- `plane_cone_section` (`:2472`, #7);
- `plane_torus_read` (`:3119`, #9, #10);
- `coaxial_cone_sphere` (`:3373`, #11);
- `coaxial_cone_torus` (`:3537`, #12);
- `cone_cylinder_section` (`:3735`, #13);
- `pair_section/gap.rs:150` `Core::of` (#16);
- `pair_section/torus.rs:348` `Ring::parallel` (#17);
- `pair_section/winds.rs` `channels` (#18).

**Claims for the reviewers to falsify.**
1. No production path constructs a conic without a `Size` mint or a
   stored `Size`.
   - Re-run the audit's grep and its second pass, and report the count.
   - The second pass covers turbofish, constructors passed as values,
     `map_scalar` and `quarter_turned_where`.
2. Every decided or proven site decides its pose **before** the mint,
   with no new band decision at the mint. No site replaces a pose row
   with the mint's refusal.
3. Every production path that stored an exactly-zero size is now either
   unreachable (an argument at the site) or refuses typed. Read in
   particular:
   - the 9 proven sites;
   - `AxisFrame::latitude` (`crates/sweep/src/revolve/axis.rs:87`; a
     pinned vertex is guarded at each caller);
   - `topo::offset_axial::latitude_circle` (`offset_axial.rs:2851`, a
     norm);
   - the cone transport arm (`replace_face.rs`, a parallel moved onto
     the apex).
4. `Curve3::map_scalar` takes S3a's sealed lift (ruled Q5), so a `Size`
   stays a `Size`.
5. `moved_radius` still refuses `MovedCircleCollapses` on a decided
   non-positive sum.
   - The torus parallel keeps its `ρ + d·cos v` pose.
   - PR 7658 m5's row stays green.
6. Each `Curve3::ellipse` caller names the decision that implies its
   minor axis positive.

**Rows owed.**
- PR 7658's `conic_sizes_are_decided` rows (`geom-brep` and `sweep`),
  `moved_radius_rows` and `latitude_radius_rows` are re-pointed.
  - Each pins the new refusal variant, not `is_err()`.
  - A mutant that skips the mint cannot compile, so each row's mutant
    becomes "the site decides its pose after the mint", red where the
    order matters.
- PR 7658's rows that pin `Escalated` are re-pointed to the Limit
  variant (ruled Q6):
  - `a_ball_moved_by_an_infinite_distance_refuses`;
  - `a_sphere_face_moved_past_every_finite_radius_refuses`.
- One row per audit-open site at `f64::MAX`-scale data, refusing and not
  panicking.
- A zero row for each claim-3 path.
- One row per CPAIR and TRACE door at an over-range size.
- A row per `Curve3::ellipse` caller at a degenerate minor axis,
  refusing at the caller's decision.

### S6 — `fillet-headroom-pass` (M, single FULL)

**Scope.**
- **Today.** `run_battery_for` (`battery.rs:2239`) runs `resolve_link`
  (`:1095`) on every edge. `resolve_link` classifies, decides the ball's
  room and computes the arm. P1 (`radius_headroom`, `:644`) runs
  afterwards (`:2268`–`2280`).
- **The split.** `resolve_link` splits into two passes:
  - a **classify** pass: everything up to the `BallRoom` token,
    `:876`–`883`;
  - a **compute** pass: the arm.
- **P1 runs over every classified link, between the two passes.**
  - Nothing that refuses today changes order, because today's arms
    never refuse (PR 7645, point 2).
  - S2's arms refuse only by overflow, which P1 now meets first where
    headroom can see it.
- **The comment.** The run comment "an out-of-scope pair refuses before
  any margin is taken" (`:2247`–`2248`) is already false. It is
  re-worded with the change.

**Claims.**
1. No refusal changes name or order, except at radii where P1 now runs
   before an arm that used to overflow.
   - Diff every `fillet_edges` and `chamfer_edges` row's refusal
     variant, base against head.
2. The lantern refuses `RadiusHeadroom` at `1.5e308`, as at `1e200`
   ("same situation, same name").
3. A chamfer (no P1 or P3, `battery.rs:2226`) is unchanged.

**Rows.**
- The lantern at `1.5e308`, refusing `RadiusHeadroom`.
- A two-link request whose link 2 is out of scope and whose link 1 is
  headroom-short. It refuses link 2's classification first, as today.

### S7 — `retire-rechecks` (H, dual)

This stage deletes at-rest gate checks, and a wrong retirement ships
silently, so it is H.

**Scope.** Everything still a second place once S3a–S6 have landed:
- **Check 1's surface radius half.**
  - `Surface::representability_margins` (`surfaces.rs:1035`) loses its
    cylinder and sphere arm (`:1038`) and the torus's minor-radius arm
    (`:1052`–`1056`).
  - Check 1's ring half at its call site (`validate.rs:7715`–`7736`)
    goes, with `ValidationError::DegenerateTorus` and
    `DegenerateTorusEscalated`.
  - `poisoned_datums`' entries for those datums go: a witness cannot
    hold NaN.
  - If nothing is left for `analytic_datum_verdicts`
    (`validate.rs:7308`) to read, it goes too.
- **The section-arm radius guards** that S3a and S3b did not reach. The
  lane re-greps for `Margin::of(radius)`-shaped rows on a surface's
  stored datum, and lists each with its disposition.
- **The interim guards** (the build row's list). Each retires in the
  stage named, or here:
  - `conic_size_read` and its readers: S5;
  - `cylinder_radius_decided`: S3a;
  - `nonpositive_size_gate` (`crates/sweep/src/blend/build.rs:271`–`276`),
    which refuses `NonpositiveSize` and `SizeBeyondRange`: here (ruled
    Q1);
  - `shell`'s `is_finite_length(thickness)`
    (`crates/topo/src/shell.rs:1587`, and its Display read at `:666`):
    here (ruled Q1). It has the `Interval` blind spot that PR 7658's M1
    named: `[MAX, ∞]` passes `x − x`.
- **Docs.** The at-rest prose is re-worded:
  - `crates/geom/src/lib.rs:79`–`84`;
  - the `Cone` and `Cylinder` field docs ("positive by convention", and
    "evaluated as-is … the net at rest is check 1").

  Re-wording follows the change and decides nothing new.

**Q1, as ruled.**
- Both doors mint a `Size` from the request through the one mint.
- The verb's variant wraps the `SizeRefusal`:
  - `BlendError::Size { refusal }` replaces `NonpositiveSize` and
    `SizeBeyondRange`;
  - `ShellError::Thickness` keeps its name, with the refusal inside.
- `shell_thickness`'s band decision stays. It is a band question, as
  `moved_radius`'s sign half is a pose question.
- **A flagged behaviour change.** At `Interval`, the blend door's
  bracket read (`lo() > 0`) refuses a straddling size, and the mint
  admits it. A straddling radius then meets P1 and P3, which escalate
  in band.
  - The PR body states this change.
  - A row pins it.
- `work/verdict/blend-size-gate-unmetered-under-epsilon.md` keeps its
  question.

**Claims.**
1. Every retired check re-decided a witness's fact on the same band.
   - A body imported with a bad datum is refused at import, with the
     import's typed error.
   - Run a STEP fixture with `radius = −1`, `0`, and an in-band torus.
2. `validate`'s tier-3 report on every finished body in the workspace's
   rows is unchanged, except for the removed variants, which no finished
   body can produce.
3. No `pncad-py` tag, `editor-core` roster entry or recourse-roster K
   name is left dangling.
4. The blend and shell door rows pin the wrapped `SizeRefusal`,
   including the `Interval` straddle.

**Rows.**
- The STEP fixtures of claim 1.
- A tier-3 row that read `DegenerateTorus`, re-pointed to the import
  refusal.
- The door rows of claim 4.

## 2. Territory

These stages edit other programs' files. Run
`python3 scripts/work.py territory` at each dispatch.

| crate / file | stages | program's territory, as PR 7658 found it |
|---|---|---|
| `geom/src/curves/conic.rs`, `curves.rs` | S1, S3b, S5 | CONIC / ONECOPY |
| `geom/src/surfaces.rs`, `rows.rs`, `scalar_lift.rs` | S3a–S4, S7 | shared |
| `geom-brep/src/intersect.rs`, `pair_section/` | S3a, S3b, S5 | shared, SECT, SECTTORUS |
| `sweep/src/blend/` | S2, S6, S7 | BAND |
| `topo/src/replace_face.rs`, `offset_axial.rs`, `shell.rs` | S5, S7 | SHELL |
| `topo/src/validate.rs` | S3c, S7 | the at-rest gate's owner |
| `step-import/src/` | S3a–S4 | IMPORT |

The rows S2 and S5 close belong to other programs: BAND, CPAIR and
TRACE. Each is moved to GERM, or closed by the stage's PR with a closing
note, as PR 7658 closed SHELL's.

## 3. Rulings and open questions

### Ruled (the orchestrator, 2026-11-26, on this spec's first review)

- **Q1. User-facing sizes at the door** (the blend size and the shell
  thickness): they go through the one mint, and the verb's variant
  wraps `SizeRefusal`. The `Interval` straddle is a flagged behaviour
  change. Built in S7.
- **Q2. Ring readers outside the section arms:** classify by band, and
  retire those on the run's band.
  - **The evidence for "no production reader on an offered band".**
    `Indeterminate::offered_band`
    (`crates/geom-core/src/predicate.rs:3490`) has two production
    callers: `geom-brep/src/dihedral.rs:1100` and `:2423`. Both re-quote
    a wedge refusal at a smaller tolerance, and neither reads a torus.
  - `require_ring_torus` on `offered` (`surfaces.rs:2569`) is test code.
  - S3b's lane confirms each reader's band source before deleting it
    (S3b claim 4).
- **Q3. Check 1's cone and poison halves** retire with the type that
  makes them vacuous: S4 for the cone, and S3a–S3c for poison.
- **Q4. The spiric** holds its torus's `RingTorus`, and
  `Curve3::spiric`'s own two decisions retire with it. Built in S3b.
- **Q5. `map_scalar`** on a witness or a `Size` takes a sealed lift
  trait. Its implementations are the production lifts PR 7658
  enumerated.
- **Q6. An over-range offset** is a `Refusal::Limit` beside
  `MovedCircleCollapses`. Built in S5.

### Open

**Q7. Which run's band does a witness belong to?** This gates S3a and
S3b.

A witness is decided on the band of the run that produced the surface.
`[sizes]` says "on the run's tolerance and never on a caller's band". A
later run that reads the body at a different ε would hold a witness
decided at the wrong ε. The paths to check are an import, the editor
changing ε, and a replay.

What the tree shows on whether any production path consumes a body at
an ε other than the one it was built at:
- **One ε per process, committed once and never loosened.**
  - `geom_core::tolerance` holds a `OnceLock` (`crates/pncad/src/tolerance.rs:1`–`11`,
    S22, D4 ¶1).
  - A document's recorded ε outranks the environment. A disagreement
    refuses at load (`ToleranceConflict`, mapped to
    `ResolveFault::EpsilonSeam` at `crates/pncad/src/workspace.rs:842`).
- **The editor changing ε replays in a fresh process.**
  - `DocEdit::SetTolerance` (`crates/editor-core/src/edit.rs:536`–`548`)
    replays through save and load.
  - In the editing process, `eval::evaluate` refuses a document whose
    recorded ε disagrees with the committed ε.
  - So every body is rebuilt from the recipe at the new ε, and its
    witnesses are re-minted.
- **Bodies are not persisted as B-rep.** No `Deserialize` exists for
  `Surface` or `Body` in `geom` or `topo`. A document is a recipe, so a
  load rebuilds and re-mints.
- **STEP import is the only B-rep-from-file path.** It mints in the
  importing process, at its committed ε.
- **A cross-document reference** (`editor-core/src/refactor.rs:1971`,
  `:3992`) sets the referencing document's ε to the referenced one's.
  It does not read the referenced document's bodies at a second ε.
- **The one cross-ε reading in production** is `offered_band`
  (`predicate.rs:3490`, see Q2). It is a smaller tolerance, used only to
  re-quote a refusal.
  - A margin decided `Positive` at ε stays `Positive` at any smaller ε′
    on the linear band, since its escalation edge `K·ε′` is smaller.
  - So a witness read at an offered tolerance would still hold. This is
    likely, not checked at every band shape.

*Reading:* no production path was found that consumes a body at an ε
other than the one it was built at. This is left open. If a path is
found, for example a cache keyed on bodies across a tolerance change,
or a band shape where a smaller ε does not preserve `Positive`, it is a
fork, and it waits for designers.

**Q8. Which decision mints the blend corner torus's ring?** This gates
S3b's corner-torus site.

The corner torus (`blend/open/planar.rs:430`) has `major_radius: two_r`
(`two_r = r + r`) and `minor_radius: r`.
- P3 does not cover it: the corner has no spine regularity.
- Its ring half is `R − r = r`, and its tube half is `r`. Both are the
  blend radius's own positivity on the band.
- Today nothing decides `r` positive on the band. The blend door's gate
  is a bracket read (ruled Q1: an exact `Size`), and
  `work/verdict/blend-size-gate-unmetered-under-epsilon.md` owns "a
  positive size below the band".

*Recommendation:*
- The corner's post-battery mint mints its `RingTorus` through
  `RingTorus::of` on the run's band, like any producer. "Minted where
  the surface comes to exist" is exactly this site. The `Size` of
  `r + r` is minted first, so overflow refuses as the arm's own
  refusal (S2).
- A sub-band `r` then refuses late, at the corner, after the battery
  has passed. That breaks the battery's "construction cannot fail"
  claim.
  - That is the verdict row's existing question.
  - Note it there, and do not add a second decision here.
- The corner spheres (`planar.rs:249`, `reach.rs:1780`) take the same
  posture through S3a's witness mint.

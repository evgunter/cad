---
id: a-mate-placed-seat-is-unproven-at-the-doors-budget
kind: issue
title: a mate-placed seat replays at Sym but stays unproven within the door's budget: the fold's frames hold component symbols renormalised at every hop
status: closed
opened: 2026-10-18
closed: 2026-10-22
priority: P0
cost: H
design: true
refs: [placed-carriers-compare-through-their-frames, mates-declare-no-contact, a-pose-is-moved-by-lengths-and-angles, the-door-reads-a-mated-seat-by-its-mate, the-census-records-a-planar-area-contact-as-one-face-pair]
---

## What

D10 says a mate-placed face is structural. The door does not prove one.
Stage 4 H (PR 5300) built the replay that would read it: each bundle
folds at `Sym`, and the kernel's carrier verdict runs on the two copies'
faces (`crates/editor-core/src/coincide/replay.rs` `prove_row`, the
`CensusAtRest` arm, with parts resolved at `Sym`). That replay is behind
`coincide::REPLAYS_PLACED_COPIES`, which is off. Switched on, a mated
seat is `Numeric`, not proven, at a cost the door cannot pay for nothing
(below). Pinned off and unproven by
`mate6_gather_mints::a_mated_record_cites_the_census_row_the_lint_reports`.
It is visible as the tour bench's three `unproven-coincidence` findings
(`demos/tour/src/assembly.rs` `bench`). The same holds for contacts
between patterned copies, which stage 4 §9 also expected to shrink.

## The cost the fix must also beat

Measured with the switch on, locally (debug), against main:

| viewer row | main | switch on |
|---|---|---|
| `tree_badges::a_union_over_a_chained_copy_points_at_the_refused_placement` | 0.28 s | 4.3 s |
| `story_assembly::the_windmill_story` | 1.07 s | 19.1 s |

About 4 s is resolving the parts at `Sym`. About 15 s more on the
windmill is the carrier verdicts on its seats, whose forms freeze past
the budget and pay the retry ladder. Turning the switch on is part of
this row's fix, and it must not cost that.

## Measured (H lane, 2026-10-18)

The carrier verdict needs the copy's face normal to equal the target's as
a polynomial identity. Cap normals built the way an extrude builds them
(`normalize(u × v)` over a `Datum::Frame`'s six component symbols):

- every factor of the fold proves at the door's 256 terms in isolation:
  a pose's frame (`mate/solve.rs` `frame_of`, the decided shortest turn)
  and the value's turn `R(d, v − atan2(…))`;
- their product does not. `F_to·F_onᵀ` applied to the copy's normal is
  unproven at 256 and proven only from 1024 terms. The spin step
  `R(n, θ)·n = n` is proven only at 16384 terms with a hand-written
  Rodrigues on the unit axis. Through `Mat3::rotation_about`, which
  renormalises an axis that is already unit, it is unproven even at 16384;
- applied as a chain of factors rather than one composed map, the same
  identities collapse step by step (`F_onᵀ·n` is exactly `z`), but only
  from 1024 terms with these frames. The door's own measure is 0.9 s at
  256 terms against 172 s at 1024 (`coincide/replay.rs` `BUDGET`).

With unit-axis frames over three free symbols (degree-1 normals) the
chain proves at 256 terms. So the cost is the frames' component symbols
and the renormalisation at every hop. It is not the fold's structure.

## First step

Re-measure after D, with the switch on, on mate6's stand and on the
tour bench: does a mated seat prove at the door's 256 terms once frames
are `Slide`/`Spin` chains, and at what cost? The identity probe is
`intent_s4_h_placed::measure_a_mated_seat_at_sym` (`#[ignore]`d; its
run command is in its doc). If a seat still does not prove, run the
part-variable fork (route 0) through designers (orchestrator,
2026-10-18), then weigh the other routes.

## Routes (a design question, weigh before building)

0. **A part's variables as constants of the assembly.** This is a fork
   between D10's "every variable a symbol" and AQ4's "instantiation
   takes no arguments". H binds each document version's variables as
   its own symbols. R1 of H's review measured the other reading: with a
   part's variables bound as numbers, mate6's mated seat proves at 256
   terms and the lint reports nothing. The cost is that it would also
   cut tolerance stack-ups across parts, which is the main thing
   symbols buy, so it is not free.

1. Frames from slides and spins, with no component slots (FORK-MOVE).
   Stage 3 D (`transform-retires-into-a-placement`) retires `Datum`
   entirely, so every construction's pose becomes a `Slide`/`Spin` chain
   over the world read. Normals then become trigonometric polynomials
   with no `sqrt` chain. This is the hypothesis the first step tests.
2. Place a copy factor by factor at `Sym` (prototyped on the H branch and
   dropped: lane-dependent arithmetic, with no proof at budget under
   today's frames). This is likely still needed after route 1, because a
   composed map hides each step's collapse.
3. Stop renormalising a vector that is unit by construction on the fold's
   path (`Affine3::rotation_about_axis` over a `UnitVec3` in
   `mate/value.rs` `settle`; the chart's `UnitVec3::new` on rotated unit
   vectors).
4. A tier capability for a `sqrt` whose argument is identically one after
   rational cancellation (SYM's ground).

`mates-declare-no-contact` (stage 4 I) waits on this. Retiring
`ContactClass` before the door proves a mated seat would turn every
resting mate into a finding.

## Other sites on the fold's path that build a map in a form the tier reads poorly (H's sweep, 2026-10-18)

These were not measured. They sit on multi-mate bundles; the measured case was one plane mate and its values:

- `mate/coset.rs` `ball_pair`: `chord.orthonormal_basis()` (min/max/select atoms) in the antipode arm.
- `mate/coset.rs` `candidate_rotation`, `clocking_about`, `ball_on_line`, `ball_pair`: `Mat3::rotation_about`, which renormalises a unit axis, over a `solve_atan2` angle.
- `mate/coset.rs` `clocking_about`, `ball_on_line`, `ball_pair`: a representative inverted by `Affine3::inverse` (the adjugate over the determinant) where the map is rigid by construction.

## Owed when `coincide::REPLAYS_PLACED_COPIES` turns on (H's review, both reviewers)

The switched-off code in `coincide/replay.rs` was reviewed but has no row. Both reviewers' mutants survived on it:

- **A positive row:** an at-rest row the carrier verdict proves `Structural`. R2 measured union rows between two copies of one part at their identity pins proving by the carrier verdict; an at-rest row that proves is still to be constructed (copies at one pin overlap, so a resting pair needs an offset pin).
- **The pin's threading:** a resolved part is bound under its own pin. The scope reaches `evaluate_at_descent` from the part store, and nothing pins that: R1's mutant passing `None` there survives. Only `var_env_symbolic_pinned` called directly is pinned.
- **The relation guard:** a verdict of another relation proves nothing (`carrier_verdict`'s `relation == wanted`).
- **`at_rest_unread`:** a copy the replay did not build says `ThroughAPart`/`Refused`, not `NotReplayed`. A replay that decides the pair otherwise needs a reason of its own, not "not two faces".
- **Cancellation:** the census arm stops canceled.
- **The band:** a band that fails reports as a band failure, not as no verdict (the arm's `Option<Option<…>>`).
- **The recourse:** a mated seat that is only over the budget is `Undischarged::Otherwise`, so its recourse reads "build the two cells as one construction". The user already built it as one construction, so the recourse must name the budget, not ask for a construction.

## Measured after D (2026-10-21)

All runs were local, in debug, on main at `eb6894fe` (stage 3 D,
PR 5506), with `coincide::REPLAYS_PLACED_COPIES` switched on unless
marked "main". `n` terms means the door's `BUDGET.max_terms` set to
`n`. The local scratch instrumentation is not committed.

**Which mated seats prove**

| case | main | on, 256 | on, 512 | on, 1024 | on, 4096 / 16384 |
|---|---|---|---|---|---|
| mate6's stand (one plane mate, cube on cube) | `NotReplayed` | `Numeric`, recourse `OneConstruction` | **proven** | proven | proven |
| a top seated on a linear pattern's copy 2 (`fix_pattern_mate_crossing`'s legs, spacing 1 and 2) | `NotReplayed` | `Numeric` | `Numeric` | `Numeric` | `Numeric` / `Numeric` |
| tour bench, 3 seats (shelf on near post #16, far post under shelf #18, crate on shelf #44) | 3 × `NotReplayed` | 3 × `Numeric` | 3 × `Numeric` | 3 × `Numeric` | 3 × `Numeric` (4096) |

- `mate6_gather_mints::a_mated_record_cites_the_census_row_the_lint_reports`
  goes red with the switch on. At 256 its row is `Numeric` with recourse
  `OneConstruction`. From 512 the stand has no findings.
- `asm2a_instantiate::a_symbolic_run_builds_a_part_and_the_door_leaves_a_row_on_one_unreplayed`
  goes red too: with the resolver, its wall row is `Numeric`/`OneConstruction`.
  That row is not a mated seat. It is a part's width against a pin written
  as a translation by `1`, so it is not an identity, and route 0 leaves it
  `Numeric`.

**The door's cost**

Viewer rows, debug, seconds, the better of two runs:

| viewer row | main | on, 256 | on, 512 | on, 256 + route 0 | on, 256 + cap normal from the sketch |
|---|---|---|---|---|---|
| `tree_badges::a_union_over_a_chained_copy_points_at_the_refused_placement` | 0.25 | 2.39 | 3.70 | 1.12 | 2.22 |
| `story_assembly::the_windmill_story` | 0.58 | 4.46 | 8.98 | 2.64 | 7.67 |

H measured 4.3 s and 19.1 s for these two rows. Today it is 2.4 s and
4.5 s at 256, about 10× and 8× main.

The tour bench's `run_checks` (the bench is the stand plus the crate)
costs:

| budget | main | 256 | 512 | 1024 | 4096 |
|---|---|---|---|---|---|
| bench | 0.026 s | 2.8 s | 9.1 s | 21.0 s | 112 s |
| stand | 0.018 s | 1.8 s | 5.3 s | 10.2 s | 24.9 s |

None of the bench's three seats proves at any of these budgets. The tour
bench's `unproven-coincidence` findings stay at 3 in every
configuration, switch off or on. Only their residual changes, from
`NotReplayed`/`Assert` to `Numeric`/`OneConstruction`.

**Patterned copies**

- **Pattern copies that touch.** A linear pattern of unit cubes at
  spacing 1, each copy placed, gives 32 `unproven-coincidence` findings
  with the switch off and the same 32 with it on. All of them stay
  `NotReplayed`. The census records an unmated flush contact as
  edge–edge and vertex–vertex `OnCarrier` rows only (16 + 16 here), and
  no face pair. The replay's census arm reads a face pair alone
  (`carrier_verdict`), so `at_rest_unread` answers `NotReplayed` for
  every row.
- **Two stood parts.** The same holds for two different parts, both
  stood by the identity bundle, flush at x = 1: 4 edge–edge and
  4 vertex–vertex rows, all `NotReplayed`, under route 0 too.
- **A seat on a pattern's copy** is `Numeric` from 256 to 16384 terms.
  It proves under route 0, and under the cap-normal change below.

**What D changed, and why**

- **The six-symbol datum frames are gone.** A construction is built in
  its own coordinates, and frames no longer carry `Datum::Frame` component
  symbols. Read at `Sym`, mate6's two seated cap normals carry no frame
  symbol. Their symbols are the parts' own variables.
- **The extrude's cap normal still renormalises over those variables.**
  `sweep/src/swept.rs` `cap_plane` takes Newell's normal,
  `normal_sum.normalize()` over the profile's points
  (`geom_brep::newell::newell_plane`), which is `(0, 0, A)/sqrt(A²)`.
  The mate path then renormalises the already-unit result again at the
  pose read, `eval/wire.rs` `pose_unit` (`UnitVec3::new`), and in the
  chart, `mate/value.rs` `Chart::of`'s `unit(…)`. The rendered forms show
  `sqrt(… sqrt(…)^2 / sqrt(…)^2 …)` chains. So the item's "component
  symbols renormalised at every hop" is now "the profile's variables,
  normalised at the cap and renormalised at every hop".
- **The probe now isolates the failing component.** Reading mate6's two
  seated cap carriers at `Sym` directly, at 256 terms:
  - `n1.x + n2.x`, `n1.y + n2.y`, `|n1 × n2|` and `(o2 − o1)·n1` prove;
  - `n1.z + n2.z`, `n1·n2 + 1`, and the offset at a free centre,
    `n1·(o1 − c) + n2·(o2 − c)`, do not. Their forms freeze;
  - all of them prove at 1024.
- **The door's verdict needs one budget step less than the probe.** It
  proves at 512. Below that it fails on the parallelism Zero,
  `|n1 × n2|·arm`, where the arm is `pair_extent`'s min/max radius over
  the two faces' vertices.
- **Exact turns help only where a spin is written as a turn.**
  `turn_to` takes `rotation_about_unit_axis_by` for a value whose angle
  is a constant fraction of a turn, with no renormalisation. The tour
  writes each mate's `Spin` as `Formula::literal(0.0, Angle)`.
  `Expr::constant_turns` reads only `0`, `turn` and rational forms of
  them, so a literal binds as a symbol and its `sin` and `cos` are atoms.
  The failing tour forms are polynomials in `sin`/`cos` of three such
  symbols, and of the swing and of `pin_at`'s gimbal turns. Those turns
  are f64 radians from `mate::pin::turns`, the nearest π/2 plus a
  symbol, not `turn/4`.
  - Writing the near post's pin as a bundle with no mate and exact zero
    spins changed nothing.
  - Removing the swing as well changed nothing.
  - What remains is the mates' own literal-zero spins, through two hops.
    This is H's spin step, `R(a, θ)·a = a` over an axis that is unit
    only after renormalisation.
  - mate6's single hop is not affected: a spin about the seat's own
    normal cancels there.
- **The identity bundle's "places by no map" does not make a seat
  trivial.**
  - mate6's lower cube stands by the identity bundle, and its seat still
    reads the mated copy's fold.
  - Between two copies that both stand by no map, the census records no
    face pair at all (above), so the replay has no row it can read.

**Route 0 (a part's variables bound as numbers in the replay)**

The scratch change bound a pinned part's variables as their values in
`analysis::var_env_symbolic_pinned`; the document's own variables stay
symbols. At 256 terms:

- **mate6:** the stand has no findings. The probe proves all of its
  identities, and its frozen forms go from 59 to 10.
- **A seat on a pattern copy:** proves.
- **Tour bench:** the shelf's seat on the near post (#16) proves. Both
  two-hop seats (#18 and #44) stay `Numeric`, at 1024 too, because their
  residue is the assembly's own spins (above), which route 0 does not
  touch.
- **Cost:** the viewer rows take 1.12 s and 2.64 s; the bench takes
  1.1–1.2 s.
- **Rows between two copies that touch:** these stay `NotReplayed`, as
  before.

**Measurements that separate the other routes**

- **Route 1 (frames as slide/spin chains) is in place**, and is not
  enough on its own. mate6 needs 512 terms, and the tour proves nothing
  at any budget measured.
- **Route 3 (stop renormalising a unit vector).** A scratch change kept
  `pose_unit`'s input as given and gave `Sym` the frame lanes' spellings
  in `Chart::of`, `turn_to` and `settle` (`spins_by_frames`). mate6
  still needs 512. Route 3 does not reach the cap's own `sqrt(A²)`.
- **A new site (route 5): the cap carrier from the construction.** The
  scratch change made `cap_plane` return the sketch plane's outward
  normal once the orientation was decided.
  - mate6 proves at 256 (frozen 59 → 12), and so does the seat on a
    pattern copy.
  - The tour stays 3 × `Numeric`. Its residue is the spins.
  - The viewer rows take 2.22 s and 7.67 s. The windmill is slower than
    with the switch alone: verdicts that froze early now run further.
  - This is D10's own wording for an extrude's cap, so it is filed as a
    defect on CARVE:
    `work/carve/an-extrude-cap-plane-normal-reads-the-profile-variables.md`.
- **Combined.** The cap change with route 0 matches route 0 alone on the
  tour: #18 and #44 stay `Numeric`.
- **The H lane's probe** (`intent_s4_h_placed::measure_a_mated_seat_at_sym`)
  proves nothing at 256. At 1024 the frame and the chained identities
  prove, while the composed map and `R(n, θ)·n` do not; H recorded the
  composed map proven from 1024. It
  builds the six-component frames D retired, so it no longer models a
  D-era seat. The direct read of the two copies' carriers above is the
  probe that does.

**What this implies for the design question** (facts for the
designers, not a recommendation):

1. **mate6's one-hop seat** is within reach of two independent changes,
   each at 256 terms:
   - the cap carrier stated from the sketch plane (route 5, a D10
     conformance fix);
   - route 0.
2. **A seat whose target is itself placed by a spun mate** is not
   reached by either change. That is the tour bench's #18 and #44, and
   any chain of seats.
   - The spin values are symbols, so the door would need `R(a, θ)·a = a`
     through a renormalised axis (routes 2–4).
   - Or a spin written as an exact turn would have to stop being a
     variable, which is route 0's question asked of the assembly's own
     values.
3. **Unmated contacts between copies** (patterned or stood copies) reach
   no rung, whatever the frames. The census records them as edge and
   vertex rows, which the replay's census arm does not read. The owed
   list's `at_rest_unread` bullet is the start of this; a reading of
   such rows is a design question of its own.
4. **The cost** of the switch at 256 is now about 2–4 s on the viewer
   rows and 2.8 s on the bench, with no seat proven there. At the 512
   that mate6 needs it is 3.7–9 s.

## Closed (2026-10-22, for mated rows)

The ruling on PR 5555 answered the design question: a mate is the first
rung's statement, so a mated seat is read by its mate and never by the
replay (fork log row 187; routes 0–3 do not apply to it). It is built
by `the-door-reads-a-mated-seat-by-its-mate`, and the switch is gone.
mate6's stand, the tour bench's three seats and a seat on a pattern copy
are proven by the first rung, and the viewer rows run at main's times.
Rows no mate states (unmated contacts between copies, recorded as edge
and vertex rows) are `the-census-records-a-planar-area-contact-as-one-face-pair`'s.

The owed list, bullet by bullet:

- **A positive row:** `mate6_gather_mints::a_row_at_rest_no_mate_states_is_read_off_the_replays_copies`.
  An at-rest row between the tops of two copies of one version, each
  stood by the identity bundle, is `Structural(CarrierVerdict)`.
- **The pin's threading:** the same row over copies of two versions of
  one document (the second relabelled) is `Numeric`: each version's
  variables are symbols under its own pin through the resolved part's
  evaluation. Passing no pin there makes the two heights one symbol,
  and the row would prove.
- **The relation guard:** `mate6_gather_mints::a_mate_states_its_own_seat_and_no_other_row`
  hands the door the seat's caps decided `SameOriented`. The first rung
  leaves it (the mate's flip is odd), and the replay's verdict decides
  `SameOpposite`, so the row is `OtherDecisions`. Without the guard the
  margin would be read, and at the door's budget it is `Numeric`.
- **`at_rest_unread`** is now `at_rest_unbuilt`, asked only of a face
  pair the replay reads. A copy the replay did not build says
  `ThroughAPart` or `Refused` (the positive row's door handed no
  resolver says `ThroughAPart`). A verdict that decides the pair
  otherwise says `OtherDecisions`, and a row of other cells says
  `NotReplayed` without replaying.
- **Cancellation:** `prove_row` checks the token once before either arm,
  so the census arm has no check of its own to drop.
- **The band:** no longer applies. A run whose tolerance makes no band
  refuses every node before an operation runs, so the replay builds no
  copy, and the row says why through `at_rest_unbuilt`. Past that, the
  band is built; a run that reaches the arm without one says `Refused`.
- **The recourse:** a mated seat is never over the budget now. Any row
  whose form freezes on the budget names it: `Recourse::Budget`, read
  off `Undischarged::OverTheBudget`, which only a term or degree bound
  answers (a node the ring's guards froze reads `Otherwise`). This moves
  the filleted box's eight mitres, the ring's fusion and the flush
  plate's rows from `OneConstruction` to `Budget`.


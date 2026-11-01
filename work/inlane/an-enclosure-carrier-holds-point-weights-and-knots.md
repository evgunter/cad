---
id: an-enclosure-carrier-holds-point-weights-and-knots
kind: issue
title: May a carrier at an enclosure scalar hold non-point weights and knots? Until it may, an arc section, or an interior averaged knot wider than a point, refuses where the loft should enclose its family
status: open
opened: 2026-10-11
priority: P1
design: true
cost: H
refs: [the-skin-at-interval-encloses-the-nominal-skin]
---


## Finding

Since `the-skin-at-interval-encloses-the-nominal-skin`, a loft or sweep
at an enclosure scalar (`Interval`, and `Dual`/`Sym` over it) skins
sections wider than a point in `T`, and the walls enclose the family
(`crates/sweep/src/loft.rs`, `LoftBuild::enclosure`). That holds only
where the family's walls are carriers. A carrier at every scalar holds
point weights (`crates/geom/src/net.rs`, `weight_channel`, refusing
`SplineError::WeightNotAPoint`, since S2 the door's `CarrierWeightNotAPoint`) and point knots
(`geom_core::spline::Knots::new`, refusing `KnotNotAPoint`), because a
certificate reads a payload's weights and knots on their `f64` value
channel (`certified_coords`'s doc in `crates/geom/src/net.rs`; C6 in
`crates/geom-brep/README.md`: "a value that names no single point
refuses typed"). Two cases hit this:

- **Every arc section.** The weight `cos(Δ/2)` of an arc's sub-arcs is
  an enclosure at `Interval`, from outward rounding alone, even at a
  point sweep. `sweep::segment_curve` refuses
  `SkinError::ArcWeightNotAPoint`. Pinned by
  `crates/sweep/tests/a_loft_computes_its_walls_in_t.rs`
  (`what_a_carrier_cannot_hold_refuses_typed_at_interval`). Every
  document arc loft at `Interval` under the guided lift reaches it
  (`crates/editor-core/tests/e6_drive_over_a_loft_at_interval.rs`: the
  arc row still bisects to its leaf budget). Sections whose values are
  all points still loft through the lifted `f64` skin.
  (S3 retired `ArcWeightNotAPoint`: the weight is held in `T` and
  stops at the carrier's door, `CarrierWeightNotAPoint`, until S4/S5.)
- **More sections than `v_degree + 1`.** The interior averaged knots
  are chord-length parameters (Book Eq. 10.8, `skin_parameters`). Over
  a wide section they are wide, and `Knots::new` refuses them as
  `SkinError::Fit(FitError::Structure(KnotNotAPoint))`. Pinned in the
  same test. With `k ≤ v_degree + 1` the knots are the clamped ends,
  and wide parameters only enter the collocation matrix, which does
  enclose.

The knot algebra (`make_compatible`'s elevation and refinement) does
not run at an enclosure scalar either (`SkinError::IncompatibleAtEnclosure`).
It is reachable only once arcs are, since two line strips are always
compatible.
(S4 runs it there, homogeneous, and retires that refusal; a rational
section's refined weights stop at the carrier's door until S5.)

## Options (a design fork; not chosen)

- **A. A carrier holds enclosure weights and knots.** Its structure is
  what every member shares: the span order certified over the knots'
  brackets, with the locator hulling the candidate spans. Every
  `Interval` reader of a carrier's `f64` structure would then read
  enclosures: `certified_coords` and the hull bounds, `RingRefinement`
  plans (built from `f64` knots), span location, and the pcurve
  caches. This is sound and closes both cases. It crosses
  geom-core/geom/geom-brep and re-words C6, which is a
  ratified-clause change.
- **B. Fix the parameterization on a witness.** The averaged knots
  become the `f64` ones of a nominal member. Over a real box the walls
  then enclose "each member interpolated at the witness's parameters",
  not each member's own `f64` loft. That is a nominal answer over a
  box, unsound against the `f64` lane at a sample (Monte Carlo, E6).
  It does nothing for arc weights.
- **C. A value-independent v-parameterization in every lane** (uniform,
  say). The knots are then points everywhere. This changes every loft's
  `f64` surface and re-baselines broadly, which is a definitional choice
  (Q8, Book Eq. 10.8). It does nothing for arc weights.

## The premise, checked

Option A amends C6's sentence "a value that names no single point
refuses typed" (`crates/geom-brep/README.md`). `git log --all -S'names
no single point refuses typed'` finds it written at f7816d7771
(2026-10-11, "C6 re-worded ... lands with u2's carrier change"). That
was the INTENT orchestrator's session, not an Ev ruling. C6's
certified-refinement sentence ("its refined weights are generally not
`f64`, so it is held only as homogeneous enclosures") is older, and
traces to the `[ev]` PR at 43d0fcad61 (fork3, 2026-10-10), so it is
Ev-ratified. A designer pair should read that PR's ruling before
weighing A, since A would let a carrier hold what that sentence keeps
in homogeneous form.

The two designers' weighing (`memories/orchestration-model.md`) comes
first. A and C touch ratified text, so whichever is chosen goes to Ev
as an `[ev]` PR.

## Recommendation (both designers, converged after one round)

Option A, framed as a correction rather than a permission (fork-log row
153, byte 96: A = Opus, B = Fable). C6 re-worded in this PR:

- A carrier at an enclosure scalar stands for a family, so its knot
  positions and weights may be enclosures as its poles are. Structure
  is the combinatorial part (degree, counts, multiplicities,
  breakpoint order, the skin's and fit's discrete decisions), decided
  once for every member: certified at an enclosure scalar (runs ascend,
  weights positive, the span read hulls every reachable span).
- A structure decision whose members may disagree refuses typed,
  naming it and carrying its margin (as `LoftError::StackingEscalated`
  does), so the E6 drive bisects and a sliver is terminal. A shortcut
  decision (equal weights, the integral test) takes the general route
  instead; `held-value-equalities-decided-on-the-value-channel` folds in.
- The sub-arc count rule moves its breakpoints off multiples of π/2 by
  a named η: an interval bracket contains the true sweep, so a
  right-angle fillet, a circle half or a bulge of 1 straddles an exact
  breakpoint whatever the rounding. At `f64` an arc whose sweep rounds
  just past `k·π/2` (a quarter-circle bulge, `4·atan(tan(π/8))`) now
  counts `k`, one fewer than before; S3's PR lists each count that
  moved. Form constants enter as `T::pi()`.
- The skin's compatibility step at Interval runs the homogeneous ring
  algebra (fork3's sentence, kept verbatim, becomes the general rule).
- The stored net stays Cartesian `(P, w)`.
- B (witness parameterization), C (uniform v) and E (an f64-lifted arc
  weight) are rejected: each picks a representative member or changes
  every loft's surface, and none covers weights.
- PR 85's signed sentence (a fit's structure is the f64 pipeline's) is
  unchanged: a loft is not a fitted cache.


## Build (the INLANE orchestrator's slices)

- **S1, knots (PR 5277):**
  - `KnotStructure` is combinatorial: it holds degree and runs, and no positions.
  - `Knots` holds wide breakpoints, certified for every member (`KnotRunsUndecided` with its margin).
  - `map` certifies what it does not keep, and the span read hulls.
  - The interim: `*_f64_interim` accessors and the ratchet gate `scripts/gates/knots-f64-interim.sh`.
  - A carrier and the fit refuse a wide breakpoint typed (`CarrierKnotNotAPoint`) until their readers move.
- **S2, weights (staged as S1, the orchestrator's ruling (A)):**
  - `Weights<T>` holds weights, certified strictly positive for every member (`WeightSignUndecided` with its margin).
  - `weights_f64_interim` refuses loud, and the gate counts it.
  - The carrier door refuses a wide weight as `CarrierWeightNotAPoint` (renamed from `WeightNotAPoint`) until S4/S5 move the knot algebra and the certificates.
  - Also in S2:
    - the gate counts the f64 knot and weight holders;
    - `Knots::value_at` refuses wide knots typed. Its callers carry the message "a carrier's door admits only point knots (`CarrierKnotNotAPoint`)", which S4/S5 grep for.
- **S3, arcs:** the η sub-arc rule and `T::pi()` (Q1, Q2).
- **S4, the homogeneous knot algebra:** `apply_certified`'s elevation and removal.
- **S5, fit and skin:** the collocation's typed refusal. S5 deletes the interim accessors and their gate.
  - **The end state (the orchestrator's ruling, 2026-10-18).** The door lifts, and a carrier may hold wide knots and weights.
    - Each panicking interim accessor becomes a typed-refusing one, e.g. `knots_f64() -> Result<&KnotVector, CarrierKnotNotAPoint>`.
    - A reader with a meaning at a wide carrier moves to the held values or the structure.
    - A reader that means something only at points propagates the typed refusal (D4) and never swallows it.
    - A ratchet beside the dying gate counts `.unwrap()`, `.expect(` and panicking `?` uses of the typed accessors, and is driven to 0 by S5z.
  - **The staging, each slice its own PR:**
    - **S5b:** the plan decisions move onto the held values, with certified margins.
    - **S5c:** the fit's collocation encloses over a wide averaged knot.
    - **S5d onwards:** the readers migrate crate by crate.
    - **S5z:** the door lifts, and the interim accessors and their gate go. Its PR cites C6's text and the `git log -S` provenance. Text C6 does not already have goes back to the orchestrator.
- **S6:** the two fold-ins, `held-value-equalities-decided-on-the-value-channel` and `an-edges-parameter-ends-enter-a-carrier-piece-as-constants`.

### Left for S2–S5 (S1's review, PR 5277)

- **`f64` knot and weight vectors copied out at an interim read.** S2 counts them as the gate's holders ratchet (44 at PR 5287).
  - `SplineCoeffs::knots`, `TensorNet::knots_u/v`, `Collocation`, `projected::breaks`, the ssi refinement's `ku/kv`, certificate weights, and the surface's internal `Column.channel`.
  - Deleting the accessors forces most of them out; S5 states where the count ends.
- **`Knots::lift` and `Weights::lift` (each carrier's `map_scalar`) trust their caller's lift contract.** They are defined at `Real`, which has no point reading. A lift that widened a value would leave a stale `f64` vector beside it. Every production lift keeps points.
- **The fit's collocation over a wide averaged knot (S5c).** Before S5c the fit refused a wide averaged knot before forming rows, and its row read one span of the hull. Now it admits one. A parameter that overlaps more than one span of the held knots gets a row hulled over every overlapped span's, with zeros outside a span's columns. Each span's piece is clamped to the `[0, 1]` its members' values lie in and is not divided by its sum (one in ℝ). The same enclosure row serves a wide parameter over point knots (`n = p + 1`). Every row is widened to the widest with exact zeros for the banded factor, whose pivots refuse typed where elimination over enclosures leaves one undecided. At a point parameter over point knots the rows are the `f64` ones, bit for bit. The wall's carrier refuses the wide knot at its door until S5z. How far the solve reaches is `the-fits-enclosure-solve-reaches-only-narrow-parameters` (P2).
- **The curve macro's interim reads report the macro invocation as their location.**

### Left for S3–S5 (S2's review, PR 5287)

- **A wide weight's `NonPositiveWeight` reports its top alone,** not its enclosure (the error's field is one `f64`).
- **The door's order of refusals differs by regime.** With a wide weight present, `Weights::new` certifies every weight before the count check (documented at `geom::net::weight_channel`).

### Left for S5 (S4)

- **The plan constructors decide over the held knots and weights (S5b).** Every carrier knot operation reaches them through `geom_core::spline::plan_inputs`. At point inputs they are the `f64` constructors over the inputs' own vectors, bit for bit. Over a wide input, a parameter's place (an insertion's, a refinement's, a removal's, a split's) is certified for every member, else `ParameterUndecided`; past a wide domain end every member is outside, so the refusal is the operation's own (`ParameterOutsideDomain`, `KnotNotPresent`). The plan is then decided against a stand-in private to the spline module (`Knots::plan_proxy`, each wide run at its bracket's midpoint, unit weights) and marked: the `f64` appliers and the projective arm refuse it (`StandInPlan`), the enclosure arm runs it over the held values. A written weight's sign is the carrier's door's, which certifies it. A removal's reinsertion reads the held breakpoint (`plan_inputs::breakpoint_at`), and a split's structure is decided there too (`split_multiplicity`, `split_structures`). `point_knots` stays for the knot-vector merge, which at a wide run decides across carriers, and for the surface's knot operations, which have not moved (their columns read the `f64` vector; S5d+). Readers: geom 218 → 212, geom-core 19 → 18.
- **The algebra's outputs at `Interval` stop at the door.** A rational net's refined weights are enclosures from rounding alone, so `make_compatible` refuses a rational section that needs refinement or elevation with `CarrierWeightNotAPoint`. A polynomial section keeps its unit weights exactly and is admitted.
- **The door does not refuse an unbounded pole** (the orchestrator's ruling, 2026-10-18, closing PR 5319). The entire interval is a legal enclosure, as pinned by `geom`'s `n2r1_probes::interval::probe_empty_versus_nai_versus_entire` and the n2r2 probes. An uncertified one is carried by its decoration, which S4's corner hull keeps. The knot algebra's enclosure branch keeps its own typed refusal of an output it wrote by dividing by a straddling ratio (`KnotAlgebraError::UnboundedAtEnclosure`).
- **`Dual` and `Sym` over `Interval` take the plain blend step.** Only `Interval` itself overrides `SpanLocate::homogeneous_step` with the corner hull, so a chain of blends still widens per step at those two scalars.
- **The surface's knot operations stay projective-only.** They have no caller at an enclosure scalar. Their column builds take the u knots through `point_knots`.

## Handoff (INLANE, 2026-10-19)

The INLANE lane stopped here at the orchestrator's call, with S5c merged and S5d not started. This section is the state a resuming lane picks up.

### Merged

| Slice | PR | What it did |
|---|---|---|
| S1, knots | 5277 | `KnotStructure` combinatorial; `Knots` holds wide breakpoints certified for every member; the interim accessors and their gate |
| S2, weights | 5287 | `Weights<T>` certified strictly positive; `weights_f64_interim`; the holders ratchet; `CarrierWeightNotAPoint` |
| S3, arcs | 5290 | the η sub-arc rule; arc sweep decisions certified (`ArcSweepUndecided`) |
| S4, the homogeneous knot algebra | 5309 | the ring applier total; `apply_homogeneous`; a carrier's knot operations at every scalar; `make_compatible` at `Interval` |
| S5 staging and the door ruling | 5321 | the S5 end state and staging (this item's Build section) |
| S5b, the plans over held values | 5322 | `spline::plan_inputs`: a parameter's place certified for every member (`ParameterUndecided`); a wide input's plan decided against a stand-in private to the spline module and marked |
| S5c, the fit's collocation | 5332 | the fit admits a wide averaged knot; enclosure rows hulled over the overlapped spans, each piece narrowed by `SpanLocate::clip_to` and clamped to `[0, 1]` |

S5a closed unmerged as PR 5319, by the door ruling below.

### Next

**S5d onwards: migrate the readers crate by crate.** Each crate is its own PR. In each, the interim accessors' readers move to the held values or the structure, or propagate a typed refusal (D4). The slices introduce the typed-refusing accessors (e.g. `knots_f64() -> Result<&KnotVector, CarrierKnotNotAPoint>`) and, beside the dying gate, a ratchet on `.unwrap()`, `.expect(` and panicking `?` uses of them.

Readers today (`scripts/gates/knots-f64-interim.counts`, 726 in all):

| crate | readers | holders |
|---|---|---|
| geom-brep | 222 | 15 |
| geom | 212 | 2 |
| sweep | 139 | — |
| step-import | 45 | — |
| mesh | 37 | — |
| topo | 36 | — |
| geom-core | 18 | 27 |
| step-export | 9 | — |
| editor-core | 6 | — |
| demos/tour | 2 | — |

Holders total 44. The typed accessors' unwrap/expect ratchet does not exist yet: S5d creates it, starting at its first crate's count, and S5z drives it to 0.

**S5z: the door lifts.**
- The carrier constructors (`NurbsCurve*::new`, `NurbsSurface::new`) admit wide knots and weights.
- The interim accessors and `knots-f64-interim.sh` go.
- Its PR cites C6's text and the `git log -S` provenance; text C6 does not already have goes back to the orchestrator.
- `the-f64-appliers-poison-where-the-projective-arm-refuses-typed` (P2) is owed by S5z.

**S6: the two fold-ins.**
- `held-value-equalities-decided-on-the-value-channel`
- `an-edges-parameter-ends-enter-a-carrier-piece-as-constants`

**The instrument unit:** `under-the-guided-lift-boss-unions-bool-pair-gap-decisions-swap-order-under-the-apart-move`.

### Rulings in force

- **S5's end state (the orchestrator, 2026-10-18).** The door lifts at S5z. Each panicking interim accessor becomes typed-refusing. A reader with a meaning at a wide carrier moves to the held values or the structure; a reader that means something only at points propagates the typed refusal and never swallows it. The unwrap/expect ratchet is driven to 0 by S5z.
- **No door check for an unbounded pole (PR 5319 closed).** An entire (`Dac`, unbounded) pole is a legal enclosure. An uncertified value carries D4's refusal by its decoration, so the door does not refuse it.
- **The S5b stand-in.** A plan over wide held inputs is decided against `Knots::plan_proxy` (each wide run at its bracket's midpoint, unit weights), which is private to the spline module. The plan is marked as no reference, and the `f64` appliers and the projective arm refuse it.
- **Every not-taken review finding gets its own work item** (the orchestrator, S5b and S5c).

### Open items this lane filed

P2:
- `the-f64-appliers-poison-where-the-projective-arm-refuses-typed`: the S5z type split, or a typed fallback.
- `the-fits-enclosure-solve-reaches-only-narrow-parameters`: correlation-keeping rows and a preconditioned or Krawczyk solve.
- `an-undecided-collocation-pivot-refuses-as-degenerate`: past the reach, a loft refuses `LsqDegenerate { pivot: NaN }` where it named the knot.

P3:
- `a-plan-step-carries-its-coefficient-twice`
- `a-stand-in-plan-refusal-is-unreachable`
- `breakpoint-at-refuses-as-a-removal-does`
- `a-wide-end-parameter-refuses-as-a-count-mismatch`
- `the-fit-rows-check-an-f64-chain-inside-an-enclosure-with-slack`
- `the-three-span-row-is-blind-to-an-unsound-end-clip`
- `a-miss-in-clip-to-answers-its-input-rather-than-poison`
- `sym-interval-takes-clip-to-as-the-identity`

### Practice the lane ran on

- Tier-H slices go to a concurrent pair on a frozen head. A unilateral MAJOR goes to a blinded coder, and one fix pass covers the union of both reports. Delta-reads happen when the orchestrator asks for them, and new logic in a fix pass has always drawn one.
- The row is the PR's last commit.
- Read the notification queue before merging, then merge on green.
- Reviewers share `CARGO_TARGET_DIR=/home/user/inlane-target`. A run can execute another worktree's binary, so a reviewer must confirm its probes are in the binary it ran.
- Disk is tight: keep the target dir clean and check `df` before dispatching.

## Evidence (JETFRAME, CLEAR, 2026-11-01)

Neither of the tour's rational walls builds at `Interval`. Both
`demo_tour::lily::leaf_a::<Interval>` and
`demo_tour::klein::bottle::<Interval>` refuse
`Skin(Structure(CarrierWeightNotAPoint { index: 1 }))`, so the
injectivity rows on their walls run at `f64` only.
`demos/tour/tests/wall_jets.rs` pins both refusals, and its rows go red
when this lands. The `Interval` twin there is the sweep crate's lens
loft (`a_lens_lofts_walls_hold_against_themselves_moved_off_the_origin_at_both_scalars`).

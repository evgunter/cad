---
id: the-splits-at-rest-gate-reads-tier-3-where-a-finished-body-owes-tier-3-prime
kind: issue
title: The split's at-rest gate reads tier 3, where a finished body owes tier 3′: a curved one-end slit ships Ok and tier 3′ refuses it
status: open
opened: 2026-10-19
priority: P1
cost: M
---


Found by the review of PR 5350 (`cleave/keel-slit-ring`).

## What

`crates/topo/README.md` tier 3 says a finished body (`AtRestBody`)
passes tier 3′ against its own contact records, and that every door
returning one pays that gate at the door that built it. Ev folded that
bar in (`fe281ddb39`, "the finished-body bar is tier 3' against the
body's own contacts (Ev's fold, measured)"). `split`'s gate,
`AtRestPolicy::gate_at_rest` (`crates/topo/src/splitting/mod.rs`,
`split`), runs `validate_geometric`, tier 3 only.

Reached on main: the curved keel open at one end
(`crates/sweep/tests/a_curved_keel_in_the_plane.rs`,
`a_curved_keel_in_the_plane_splits_sound_and_sections_with_no_spur`)
splits `Ok`. On every row measured (6 of 6), the side carrying the slit
then fails `validate_pseudomanifold` with no records: one
`UnrecordedContact { EdgeEdgeOverlap }` on the slit's two coincident arcs
(`a-curved-slit-fails-tier-3-prime-on-its-two-coincident-arcs`). PR 5350
refuses the closed form of the same keel typed (`CurvedSlitTrace`),
rather than ship it.

## Owed

The gate changes, not the doc: the doc's bar is Ev's. `split` runs
tier 3′ over each side against the records the split returns for it.
The curved slit's records (or its shared curve entry) come first, or the
one-end curved slit starts refusing at the gate. Measure what else the
3′ gate turns from `Ok` to a refusal across the split suites at three ε
before landing it.

A second witness: `a-hole-in-an-elliptic-section-keeps-a-face-of-its-own-that-tier-3-prime-refuses` (a hole left on a face of its own ships `Ok`; 3′ refuses).

## Slate triage (2026-10-26, CLEAVE)

- Re-banded P2 → P1: the Boolean pays tier 3′ at its door and the split does not, so the split ships finished bodies 3′ refuses: a special case that should be handled uniformly, P1 by `work/README.md` Priority.
- Moved to SPLITDOOR (`work/splitdoor/`) by `git mv`, id and body unchanged: the P1 track opened for split and boolean shapes that refuse, or ship a side their own gate fails, where a uniform reading would build: the split door's tier-3′ bar, tangent and pinch poses, and the curved-carrier lanes that read lines and circles only. SPLITDOOR holds its witnesses (the curved slit, the elliptic hole, the bore rim). CLEAVE measured 270.5 budget points against 30 and split on its priority seam (Ev, in chat, 2026-10-26).

## Evidence: planes through a cone's apex (2026-10-28, SPLITDOOR, PR 5956)

PR 5956 lets a plane through a cone's apex split the cone. On main every
pose below refused in the reduction. Every side that now ships measures at
its closed form, and the split's gate (tier 3) passes it, but tier 3′
refuses some of them. These are the probe tallies from
`crates/sweep/tests/review_5956_r1_probes.rs` and `review_5956_r2_probes.rs`,
which count tier 3′ findings and do not assert them:

- A cone revolved `Partial(w)` with w over a half-turn, whose two lobes
  either side of the plane meet at the apex, refuses `CensusUndecidable`.
  - 40° cone, apex up: w = π + 0.3: 4/6/6 sides at ε 1e-6/1e-9/1e-12;
    w = 4.0: 12; w = 4.5: 12; w = 5.5: 42; w = 5.8: 46/48/48.
  - r2's partials (45° apex up, w ∈ {1, 2, 3, 4, 5.5}): 20 sides at every ε.
  - 30° apex down, `Partial(4.5)`: 4 sides at every ε.
  - Windows of a half-turn or less: none.
- A 30° pit's apex-only touch (r2 `pit`) refuses
  `UnrecordedContact { VertexOnFace }`: 35 sides at every ε. Here the plane
  meets the pit only at its apex with material on both sides. The planar
  analogue, a notch corner touching the plane, does the same on main
  (r2's ignored `r2_planar_analogue_a_notch_corner_touching_the_plane`).
- Near-axial planes at 1e-12 refuse `CensusUnsupported`:
  - the spinning top (a cone on a cylinder, r1): 40 sides;
  - the pit: 14 sides;
  - r2's cone on a cylinder: 3 sides.

The full cones (30°, 45°, 60°, 10°, 80°, apex up and down) pass tier 3′ on
every landed side at all three ε.

## Measured (2026-10-28, SPLITDOOR, branch `splitdoor/split-gate-3prime` fefc1eea99)

Measurement only: `split` gated each side at tier 3′ against the records the split returns.

- Rows run: debug, the ten crates that depend on topo, `--no-fail-fast`. The 1e-9, 1e-6 and 1e-12 rows are complete (8619 tests each). editor-core was filtered to its split, corpus and pinch tests (1715): 1e-9 and 1e-6 are complete, 1e-12 was cut short at about 1100.
- Movers: 20 + 7 at 1e-9, 18 + 7 at 1e-6, 25 at 1e-12 (the second figure in each is editor-core).

**Premise corrections.**
- `split` returns no per-side contact records. `SplitResult` holds only its pinches' `Coincidence` verdicts. Records exist only through the caller's `carry_contacts(operand_records, …)`, and the door takes an `AtRestBody`, which carries none.
- The Boolean door does not pay tier 3′ either. Its result meets tier 3 without the census (`BooleanBody` docs; parked on `work/reachhold/boolean-door-runs-the-census-over-its-result.md`). So this row's "the Boolean already pays it" is false today; the bar is DESIGN's for every finished body.

**Movers by cause:**
- **A. The operand's own records never reach the gate.** Sides are sound: `divided_cell_carry` and `one_record_carry` already pass tier 3′ with the carried records.
  - Payloads: `UnrecordedContact` of every kind; `CensusUndecidable MayOverlap` on a curved rod.
  - Tests: `divided_cell_carry` ×4, `one_record_carry` ×3, `review_4730_divided_carry_probes`, `census::split_wedge_tests`, sweep `a_carried_record_no_reading_covers` ×2.
  - Fix shape: the door takes the operand's records (or a finished body that carries them) and gates each side against what `carry_contacts` lands.
- **B. The split's own touches go unrecorded.** Filed as `the-splits-own-pinch-and-cut-copies-take-two-point-keys-and-no-record`.
- **W1. Curved slit, all three ε.** Matches its witness (missing record or one curve entry).
- **W2. Elliptic hole, all three ε.** Matches its witness. It is NOT a records problem: the side is wrong-shaped, and needs the nesting fix.
- **W3. Bore rim: no mover.** No suite test reaches it.
- **C. The census cannot settle a curved edge-edge count**, on sliver, graze, near-seam and apex poses. `CensusUnsupported` NotCertifiable. Filed on ATREST: `the-census-cannot-settle-a-curved-edge-edge-count-on-a-splits-sliver-sides`.
- **D. The census escalates in band, but the split types it as the kernel's `ResultNotAtRest`.** Filed: `a-split-side-whose-census-escalates-in-band-refuses-as-not-at-rest`.
- **E. 1e-12 only: several solids' curved faces lie within reach.** `CensusUndecidable MayOverlap`. This is the cross-solid curved census lane that also keeps the Boolean off tier 3′ (REACHHOLD's row).
  - Tests: `split_seam_vanishing_tilt`, and `wedge_end_doors::a_cut_turned_about_the_ruling_refuses_before_the_chord` (6 poses go Ok → Err).
  - That second test passes only because it accepts any `Err(_)`.

**Order.** "Records first, then the gate" does not hold: only W1 is a records fix.
1. Change the door to take the operand's records (A). This is a door-signature question, so `design: true`.
2. B and W1.
3. W2's nesting.
4. C, D and E, each fixed or typed and parked.
5. Flip the gate, then re-measure at three ε, including the unfiltered editor-core suite at 1e-12.

Cone-apex instances (pit apex-only touch, partial lobes, near-axial at 1e-12) were seen by PR 5956's review and belong to A–E above.

## Decided (2026-10-28, Ev on PR 5967)

All three of the designer pair's converged recommendations, as recommended:

1. **A finished body carries its records, at every door.** `AtRestBody` holds its contact records. Every door carries its operand's records through its own lineage, adds the records of the coincidences it decided, and gates each result at tier 3′. Caller-side `carry_contacts` goes away.
2. **The split's own touches.**
   - Crossings a cut mints on two edges recorded as one locus share one point key.
   - Coincident edges an op mints, straight or curved, are recorded, citing a `SplitOn` row.
3. **`census.rs` `ee_collinear_lane`'s shared-bounds exemption retires**, once the split and the Boolean's pinch and slit lanes emit those records.

`crates/topo/README.md` tier 3 states 1 and 2.

The order:
1. The type and door change.
2. The split takes records in, and crossings share keys.
3. The split records its own coincident edges, and the exemption comes out.
4. W2's nesting, then C, D and E.
5. Flip the gate and re-measure at three ε.

## Moved (2026-10-28, SPLITDOOR)

- Moved to HELDRECORDS (`work/heldrecords/`) by `git mv`, id and body unchanged: SPLITDOOR's seam cut put the records chain Ev ratified on PR 5967 on a track of its own (`work/README.md` Track size).

## Evidence for E (2026-10-28, SPLITDOOR `splitdoor/boolean-strut-order`)

A Boolean instance of E at ε = 1e-9, from PR 5968's review sweep
(`crates/sweep/tests/boolean_tangent_rim_review.rs`, run by name).

- The shape: a circle boss over `z ∈ [1, 2]` on the plate
  `[-2, 2]² × [0, 1]`, of radius 0.5, 1 or 1.7, one or two arcs, turned
  `t > 0` about x.
- The cut: A − B against the brick whose top face is tangent to the
  boss's foot rim at its vertex.
- The result: 48 bodies build at their closed-form volume and pass
  tiers 1–3, then fail tier 3′ over their own records with
  `CensusUndecidable { what: "a curved face of one is within reach of
  the other, …" }` (`census.rs` `CurvedWithinReach`). Which solids'
  faces those are was not read.
- Not seen in A ∩ B or A ∪ B, nor at `t < 0`.

## Evidence: a plane along a union pinch's face (2026-10-29, SPLITDOOR, branch `splitdoor/pinch-corner-split`)

Cause B, an operand pinch vertex on the section face. The L-prism ∪ the
tilted cube (`crates/topo/tests/split_through_a_union_pinch.rs`) keeps
two vertices at the pinch (1, 1, 1). Split by z = 1, the L's top plane,
either way up, both sides pass tier 3. The z ≤ 1 side fails tier 3′
against no records with one finding:

    UnrecordedContact { contact: VertexOnFace { vertex, face }, witness: "(1.0, 1.0, 1.0)", margin: Some(0.0) }

`SplitResult::coincidences` is empty. The plane leaves the reflex notch's
cone vertex one-sided, so `splitting::reduce` mints no null edge there
and records nothing: it pushes a `SplitOn` coincidence only for an ON
vertex with two or more runs (`reduce`, the `runs.len() >= 2` arm). The
vertex then sits on the side's section face, a touch with no record.
Measured at ε = 1e-9 for five cube tilts and for the three-cone variant
(a parallelepiped in the notch's gap). The fusion that branch adds
leaves it unchanged; it reproduces with the fusion switched off.

### In-band tilts at a fused pinch (measured by PR 5981's review)

The ignored sweep `crates/topo/tests/review_pinch_corner_split.rs`,
`review_sweep_normals_through_the_pinch`, run with `REVIEW_PERTURB=1` at
ε = 1e-9, splits through the corner at normals 1e-7 off the cube's
bottom-face plane, unit (1, 1.3, 0.7) either way. 12 sides fail tier 3′
against the union's records carried through the split with
`CensusUndecidable` ("they touch where a face of one, seen from the
touch, has corners or edges too nearly in line to trace at this
tolerance"):

| union | sides |
|---|---|
| three cones (the wedge in the notch's gap) | 4 (2 normals × ±; cones 3, kept 2) |
| three cones (the wedge in the gap above the top) | 4 (the same) |
| four cones | 4 (cones 4, kept 3) |

The review measured every one as a cell that refuses with the fusion
switched off. Tier 3 holds on each; the census cannot decide the touch
in band. Typed, not fixed here.

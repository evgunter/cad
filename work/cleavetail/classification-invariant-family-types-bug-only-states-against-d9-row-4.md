---
id: classification-invariant-family-types-bug-only-states-against-d9-row-4
kind: issue
title: BooleanError::ClassificationInvariant and its kin return typed errors for states D9 row 4 says MUST panic; which are bug-only is the D2 addendum's rows' call
status: open
opened: 2026-10-03
priority: P3
cost: M
---


Filed by the CLEAVE orchestrator (2026-10-03). Both designers on the
mints fork (`topo-mints-indeterminates-outside-the-funnel`) raised it.

D9's "a state that can only be a kernel bug MUST panic … downgrading one
to a typed error launders a bug into a supported outcome" (`a0781edfa`)
and row 4 of its taxonomy ("observable in a branch → `unreachable!`") sit
beside typed errors for bug-only states:
`BooleanError::ClassificationInvariant` (PR 4's skeleton, `305c8b1f1`,
which predates the restatement), `VolumeCorrupt` and similar, all ending
in `KERNEL_DEFECT_ENDING`. Mint site I2 ("the declared ladder returned
`Distinct`") is one of them.

`JoinDesync` is reachable by API misuse under the open S14 ruling, so a
typed error is right there.

`decide_invariant`'s "Corrupt-class typed error, never a panic" (#213,
`2d44ce70d`) covers integral-result backstops, whose firing is evidence
of a bug rather than a state that can only be one. That reading has not
been reconciled with D9 anywhere.

Owed: classify each variant in the family against the D2 addendum's rows,
then either panic or say why the state is reachable. PR 2928's body
claims "`unreachable!` (denied here)", but `Cargo.toml` has
`panic = "warn"` and `unreachable!` sits outside that family; correct the
claim wherever it is repeated.

## Evidence: one state, two D9 rows (PR 4096, 2026-10-06)

"An earlier run at the vertex carried a strut's corner to its copy" is
now typed in one lane and a panic in its twin:

- `crates/topo/src/boolean/insert.rs` `corner_bound` returns
  `ClassificationInvariant` for it.
- `crates/topo/src/boolean/vtxfac.rs` `classify_vertex_on_face` (the
  bisector-run arm) and `crates/topo/src/splitting/insert.rs`
  `insert_null_edges` (the duplicate-run arm) prove it cannot happen
  and `unreachable!` on it: the runs there are maximal, and no other
  run's mint touches the corner's half.

`insert.rs` was reachable by input before JOIN's PR 4036 (56
`rc_wide_battery` poses). Since then `mint_plans`' `keyed` sort mints a
shared vertex's struts before any fan, and the TOPO row's close says no
battery reaches the check. If that ordering is a proof, `corner_bound`'s
refusal is bug-only and belongs in row 4 with the other two. If it is
not, its comment should name the input that reaches it.

## Ruling for `CountDisagrees` (CLEAVE unit `point-in-solid-mints-outside-its-period-and-nappe-rows`, 2026-10-10; corrected 2026-10-11)

**`TorusRoots::CountDisagrees` is reachable by input, so it is typed,
not a panic. The cause is a kernel defect: the certified count is wrong.**
The shared quartic ladder (`solid_contain.rs` `depressed_quartic_roots`)
reads its count off `Δ`'s sign, and `Δ`'s `f64` rounding (3e-7 to 6e-6 in
`Δ/L¹¹`, against a 1e-8 band) is charged to no decision. Counted exactly,
the four quartics the half-angle lane hands it
(`torus_predicate_rows::the_ladder_disagrees_with_itself_on_a_lanes_coefficients`)
have 2, 4, 4 and 4 real roots. The factorization builds those counts; the
sign ladder certified 4, 2, 2, 2. A ray at `R = 1e6`, `r = 1e4` reaches it
too (`a_far_ray_reaches_the_count_disagreement`). The same defect also
gives silent wrong `Miss` answers that no rung sees. INSIDE's P1
`ray-torus-count-certifies-on-an-uncharged-discriminant` owns the fix.

- **C17** (`point_in_solid`'s torus arm): `PointInSolidError::RootCountDisagrees
  { face, certified, constructed }`. It blocks that ray (`confined`,
  `RayFault::Blocked`, through `torus_crossings`), since another ray or
  witness can settle the point. At every door it ends on its lever plus
  "this may indicate a kernel bug worth reporting", which is true.
- **C18** (`sphere_region` `ray`): D9 row 0. `CircleRoots::CountDisagrees`
  is gone, and the first-harmonic doors answer
  `circle_roots::FirstHarmonicRoots`, which has neither it nor `AtApex`.
- **`reduce.rs` `line_roots`** keeps `ClassificationInvariant` with the
  kernel-defect ending for `TorusRoots::CountDisagrees`, and that ending is
  right: the cause is the defect above. (The first version of this note
  suggested moving it. That is retracted.)

The same unit found the two wall-junction mints K-dependent rather than
bug-only. Both are reachable because `Tolerance` accepts any finite K > 1:
- C13 (sides disagree at an active junction) is reachable for K below
  2, or about 3.2 with radial misses, in
  `PointInSolidError::JunctionSidesDisagree`.
- C14 (both ends of a piece active) is reachable at K < 2. The parity there
  has a definite answer, and the arm gives it.

## Evidence: three near-tangent sites (branch `cleave/near-tangent-invariants`, 2026-10-10)

`near-tangent-pierce-poses-reach-three-classification-invariants`
measured three `ClassificationInvariant` sites reachable by valid input
(JOIN's near-tangent census, 28 800 runs per ε row: 345 at 1e-9, 108 at
1e-6, 552 at 1e-12). None was a bug-only state; each was an earlier
decision and a later one reading one question two ways. After that
branch:

- `boolean/vtxfac.rs` `pierce_germ_dir`, "pierce germ direction not
  uniquely within its sector": **retired** (row 0). The ray is the
  transition's own (`OutBound`, from the codes that made the sector a
  transition), so no reading can return both or neither.
- `boolean/vtxfac.rs`, "geometrically coplanar sector with
  definitely-distinct plane": the unglued pair's extent ball was
  centred at the world origin, so the offset rung read the tilt at the
  corner's distance from the origin while `bool_sector_coplanar` read
  it at the arm. The ball is now centred at the pierce point. It reaches
  to the sector's farther bound, and the unglued pair's tilt is read at
  that same reach, so the parallel rung reads the tilt's own margin,
  decided zero. The offset is `p`'s standing off two planes it lies on.
  `Distinct` is now bug-only: a row 4 candidate.
- `boolean/recl.rs` `require_same`, "geometrically-ON sector pair with
  definitely-distinct carriers": the same origin-centred ball, now
  centred at the corner and reaching over the region the coplanarity
  was read on (`ReadOn`).
  - For an all-On record, that region is `pair_search`'s
    `bool_faces_parallel` at the pair's arm: the parallel rung reads that
    margin, and `Distinct` is bug-only (row 4 candidate).
  - For a flanker or a wedge representative, the first sector's bounds
    read On at their reach, and a thin sector tilted about a bound
    reaches `Distinct` at that reach. That is now a typed refusal on the
    bounds' margin (`recl::unglued_tilt`). Its own `Ok` arm ("a sector
    bound read On departs definitely from the other's plane") is bug-only.

## Evidence: the fourth near-tangent site (branch `cleave/single-on-edge-record`, 2026-10-11)

`boolean/recl.rs` `resolve_edge_sector`, "single-on-edge crossing
without a flanking record": 36 runs of the `w345` prism at ε = 1e-12
(`single-on-edge-crossing-without-a-flanking-record-at-1e-12`), and the
census's only `ClassificationInvariant` left after PR 4581. Reachable by
valid input, not bug-only: the fold rule names the flanker nearly
coplanar with the reference, and `pair_search` reads that pair off its
planes' common line, which swings off the shared edge by the edge's
in-band offset over the planes' sine. The `Yes` cell now mints the
pair's record, as the edge-edge rule does, so that arm no longer
refuses. The arm left (a `Rule` or `NotRule` cell with no record of
its flanker) is reached by no census row, and the minting argument does
not hold there, because a flanker coplanar with the reference holds the
bound without the ray needing to lie in the reference. It is a row 4
candidate.

Two sibling lookups in `recl.rs`, both measured reachable by valid input
off the census's integer sixteenths (fix pass on the same branch):

- `resolve_bisector_graze`, "bisector-graze crossing without a twin
  record" (126 runs at ε = 1e-6 in the review's sweep): the lookup never
  tried the reference's other twin. It does now, and the arm left
  (no twin of either solid carries the record) is reached by no row
  measured.
- `resolve_edge_sector`, "on-edge event without a reference sector" (9
  runs at each of 1e-9 and 1e-12, `vee300 e0 a11.75`, `cp` order): an
  earlier event's germ rewrote the only record naming the bound. The
  reference is read off the codes as first read, which no germ
  rewrites; the arm left (no first-read record names a bound some
  record named) can only be a bug: a row 4 candidate.

A candidate the same mechanism could reach, unmeasured: `recl.rs`
`resolve_bisector_graze` reads its reference off `records` as they
stand (`find_ref_sector(records, …)`, ~:1544) and returns `Ok(None)`,
"not crossed", where none names the bisector. `mark_germ` (~:587)
rewrites every On of a germ record, by design so as to fold a grazing
bisector on the record's other side into that germ. Where the
bisector's ray and the germ's are read apart by `parallel_same_dir`
(~:719; since 2026-10-29 `sectors::parallel_same` passed
`RAY_COLLINEAR`, PR 5983), as the vee300 edges are, the graze is a second event whose
On an earlier germ consumed, and the `Ok(None)` is a silent skip, not
a resolution. Both On codes of one record lie On the other sector's
plane, so both lie on the pair's one common line to the band, and a
record crosses along one ray: the graze is most likely that germ read
twice. That fails only where the two Ons lie on opposite rays of the
line. Whether any census row reaches the arm after an earlier germ
consumed its On has not been instrumented.

## Evidence: the census of typed kernel-defect endings (ENCL, 2026-10-20)

The census in `work/encl/a-typed-kernel-defect-ending-may-launder-a-bug-d2-forbids.md`, on main 5df6e6c854, classifies the `BooleanError`
`Display` arms in `crates/topo/src/boolean/mod.rs` that append
`KERNEL_DEFECT_ENDING`. It leaves them to this row instead of filing a
second one. Each is class 2, "only a kernel bug reaches it", and
`likely`, unless marked otherwise:

- `SplitLineage` (~4819): the split records are the Boolean's own.
- `LineageRow` (~4823): `lineage::RowRefusal` is refused at append, on
  rows the op writes itself.
- `ResultVolumeImplausible` (~4937): **class 1, sure.** The public
  `topo::AtRestPolicy::gate_volume_backstop(op, a, b, result, band, tol)`
  (`props.rs`, re-exported from `lib.rs`) takes a caller's `result`,
  and `props.rs`'s test (~5355) plants one and gets this refusal. Only
  the Boolean's own call (`ops.rs` ~1005) is a kernel postcondition.
- `VolumeCorrupt { operand: None }` (~4953): **class 1, likely**,
  through the same public door, whose "result" is the caller's body.
  The Boolean's own call is the class-2 postcondition. The `Some` arm
  already names the file.

For both, the repair splits the public door's input case, which names
the caller's lever, from the internal postcondition, which panics.
Panicking on the variant as a whole would let input reach the panic.
- `EdgeInFace { Crossed | TwinIncident }` (~5001): a postcondition on
  the result's edges.
- `ContactContradicted` with a `Tangent` declaration (~4585): `unsure`.
  It is HELD under D10 (contact records), so it is classified there and
  not carried here.

The prose-ending arms (`JoinDesync`, `TornComponent`, `SeamOrientation`,
`ZipCorrespondence`) are in the census's second pass, unclassified.

## Slate triage (2026-10-26, CLEAVE)

- Moved to CLEAVETAIL (`work/cleavetail/`) by `git mv`, id and body unchanged: the track opened for CLEAVE's P3 and P4 tail: refusal stories, guards that cannot go red, latent soundness and hygiene. CLEAVE measured 270.5 budget points against 30 and split on its priority seam (Ev, in chat, 2026-10-26).

## The volume backstop left the operation path (2026-10-27)

PR 5842, under `[ev]` PR 5780 (the boolean's volume relations are a test oracle, `topo::test_support::volume_relations`), deleted the backstop: `BooleanError::{ResultVolumeImplausible, VolumeUnmeasured, VolumeCorrupt, VolumeUndecided}`, `BooleanDecision::VolumeBackstop`, `AtRestPolicy::gate_volume_backstop` and the `volume_backstop*` predicates are gone. Its `ResultVolumeImplausible` and `VolumeCorrupt` items, and the public door that made them class 1, are gone; the rest of the family stands.

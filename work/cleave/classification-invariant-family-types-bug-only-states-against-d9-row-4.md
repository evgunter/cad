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

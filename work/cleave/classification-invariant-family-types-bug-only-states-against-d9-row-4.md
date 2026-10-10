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
  it at the arm. Centred at the pierce point, the parallel rung reads
  that same margin at the same arm, and the offset is `p`'s standing
  off the two planes it lies on. `Distinct` is now bug-only: a row 4
  candidate.
- `boolean/recl.rs` `require_same`, "geometrically-ON sector pair with
  definitely-distinct carriers": the same origin-centred ball, now
  centred at the corner. The offset arm is bug-only for the same
  reason. The parallel arm is NOT shown bug-only: an all-On record says
  each bound departs within the band at its reach, and a thin sector
  tilted about its bisector satisfies that while its normals' cross at
  the arm reads definitely off. `vtxfac`'s coplanar lump has an arm for
  that state (the bounds' own margin, `bool_sector_coplanar` on the
  steeper bound); `recl` has none. Unmeasured: no census run reached
  it after the branch.

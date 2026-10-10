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

## Ruling for `CountDisagrees` (CLEAVE unit `point-in-solid-mints-outside-its-period-and-nappe-rows`, 2026-10-10)

**`TorusRoots::CountDisagrees` is reachable, so it is typed, not a panic.**
The shared quartic ladder (`solid_contain.rs` `depressed_quartic_roots`)
builds a root count its own sign ladder did not certify on coefficients an
input gives. The half-angle lane hands it such quartics for
`circle_wall_rows::a_graze_is_read_by_its_depth` and
`ellipse_rows::a_graze_is_read_by_its_depth`, with every sign decided past
the band. `torus_predicate_rows::the_ladder_disagrees_with_itself_on_a_lanes_coefficients`
pins four of them (certified 4 / built 2, and certified 2 / built 4). The
factorization's rounding is not charged to its band decisions, so the
agreement of the two counts is not a property of the f64 computation. No
ray×torus input has reached it.

- **C17** (`point_in_solid`'s torus arm): `PointInSolidError::RootCountDisagrees
  { face, certified, constructed }`, a limit of that ray's computation
  (`confined`, `RayFault::Blocked`), so another ray may settle.
- **C18** (`sphere_region` `ray`): row 0. `CircleRoots::CountDisagrees` is
  gone. Its only producer was the conversion from `TorusRoots` in
  `reduce.rs`'s line lane, which now matches `TorusRoots` itself, so no
  circle root door can answer it.
- **Left on this row:** `reduce.rs` `line_roots` still answers
  `TorusRoots::CountDisagrees` with `ClassificationInvariant` and the
  kernel-defect ending. By this ruling the state is reachable, so that
  ending is a false story there. The span lane's honest reading is
  probably `Uncertain`'s (localize and read against the face), which is a
  behaviour change in the reduction and was not made by this unit.

The same unit found the two wall-junction mints K-dependent rather than
bug-only. Both are reachable because `Tolerance` accepts any finite K > 1:
- C13 (sides disagree at an active junction) at K below a few, in
  `PointInSolidError::JunctionSidesDisagree`.
- C14 (both ends of a piece active) at K < 2, where the parity has a definite
  answer and the arm now gives it.

---
id: one-solid-holding-two-outer-shells-is-what-five-kernel-doors-produce
kind: issue
title: a solid holding several Outer shells is a state four kernel doors produce deliberately, answered at the document layer — tier 3 may not refuse the count
status: closed
priority: P1
cost: E
parent: ATREST-1
opened: 2026-09-20
closed: 2026-09-21
refs: [2977]
---


**This row is a RECORD, not work.** It exists because the measurement
that produced it is the most valuable thing ATREST-1 made, and it would
otherwise die with the branch that made it.

## What was measured

`docs/ATREST-1-SPEC.md`'s D-C stated, as a settled decision, that *each
solid has exactly one shell that classifies `Outer`, and every other
shell of that solid classifies `Void`*. ATREST-1 implemented exactly
that as tier-3 check 10 and pushed it. Hosted CI (run 35565008331,
`464244c93`; twelve `test (…)` jobs, all red) answered: **36 distinct
pinned rows refuse, and every one of them is a body a kernel verb
produces on purpose.** No failure mentioned `NegativeVolume` — check
7's per-solid half was green on the same run, so the measurement
isolates the claim.

**The id says five and the doors are FOUR.** The id is stable and stays
as it is; the count below is the corrected one, and what follows the
list says which two rows were struck and why.

The four doors, named by the rows that pin them:

- **`graft onto`** — `topo/tests/graft_disjoint.rs`'s
  `the_onto_door_fuses_into_one_solid_without_changing_the_census`.
  The row's own name says the door fuses two disjoint bodies into ONE
  solid.
- **The boolean coplanar split** — `m3_pr3_split::notched_block_end_to_end`
  and `bool1_r1_probes::coplanar_split_e2e_volume_and_watertight`
  produce one solid with THREE definitely-positive shells, and
  `bool1_r2_probes::two_successive_coplanar_splits_stay_tier3` pins
  that tier 3 accepts it.
- **`subtract`** —
  `m5_s12_curved_ops::subtract_makes_a_through_hole_and_a_two_shell_complement`.
- **The editor's placed union** — `lib_placedunion::the_fin_group_is_one_node_and_one_body`
  and `a_circular_group_places_around_a_datum_axis`.
The strongest of the four is the coplanar split, which asserts three
shells under one solid in so many words.

**Two rows were listed here and are NOT evidence.** They were struck on
2026-09-22 rather than silently dropped, so that a later reader does not
re-derive them from their names:

- `shell5_r2_probes::r2_the_new_door_mints_a_solid_with_no_outer_shell`
  mints a solid with **zero** `Outer` shells. That is evidence about
  "exactly one" and never about "several `Outer` is legitimate" — and
  ATREST-1 makes tier 3 REFUSE that body, so citing it as a door whose
  product must be admitted argues against the change that cites it.
- `shell8_r1_probes::r1_the_roles_read_is_per_hollow_solid` counts
  `chk_shell_volume_sign` verdicts through `topo::shell` and never calls
  `validate_geometric`, so it demonstrates nothing about what tier 3
  admits.

## Why the count is not tier 3's to refuse

The question already has a home, one layer up and with a different
posture. `crates/editor-core/src/checks.rs`'s `CheckId::Connectedness`
counts `Outer` shells over a product and reports a **finding** against
an authored `expected_components` number — and it is allowed to offer
`error` at all only because it ships that per-finding acknowledgment
record, which is DS6's waiver rule as an `iff` (DISCIPLINES-DESIGN,
round 3, Ev). `dsc_checks::disjoint_union_is_one_finding` and
`a_deliberately_disjoint_body_draws_no_separation_finding` pin that a
deliberate disjoint union is a VALID body that draws a document-level
finding.

So the tree's position is: *a solid may hold several material
components; how many a product SHOULD have is the document's business,
stated as data by the author.* Tier 3 asserting the opposite would make
tier 3 wrong, not the five doors. The claim was dropped on that ground
by the ATREST orchestrator (2026-09-21) — no ruling from Ev needed,
because the posture is already ratified where it lives.

## What survives

`crates/topo/src/tier3_tests.rs`'s
`a_solid_holding_several_outer_shells_still_certifies` is the
executable form of this row: it builds the three-shell solid, pins its
premise through `topo::classify_shells` (two `Outer`, one `Void`) and
through the shells' own extents (the island nested in the cavity nested
in the wall), and asserts tier 3 admits it — so a count-level refusal
put back at this tier reds there, and the row cannot drift into being
about some other shape.

What IS still unchecked is the NESTING —
`work/atrest/tier-3-does-not-check-shell-roles-per-solid`, re-stated
to its real subject.

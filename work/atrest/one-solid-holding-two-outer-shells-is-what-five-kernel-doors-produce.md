---
id: one-solid-holding-two-outer-shells-is-what-five-kernel-doors-produce
kind: issue
title: check 10 cannot land as specced: one solid holding two outer shells is what graft-onto, the boolean split, the placed union and two shell doors produce today, on 36 pinned rows
status: open
priority: P0
cost: H
parent: ATREST-1
refs: [2977]
opened: 2026-09-20
---

## The finding

ATREST-1 implemented check 10 as `docs/ATREST-1-SPEC.md` D-C states it
— *each solid has exactly one shell that classifies `Outer`* — and
hosted CI measured what that claim costs on this tree: **36 distinct
pinned rows go red, and every one of them is a body the kernel's own
verbs produce on purpose.** CI run 35565008331 on `464244c93`
(12 `test (…)` jobs, all failing on this and nothing else; the k-lint
release rows, the python suite, the render lanes and the viewer
display-budget rows fail on the same bodies through the demo tour).

Five doors, named by the rows that pin them:

- **`graft onto`** — `topo/tests/graft_disjoint.rs`'s
  `the_onto_door_fuses_into_one_solid_without_changing_the_census`.
  The door's own row name says it fuses two disjoint bodies into ONE
  solid.
- **The boolean coplanar split** — `m3_pr3_split::notched_block_end_to_end`
  and `bool1_r1_probes::coplanar_split_e2e_volume_and_watertight`
  produce one solid with THREE definitely-positive shells;
  `bool1_r2_probes::two_successive_coplanar_splits_stay_tier3` pins
  that tier 3 accepts it.
- **`subtract`** —
  `m5_s12_curved_ops::subtract_makes_a_through_hole_and_a_two_shell_complement`.
  The same shape `work/bool/subtract-of-a-hollow-operand-files-the-island-under-one-solid`
  filed.
- **The editor's placed union** — `editor-core/tests/lib_placedunion.rs`'s
  `the_fin_group_is_one_node_and_one_body` and
  `a_circular_group_places_around_a_datum_axis`.
- **The shell doors** —
  `shell5_r2_probes::r2_the_new_door_mints_a_solid_with_no_outer_shell`
  (a solid with NO outer shell, deliberately) and
  `shell8_r1_probes::r1_the_roles_read_is_per_hollow_solid`.

## Why this is a design call and not a lane's fix

The tree already has a home for the claim, one layer up and with a
different posture. `editor-core/src/checks.rs`'s
`CheckId::Connectedness` counts `Outer` shells over a product and
reports a **finding** against a per-document expectation — and
`dsc_checks::disjoint_union_is_one_finding` and
`a_deliberately_disjoint_body_draws_no_separation_finding` pin that a
deliberate disjoint union is a valid body that draws a document-level
finding, not an at-rest refusal.

So the tree's ratified position today is: *a solid may hold several
material components; how many a product should have is the document's
business.* D-C states the opposite, at tier 3, as a refusal. One of the
two has to move, and which one moves is a decision above any single
lane:

1. **Tier 3 keeps the claim** and the five doors are fixed to file each
   material component under its own solid. That is the direction
   `work/bool/subtract-of-a-hollow-operand-files-the-island-under-one-solid`
   already points, but it is a cross-program change (topo's graft, the
   boolean engine, editor-core's union, two shell doors) with 36 pinned
   rows to re-argue, not to re-baseline.
2. **Tier 3 does not make the claim** and the SHELL-5 row's defect is
   re-stated as a connectedness question owned by the document layer,
   where a door already answers it.
3. **A narrower claim** that separates the shapes: a solid holding an
   outer shell INSIDE another of its own shells' cavity (SHELL-5's
   actual subject) is not the same as a solid holding two disjoint
   outer shells (a union's ordinary output). Tier 3 could refuse the
   first and admit the second — but that is a NESTING read, which tier
   3 has no at-rest walk for
   (`check-10-states-one-outer-and-the-rest-void-but-not-that-a-void-lies-inside-it`).

Option 3 is the one that matches what
`tier-3-does-not-check-shell-roles-per-solid` actually describes — *an
`Outer`, a `Void`, and a second `Outer` **inside that void*** — and it
is the one the residue row above says tier 3 cannot currently make.

## State

PR #2977 carries check 10 as specced, red on the rows above, so the
measurement is reproducible rather than described. Check 7's half of
ATREST-1 (the per-solid volume sign) is green on the same run and is
independent of this.

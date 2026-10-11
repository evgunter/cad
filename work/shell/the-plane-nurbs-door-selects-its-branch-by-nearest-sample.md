---
id: the-plane-nurbs-door-selects-its-branch-by-nearest-sample
kind: issue
title: offset_derive::plane_wall_section selects its branch by nearest sample, not by the certified tube holding a seed (branch_at)
status: closed
opened: 2026-10-10
closed: 2026-10-11
branch: shell/plane-nurbs-branch-at
pr: 4677
priority: P2
cost: M
refs: [a-wall-seam-between-two-fits-has-no-section]
---

Ev's ruling on `a-wall-seam-between-two-fits-has-no-section` (PR 4515,
its `## Decided`): "`offset_derive::plane_wall_section` moves from
nearest-sample selection to `branch_at` on the plane × NURBS door. That
door gets the same handle, in the arm's unit or the next." The arm's
unit built the handle for NURBS × NURBS (`geom_brep::ssi::Section`) and
left the plane × NURBS door as it was; this is the next unit.

Today `crates/topo/src/offset_derive.rs`, `plane_wall_section`, runs the
whole `plane_nurbs_ssi` (boundary pass, every branch, the accounting)
and keeps the branch whose 64-sample schedule passes nearest the old
edge's middle (`nearest_sample`): the selection is sampled, not proved.

Owed: a plane × NURBS `Section`-shaped handle (`of`, `branch_at`,
`all`) over the existing lane (`ssi::boundary`, `ends::Ends`, the chart
tube), `plane_nurbs_ssi` becoming `all()`, and `plane_wall_section`
calling `branch_at(mid)`, the answer the branch whose certified chart
tube holds the settled seed. Re-baseline the rows that read the plane
door's refusals (`encl_curved_loft_shell`, `offd_r1_probes`).

## Closed

The plane × NURBS door has the handle Ev's ruling asks for:
`geom_brep::ssi::PlaneSection` (`ssi/plane.rs`), `of`, `branch_at` and
`all` under the NURBS × NURBS handle's contract, the boundary pass run
once per handle at the first reading that needs it. `branch_at`
answers what holds the settled seed (`SectionReading`): the branch
whose certified chart tube holds it, or the `Side` or `Corner` region
the pass reported. `plane_nurbs_ssi` is `PlaneSection::of(..)?.all()`.
`offset_derive::plane_wall_section` calls `branch_at` at the old edge's
middle:
- a branch is the edge's carrier;
- a `Side` region is the wall's boundary row there, extracted as
  `level_row` extracts a row (`wall_row`);
- a `Corner` region refuses `SectionVerdict::Corner`.

Both seeded doors report a middle that settles outside the neighbour's
window as `NoBranch`, the exact-row reading's verdict for the same
cause, and any other refusal of the trace as `FromMiddle`. The rules
the two handles share (settling a seed, the branch through it and its
tube check, the crossings a branch already ends at, the closed loops
from the seeds, the lazily run pass) are one module, `ssi/handle.rs`.

`all()`'s outcomes and bits are unchanged (a merge-base differential of
every `plane_nurbs_ssi` call geom-brep's suite makes, at 1e-9, 1e-6
and 1e-12). The door rows that moved:
- At 1e-12, `offd_r1_probes`'s loft rims now refuse `NoBranch`.
  Nearest-sample selection had derived the bottom rim from a partial
  branch that never reached the old middle, and refused the top rim on
  another branch's tube (`TubeNotOneArc`).
- The pair door's seam verdicts (`common::approx::assert_seam_verdict`)
  read `FromMiddle`, and `NoBranch` where they read `SeedOffDomain`.

Filed: `work/ssiedge/all-depends-on-the-branch-at-calls-before-it.md`.

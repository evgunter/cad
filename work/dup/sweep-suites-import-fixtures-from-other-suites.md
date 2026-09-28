---
id: sweep-suites-import-fixtures-from-other-suites
kind: issue
title: Twenty-two sweep suites import a fixture from another suite rather than from tests/common
status: open
opened: 2026-09-28
priority: P4
cost: D
---


## Finding

Measured by the `dup/b8-a` lane at merge base `c1b202b2e`, while it
closed `one-suite-owns-another-suites-certified-fixture-group`: that
row's shape (a suite reading its fixture out of another SUITE) is a
class in `crates/sweep/tests`, not one site.

Ev ruled (2026-09-27, `two-rules-disagree-on-when-a-fixture-leaves-a-suite`)
that `tests/common`'s narrowest-home rule governs. By that rule an item
two suites share lives in `tests/common`, and a suite that exports one
to another suite owns the other's fixture.

**Owners: seven suites. Consumers: twenty-two.**

- `verbs_shell` (`vessel`, `hollow_box`, `outer_and_void`,
  `two_void_box`, `tube`, `prism`, `cut`, `roles_by_solid`, `v`) ←
  `shell5_r1_probes`, `shell5_r2_probes`, `shell8_dump`,
  `shell8_multi_solid`, `shell8_r1_probes`, `shell8_r2_probes`,
  `shell9_r1_probes`, `shell9_r2_dump`, `shell9_r2_probes`,
  `shell9_rows`, `shell10_r1_probes`, `shell10_r2_cost`,
  `shell10_r2_dump`, `shell10_r2_probes`, `shell10_scoped_walks`;
- `shell9_rows` (`rows`, `print_rows`) ← `shell9_probe`,
  `shell10_r2_dump`, `shell10_scoped_walks`, `shell5_r1_dump`,
  `shell7_dump`, `shell8_dump`;
- `cert_m2r1_passes::corpus` ← `lane1_r2_probes`;
- `bitdump::dump` ← `review_arms2_r1_probes`;
- `review_contact_edge_must_carry_r1_probes` ← `contact_edge_must_carry`
  (inside one row);
- `torax_axial` (`torus_barrel`, `torus_belly`) and `spiric_rim`
  (`vessel_cavity`) ← `pis_arc_capped_poses`, by inline path.

**Instruments.** The first is a `git grep` of
`use (crate|super)::<module>::` over `crates/sweep/tests`, excluding
`common::` and the `*_common` homes. Its blind spot is a path spelled
inline without a `use`. A second pass over
`(crate|super)::<module>::<item>` outside comments and `use` lines
found the three inline-only consumers (`pis_arc_capped_poses`,
`shell5_r1_dump`, `shell7_dump`). Neither pass can see a glob
re-export through a third module (none found under `crates/sweep/tests`).

## Why filed rather than folded

Size. `verbs_shell`'s group alone is nine items read by fifteen
suites, and `shell9_rows` is a row table as well as a printer. Each
group needs the treatment `common::sphere_recut` got: decide what stays
together, write the new module's not-absorbed list, and plant it. That
is a unit, not a drive-by beside the one it was found from. Method
item 6 applies at each site. `v` in `verbs_shell` is a closed-form
volume, which `common/oracles.rs`'s rule may route differently from
the bodies.

## Why this row is on this slate

The ground is S-TCOST's and S-TINT's (`crates/sweep/tests/*`). The
question is where a shared fixture lives, which is this program's
subject, as the row it was found from was.

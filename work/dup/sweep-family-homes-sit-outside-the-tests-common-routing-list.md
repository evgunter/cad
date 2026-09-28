---
id: sweep-family-homes-sit-outside-the-tests-common-routing-list
kind: issue
title: sweep tests: shell7_common, shell8_common and mate2_common are shared homes the tests/common routing list does not name
status: open
opened: 2026-09-28
priority: P4
cost: M
design: true
---


## Finding

Found by the `dup/b9-b` lane at merge base `2adacbc0e`, closing
`sweep-suites-import-fixtures-from-other-suites`, whose census excluded
the `*_common` homes by design.

`crates/sweep/tests/common/mod.rs` opens with a routing rule — "These
are the places a `sweep` suite can share from", ending "so a further
home does not appear without one" — and Ev ruled (2026-09-27,
`two-rules-disagree-on-when-a-fixture-leaves-a-suite`) that its
narrowest-home rule governs. The list names `revolve_common`. It does
not name three more shared homes the `all` binary carries:

- `shell7_common.rs` — registered as a SUITE (`#[path]` in `all.rs`),
  read by `common/latitude_seam.rs` and nine suites
  (`revert_periodic_wrap`, `revert_plane_charts`, `shell7_r2_probes`,
  `shell7_seam_corner`, `shell9_probe`, `shell9_r1_probes`,
  `shell9_r2_dump`, `shell9_r2_probes`, `shell9_rows`);
- `shell8_common.rs` — registered as a suite, read by thirteen
  (`shell8_*`, `shell9_*`, `shell10_*`);
- `mate2_common/` — a helper tree declared beside `common` in `all.rs`,
  read by `curved_mergedoor`, `mate2_cyl_rest`, `mate2_r1_probes`,
  `mate2_r2_probes`, `r1_probes_m9_3`.

`common/latitude_seam.rs` reaches `shell7_common` by `crate::` path and
says why ("that tree is the SHELL-7 suites' own and is not a `common::`
module"), so a `common::` module now depends on a suite file.

**The question, and why `design: true`.** Two answers are viable. A
family home can be ruled a legitimate rung of the routing list (as
`revolve_common` already is) and listed with the rule for when one may
exist; or the three fold into `tests/common` modules the way the seven
suite exports did (`shell_operands`, `torus_walls`, …). The first keeps
a family's vocabulary beside the family; the second leaves one place to
look. Either is mechanical once chosen; the choice is the row.

Measured with `git grep -lE '(crate|super)::<home>\b' -- crates/sweep/tests`.

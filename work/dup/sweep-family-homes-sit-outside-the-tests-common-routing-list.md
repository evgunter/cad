---
id: sweep-family-homes-sit-outside-the-tests-common-routing-list
kind: issue
title: sweep tests: shell7_common, shell8_common and mate2_common are shared homes the tests/common routing list does not name
status: open
opened: 2026-09-28
priority: P4
cost: M
design: true
needs_ev: true
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

(`common/latitude_seam.rs` used to reach `shell7_common` by `crate::`
path, so a `common::` module depended on a suite file; PR 3371 cut that
edge by building its bodies through `sweep::test_support` and reading
vertices through `topo::readback::vertex_point`. No `common::` module
names a family home now.)

**The question, and why `design: true`.** Whether the three family
homes stay as named homes or fold into `common`. Two answers are
viable. A family home can be ruled a legitimate rung of the routing
list (as `revolve_common` already is) and listed with the rule for when
one may exist; or the three fold into `tests/common` modules the way
the suite exports did (`shell_operands`, `torus_walls`, …). The first keeps
a family's vocabulary beside the family; the second leaves one place to
look. Either is mechanical once chosen; the choice is the row.

Measured with `git grep -lE '(crate|super)::<home>\b' -- crates/sweep/tests`.

## Why this row is on this slate

The ground is S-TCOST's and S-TINT's (`crates/sweep/tests/*`), and
SHELL's for the two shell homes. The question is where a shared fixture
lives — the narrowest-home rule's reach — which is this program's
subject, as the row it was found from
(`sweep-suites-import-fixtures-from-other-suites`) was.

## The question for Ev, and the recommendation

Two designers weighed it independently and recommend the same end state: **no family homes.**

- `shell7_common`, `shell8_common`, `mate2_common` and `revolve_common` all fold into `tests/common`, into modules named for what they hold (body authoring, readers, checks, oracles), never for the unit that first needed them.
- Items with one reader go back to that suite. Surviving copies fold or carry the ``NOT `common::`` marker.
- The routing list's `revolve_common` bullet becomes one rule: `tests/common` is the binary's only shared tree, and a module in it is named for what it holds. A test or the existing suite/helper instrument keeps a second tree from appearing.

**Why.** The narrowest-home rule ranks homes by reach, and a family home's reach is identical to `common`'s, so the rule cannot tell them apart. None of the four is a family home today by its own readers:
- `shell7_common` has 2 of 9 readers from SHELL-7;
- `shell8_common` has 4 of 13 from SHELL-8;
- `mate2_common` has 3 of 5 from MATE-2;
- `revolve_common` has 8 of 33 from the revolve suites.

Two of the headers name readers that do not read them. `shell7_common.rs` and `shell8_common.rs` are also registered as `#[path]` suites with no tests.

**What is changed.** The `revolve_common` bullet and the list's framing are lane text, not a ruling (their commits are agent-written). Ev's 2026-09-27 ruling settled only that the narrowest-home rule governs, and this applies it. The follow-up is a mechanical fold, sequenced with the open `revolved`, planar-cap-finder and solid-walk rows that own several of these items.

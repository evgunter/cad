---
id: gated-marker-path-mount
kind: issue
title: a #[path]-mounted src test module derives a marker term that matches nothing, silently
status: open
opened: 2026-09-03
---


`scripts/ci-filter.py`'s `_suite_term` derives a `src/` marker's nextest
prefix from the FILE PATH — "the module path IS the file path"
(`scripts/ci-filter.py:1315`). That is false for a `#[cfg(test)]` module
mounted with `#[path = "…"]` from a sibling file, and the tree has three
such modules today:

| file | mounted from | real test-id prefix | prefix `_suite_term` would derive |
|---|---|---|---|
| `crates/topo/src/boolean/r1_probes.rs` | `crates/topo/src/boolean/solid_contain.rs:3491` | `boolean::solid_contain::r1_probes::` | `boolean::r1_probes::` |
| `crates/topo/src/boolean/torus_predicate_rows.rs` | `crates/topo/src/boolean/solid_contain.rs:3498` | `boolean::solid_contain::torus_predicate_rows::` | `boolean::torus_predicate_rows::` |
| `crates/topo/src/chart_region_r2_probes.rs` | `crates/topo/src/chart_region.rs:3400` | `chart_region::chart_region_r2_probes::` | `chart_region_r2_probes::` |

None of the three carries a marker today, so nothing is broken in the
tree. What is broken is the GUARD. A term that matches no test excludes
no test, so a marker on one of these files would leave its suite running
on every pull request while reading, in the file, as a gate — and
`scripts/gates/gated-suite-paths.sh` cannot see it: `--gated-check`
(`scripts/ci-filter.py:1489`) asks only whether the marked file contains
`#[test]` or `#[cfg(test)]`, never whether the derived prefix selects
anything. This is precisely the failure mode that gate's own header
argues it exists to make loud ("a marker sited where nothing reads it, a
file marked but holding no test … fail here too").

It is also the one direction `--gated-set` cannot catch. The nightly
re-take runs what it derives; a term matching nothing quietly shrinks the
re-take instead of reddening it, so the suite is neither gated on a PR
nor re-taken at night while the tree reports two green gates.

TCOST-9 hit this while gating TCOST-4's torus counterexample-search row,
which lives in `crates/topo/src/boolean/r1_probes.rs`. The workaround
there was to put the gated row in a file whose PATH matches its module
path (`crates/topo/src/boolean/solid_contain/r1_generic_poses.rs`, a
plain `mod` and no `#[path]`), which is correct for that row but is not
a fix for the class.

Two candidate fixes, both cheap and neither this unit's to choose:

1. **Resolve the mount.** `_all_rs_modules` already reads `#[path]`/`mod`
   pairs out of `tests/all.rs`; the same reader over `crates/<c>/src/**`
   would let `_suite_term` follow a `#[path]` mount to the module path
   the compiler actually gives the file.
2. **Make the silence loud.** Have `--gated-check` refuse a marker on any
   `src/` file that some other `src/` file mounts with `#[path]`, naming
   the mounting line — a two-line scan that costs no toolchain and turns
   the whole class into a red discipline row.

The sweep behind the table: `grep -rn '#\[path = "' crates/*/src
--include=*.rs`, seven hits — three prose mentions in
`crates/test-utils/src/source.rs`, the three above, and
`crates/mesh/src/lib.rs:285`, which mounts `../tests/common/witness_bodies.rs`
into the LIB. That last one is already loud rather than silent: a marker
there is scanned under the `tests/` shape, `crates/mesh/tests/all.rs`
declares no such module, and `--gated-check` reds. The pattern cannot
match a mount whose `#[path]` is written on a different line from the
`mod`, or one assembled by a macro; I found none of either.

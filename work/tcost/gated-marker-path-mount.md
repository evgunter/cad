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

## Re-homed (2026-09-04)

Moved from `work/issues/` to `work/tcost/` in the tracker-wide
re-home sweep of 2026-09-04 (Ev's direction, in-chat), which read every
open `work/issues/` file against every open program's `paths` and
against the code-quality K–X fences. Id, body and header are unchanged;
the directory is the claim (`work/README.md`). Any `## Home` section
above naming `work/issues/` is superseded by this line and is kept as
the record of why the file was parked there.

## Sized (2026-09-11), while closing its sibling

Sized against the code, not estimated: the sibling row
(`gated-marker-omits-sibling-helper-imports`) landed its arm in
`--gated-check` on 2026-09-11 and built the reader this one would use, so
the two candidate fixes can now be priced properly. **They are not the
same size, and they do not do the same thing** — the choice is Ev's, not
a lane's.

**Candidate 2, "make the silence loud": small, ~15 lines, no new
concept.** In `gated_check`, walk `crates/*/src/**` for
`#[path = "..."]` / `mod` pairs the way `_tests_sibling_files` now walks
`tests/all.rs`, resolve each mount's target, and red on a marker sited on
any file that another `src/` file mounts — naming the mounting line. The
regexes exist (`_ALL_RS_PATH_RE`, `_ALL_RS_MOD_RE`), the walk exists, and
the planter harness in `scripts/gates/gated-suite-paths.sh` takes one
more case the way it just took two. **What it buys**: the class becomes
a red discipline row instead of a silent one. **What it costs**: the
three files stay ungatable, so a row that belongs in one of them is
written where its path matches its module path instead — which is what
TCOST-9 did by hand for the torus counterexample-search row, and which
the row's own note calls correct for that row and not a fix for the
class.

**Candidate 1, "resolve the mount": larger, and it is the one with a
design question in it.** `_suite_term` would follow a `#[path]` mount to
the module path the compiler actually gives the file. The mechanical part
is comparable in size; what is not mechanical is that a `src/` mount can
nest (a mounted file may itself mount another) and that the mount's
module path depends on WHERE the mounting `mod` sits in its own file's
module tree — `solid_contain.rs:3491` mounts into
`boolean::solid_contain`, which the current reader gets right only
because the mounting file's path happens to name it. A reader that
resolves one level and calls it done would derive a wrong prefix on a
nested mount and go back to selecting nothing, silently: the same failure
this row is about, one level deeper. **What it buys**: markers work
everywhere, and the three files become gatable. **What it costs**: a
module-tree walk of `crates/*/src`, which is a real reader with real
edge cases, for three files today.

**Recommendation, unchanged from the row's own text (*"neither this
unit's to choose"*)**: take candidate 2 now — it is cheap, it converts
the whole class from silent to loud, and it is not thrown away if
candidate 1 ever lands. Take candidate 1 only when a row genuinely wants
gating in a mounted file and the workaround is worse than the reader.

Still true on the tree at 2026-09-11: none of the three files carries a
marker, so nothing is broken today. The guard is.
